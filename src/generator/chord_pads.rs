use rand::rngs::StdRng;
use rand::Rng;

use super::common::{chord_pitches_in_range, note_duration, velocity_for};
use super::{ChordEvent, GeneratorSettings, NoteEvent};

pub(crate) fn generate_chord_pads(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let mut previous_voicing: Option<Vec<u8>> = None;

    for chord in chords {
        let mut pitches = chord_pad_pitches(settings, chord, rng);
        if let Some(previous) = previous_voicing.as_deref() {
            pitches = voice_lead_chord_pad_voicing(settings, pitches, previous);
        }
        previous_voicing = Some(pitches.clone());
        for (index, pitch) in pitches.into_iter().enumerate() {
            let strum_ticks = (index as u32) * (12 + rng.gen_range(0..=18));
            if strum_ticks >= chord.duration_ticks {
                continue;
            }
            let start_ticks = chord.start_ticks + strum_ticks;
            let available = chord.duration_ticks - strum_ticks;
            let duration_ticks = note_duration(settings, available, rng)
                .min(available)
                .max(1);
            let base_velocity = velocity_for(settings, chord.start_ticks, rng);
            let velocity = (base_velocity as i16 - index as i16 * 5 + rng.gen_range(-4..=4))
                .clamp(36, 118) as u8;
            notes.push(NoteEvent {
                pitch,
                start_ticks,
                duration_ticks,
                velocity,
            });
        }
    }

    notes
}

pub(crate) fn chord_pad_pitches(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    rng: &mut StdRng,
) -> Vec<u8> {
    let candidates = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    if candidates.is_empty() {
        return vec![settings.low_pitch()];
    }

    let octave_span = settings.max_octave.saturating_sub(settings.min_octave) as usize;
    let target_count = (chord.tones().len() + octave_span).clamp(2, 8);
    let voicing = spread_voicing(candidates, target_count);

    maybe_invert_chord_pad_voicing(settings, voicing, rng)
}

pub(crate) fn spread_voicing(candidates: Vec<u8>, target_count: usize) -> Vec<u8> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    candidates.dedup();

    if candidates.len() <= target_count {
        return candidates;
    }

    let last = candidates.len() - 1;
    let picks = target_count.max(2);
    let mut pitches = Vec::with_capacity(picks);
    for index in 0..picks {
        let candidate_index = ((index * last) + (picks - 1) / 2) / (picks - 1);
        let pitch = candidates[candidate_index];
        if !pitches.contains(&pitch) {
            pitches.push(pitch);
        }
    }

    pitches
}

pub(crate) fn maybe_invert_chord_pad_voicing(
    settings: &GeneratorSettings,
    voicing: Vec<u8>,
    rng: &mut StdRng,
) -> Vec<u8> {
    if settings.chord_inversion_amount == 0
        || voicing.len() < 3
        || rng.gen_range(0..100) >= settings.chord_inversion_amount
    {
        return voicing;
    }

    let inversion_depth = if voicing.len() >= 4 && rng.gen_bool(0.45) {
        2
    } else {
        1
    };
    invert_chord_pad_voicing(
        &voicing,
        inversion_depth,
        settings.low_pitch(),
        settings.high_pitch(),
    )
    .unwrap_or(voicing)
}

pub(crate) fn invert_chord_pad_voicing(
    voicing: &[u8],
    inversion_depth: usize,
    low_pitch: u8,
    high_pitch: u8,
) -> Option<Vec<u8>> {
    if voicing.len() < 3 || inversion_depth == 0 {
        return None;
    }

    let mut inverted = voicing.to_vec();
    inverted.sort_unstable();
    let moves = inversion_depth.min(inverted.len() - 1);
    for _ in 0..moves {
        let lowest = inverted.remove(0);
        let raised = lowest.checked_add(12)?;
        if raised > high_pitch {
            return None;
        }
        inverted.push(raised);
        inverted.sort_unstable();
    }
    inverted.dedup();

    if inverted.len() < 3
        || !inverted
            .iter()
            .all(|pitch| (low_pitch..=high_pitch).contains(pitch))
    {
        None
    } else {
        Some(inverted)
    }
}

pub(crate) fn voice_lead_chord_pad_voicing(
    settings: &GeneratorSettings,
    voicing: Vec<u8>,
    previous: &[u8],
) -> Vec<u8> {
    if voicing.is_empty() || previous.is_empty() {
        return voicing;
    }

    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let previous_center = voicing_center(previous);
    let mut candidates = vec![voicing.clone()];

    for shift in [-12i16, 12] {
        let shifted: Option<Vec<u8>> = voicing
            .iter()
            .map(|pitch| {
                let shifted = *pitch as i16 + shift;
                if (low as i16..=high as i16).contains(&shifted) {
                    Some(shifted as u8)
                } else {
                    None
                }
            })
            .collect();
        if let Some(shifted) = shifted {
            candidates.push(shifted);
        }
    }

    for depth in 1..=2 {
        if let Some(inverted) = invert_chord_pad_voicing(&voicing, depth, low, high) {
            candidates.push(inverted);
        }
    }

    candidates
        .into_iter()
        .min_by_key(|candidate| {
            ((voicing_center(candidate) - previous_center).abs() * 100.0).round() as i32
        })
        .unwrap_or(voicing)
}

pub(crate) fn voicing_center(voicing: &[u8]) -> f32 {
    let sum: u32 = voicing.iter().map(|pitch| *pitch as u32).sum();
    sum as f32 / voicing.len() as f32
}
