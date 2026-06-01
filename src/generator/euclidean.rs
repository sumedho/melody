use rand::rngs::StdRng;

use super::common::{
    choose_chord_or_scale_pitch, chord_at, note_duration, rhythm_density, ticks_per_bar,
    velocity_for,
};
use super::{ChordEvent, GeneratorSettings, NoteEvent};

pub(crate) fn generate_euclidean(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let steps_per_bar = 16;
    let pulses = ((rhythm_density(settings) as usize * steps_per_bar) / 120).clamp(2, 14);
    let pattern = euclidean_pattern(
        steps_per_bar,
        pulses,
        settings.surprise as usize % steps_per_bar,
    );
    let mut notes = Vec::new();
    let step_ticks = ticks_per_bar() / steps_per_bar as u32;

    for bar in 0..settings.bars as u32 {
        for (step, active) in pattern.iter().enumerate() {
            if !active {
                continue;
            }
            let start = bar * ticks_per_bar() + step as u32 * step_ticks;
            let chord = chord_at(chords, start);
            let pitch = choose_chord_or_scale_pitch(settings, chord, rng);
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks, rng),
                velocity: velocity_for(settings, start, rng),
            });
        }
    }

    notes
}

pub(crate) fn euclidean_pattern(steps: usize, pulses: usize, rotation: usize) -> Vec<bool> {
    (0..steps)
        .map(|step| {
            let rotated = (step + rotation) % steps;
            (rotated * pulses) % steps < pulses
        })
        .collect()
}
