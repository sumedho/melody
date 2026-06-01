use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::{CHIPTUNE_OCTAVE_INTERVAL, CHIPTUNE_OCTAVE_JUMP_CHANCE, STEPS_PER_BEAT};

use super::common::{
    chord_at, nearest_pitch_class, note_duration, rhythm_density, scale_pitch, velocity_for,
};
use super::{ChordEvent, GeneratorSettings, NoteEvent, PPQN};

pub(crate) fn generate_chiptune(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let total_steps = settings.bars as u32 * 16;
    let motif = [0, 2, 4, 7, 4, 2, 0, CHIPTUNE_OCTAVE_INTERVAL];

    for step in 0..total_steps {
        let start = step * step_ticks;
        if step % 2 != 0 && rng.gen_range(0..100) > rhythm_density(settings) {
            continue;
        }
        let chord = chord_at(chords, start);
        let degree = motif[(step as usize + settings.seed as usize) % motif.len()];
        let base = scale_pitch(settings, degree, settings.min_octave as i8 + 1)
            .clamp(settings.low_pitch(), settings.high_pitch());
        let chord_tones = chord.tones();
        let pitch = if step % 4 == 0 {
            nearest_pitch_class(settings, base, &chord_tones)
        } else if rng.gen_bool(CHIPTUNE_OCTAVE_JUMP_CHANCE) {
            base.saturating_add(CHIPTUNE_OCTAVE_INTERVAL as u8)
                .min(settings.high_pitch())
        } else {
            base
        };

        notes.push(NoteEvent {
            pitch,
            start_ticks: start,
            duration_ticks: note_duration(settings, step_ticks, rng),
            velocity: velocity_for(settings, start, rng),
        });
    }

    notes
}
