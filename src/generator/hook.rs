use rand::rngs::StdRng;
use rand::Rng;

use super::common::{
    chord_at, chord_pitches_in_range, nearest_scale_pitch, note_duration, pitch_class_for_degree,
    scale_pitches_in_range, velocity_for,
};
use super::{ChordEvent, GeneratorSettings, HookType, NoteEvent, PPQN};

pub(crate) fn generate_hook(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    match settings.hook_type {
        HookType::FourNoteLoop => generate_four_note_loop(settings, chords, rng),
        HookType::CallResponse => generate_call_response(settings, chords, rng),
        HookType::MotifDevelop => generate_motif_develop(settings, chords, rng),
        HookType::StutterHook => generate_stutter_hook(settings, chords, rng),
        HookType::DescendingBass => generate_descending_bass(settings, rng),
    }
}

fn generate_four_note_loop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let seed = four_note_seed(settings, chords, rng);
    let mut notes = Vec::new();
    let beat_ticks = PPQN as u32;
    let bar_ticks = super::common::ticks_per_bar();

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        for (index, seed_pitch) in seed.iter().enumerate() {
            let start = bar_start + index as u32 * beat_ticks;
            let mut pitch = *seed_pitch;
            if bar > 0 && rng.gen_range(0..100) < settings.variation_amount {
                pitch = nearby_scale_pitch(settings, pitch, rng);
            }
            notes.push(note(settings, start, beat_ticks, pitch, rng));
        }
    }

    notes
}

fn generate_call_response(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let seed = four_note_seed(settings, chords, rng);
    let mut notes = Vec::new();
    let eighth = PPQN as u32 / 2;
    let bar_ticks = super::common::ticks_per_bar();

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        let call_a = seed[0];
        let call_b = seed[1];
        let response_a = nearby_scale_pitch(settings, seed[2], rng);
        let response_b = nearby_scale_pitch(settings, seed[3], rng);

        notes.push(note(settings, bar_start, eighth, call_a, rng));
        notes.push(note(settings, bar_start + eighth, eighth, call_b, rng));
        notes.push(note(
            settings,
            bar_start + PPQN as u32 * 2,
            eighth,
            response_a,
            rng,
        ));
        notes.push(note(
            settings,
            bar_start + PPQN as u32 * 2 + eighth,
            eighth,
            response_b,
            rng,
        ));
    }

    notes
}

fn generate_motif_develop(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let seed = four_note_seed(settings, chords, rng);
    let mut notes = Vec::new();
    let bar_ticks = super::common::ticks_per_bar();
    let eighth = PPQN as u32 / 2;

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        let pitches = developed_motif(settings, &seed, bar as usize, rng);
        for (index, pitch) in pitches.iter().enumerate() {
            let start = bar_start + index as u32 * eighth;
            if start < bar_start + bar_ticks {
                notes.push(note(settings, start, eighth, *pitch, rng));
            }
        }
    }

    notes
}

fn generate_stutter_hook(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let seed = four_note_seed(settings, chords, rng);
    let mut notes = Vec::new();
    let beat_ticks = PPQN as u32;
    let bar_ticks = super::common::ticks_per_bar();

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        for beat in 0..4 {
            let repeats = beat + 1;
            let slot = beat_ticks / repeats;
            for repeat in 0..repeats {
                let density_threshold = settings.density.saturating_add((beat * 8) as u8);
                if rng.gen_range(0..100) <= density_threshold {
                    let start = bar_start + beat as u32 * beat_ticks + repeat as u32 * slot;
                    let pitch = seed[(beat as usize).min(seed.len() - 1)];
                    notes.push(note(settings, start, slot, pitch, rng));
                }
            }
        }
    }

    notes
}

fn generate_descending_bass(settings: &GeneratorSettings, rng: &mut StdRng) -> Vec<NoteEvent> {
    let degrees = [5usize, 3, 0, 4];
    let mut notes = Vec::new();
    let beat_ticks = PPQN as u32;
    let bar_ticks = super::common::ticks_per_bar();

    for bar in 0..settings.bars as u32 {
        let degree = degrees[bar as usize % degrees.len()];
        let root = pitch_for_degree_low(settings, degree);
        let fifth = nearest_pitch_class(settings, root.saturating_add(7), (root + 7) % 12);
        let octave = nearest_pitch_class(settings, root.saturating_add(12), root % 12);
        let pattern = [root, root, fifth, octave];
        let bar_start = bar * bar_ticks;

        for (beat, pitch) in pattern.iter().enumerate() {
            let start = bar_start + beat as u32 * beat_ticks;
            notes.push(note(settings, start, beat_ticks, *pitch, rng));
        }
    }

    notes
}

fn four_note_seed(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> [u8; 4] {
    let beat_ticks = PPQN as u32;
    let mut seed = [settings.low_pitch(); 4];

    for (index, pitch) in seed.iter_mut().enumerate() {
        let tick = index as u32 * beat_ticks;
        let chord = chord_at(chords, tick);
        let tones = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
        *pitch = if tones.is_empty() {
            scale_pitches_in_range(settings)
                .get(index)
                .copied()
                .unwrap_or_else(|| settings.low_pitch())
        } else {
            tones[rng.gen_range(0..tones.len())]
        };
    }

    seed
}

fn developed_motif(
    settings: &GeneratorSettings,
    seed: &[u8; 4],
    bar: usize,
    rng: &mut StdRng,
) -> Vec<u8> {
    match bar % 4 {
        0 => vec![seed[0]],
        1 => vec![seed[0], seed[1]],
        2 => vec![
            seed[0],
            seed[1],
            nearby_scale_pitch(settings, seed[1], rng),
            seed[2],
        ],
        _ => vec![
            seed[0],
            seed[1],
            nearby_scale_pitch(settings, seed[1], rng),
            seed[2],
            nearby_scale_pitch(settings, seed[3], rng),
        ],
    }
}

fn nearby_scale_pitch(settings: &GeneratorSettings, pitch: u8, rng: &mut StdRng) -> u8 {
    let direction = if rng.gen_bool(0.5) { 1 } else { -1 };
    let offset = direction * rng.gen_range(1..=2);
    nearest_scale_pitch(settings, pitch as i32 + offset) as u8
}

fn pitch_for_degree_low(settings: &GeneratorSettings, degree: usize) -> u8 {
    let pitch_class = pitch_class_for_degree(settings.key, settings.scale, degree);
    (settings.low_pitch()..=settings.high_pitch())
        .find(|pitch| pitch % 12 == pitch_class)
        .unwrap_or_else(|| nearest_pitch_class(settings, settings.low_pitch(), pitch_class))
}

fn nearest_pitch_class(settings: &GeneratorSettings, target: u8, pitch_class: u8) -> u8 {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|pitch| pitch % 12 == pitch_class)
        .min_by_key(|pitch| (*pitch as i16 - target as i16).abs())
        .unwrap_or_else(|| settings.low_pitch())
}

fn note(
    settings: &GeneratorSettings,
    start_ticks: u32,
    slot_ticks: u32,
    pitch: u8,
    rng: &mut StdRng,
) -> NoteEvent {
    NoteEvent {
        pitch,
        start_ticks,
        duration_ticks: note_duration(settings, slot_ticks, rng),
        velocity: velocity_for(settings, start_ticks, rng),
    }
}
