use rand::rngs::StdRng;
use rand::Rng;

use super::common::{chord_pitches_in_range, note_duration, scale_pitches_in_range, velocity_for};
use super::{ArpPattern, ArpRotation, ChordEvent, GeneratorSettings, NoteEvent, RhythmStyle};

pub(crate) fn generate_arp(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let rate = match settings.rhythm_style {
        RhythmStyle::Busy => 240,
        RhythmStyle::Sparse => 960,
        RhythmStyle::Dotted => 360,
        _ if settings.density > 70 => 240,
        _ => 480,
    };
    let mut cycle = 0usize;

    for chord in chords {
        let pattern_pitches = arp_pattern_pitches(settings, chord);
        let order = arp_order(settings.arp_pattern, pattern_pitches.len(), rng);
        let mut cursor = chord.start_ticks;
        let mut index = 0;
        while cursor < chord.start_ticks + chord.duration_ticks {
            let pattern_index = order[index % order.len()];
            let pitch = if settings.arp_rotation != ArpRotation::Off
                && pattern_index + 1 == settings.arp_rotate_slot as usize
            {
                rotating_arp_pitch(settings, cycle)
            } else {
                pattern_pitches[pattern_index]
            };
            notes.push(NoteEvent {
                pitch,
                start_ticks: cursor,
                duration_ticks: note_duration(settings, rate, rng),
                velocity: velocity_for(settings, cursor, rng),
            });
            cursor += rate;
            index += 1;
            if index.is_multiple_of(order.len()) {
                cycle += 1;
            }
        }
    }

    notes
}

pub(crate) fn arp_pattern_pitches(settings: &GeneratorSettings, chord: &ChordEvent) -> Vec<u8> {
    let mut pitches = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    pitches.sort_unstable();
    pitches.dedup();

    let scale_pitches = scale_pitches_in_range(settings);
    for pitch in scale_pitches {
        if pitches.len() >= settings.arp_note_count as usize {
            break;
        }
        if !pitches.contains(&pitch) {
            pitches.push(pitch);
        }
    }

    pitches.sort_unstable();
    while pitches.len() < settings.arp_note_count as usize {
        let fallback = pitches
            .last()
            .copied()
            .or_else(|| scale_pitches_in_range(settings).first().copied())
            .unwrap_or_else(|| settings.low_pitch());
        pitches.push(fallback);
    }

    pitches.truncate(settings.arp_note_count as usize);
    pitches
}

pub(crate) fn arp_order(pattern: ArpPattern, note_count: usize, rng: &mut StdRng) -> Vec<usize> {
    match pattern {
        ArpPattern::Up => (0..note_count).collect(),
        ArpPattern::Down => (0..note_count).rev().collect(),
        ArpPattern::UpDown => {
            if note_count <= 2 {
                (0..note_count).collect()
            } else {
                (0..note_count).chain((1..note_count - 1).rev()).collect()
            }
        }
        ArpPattern::RandomWalk => random_walk_order(note_count, rng),
    }
}

pub(crate) fn random_walk_order(note_count: usize, rng: &mut StdRng) -> Vec<usize> {
    let steps = note_count.max(2) * 2;
    let mut current = rng.gen_range(0..note_count);
    let mut order = Vec::with_capacity(steps);
    for _ in 0..steps {
        order.push(current);
        let direction: isize = if rng.gen_bool(0.5) { 1 } else { -1 };
        current = (current as isize + direction).rem_euclid(note_count as isize) as usize;
    }
    order
}

pub(crate) fn rotating_arp_pitch(settings: &GeneratorSettings, cycle: usize) -> u8 {
    let pitches = scale_pitches_in_range(settings);
    if pitches.is_empty() {
        return settings.low_pitch();
    }

    let start = (settings.arp_rotate_slot as usize - 1) % pitches.len();
    let index = match settings.arp_rotation {
        ArpRotation::Off | ArpRotation::Up => (start + cycle) % pitches.len(),
        ArpRotation::Down => (start + pitches.len() - (cycle % pitches.len())) % pitches.len(),
    };
    pitches[index]
}
