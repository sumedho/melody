use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::STEPS_PER_BEAT;

use super::common::{
    apply_bar_density, chord_at, chord_pitches_in_range, density_notes_per_bar,
    nearest_scale_pitch, note_duration, scale_pitches_in_range, ticks_per_bar, velocity_for,
};
use super::{melody, ChordEvent, GeneratorSettings, NoteEvent, PPQN};

#[derive(Debug, Clone)]
pub(crate) struct CounterMelodyParts {
    pub main: Vec<NoteEvent>,
    pub counter: Vec<NoteEvent>,
}

pub(crate) fn generate_counter_melody(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let parts = generate_counter_melody_parts(settings, chords, rng);
    let mut notes = parts.main;
    notes.extend(parts.counter);
    notes.sort_by_key(|note| (note.start_ticks, note.pitch, note.duration_ticks));
    notes
}

pub(crate) fn generate_counter_melody_parts(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> CounterMelodyParts {
    let (lower, upper) = complementary_ranges(settings);
    let mut main_settings = *settings;
    main_settings.min_octave = upper.0;
    main_settings.max_octave = upper.1;
    main_settings.density = ((settings.density as u16 * 55) / 100).clamp(20, 100) as u8;

    let main = apply_bar_density(
        &main_settings,
        melody::generate_melodic(&main_settings, chords, rng),
    );
    let counter = generate_counter_line(settings, &main, chords, lower, rng);

    CounterMelodyParts { main, counter }
}

fn complementary_ranges(settings: &GeneratorSettings) -> ((u8, u8), (u8, u8)) {
    if settings.min_octave >= settings.max_octave {
        return (
            (settings.min_octave, settings.max_octave),
            (settings.min_octave, settings.max_octave),
        );
    }

    let split = settings.min_octave + (settings.max_octave - settings.min_octave) / 2;
    (
        (settings.min_octave, split),
        ((split + 1).min(settings.max_octave), settings.max_octave),
    )
}

fn generate_counter_line(
    settings: &GeneratorSettings,
    main: &[NoteEvent],
    chords: &[ChordEvent],
    range: (u8, u8),
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut counter_settings = *settings;
    counter_settings.min_octave = range.0;
    counter_settings.max_octave = range.1;
    let step_ticks = PPQN as u32 / STEPS_PER_BEAT;
    let bar_ticks = ticks_per_bar();
    let max_per_bar = density_notes_per_bar(settings);
    let mut counter = Vec::new();
    let mut last_counter_pitch: Option<u8> = None;

    for bar in 0..settings.bars as u32 {
        let bar_start = bar * bar_ticks;
        let bar_end = bar_start + bar_ticks;
        let main_count = notes_starting_in_range(main, bar_start, bar_end);
        let target_counter_count = max_per_bar.saturating_sub(main_count);
        if target_counter_count == 0 {
            continue;
        }

        let mut candidates: Vec<u32> = (0..16)
            .map(|step| bar_start + step * step_ticks)
            .filter(|tick| !note_active_at(main, *tick))
            .collect();
        candidates.sort_by_key(|tick| counter_slot_score(*tick, main, bar_start));

        for start in candidates.into_iter().take(target_counter_count) {
            let chord = chord_at(chords, start);
            let strong = start % bar_ticks == 0 || start % PPQN as u32 == 0;
            let target = contrary_target(main, start)
                .or_else(|| last_counter_pitch.map(i32::from))
                .unwrap_or_else(|| i32::from(counter_settings.low_pitch()));
            let pitch = if strong {
                counter_chord_tone(&counter_settings, chord, target)
            } else {
                counter_step_pitch(&counter_settings, target, last_counter_pitch, rng)
            };
            last_counter_pitch = Some(pitch);
            counter.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: note_duration(&counter_settings, step_ticks, rng).min(step_ticks),
                velocity: velocity_for(settings, start, rng)
                    .saturating_sub(14)
                    .max(48),
            });
        }
    }

    counter.sort_by_key(|note| (note.start_ticks, note.pitch));
    counter
}

fn notes_starting_in_range(notes: &[NoteEvent], start: u32, end: u32) -> usize {
    notes
        .iter()
        .filter(|note| note.start_ticks >= start && note.start_ticks < end)
        .count()
}

fn note_active_at(notes: &[NoteEvent], tick: u32) -> bool {
    notes
        .iter()
        .any(|note| tick >= note.start_ticks && tick < note.start_ticks + note.duration_ticks)
}

fn counter_slot_score(tick: u32, main: &[NoteEvent], bar_start: u32) -> (u8, u32) {
    let local = tick - bar_start;
    let beat_score = if local % PPQN as u32 == PPQN as u32 / 2 {
        0
    } else if local % PPQN as u32 == 0 {
        1
    } else {
        2
    };
    let distance = main
        .iter()
        .map(|note| note.start_ticks.abs_diff(tick))
        .min()
        .unwrap_or(u32::MAX);
    (beat_score, distance)
}

fn contrary_target(main: &[NoteEvent], tick: u32) -> Option<i32> {
    let previous = main.iter().rev().find(|note| note.start_ticks < tick)?;
    let next = main.iter().find(|note| note.start_ticks > tick)?;
    let motion = next.pitch as i32 - previous.pitch as i32;
    if motion == 0 {
        return Some(previous.pitch as i32 - 7);
    }

    let contrary = if motion > 0 { -5 } else { 5 };
    Some(previous.pitch as i32 + contrary)
}

fn counter_chord_tone(settings: &GeneratorSettings, chord: &ChordEvent, target: i32) -> u8 {
    let candidates = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    candidates
        .into_iter()
        .min_by_key(|pitch| (*pitch as i32 - target).abs())
        .unwrap_or_else(|| nearest_scale_pitch(settings, target) as u8)
}

fn counter_step_pitch(
    settings: &GeneratorSettings,
    target: i32,
    last_counter_pitch: Option<u8>,
    rng: &mut StdRng,
) -> u8 {
    if let Some(last) = last_counter_pitch {
        let direction = if target >= last as i32 { 1 } else { -1 };
        let candidate = last as i32 + direction * rng.gen_range(1..=3);
        nearest_scale_pitch(settings, candidate) as u8
    } else {
        let scale = scale_pitches_in_range(settings);
        scale
            .into_iter()
            .min_by_key(|pitch| (*pitch as i32 - target).abs())
            .unwrap_or_else(|| settings.low_pitch())
    }
}
