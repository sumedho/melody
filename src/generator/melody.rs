use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::MELODIC_STRONG_NOTE_CHANCE;

use super::common::{
    choose_chord_or_scale_pitch, chord_at, nearest_scale_pitch, note_duration, rhythm_density,
    ticks_per_bar, velocity_for,
};
use super::{ChordEvent, GeneratorSettings, NoteEvent, RhythmStyle, PPQN};

pub(crate) fn generate_melodic(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let mut cursor = 0;
    let total = ticks_per_bar() * settings.bars as u32;
    let mut last_pitch = 60 + settings.key.semitone() as i32;

    while cursor < total {
        let patterns = melodic_rhythm_patterns(settings.rhythm_style);
        let pattern = patterns[rng.gen_range(0..patterns.len())];
        for duration in pattern {
            if cursor >= total {
                break;
            }
            let dur = (*duration).min(total - cursor);
            if rng.gen_range(0..100) <= rhythm_density(settings) {
                let chord = chord_at(chords, cursor);
                let strong =
                    cursor.is_multiple_of(ticks_per_bar()) || cursor.is_multiple_of(PPQN as u32);
                let pitch = choose_melodic_pitch(settings, chord, last_pitch, strong, rng);
                last_pitch = pitch as i32;
                notes.push(NoteEvent {
                    pitch,
                    start_ticks: cursor,
                    duration_ticks: note_duration(settings, dur, rng),
                    velocity: velocity_for(settings, cursor, rng),
                });
            }
            cursor += dur;
        }
    }
    notes
}

pub(crate) fn melodic_rhythm_patterns(style: RhythmStyle) -> &'static [&'static [u32]] {
    match style {
        RhythmStyle::Straight => &[
            &[480, 480, 480, 480],
            &[960, 480, 480],
            &[720, 240, 480, 480],
        ],
        RhythmStyle::Syncopated => &[
            &[240, 720, 240, 720],
            &[360, 120, 480, 960],
            &[240, 240, 480, 960],
        ],
        RhythmStyle::Sparse => &[&[960, 960], &[1440, 480], &[1920]],
        RhythmStyle::Busy => &[
            &[240, 240, 240, 240, 480, 480],
            &[240; 8],
            &[360, 120, 240, 240, 480, 480],
        ],
        RhythmStyle::Dotted => &[&[720, 240, 720, 240], &[360, 120, 360, 120, 960]],
    }
}

pub(crate) fn choose_melodic_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    last_pitch: i32,
    strong: bool,
    rng: &mut StdRng,
) -> u8 {
    if strong || rng.gen_bool(MELODIC_STRONG_NOTE_CHANCE) {
        choose_chord_or_scale_pitch(settings, chord, rng)
    } else {
        let step = if rng.gen_bool(0.55) { 1 } else { -1 };
        let candidate = last_pitch + step * rng.gen_range(1..=2);
        nearest_scale_pitch(settings, candidate) as u8
    }
}
