use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::{
    BEATS_PER_BAR, MIN_NOTE_GATE_RATIO, NOTE_DURATION_VARIATION_RATIO, NOTE_GATE_RANGE_RATIO,
    STEPS_PER_BEAT,
};

use super::{
    ChordEvent, ChordQuality, GeneratorSettings, Key, NoteEvent, RhythmStyle, Scale, VelocityMode,
    PPQN,
};

pub(crate) fn rhythm_density(settings: &GeneratorSettings) -> u8 {
    let adjusted = match settings.rhythm_style {
        RhythmStyle::Straight => settings.density as i16,
        RhythmStyle::Syncopated => settings.density as i16 + 8,
        RhythmStyle::Sparse => settings.density as i16 - 24,
        RhythmStyle::Busy => settings.density as i16 + 18,
        RhythmStyle::Dotted => settings.density as i16 - 4,
    };

    adjusted.clamp(5, 100) as u8
}

pub(crate) fn density_notes_per_bar(settings: &GeneratorSettings) -> usize {
    ((rhythm_density(settings) as usize * 16).div_ceil(100)).clamp(1, 16)
}

pub(crate) fn apply_bar_density(
    settings: &GeneratorSettings,
    mut notes: Vec<NoteEvent>,
) -> Vec<NoteEvent> {
    let max_per_bar = density_notes_per_bar(settings);
    let bar_ticks = ticks_per_bar();

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        let bar_end = bar_start + bar_ticks;
        let mut bar_notes: Vec<(usize, i32)> = notes
            .iter()
            .enumerate()
            .filter(|(_, note)| note.start_ticks >= bar_start && note.start_ticks < bar_end)
            .map(|(index, note)| {
                let local_start = note.start_ticks - bar_start;
                let grid_score = if local_start == 0 {
                    3
                } else if local_start.is_multiple_of(PPQN as u32) {
                    2
                } else if local_start.is_multiple_of(PPQN as u32 / STEPS_PER_BEAT) {
                    1
                } else {
                    0
                };
                let score = grid_score * 1_000_000 + note.velocity as i32 * 1_000
                    - (local_start / (PPQN as u32 / STEPS_PER_BEAT)) as i32;
                (index, score)
            })
            .collect();

        if bar_notes.len() <= max_per_bar {
            continue;
        }

        bar_notes.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        for (index, _) in bar_notes.iter().skip(max_per_bar) {
            notes[*index].duration_ticks = 0;
        }
    }

    cleanup_notes(settings, notes)
}

pub(crate) fn apply_phrase_memory(
    settings: &GeneratorSettings,
    mut notes: Vec<NoteEvent>,
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let phrase_ticks = ticks_per_bar() * settings.phrase_length as u32;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    if settings.repeat_amount == 0 || phrase_ticks == 0 || phrase_ticks >= total_ticks {
        return cleanup_notes(settings, notes);
    }

    let template: Vec<NoteEvent> = notes
        .iter()
        .filter(|note| note.start_ticks < phrase_ticks)
        .cloned()
        .collect();
    if template.is_empty() {
        return cleanup_notes(settings, notes);
    }

    let mut phrase_start = phrase_ticks;
    while phrase_start < total_ticks {
        let phrase_end = (phrase_start + phrase_ticks).min(total_ticks);
        if rng.gen_range(0..100) < settings.repeat_amount {
            notes.retain(|note| note.start_ticks < phrase_start || note.start_ticks >= phrase_end);
            for source in &template {
                let start_ticks = phrase_start + source.start_ticks;
                if start_ticks >= phrase_end {
                    continue;
                }

                let mut copied = source.clone();
                copied.start_ticks = start_ticks;
                copied.duration_ticks = copied.duration_ticks.min(phrase_end - start_ticks).max(1);
                if rng.gen_range(0..100) < settings.variation_amount {
                    copied.pitch = vary_pitch_by_scale_step(settings, copied.pitch, rng);
                    copied.velocity = vary_velocity(copied.velocity, rng);
                }
                notes.push(copied);
            }
        }
        phrase_start += phrase_ticks;
    }

    cleanup_notes(settings, notes)
}

pub(crate) fn vary_pitch_by_scale_step(
    settings: &GeneratorSettings,
    pitch: u8,
    rng: &mut StdRng,
) -> u8 {
    let scale = scale_pitches_in_range(settings);
    if scale.is_empty() {
        return pitch.clamp(settings.low_pitch(), settings.high_pitch());
    }

    let index = scale
        .iter()
        .enumerate()
        .min_by_key(|(_, candidate)| (**candidate as i16 - pitch as i16).abs())
        .map(|(index, _)| index)
        .unwrap_or(0);
    let direction: isize = if rng.gen_bool(0.5) { 1 } else { -1 };
    let next = (index as isize + direction).clamp(0, scale.len() as isize - 1) as usize;
    scale[next]
}

pub(crate) fn vary_velocity(velocity: u8, rng: &mut StdRng) -> u8 {
    let offset: i16 = rng.gen_range(-8..=8);
    (velocity as i16 + offset).clamp(1, 127) as u8
}

pub(crate) fn cleanup_notes(
    settings: &GeneratorSettings,
    mut notes: Vec<NoteEvent>,
) -> Vec<NoteEvent> {
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

pub(crate) fn apply_velocity_range(
    settings: &GeneratorSettings,
    mut notes: Vec<NoteEvent>,
) -> Vec<NoteEvent> {
    let low = settings
        .random_velocity_min
        .min(settings.random_velocity_max);
    let high = settings
        .random_velocity_min
        .max(settings.random_velocity_max);
    for note in &mut notes {
        note.velocity = note.velocity.clamp(low, high);
    }
    notes
}

pub(crate) fn choose_chord_or_scale_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let chord_tones = chord_pitches_in_range(chord, low, high);
    if rng.gen_bool(0.72) && !chord_tones.is_empty() {
        chord_tones[rng.gen_range(0..chord_tones.len())]
    } else {
        let octave = rng.gen_range(settings.min_octave..=settings.max_octave) as i8;
        let pitch = scale_pitch(
            settings,
            rng.gen_range(0..settings.scale.degree_count()),
            octave,
        );
        if (low..=high).contains(&pitch) {
            pitch
        } else {
            nearest_scale_pitch(settings, pitch as i32) as u8
        }
    }
}

pub(crate) fn chord_pitches_in_range(chord: &ChordEvent, low: u8, high: u8) -> Vec<u8> {
    let tones = chord.tones();
    (low..=high)
        .filter(|pitch| tones.contains(&(pitch % 12)))
        .collect()
}

pub(crate) fn chord_at(chords: &[ChordEvent], tick: u32) -> &ChordEvent {
    chords
        .iter()
        .find(|chord| tick >= chord.start_ticks && tick < chord.start_ticks + chord.duration_ticks)
        .unwrap_or_else(|| chords.last().expect("at least one chord"))
}

pub(crate) fn velocity_for(settings: &GeneratorSettings, start: u32, rng: &mut StdRng) -> u8 {
    match settings.velocity_mode {
        VelocityMode::Fixed => 92,
        VelocityMode::Random => {
            rng.gen_range(settings.random_velocity_min..=settings.random_velocity_max)
        }
        VelocityMode::Accented => {
            if start.is_multiple_of(ticks_per_bar()) {
                116
            } else if start.is_multiple_of(PPQN as u32) {
                98
            } else {
                76
            }
        }
        VelocityMode::Humanized => {
            let base = if start.is_multiple_of(ticks_per_bar()) {
                108
            } else if start.is_multiple_of(PPQN as u32) {
                92
            } else {
                74
            };
            (base + rng.gen_range(0..=12)).min(127)
        }
    }
}

pub(crate) fn note_duration(
    settings: &GeneratorSettings,
    slot_ticks: u32,
    rng: &mut StdRng,
) -> u32 {
    let fixed_gate = PPQN as u32 / STEPS_PER_BEAT;
    if settings.note_length == 0 {
        return fixed_gate.max(1);
    }

    let normalized = settings.note_length as f32 / 100.0;
    let base_multiplier = MIN_NOTE_GATE_RATIO + normalized * NOTE_GATE_RANGE_RATIO;
    let variation = if settings.note_length < 25 {
        0.0
    } else {
        let spread = normalized * NOTE_DURATION_VARIATION_RATIO;
        rng.gen_range(-spread..=spread)
    };

    ((slot_ticks as f32 * (base_multiplier + variation)).round() as u32).max(1)
}

pub(crate) fn quality_for_degree(scale: Scale, degree: usize) -> ChordQuality {
    let qualities: &[ChordQuality] = match scale {
        Scale::Major => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
        ],
        Scale::NaturalMinor => &[
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
        ],
        Scale::HarmonicMinor => &[
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Dominant,
            ChordQuality::Major,
            ChordQuality::Diminished,
        ],
        Scale::MajorPentatonic => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Minor,
        ],
        Scale::MinorPentatonic => &[
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Major,
        ],
        Scale::Blues => &[
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Suspended,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor7,
        ],
        Scale::Dorian => &[
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
        ],
        Scale::Mixolydian => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
        ],
    };

    qualities[degree % qualities.len()]
}

pub(crate) fn pitch_class_for_degree(key: Key, scale: Scale, degree: usize) -> u8 {
    let intervals = scale.intervals();
    ((key.semitone() + intervals[degree % intervals.len()]) as i16).rem_euclid(12) as u8
}

pub(crate) fn scale_pitch(settings: &GeneratorSettings, degree: usize, octave: i8) -> u8 {
    let intervals = settings.scale.intervals();
    let octaves = degree / intervals.len();
    let interval = intervals[degree % intervals.len()];
    (12 * octave + settings.key.semitone() + interval + 12 * octaves as i8) as u8
}

pub(crate) fn scale_pitches_in_range(settings: &GeneratorSettings) -> Vec<u8> {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|pitch| {
            let pc = ((*pitch as i8 - settings.key.semitone()) as i16).rem_euclid(12) as i8;
            settings.scale.intervals().contains(&pc)
        })
        .collect()
}

pub(crate) fn nearest_scale_pitch(settings: &GeneratorSettings, pitch: i32) -> i32 {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|candidate| {
            let pc = ((*candidate as i8 - settings.key.semitone()) as i16).rem_euclid(12) as i8;
            settings.scale.intervals().contains(&pc)
        })
        .min_by_key(|candidate| (*candidate as i32 - pitch).abs())
        .map(i32::from)
        .unwrap_or_else(|| i32::from(settings.low_pitch()))
}

pub(crate) fn nearest_pitch_class(settings: &GeneratorSettings, pitch: u8, classes: &[u8]) -> u8 {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|candidate| classes.contains(&(candidate % 12)))
        .min_by_key(|candidate| (*candidate as i16 - pitch as i16).abs())
        .unwrap_or(pitch)
}

pub(crate) fn octave_to_midi_c(octave: u8) -> u8 {
    12 * (octave + 1)
}

pub fn ticks_per_bar() -> u32 {
    PPQN as u32 * BEATS_PER_BAR
}
