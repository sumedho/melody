use rand::rngs::StdRng;
use rand::Rng;

use crate::constants::DEGREE_STABILITY_PROBABILITY;

use super::common::{pitch_class_for_degree, quality_for_degree, ticks_per_bar};
use super::{ChordEvent, ChordQuality, ChordStyle, GeneratorSettings};

pub(crate) fn generate_chords(settings: &GeneratorSettings, rng: &mut StdRng) -> Vec<ChordEvent> {
    if settings.chord_style == ChordStyle::BoardsOfCanada {
        return generate_boards_of_canada_chords(settings, rng);
    }

    let bars_per_chord = if settings.density > 75 || settings.surprise > 80 {
        1
    } else {
        2
    };
    let chord_ticks = ticks_per_bar() * bars_per_chord;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut start = 0;
    let mut degree = 0;
    let mut chord_index = 0usize;
    let mut chords = Vec::new();

    while start < total_ticks {
        let remaining = total_ticks - start;
        let duration = chord_ticks.min(remaining);
        let is_final = start + duration >= total_ticks;
        let is_penultimate = !is_final && start + duration * 2 >= total_ticks;
        let patterned_degree = if settings.chord_style != ChordStyle::Balanced {
            Some(chord_style_degree(
                settings.chord_style,
                chord_index,
                settings.scale.degree_count(),
            ))
        } else {
            None
        };
        let effective_cadence = settings.cadence.saturating_sub(settings.surprise / 2);
        let next_degree = if let Some(patterned_degree) = patterned_degree {
            if settings.chord_style == ChordStyle::PopDescent {
                patterned_degree
            } else if is_penultimate && rng.gen_range(0..100) < effective_cadence {
                cadence_approach_degree(settings, rng)
            } else if is_final && rng.gen_range(0..100) < effective_cadence {
                0
            } else if rng.gen_range(0..100) < settings.surprise {
                surprising_degree(patterned_degree, settings.scale.degree_count(), rng)
            } else {
                patterned_degree
            }
        } else if is_penultimate && rng.gen_range(0..100) < effective_cadence {
            cadence_approach_degree(settings, rng)
        } else if is_final && rng.gen_range(0..100) < effective_cadence {
            0
        } else {
            choose_next_degree(degree, settings, rng)
        };
        degree = next_degree;
        let mut root = pitch_class_for_degree(settings.key, settings.scale, degree);
        let mut quality = quality_for_degree(settings.scale, degree);
        if let Some((borrowed_root, borrowed_quality)) = borrowed_chord(root, settings, rng) {
            root = borrowed_root;
            quality = borrowed_quality;
        }

        if should_surprise_quality(settings, rng) {
            quality = surprise_quality(quality, rng);
        }

        quality = tension_quality(settings, degree, quality, is_penultimate, rng);
        let slash_bass = slash_bass_for_chord(root, quality, settings, rng);
        quality = extension_quality(settings, degree, quality, rng);

        chords.push(ChordEvent {
            root,
            quality,
            slash_bass,
            degree,
            start_ticks: start,
            duration_ticks: duration,
            tension: settings.tension,
        });
        start += duration;
        chord_index += 1;
    }

    chords
}

pub(crate) fn generate_boards_of_canada_chords(
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> Vec<ChordEvent> {
    let pattern = boc_progression_pattern(settings, rng);
    let bars_per_chord = if settings.bars >= 8 { 2 } else { 1 };
    let chord_ticks = ticks_per_bar() * bars_per_chord;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut chords = Vec::new();
    let mut start = 0;
    let mut index = 0usize;

    while start < total_ticks {
        let remaining = total_ticks - start;
        let duration = chord_ticks.min(remaining);
        let is_final = start + duration >= total_ticks;
        let mut offset = pattern[index % pattern.len()];
        if is_final && rng.gen_range(0..100) < settings.cadence {
            offset = 0;
        }

        let root = boc_root_for_offset(settings, offset);
        let mut quality = boc_chord_quality(settings, index, rng);
        let slash_bass = slash_bass_for_chord(root, quality, settings, rng);
        quality = extension_quality(
            settings,
            boc_degree_label(offset, settings.scale.degree_count()),
            quality,
            rng,
        );
        chords.push(ChordEvent {
            root,
            quality,
            slash_bass,
            degree: boc_degree_label(offset, settings.scale.degree_count()),
            start_ticks: start,
            duration_ticks: duration,
            tension: settings.tension,
        });

        start += duration;
        index += 1;
    }

    chords
}

pub(crate) fn boc_progression_pattern(
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> &'static [i8] {
    let grounded: &[&[i8]] = &[&[0, 3, 7, 0], &[0, 3, 5, 0], &[0, 8, 5, 3]];
    let wandering: &[&[i8]] = &[
        &[0, 3, 7, 0],
        &[0, 3, 5, 0],
        &[0, 8, 5, 3],
        &[0, 7, 3, 5],
        &[0, 3, 10, 5],
    ];
    let pool = if settings.surprise > 50 {
        wandering
    } else {
        grounded
    };
    pool[rng.gen_range(0..pool.len())]
}

pub(crate) fn boc_root_for_offset(settings: &GeneratorSettings, offset: i8) -> u8 {
    ((settings.key.semitone() + offset) as i16).rem_euclid(12) as u8
}

pub(crate) fn boc_degree_label(offset: i8, scale_degree_count: usize) -> usize {
    let normalized = offset.rem_euclid(12);
    match normalized {
        0 => 0,
        2 => 1,
        3 => 2,
        5 => 3,
        7 => 4,
        8 => 5,
        10 => 6,
        _ => normalized as usize % scale_degree_count,
    }
}

pub(crate) fn boc_chord_quality(
    settings: &GeneratorSettings,
    index: usize,
    rng: &mut StdRng,
) -> ChordQuality {
    if settings.tension > 70 && rng.gen_range(0..100) < settings.tension / 2 {
        if settings.surprise > 70 && rng.gen_bool(0.25) {
            ChordQuality::Add9
        } else if rng.gen_bool(0.55) {
            ChordQuality::Min9
        } else {
            ChordQuality::Sus2
        }
    } else if settings.tension > 45 && index % 4 == 2 && rng.gen_bool(0.5) {
        ChordQuality::Minor7
    } else if settings.surprise > 65 && rng.gen_range(0..100) < settings.surprise / 3 {
        ChordQuality::Sus2
    } else {
        ChordQuality::MinorDyad
    }
}

pub(crate) fn chord_style_degree(
    style: ChordStyle,
    index: usize,
    scale_degree_count: usize,
) -> usize {
    let pattern: &[usize] = match style {
        ChordStyle::Balanced => &[0, 3, 4, 0],
        ChordStyle::Pop => &[0, 4, 5, 3],
        ChordStyle::PopDescent => &[5, 3, 0, 4],
        ChordStyle::Modal => &[0, 3, 0, 6],
        ChordStyle::Jazz => &[1, 4, 0, 5],
        ChordStyle::MinorCinematic => &[0, 5, 2, 6],
        ChordStyle::AcidMinimal => &[0, 0, 6, 0],
        ChordStyle::ChiptuneLoop => &[0, 4, 5, 3],
        ChordStyle::BoardsOfCanada => &[0, 2, 4, 0],
    };

    pattern[index % pattern.len()] % scale_degree_count
}

pub(crate) fn choose_next_degree(
    current: usize,
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> usize {
    let count = settings.scale.degree_count();
    let stable_targets = if settings.scale.is_minorish() {
        [0, 2, 3, 4, 5]
    } else {
        [0, 1, 3, 4, 5]
    };
    let functional_moves = [3, 4, 5, 1, 0];

    if rng.gen_range(0..100) < settings.surprise {
        return surprising_degree(current, count, rng);
    }

    if rng.gen_range(0..100) < settings.tension {
        functional_moves[rng.gen_range(0..functional_moves.len())] % count
    } else {
        let step = if rng.gen_bool(0.5) { 1 } else { count - 1 };
        let candidate = (current + step) % count;
        if rng.gen_bool(DEGREE_STABILITY_PROBABILITY) {
            candidate
        } else {
            stable_targets[rng.gen_range(0..stable_targets.len())] % count
        }
    }
}

pub(crate) fn surprising_degree(current: usize, count: usize, rng: &mut StdRng) -> usize {
    if count <= 1 {
        return 0;
    }

    let leaps = [2usize, 3, 4, 5];
    let offset = leaps[rng.gen_range(0..leaps.len())] % count;
    let candidate = (current + offset) % count;
    if candidate == current {
        (current + 1) % count
    } else {
        candidate
    }
}

pub(crate) fn cadence_approach_degree(settings: &GeneratorSettings, rng: &mut StdRng) -> usize {
    let count = settings.scale.degree_count();
    let candidates: &[usize] = if settings.scale.is_minorish() {
        &[4, 3, 1, 6]
    } else {
        &[4, 3, 1]
    };
    candidates[rng.gen_range(0..candidates.len())] % count
}

pub(crate) fn borrowed_chord(
    root: u8,
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> Option<(u8, ChordQuality)> {
    if settings.surprise <= 35 {
        return None;
    }
    if rng.gen_range(0..100) >= (settings.surprise - 35) / 2 {
        return None;
    }

    let colors = [
        (1u8, ChordQuality::Major),
        (3, ChordQuality::Minor),
        (6, ChordQuality::Dominant),
        (8, ChordQuality::Major),
        (10, ChordQuality::Major),
    ];
    let (offset, quality) = colors[rng.gen_range(0..colors.len())];
    Some(((root + offset) % 12, quality))
}

pub(crate) fn should_surprise_quality(settings: &GeneratorSettings, rng: &mut StdRng) -> bool {
    settings.surprise > 30 && rng.gen_range(0..100) < settings.surprise / 3
}

pub(crate) fn surprise_quality(current: ChordQuality, rng: &mut StdRng) -> ChordQuality {
    let colors = [
        ChordQuality::Dominant,
        ChordQuality::Suspended,
        ChordQuality::Minor7,
        ChordQuality::Sus2,
        ChordQuality::Add9,
        ChordQuality::Maj7,
        ChordQuality::Min9,
        ChordQuality::Sus4,
    ];
    let picked = colors[rng.gen_range(0..colors.len())];
    if picked == current {
        ChordQuality::Suspended
    } else {
        picked
    }
}

pub(crate) fn tension_quality(
    settings: &GeneratorSettings,
    degree: usize,
    current: ChordQuality,
    is_penultimate: bool,
    rng: &mut StdRng,
) -> ChordQuality {
    if settings.tension <= 55 || rng.gen_range(0..100) >= settings.tension / 2 {
        return current;
    }

    let scale_degree = degree % settings.scale.degree_count();
    if is_penultimate || scale_degree == 4 {
        ChordQuality::Dominant
    } else if matches!(scale_degree, 1 | 3) {
        if rng.gen_bool(0.6) {
            ChordQuality::Sus4
        } else {
            ChordQuality::Add9
        }
    } else if rng.gen_bool(0.35) {
        ChordQuality::Sus2
    } else {
        current
    }
}

pub(crate) fn extension_quality(
    settings: &GeneratorSettings,
    degree: usize,
    current: ChordQuality,
    rng: &mut StdRng,
) -> ChordQuality {
    if current == ChordQuality::Diminished {
        return current;
    }

    let extension_chance = match settings.chord_style {
        ChordStyle::Jazz => 45 + settings.tension / 3,
        ChordStyle::Pop | ChordStyle::PopDescent | ChordStyle::Modal => 25 + settings.tension / 4,
        ChordStyle::BoardsOfCanada => 35 + settings.tension / 4,
        ChordStyle::ChiptuneLoop => 10 + settings.tension / 8,
        ChordStyle::AcidMinimal | ChordStyle::MinorCinematic => {
            8 + settings.tension / 6 + settings.surprise / 8
        }
        ChordStyle::Balanced => 15 + settings.tension / 5 + settings.surprise / 10,
    }
    .min(85);

    if rng.gen_range(0..100) >= extension_chance {
        return current;
    }

    let scale_degree = degree % settings.scale.degree_count();
    let minorish = is_minor_quality(current)
        || matches!(scale_degree, 1 | 2 | 5)
        || settings.scale.is_minorish();
    let palette: &[ChordQuality] = match settings.chord_style {
        ChordStyle::Jazz => {
            if minorish {
                &[
                    ChordQuality::Minor7,
                    ChordQuality::Min9,
                    ChordQuality::Dominant,
                    ChordQuality::Add13,
                ]
            } else {
                &[
                    ChordQuality::Maj7,
                    ChordQuality::Maj9,
                    ChordQuality::Dominant,
                    ChordQuality::Add13,
                ]
            }
        }
        ChordStyle::Pop | ChordStyle::PopDescent | ChordStyle::Modal => {
            if minorish {
                &[
                    ChordQuality::Minor7,
                    ChordQuality::Min9,
                    ChordQuality::Add9,
                    ChordQuality::Sus2,
                    ChordQuality::Sus4,
                ]
            } else {
                &[
                    ChordQuality::Add9,
                    ChordQuality::Maj7,
                    ChordQuality::Maj9,
                    ChordQuality::Sus2,
                    ChordQuality::Sus4,
                ]
            }
        }
        ChordStyle::BoardsOfCanada => &[
            ChordQuality::Minor7,
            ChordQuality::Min9,
            ChordQuality::Sus2,
            ChordQuality::Add9,
        ],
        ChordStyle::ChiptuneLoop => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Sus2,
            ChordQuality::Sus4,
            ChordQuality::Add9,
        ],
        ChordStyle::AcidMinimal | ChordStyle::MinorCinematic => {
            if minorish {
                &[
                    ChordQuality::Minor7,
                    ChordQuality::Min9,
                    ChordQuality::Sus2,
                    ChordQuality::Add9,
                ]
            } else {
                &[
                    ChordQuality::Add9,
                    ChordQuality::Sus2,
                    ChordQuality::Sus4,
                    ChordQuality::Maj7,
                ]
            }
        }
        ChordStyle::Balanced => {
            if minorish {
                &[
                    ChordQuality::Minor7,
                    ChordQuality::Min9,
                    ChordQuality::Add9,
                    ChordQuality::Sus2,
                ]
            } else {
                &[
                    ChordQuality::Add9,
                    ChordQuality::Maj7,
                    ChordQuality::Sus2,
                    ChordQuality::Add11,
                ]
            }
        }
    };

    let picked = palette[rng.gen_range(0..palette.len())];
    if preserves_minor_color(current, picked) {
        picked
    } else {
        current
    }
}

fn is_minor_quality(quality: ChordQuality) -> bool {
    matches!(
        quality,
        ChordQuality::Minor | ChordQuality::MinorDyad | ChordQuality::Minor7 | ChordQuality::Min9
    )
}

fn preserves_minor_color(current: ChordQuality, picked: ChordQuality) -> bool {
    !is_minor_quality(current)
        || is_minor_quality(picked)
        || matches!(
            picked,
            ChordQuality::Sus2 | ChordQuality::Sus4 | ChordQuality::Add9
        )
}

fn slash_bass_for_chord(
    root: u8,
    quality: ChordQuality,
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> Option<u8> {
    let chance = settings
        .surprise
        .saturating_add(settings.tension / 2)
        .saturating_sub(75)
        / 3;
    if chance == 0 || rng.gen_range(0..100) >= chance {
        return None;
    }

    let tones = match quality {
        ChordQuality::Minor
        | ChordQuality::MinorDyad
        | ChordQuality::Minor7
        | ChordQuality::Min9 => [3, 7],
        ChordQuality::Sus2 => [2, 7],
        ChordQuality::Suspended | ChordQuality::Sus4 => [5, 7],
        ChordQuality::Diminished => [3, 6],
        _ => [4, 7],
    };
    Some((root + tones[rng.gen_range(0..tones.len())]) % 12)
}
