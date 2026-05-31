use std::collections::HashMap;

use crate::generator::{NoteEvent, PPQN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridLine {
    Bar,
    Beat,
    Step,
}

#[derive(Debug, Clone)]
pub struct PreviewNoteIndex {
    starts: HashMap<(u8, u32), NoteSegment>,
    occupied_by: HashMap<(u8, u32), u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteSegment {
    pub velocity: u8,
    pub span_steps: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewStep {
    Empty,
    NoteStart(NoteSegment),
    NoteContinuation,
}

impl PreviewNoteIndex {
    pub fn new(notes: &[NoteEvent], bars: u16, low_pitch: u8, high_pitch: u8) -> Self {
        let step_ticks = PPQN as u32 / 4;
        let total_steps = bars as u32 * 16;
        let mut starts: HashMap<(u8, u32), NoteSegment> = HashMap::new();
        let mut occupied_by: HashMap<(u8, u32), u32> = HashMap::new();
        let mut candidates: Vec<(u8, u32, u32, u8)> = notes
            .iter()
            .filter_map(|note| {
                if !(low_pitch..=high_pitch).contains(&note.pitch) || note.duration_ticks == 0 {
                    return None;
                }

                let start_step = note.start_ticks / step_ticks;
                if start_step >= total_steps {
                    return None;
                }

                let end_tick = note.start_ticks.saturating_add(note.duration_ticks);
                let end_step = end_tick.div_ceil(step_ticks).min(total_steps);
                let span_steps = end_step.saturating_sub(start_step).max(1);
                Some((note.pitch, start_step, span_steps, note.velocity))
            })
            .collect();
        candidates.sort_by_key(|(pitch, start_step, _, _)| (*pitch, *start_step));

        for (pitch, start_step, span_steps, velocity) in candidates {
            let key = (pitch, start_step);
            if starts.contains_key(&key) {
                let segment = starts.get_mut(&key).expect("checked above");
                segment.velocity = segment.velocity.max(velocity);
                if span_steps > segment.span_steps {
                    segment.span_steps = span_steps;
                    for step in start_step..start_step + span_steps {
                        occupied_by.insert((pitch, step), start_step);
                    }
                }
                continue;
            }

            if occupied_by.contains_key(&key) {
                continue;
            }

            starts.insert(
                key,
                NoteSegment {
                    velocity,
                    span_steps,
                },
            );
            for step in start_step..start_step + span_steps {
                occupied_by.insert((pitch, step), start_step);
            }
        }

        Self {
            starts,
            occupied_by,
        }
    }

    pub fn step_at(&self, pitch: u8, step: u32) -> PreviewStep {
        if let Some(segment) = self.starts.get(&(pitch, step)) {
            PreviewStep::NoteStart(*segment)
        } else if self.occupied_by.contains_key(&(pitch, step)) {
            PreviewStep::NoteContinuation
        } else {
            PreviewStep::Empty
        }
    }
}

pub fn grid_line_for_step(step: u32) -> GridLine {
    if step % 16 == 0 {
        GridLine::Bar
    } else if step % 4 == 0 {
        GridLine::Beat
    } else {
        GridLine::Step
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::ticks_per_bar;

    #[test]
    fn sustained_notes_are_single_segment() {
        let notes = vec![NoteEvent {
            pitch: 60,
            start_ticks: 0,
            duration_ticks: ticks_per_bar(),
            velocity: 90,
        }];
        let index = PreviewNoteIndex::new(&notes, 1, 60, 72);

        assert_eq!(
            index.step_at(60, 0),
            PreviewStep::NoteStart(NoteSegment {
                velocity: 90,
                span_steps: 16
            })
        );
        assert_eq!(index.step_at(60, 1), PreviewStep::NoteContinuation);
    }

    #[test]
    fn adjacent_notes_remain_separate_starts() {
        let step_ticks = PPQN as u32 / 4;
        let notes = vec![
            NoteEvent {
                pitch: 60,
                start_ticks: 0,
                duration_ticks: step_ticks,
                velocity: 70,
            },
            NoteEvent {
                pitch: 60,
                start_ticks: step_ticks,
                duration_ticks: step_ticks,
                velocity: 105,
            },
        ];
        let index = PreviewNoteIndex::new(&notes, 1, 60, 72);

        assert_eq!(
            index.step_at(60, 0),
            PreviewStep::NoteStart(NoteSegment {
                velocity: 70,
                span_steps: 1
            })
        );
        assert_eq!(
            index.step_at(60, 1),
            PreviewStep::NoteStart(NoteSegment {
                velocity: 105,
                span_steps: 1
            })
        );
    }

    #[test]
    fn duplicate_starts_keep_max_velocity_and_longest_span() {
        let step_ticks = PPQN as u32 / 4;
        let notes = vec![
            NoteEvent {
                pitch: 60,
                start_ticks: 0,
                duration_ticks: step_ticks,
                velocity: 70,
            },
            NoteEvent {
                pitch: 60,
                start_ticks: 0,
                duration_ticks: step_ticks * 3,
                velocity: 105,
            },
        ];
        let index = PreviewNoteIndex::new(&notes, 1, 60, 72);

        assert_eq!(
            index.step_at(60, 0),
            PreviewStep::NoteStart(NoteSegment {
                velocity: 105,
                span_steps: 3
            })
        );
    }

    #[test]
    fn out_of_range_notes_are_ignored() {
        let notes = vec![NoteEvent {
            pitch: 84,
            start_ticks: 0,
            duration_ticks: PPQN as u32,
            velocity: 100,
        }];
        let index = PreviewNoteIndex::new(&notes, 1, 60, 72);

        assert_eq!(index.step_at(84, 0), PreviewStep::Empty);
    }

    #[test]
    fn notes_extending_past_visible_range_are_clipped() {
        let notes = vec![NoteEvent {
            pitch: 60,
            start_ticks: ticks_per_bar() - PPQN as u32 / 4,
            duration_ticks: PPQN as u32,
            velocity: 100,
        }];
        let index = PreviewNoteIndex::new(&notes, 1, 60, 72);

        assert_eq!(
            index.step_at(60, 15),
            PreviewStep::NoteStart(NoteSegment {
                velocity: 100,
                span_steps: 1
            })
        );
    }

    #[test]
    fn grid_line_marks_bars_beats_and_steps() {
        assert_eq!(grid_line_for_step(0), GridLine::Bar);
        assert_eq!(grid_line_for_step(4), GridLine::Beat);
        assert_eq!(grid_line_for_step(3), GridLine::Step);
    }
}
