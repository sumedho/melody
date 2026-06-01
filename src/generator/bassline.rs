use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::{STEPS_PER_BEAT, UKG_SWING_FACTOR};

use super::common::{
    chord_at, note_duration, pitch_class_for_degree, rhythm_density, scale_pitches_in_range,
    ticks_per_bar, velocity_for,
};
use super::{BasslineStyle, ChordEvent, GeneratorSettings, NoteEvent, PPQN};

const BASS_REGISTER_SPAN: u8 = 24;

pub(crate) fn generate_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    match settings.bassline_style {
        BasslineStyle::Techno => generate_techno_bassline(settings, chords, rng),
        BasslineStyle::House => generate_house_bassline(settings, chords, rng),
        BasslineStyle::Drill => generate_drill_bassline(settings, chords, rng),
        BasslineStyle::HipHop => generate_hiphop_bassline(settings, chords, rng),
        BasslineStyle::UkGarage => generate_uk_garage_bassline(settings, chords, rng),
        BasslineStyle::DrumAndBass => generate_drum_and_bassline(settings, chords, rng),
    }
}

pub(crate) fn generate_techno_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let total_steps = settings.bars as u32 * 16;
    let mut previous_was_rest = true;
    let mut previous_pitch = None;

    for step in 0..total_steps {
        let start = step * step_ticks;
        let beat_step = step % 16;
        let base_probability = rhythm_density(settings) as i16;
        let downbeat_bonus = if beat_step == 0 || beat_step == 8 {
            18
        } else {
            0
        };
        let syncopation_bonus = if matches!(beat_step, 3 | 6 | 10 | 14) {
            settings.bassline_mutation as i16 / 4
        } else {
            0
        };
        let active_probability =
            (base_probability + downbeat_bonus + syncopation_bonus).clamp(0, 100);

        if rng.gen_range(0..100) >= active_probability {
            previous_was_rest = true;
            continue;
        }

        let chord = chord_at(chords, start);
        let pitch = choose_bassline_pitch(settings, chord, step, rng);
        let accented = is_bassline_accented(settings, beat_step, previous_was_rest, rng);
        let sliding = should_bassline_slide(settings, previous_pitch, pitch, rng);
        let duration_ticks = if sliding {
            ((step_ticks as f32) * 1.35).round() as u32
        } else {
            note_duration(settings, step_ticks, rng).min(step_ticks)
        };
        let velocity = if accented {
            116
        } else {
            velocity_for(settings, start, rng).min(96)
        };

        notes.push(NoteEvent {
            pitch,
            start_ticks: start,
            duration_ticks: duration_ticks.max(1),
            velocity,
        });

        previous_was_rest = false;
        previous_pitch = Some(pitch);
    }

    notes
}

pub(crate) fn generate_house_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let pattern = [2_u32, 4, 6, 10, 12, 14];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if step != 2 && rng.gen_range(0..100) > rhythm_density(settings) {
                continue;
            }
            let start = bar * ticks_per_bar() + step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = match step {
                2 | 10 => 0,
                4 | 12 => 2,
                6 | 14 => 4,
                _ => 0,
            };
            let pitch = choose_bass_degree_pitch(settings, chord, degree, rng);
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if matches!(step, 2 | 10) {
                    108
                } else {
                    velocity_for(settings, start, rng)
                },
            });
        }
    }

    notes
}

pub(crate) fn generate_drill_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let total_bars = settings.bars as u32;
    let pattern = [0_u32, 6, 11, 16, 24, 30, 42, 48, 54, 59];

    for bar_group in (0..total_bars).step_by(4) {
        for step in pattern {
            let absolute_step = bar_group * 16 + step;
            if absolute_step >= total_bars * 16 {
                continue;
            }
            if step % 16 != 0 && rng.gen_range(0..100) > rhythm_density(settings) + 10 {
                continue;
            }

            let start = absolute_step * step_ticks;
            let chord = chord_at(chords, start);
            let slide_pick =
                matches!(step, 11 | 30 | 54) && rng.gen_range(0..100) < settings.bassline_slide;
            let degree = if slide_pick { 2 } else { 0 };
            let mut pitch = choose_bass_degree_pitch(settings, chord, degree, rng);
            if slide_pick {
                pitch = pitch.saturating_add(3).min(settings.high_pitch());
            }
            let duration = if slide_pick {
                step_ticks * 3
            } else {
                note_duration(settings, step_ticks * 4, rng).max(step_ticks * 2)
            };
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: duration,
                velocity: if slide_pick { 118 } else { 102 },
            });
        }
    }

    notes
}

pub(crate) fn generate_hiphop_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let pattern = [0_u32, 7, 12, 22, 32, 38, 44, 55];

    for bar_group in (0..settings.bars as u32).step_by(4) {
        for step in pattern {
            let absolute_step = bar_group * 16 + step;
            if absolute_step >= settings.bars as u32 * 16 {
                continue;
            }
            if step != 0 && rng.gen_range(0..100) > rhythm_density(settings) + 5 {
                continue;
            }

            let start = absolute_step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = if rng.gen_range(0..100) < settings.bassline_mutation {
                4
            } else {
                0
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 4, rng).max(step_ticks * 2),
                velocity: velocity_for(settings, start, rng).max(86),
            });
        }
    }

    notes
}

pub(crate) fn generate_uk_garage_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let swing_ticks = ((step_ticks as f32) * UKG_SWING_FACTOR).round() as u32;
    let pattern = [0_u32, 5, 7, 10, 13, 15];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if !matches!(step, 0 | 7 | 13) && rng.gen_range(0..100) > rhythm_density(settings) {
                continue;
            }
            let unswung = bar * ticks_per_bar() + step * step_ticks;
            let start = if step % 2 == 1 {
                unswung + swing_ticks
            } else {
                unswung
            };
            let chord = chord_at(chords, start);
            let degree = match step {
                5 | 13 => 4,
                7 | 15 => 2,
                _ => 0,
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if step % 2 == 1 { 110 } else { 94 },
            });
        }
    }

    notes
}

pub(crate) fn generate_drum_and_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let pattern = [0_u32, 3, 7, 10, 14];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if matches!(step, 3 | 10) && rng.gen_range(0..100) > settings.bassline_mutation + 30 {
                continue;
            }
            let start = bar * ticks_per_bar() + step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = match step {
                7 | 14 => 4,
                3 | 10 => rng.gen_range(0..settings.scale.degree_count()),
                _ => 0,
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if matches!(step, 0 | 14) { 116 } else { 92 },
            });
        }
    }

    notes
}

pub(crate) fn choose_bassline_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    step: u32,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let mut candidates = bassline_chord_candidates(chord, low, high);

    if candidates.is_empty() {
        candidates = scale_pitches_in_range(settings);
    }
    if candidates.is_empty() {
        candidates.push(low);
    }

    let root_class = chord.root % 12;
    let fifth_class = (chord.root + 7) % 12;
    let pitch = if step.is_multiple_of(8) {
        candidates
            .iter()
            .copied()
            .find(|candidate| candidate % 12 == root_class)
            .unwrap_or(candidates[0])
    } else if rng.gen_range(0..100) < 40 {
        candidates
            .iter()
            .copied()
            .find(|candidate| candidate % 12 == fifth_class)
            .unwrap_or_else(|| candidates[rng.gen_range(0..candidates.len())])
    } else {
        candidates[rng.gen_range(0..candidates.len())]
    };
    let pitch = prefer_bass_register(pitch, low, high);

    if rng.gen_range(0..100) < settings.bassline_octave_jump && pitch + 12 <= high {
        pitch + 12
    } else {
        pitch
    }
}

pub(crate) fn choose_bass_degree_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    degree: usize,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let target_class = bass_target_class(settings, chord, degree);
    let mut candidates: Vec<u8> = (low..=high)
        .filter(|pitch| pitch % 12 == target_class)
        .collect();
    if candidates.is_empty() {
        candidates = bassline_chord_candidates(chord, low, high);
    }
    if candidates.is_empty() {
        candidates = scale_pitches_in_range(settings);
    }
    let mut pitch = candidates
        .get(rng.gen_range(0..candidates.len().max(1)))
        .copied()
        .unwrap_or(low);
    pitch = prefer_bass_register(pitch, low, high);
    if rng.gen_range(0..100) < settings.bassline_octave_jump && pitch + 12 <= high {
        pitch += 12;
    }
    pitch
}

fn bass_target_class(settings: &GeneratorSettings, chord: &ChordEvent, degree: usize) -> u8 {
    if degree == 0 {
        return chord.root % 12;
    }

    let chord_tones = chord.tones();
    if degree.is_multiple_of(2) {
        if let Some(tone) = chord_tones.get(degree / 2) {
            return *tone % 12;
        }
    }

    pitch_class_for_degree(settings.key, settings.scale, chord.degree + degree) % 12
}

fn prefer_bass_register(pitch: u8, low: u8, high: u8) -> u8 {
    let preferred_high = low.saturating_add(BASS_REGISTER_SPAN).min(high);
    let mut candidate = pitch;
    while candidate > preferred_high && candidate >= low.saturating_add(12) {
        candidate -= 12;
    }
    candidate
}

pub(crate) fn bassline_chord_candidates(chord: &ChordEvent, low: u8, high: u8) -> Vec<u8> {
    let mut tone_classes = chord.tones();
    tone_classes.push((chord.root + 3) % 12);
    tone_classes.sort_unstable();
    tone_classes.dedup();

    (low..=high)
        .filter(|pitch| tone_classes.contains(&(pitch % 12)))
        .collect()
}

pub(crate) fn is_bassline_accented(
    settings: &GeneratorSettings,
    beat_step: u32,
    previous_was_rest: bool,
    rng: &mut StdRng,
) -> bool {
    let structural_bonus = if beat_step == 0 || beat_step == 8 {
        30
    } else if previous_was_rest {
        20
    } else if matches!(beat_step, 3 | 6 | 10 | 14) {
        12
    } else {
        0
    };
    rng.gen_range(0..100) < (settings.bassline_accent + structural_bonus).min(100)
}

pub(crate) fn should_bassline_slide(
    settings: &GeneratorSettings,
    previous_pitch: Option<u8>,
    pitch: u8,
    rng: &mut StdRng,
) -> bool {
    previous_pitch.is_some_and(|previous| {
        previous != pitch && rng.gen_range(0..100) < settings.bassline_slide
    })
}
