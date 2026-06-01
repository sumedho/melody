use rand::rngs::StdRng;
use rand::Rng;

use super::common::{
    chord_at, chord_pitches_in_range, nearest_pitch_class, nearest_scale_pitch,
    scale_pitches_in_range, ticks_per_bar,
};
use super::{ChordEvent, DropType, GeneratorSettings, NoteEvent, PPQN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BuildupDropSections {
    pub drop_start: u32,
    pub silence_start: u32,
}

pub(crate) fn buildup_drop_sections(settings: &GeneratorSettings) -> BuildupDropSections {
    let bar_ticks = ticks_per_bar();
    let buildup_bars = (settings.bars as u32 / 2).max(1);
    let drop_start = buildup_bars * bar_ticks;
    let silence_start = if settings.bars >= 4 {
        drop_start.saturating_sub(bar_ticks / 2)
    } else {
        drop_start
    };

    BuildupDropSections {
        drop_start,
        silence_start,
    }
}

pub(crate) fn generate_buildup_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let sections = buildup_drop_sections(settings);
    let mut notes = Vec::new();

    generate_buildup(settings, chords, sections, rng, &mut notes);
    generate_riser(settings, sections, &mut notes);
    generate_impact(settings, chords, sections.drop_start, &mut notes);
    generate_drop_pattern(settings, chords, sections.drop_start, rng, &mut notes);

    let total_ticks = ticks_per_bar() * settings.bars as u32;
    notes.retain(|note| {
        note.start_ticks < total_ticks
            && note.duration_ticks > 0
            && (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)
    });
    notes.sort_by_key(|note| (note.start_ticks, note.pitch, note.duration_ticks));
    notes.dedup_by_key(|note| (note.start_ticks, note.pitch));
    notes
}

fn generate_buildup(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    sections: BuildupDropSections,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    let bar_ticks = ticks_per_bar();
    let step_ticks = PPQN as u32 / 4;
    let buildup_bars = (sections.drop_start / bar_ticks).max(1);
    let slots = [0_u32, 4, 8, 10, 12, 14];

    for bar in 0..buildup_bars {
        let bar_start = bar * bar_ticks;
        let available_slots = slots
            .iter()
            .copied()
            .filter(|slot| bar_start + slot * step_ticks < sections.silence_start)
            .collect::<Vec<_>>();
        let progress = (bar + 1) as f32 / buildup_bars as f32;
        let target_count = ((available_slots.len() as f32 * progress).ceil() as usize).max(1);

        for slot in available_slots.into_iter().take(target_count) {
            let start = bar_start + slot * step_ticks;
            let chord = chord_at(chords, start);
            let target = settings.low_pitch() as i32
                + ((settings.high_pitch() - settings.low_pitch()) as f32 * (0.45 + progress * 0.35))
                    as i32
                + rng.gen_range(-3..=3);
            notes.push(NoteEvent {
                pitch: buildup_pitch(settings, chord, target),
                start_ticks: start,
                duration_ticks: step_ticks,
                velocity: (62.0 + progress * 34.0) as u8,
            });
        }
    }
}

fn generate_riser(
    settings: &GeneratorSettings,
    sections: BuildupDropSections,
    notes: &mut Vec<NoteEvent>,
) {
    if sections.silence_start == 0 {
        return;
    }

    let riser_start = sections.silence_start.saturating_sub(ticks_per_bar());
    let step_ticks = PPQN as u32 / 4;
    let riser_steps = ((sections.silence_start - riser_start) / step_ticks).max(1);
    for index in 0..riser_steps {
        let start = riser_start + index * step_ticks;
        if start >= sections.silence_start {
            break;
        }
        let progress = index as f32 / riser_steps.saturating_sub(1).max(1) as f32;
        let target = settings.low_pitch() as i32
            + ((settings.high_pitch() - settings.low_pitch()) as f32 * progress) as i32;
        let duration_ticks = ((step_ticks as f32) * (1.4 - progress * 0.9)).round() as u32;
        notes.push(NoteEvent {
            pitch: nearest_scale_pitch(settings, target) as u8,
            start_ticks: start,
            duration_ticks: duration_ticks.clamp(1, step_ticks),
            velocity: (64.0 + progress * 55.0) as u8,
        });
    }
}

fn generate_impact(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    notes: &mut Vec<NoteEvent>,
) {
    let chord = chord_at(chords, drop_start);
    let bass = nearest_pitch_class(settings, settings.low_pitch(), &[chord.root]);
    notes.push(NoteEvent {
        pitch: bass,
        start_ticks: drop_start,
        duration_ticks: PPQN as u32 * 2,
        velocity: 124,
    });

    let stab_candidates =
        chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    for pitch in stab_candidates
        .into_iter()
        .filter(|pitch| *pitch > bass)
        .take(4)
    {
        notes.push(NoteEvent {
            pitch,
            start_ticks: drop_start,
            duration_ticks: PPQN as u32,
            velocity: 116,
        });
    }
}

fn generate_drop_pattern(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    match settings.drop_type {
        DropType::BassDrop => bass_drop(settings, chords, drop_start, rng, notes),
        DropType::SupersawDrop => supersaw_drop(settings, chords, drop_start, rng, notes),
        DropType::HalfTimeDrop => half_time_drop(settings, chords, drop_start, notes),
        DropType::FillDrop => fill_drop(settings, chords, drop_start, rng, notes),
        DropType::VocalDrop => vocal_drop(settings, chords, drop_start, rng, notes),
    }
}

fn bass_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    let step_ticks = PPQN as u32 / 4;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let pattern = [0_u32, 6, 8, 14];
    let mut bar_start = drop_start;
    while bar_start < total_ticks {
        for step in pattern {
            let start = bar_start + step * step_ticks;
            if start >= total_ticks {
                continue;
            }
            let chord = chord_at(chords, start);
            let pitch = if step == 0 || step == 8 {
                nearest_pitch_class(settings, settings.low_pitch(), &[chord.root])
            } else {
                nearest_pitch_class(
                    settings,
                    settings.low_pitch().saturating_add(7),
                    &[(chord.root + 7) % 12],
                )
            };
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: if step == 0 {
                    PPQN as u32
                } else {
                    step_ticks * 2
                },
                velocity: if step == 0 {
                    122
                } else {
                    104 + rng.gen_range(0..=12)
                },
            });
        }
        bar_start += ticks_per_bar();
    }
}

fn supersaw_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    let step_ticks = PPQN as u32 / 4;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut bar_start = drop_start;
    while bar_start < total_ticks {
        for beat in [0_u32, 4, 8, 12] {
            let start = bar_start + beat * step_ticks;
            if start >= total_ticks {
                continue;
            }
            let chord = chord_at(chords, start);
            for pitch in chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch())
                .into_iter()
                .rev()
                .take(3)
            {
                notes.push(NoteEvent {
                    pitch,
                    start_ticks: start,
                    duration_ticks: PPQN as u32 / 2,
                    velocity: 108,
                });
            }
        }
        for step in [2_u32, 6, 10, 14] {
            let start = bar_start + step * step_ticks;
            if start < total_ticks {
                notes.push(NoteEvent {
                    pitch: lead_pitch(settings, chords, start, rng),
                    start_ticks: start,
                    duration_ticks: step_ticks,
                    velocity: 98,
                });
            }
        }
        bar_start += ticks_per_bar();
    }
}

fn half_time_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    notes: &mut Vec<NoteEvent>,
) {
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut start = drop_start;
    while start < total_ticks {
        let chord = chord_at(chords, start);
        let pitch = nearest_pitch_class(settings, settings.low_pitch(), &[chord.root]);
        notes.push(NoteEvent {
            pitch,
            start_ticks: start,
            duration_ticks: PPQN as u32 * 2,
            velocity: 122,
        });
        start += PPQN as u32 * 2;
    }
}

fn fill_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    let step_ticks = PPQN as u32 / 4;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let pattern = [0_u32, 3, 5, 7, 10, 12, 15];
    let mut bar_start = drop_start;
    while bar_start < total_ticks {
        for step in pattern {
            let start = bar_start + step * step_ticks;
            if start < total_ticks {
                notes.push(NoteEvent {
                    pitch: if step == 0 {
                        nearest_pitch_class(
                            settings,
                            settings.low_pitch(),
                            &[chord_at(chords, start).root],
                        )
                    } else {
                        lead_pitch(settings, chords, start, rng)
                    },
                    start_ticks: start,
                    duration_ticks: step_ticks,
                    velocity: if step == 0 {
                        122
                    } else {
                        92 + rng.gen_range(0..=18)
                    },
                });
            }
        }
        bar_start += ticks_per_bar();
    }
}

fn vocal_drop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    drop_start: u32,
    rng: &mut StdRng,
    notes: &mut Vec<NoteEvent>,
) {
    let step_ticks = PPQN as u32 / 4;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let pattern = [0_u32, 1, 3, 6, 8, 9, 12, 14];
    let mut bar_start = drop_start;
    while bar_start < total_ticks {
        for step in pattern {
            let start = bar_start + step * step_ticks;
            if start >= total_ticks {
                continue;
            }
            notes.push(NoteEvent {
                pitch: vocal_chop_pitch(settings, chords, start, rng),
                start_ticks: start,
                duration_ticks: step_ticks / 2,
                velocity: 96 + rng.gen_range(0..=18),
            });
        }
        bar_start += ticks_per_bar();
    }
}

fn buildup_pitch(settings: &GeneratorSettings, chord: &ChordEvent, target: i32) -> u8 {
    chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch())
        .into_iter()
        .min_by_key(|pitch| (*pitch as i32 - target).abs())
        .unwrap_or_else(|| nearest_scale_pitch(settings, target) as u8)
}

fn lead_pitch(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    start: u32,
    rng: &mut StdRng,
) -> u8 {
    let chord = chord_at(chords, start);
    let target = settings.high_pitch().saturating_sub(rng.gen_range(0..=12)) as i32;
    buildup_pitch(settings, chord, target)
}

fn vocal_chop_pitch(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    start: u32,
    rng: &mut StdRng,
) -> u8 {
    let chord = chord_at(chords, start);
    let upper_floor = settings
        .low_pitch()
        .saturating_add((settings.high_pitch() - settings.low_pitch()) / 2);
    let mut candidates = chord_pitches_in_range(chord, upper_floor, settings.high_pitch());
    if candidates.is_empty() {
        candidates = scale_pitches_in_range(settings)
            .into_iter()
            .filter(|pitch| *pitch >= upper_floor)
            .collect();
    }
    candidates
        .get(rng.gen_range(0..candidates.len().max(1)))
        .copied()
        .unwrap_or_else(|| settings.high_pitch())
}
