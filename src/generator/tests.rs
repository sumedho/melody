use super::arp::{arp_order, rotating_arp_pitch};
use super::chord_pads::{
    chord_pad_pitches, spread_voicing, voice_lead_chord_pad_voicing, voicing_center,
};
use super::chords::{borrowed_chord, chord_style_degree, generate_chords};
use super::common::{
    apply_phrase_memory, chord_pitches_in_range, note_duration, octave_to_midi_c,
    quality_for_degree, rhythm_density, scale_pitches_in_range,
};
use super::euclidean::euclidean_pattern;
use super::*;

fn note_signature(notes: &[NoteEvent]) -> Vec<(u8, u32, u32, u8)> {
    notes
        .iter()
        .map(|note| {
            (
                note.pitch,
                note.start_ticks,
                note.duration_ticks,
                note.velocity,
            )
        })
        .collect()
}

#[test]
fn euclidean_pattern_has_requested_pulses() {
    let pattern = euclidean_pattern(16, 5, 3);
    assert_eq!(pattern.len(), 16);
    assert_eq!(pattern.iter().filter(|active| **active).count(), 5);
}

#[test]
fn chord_timeline_fills_requested_bars() {
    let settings = GeneratorSettings {
        bars: 5,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let total: u32 = chords.iter().map(|chord| chord.duration_ticks).sum();
    assert_eq!(total, ticks_per_bar() * settings.bars as u32);
    assert_eq!(chords.first().map(|chord| chord.start_ticks), Some(0));
}

#[test]
fn locked_chords_are_reused_exactly_across_seeds() {
    let settings = GeneratorSettings {
        seed: 1,
        ..GeneratorSettings::default()
    };
    let source = generate_song(&settings);
    let regenerated = generate_song_with_chords(
        &GeneratorSettings {
            seed: 999,
            ..settings
        },
        Some(&source.chords),
    );

    assert_eq!(regenerated.chords, source.chords);
}

#[test]
fn locked_chords_still_allow_seeded_note_changes() {
    let settings = GeneratorSettings::default();
    let source = generate_song(&settings);
    let first = generate_song_with_chords(
        &GeneratorSettings {
            seed: 11,
            ..settings
        },
        Some(&source.chords),
    );
    let second = generate_song_with_chords(
        &GeneratorSettings {
            seed: 12,
            ..settings
        },
        Some(&source.chords),
    );

    assert_eq!(first.chords, source.chords);
    assert_eq!(second.chords, source.chords);
    assert_ne!(note_signature(&first.notes), note_signature(&second.notes));
}

#[test]
fn locked_chords_clip_when_bars_are_reduced() {
    let source_settings = GeneratorSettings {
        bars: 4,
        ..GeneratorSettings::default()
    };
    let source = generate_song(&source_settings);
    let clipped = generate_song_with_chords(
        &GeneratorSettings {
            bars: 1,
            ..source_settings
        },
        Some(&source.chords),
    );

    assert_eq!(clipped.chords.len(), 1);
    assert_eq!(clipped.chords[0].root, source.chords[0].root);
    assert_eq!(clipped.chords[0].quality, source.chords[0].quality);
    assert_eq!(clipped.chords[0].start_ticks, 0);
    assert_eq!(clipped.chords[0].duration_ticks, ticks_per_bar());
}

#[test]
fn locked_chords_repeat_when_bars_are_expanded() {
    let source_settings = GeneratorSettings {
        bars: 4,
        ..GeneratorSettings::default()
    };
    let source = generate_song(&source_settings);
    let expanded = generate_song_with_chords(
        &GeneratorSettings {
            bars: 6,
            ..source_settings
        },
        Some(&source.chords),
    );

    assert_eq!(expanded.chords.len(), 3);
    assert_eq!(expanded.chords[0], source.chords[0]);
    assert_eq!(expanded.chords[1], source.chords[1]);
    assert_eq!(expanded.chords[2].root, source.chords[0].root);
    assert_eq!(expanded.chords[2].quality, source.chords[0].quality);
    assert_eq!(expanded.chords[2].start_ticks, ticks_per_bar() * 4);
    assert_eq!(expanded.chords[2].duration_ticks, ticks_per_bar() * 2);
}

#[test]
fn locked_chords_are_reused_by_every_generator_mode() {
    let locked_chords = vec![
        ChordEvent {
            root: 1,
            quality: ChordQuality::Minor7,
            degree: 0,
            start_ticks: 0,
            duration_ticks: ticks_per_bar() * 2,
            tension: 70,
        },
        ChordEvent {
            root: 8,
            quality: ChordQuality::Dominant,
            degree: 4,
            start_ticks: ticks_per_bar() * 2,
            duration_ticks: ticks_per_bar() * 2,
            tension: 82,
        },
    ];

    for mode in GeneratorMode::ALL {
        let song = generate_song_with_chords(
            &GeneratorSettings {
                mode,
                bars: 4,
                density: 100,
                seed: 9876,
                ..GeneratorSettings::default()
            },
            Some(&locked_chords),
        );

        assert_eq!(song.chords, locked_chords, "{mode} ignored locked chords");
        assert!(!song.notes.is_empty(), "{mode} generated no notes");
    }
}

#[test]
fn locked_chords_are_reused_by_every_bassline_style() {
    let locked_chords = vec![
        ChordEvent {
            root: 3,
            quality: ChordQuality::Minor,
            degree: 0,
            start_ticks: 0,
            duration_ticks: ticks_per_bar() * 2,
            tension: 35,
        },
        ChordEvent {
            root: 10,
            quality: ChordQuality::Suspended,
            degree: 3,
            start_ticks: ticks_per_bar() * 2,
            duration_ticks: ticks_per_bar() * 2,
            tension: 60,
        },
    ];

    for bassline_style in BasslineStyle::ALL {
        let song = generate_song_with_chords(
            &GeneratorSettings {
                mode: GeneratorMode::Bassline,
                bassline_style,
                bars: 4,
                density: 100,
                seed: 6789,
                ..GeneratorSettings::default()
            },
            Some(&locked_chords),
        );

        assert_eq!(
            song.chords, locked_chords,
            "{bassline_style} bassline ignored locked chords"
        );
        assert!(
            !song.notes.is_empty(),
            "{bassline_style} bassline generated no notes"
        );
    }
}

#[test]
fn unlocked_generation_can_change_chords_between_seeds() {
    let settings = GeneratorSettings {
        surprise: 100,
        cadence: 0,
        ..GeneratorSettings::default()
    };
    let first = generate_song(&GeneratorSettings {
        seed: 1,
        ..settings
    });
    let second = generate_song(&GeneratorSettings {
        seed: 2,
        ..settings
    });

    assert_ne!(first.chords, second.chords);
}

#[test]
fn every_generator_produces_notes() {
    for mode in GeneratorMode::ALL {
        let settings = GeneratorSettings {
            mode,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(!song.chords.is_empty(), "{mode} generated no chords");
        assert!(!song.notes.is_empty(), "{mode} generated no notes");
        assert!(song
            .notes
            .iter()
            .all(|note| note.start_ticks < ticks_per_bar() * settings.bars as u32));
    }
}

#[test]
fn octave_range_auto_clamps_when_crossed() {
    let mut settings = GeneratorSettings::default();
    settings.set_min_octave(7);
    assert_eq!(settings.min_octave, 7);
    assert_eq!(settings.max_octave, 7);

    settings.set_max_octave(2);
    assert_eq!(settings.min_octave, 2);
    assert_eq!(settings.max_octave, 2);
}

#[test]
fn every_generator_respects_octave_range() {
    for mode in GeneratorMode::ALL {
        let settings = GeneratorSettings {
            mode,
            min_octave: 2,
            max_octave: 4,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(
            song.notes
                .iter()
                .all(|note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)),
            "{mode} generated a note outside the selected octave range"
        );
    }
}

#[test]
fn zero_note_length_uses_identical_gate() {
    let settings = GeneratorSettings {
        note_length: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let first_duration = song.notes.first().unwrap().duration_ticks;
    assert!(song
        .notes
        .iter()
        .all(|note| note.duration_ticks == first_duration));
}

#[test]
fn random_velocity_uses_configured_range() {
    let settings = GeneratorSettings {
        velocity_mode: VelocityMode::Random,
        random_velocity_min: 20,
        random_velocity_max: 24,
        seed: 9,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);

    assert!(!song.notes.is_empty());
    assert!(song
        .notes
        .iter()
        .all(|note| (20..=24).contains(&note.velocity)));
}

#[test]
fn random_velocity_range_is_honored_by_every_generator_mode() {
    for mode in GeneratorMode::ALL {
        let settings = GeneratorSettings {
            mode,
            velocity_mode: VelocityMode::Random,
            random_velocity_min: 101,
            random_velocity_max: 104,
            density: 100,
            seed: 1122,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);

        assert!(!song.notes.is_empty(), "{mode} generated no notes");
        assert!(
            song.notes
                .iter()
                .all(|note| (101..=104).contains(&note.velocity)),
            "{mode} did not honor the random velocity range"
        );
    }
}

#[test]
fn random_velocity_range_is_honored_by_every_bassline_style() {
    for bassline_style in BasslineStyle::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style,
            velocity_mode: VelocityMode::Random,
            random_velocity_min: 96,
            random_velocity_max: 99,
            density: 100,
            seed: 2211,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);

        assert!(
            !song.notes.is_empty(),
            "{bassline_style} bassline generated no notes"
        );
        assert!(
            song.notes
                .iter()
                .all(|note| (96..=99).contains(&note.velocity)),
            "{bassline_style} bassline did not honor the random velocity range"
        );
    }
}

#[test]
fn random_velocity_range_clamps_when_crossed() {
    let mut settings = GeneratorSettings::default();
    settings.set_random_velocity_min(120);
    assert_eq!(settings.random_velocity_min, 120);
    assert_eq!(settings.random_velocity_max, 120);

    settings.set_random_velocity_max(40);
    assert_eq!(settings.random_velocity_min, 40);
    assert_eq!(settings.random_velocity_max, 40);
}

#[test]
fn max_note_length_can_overlap_slots() {
    let settings = GeneratorSettings {
        note_length: 100,
        mode: GeneratorMode::Euclidean,
        density: 100,
        ..GeneratorSettings::default()
    };
    let slot_ticks = ticks_per_bar() / 16;
    let mut rng = StdRng::seed_from_u64(settings.seed);
    assert!(note_duration(&settings, slot_ticks, &mut rng) > slot_ticks);
}

#[test]
fn arp_note_count_clamps_rotating_slot() {
    let mut settings = GeneratorSettings {
        arp_note_count: 6,
        arp_rotate_slot: 6,
        ..GeneratorSettings::default()
    };
    settings.set_arp_note_count(3);
    assert_eq!(settings.arp_note_count, 3);
    assert_eq!(settings.arp_rotate_slot, 3);

    settings.set_arp_rotate_slot(8);
    assert_eq!(settings.arp_rotate_slot, 3);
}

#[test]
fn arp_orders_match_selected_pattern() {
    let mut rng = StdRng::seed_from_u64(1);
    assert_eq!(arp_order(ArpPattern::Up, 4, &mut rng), vec![0, 1, 2, 3]);
    assert_eq!(arp_order(ArpPattern::Down, 4, &mut rng), vec![3, 2, 1, 0]);
    assert_eq!(
        arp_order(ArpPattern::UpDown, 4, &mut rng),
        vec![0, 1, 2, 3, 2, 1]
    );
}

#[test]
fn random_walk_order_is_seeded_and_neighboring() {
    let mut first = StdRng::seed_from_u64(99);
    let mut second = StdRng::seed_from_u64(99);
    let order = arp_order(ArpPattern::RandomWalk, 5, &mut first);
    assert_eq!(order, arp_order(ArpPattern::RandomWalk, 5, &mut second));
    assert_eq!(order.len(), 10);
    for pair in order.windows(2) {
        let distance = pair[0].abs_diff(pair[1]);
        assert!(distance == 1 || distance == 4);
    }
}

#[test]
fn rotating_arp_pitch_moves_by_scale_degree() {
    let settings = GeneratorSettings {
        arp_rotation: ArpRotation::Up,
        arp_rotate_slot: 1,
        min_octave: 4,
        max_octave: 4,
        ..GeneratorSettings::default()
    };
    let pitches = scale_pitches_in_range(&settings);
    assert_eq!(rotating_arp_pitch(&settings, 0), pitches[0]);
    assert_eq!(rotating_arp_pitch(&settings, 1), pitches[1]);
    assert_eq!(rotating_arp_pitch(&settings, 2), pitches[2]);
}

#[test]
fn rotating_arp_pitch_wraps_within_octave_range() {
    let settings = GeneratorSettings {
        arp_rotation: ArpRotation::Down,
        arp_rotate_slot: 1,
        min_octave: 4,
        max_octave: 4,
        ..GeneratorSettings::default()
    };
    let pitches = scale_pitches_in_range(&settings);
    assert_eq!(rotating_arp_pitch(&settings, 0), pitches[0]);
    assert_eq!(rotating_arp_pitch(&settings, 1), *pitches.last().unwrap());
    assert!((settings.low_pitch()..=settings.high_pitch())
        .contains(&rotating_arp_pitch(&settings, pitches.len() + 2)));
}

#[test]
fn arp_generator_uses_configured_note_count() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Arp,
        arp_note_count: 3,
        arp_pattern: ArpPattern::Up,
        arp_rotation: ArpRotation::Off,
        bars: 1,
        density: 60,
        min_octave: 4,
        max_octave: 4,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let unique_first_cycle: Vec<u8> = song
        .notes
        .iter()
        .take(settings.arp_note_count as usize)
        .map(|note| note.pitch)
        .collect();
    assert_eq!(unique_first_cycle.len(), 3);
    assert_eq!(unique_first_cycle, vec![60, 64, 67]);
}

#[test]
fn generator_modes_include_bassline() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::Bassline));
}

#[test]
fn generator_modes_include_chord_pads() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::ChordPads));
}

#[test]
fn chord_styles_include_boards_of_canada() {
    assert!(ChordStyle::ALL.contains(&ChordStyle::BoardsOfCanada));
}

#[test]
fn boards_of_canada_chords_are_deterministic_for_fixed_seed() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::BoardsOfCanada,
        seed: 808,
        ..GeneratorSettings::default()
    };
    let first = generate_song(&settings);
    let second = generate_song(&settings);
    assert_eq!(first.chords, second.chords);
}

#[test]
fn boards_of_canada_chords_are_mostly_minor_colored() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::BoardsOfCanada,
        bars: 8,
        tension: 70,
        seed: 123,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    assert!(!song.chords.is_empty());
    assert!(song.chords.iter().all(|chord| matches!(
        chord.quality,
        ChordQuality::MinorDyad | ChordQuality::Minor7 | ChordQuality::Sus2
    )));
}

#[test]
fn boards_of_canada_high_surprise_can_borrow_roots() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::BoardsOfCanada,
        surprise: 100,
        cadence: 0,
        bars: 8,
        seed: 2,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let diatonic_roots: Vec<u8> = settings
        .scale
        .intervals()
        .iter()
        .map(|interval| ((settings.key.semitone() + *interval) as i16).rem_euclid(12) as u8)
        .collect();
    assert!(song
        .chords
        .iter()
        .any(|chord| !diatonic_roots.contains(&chord.root)));
}

#[test]
fn chord_pads_emit_stacked_notes() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        chord_style: ChordStyle::BoardsOfCanada,
        seed: 14,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let first_start = song.notes.first().map(|note| note.start_ticks).unwrap_or(0);
    assert!(
        song.notes
            .iter()
            .filter(|note| note.start_ticks <= first_start + 64)
            .count()
            >= 2
    );
}

#[test]
fn chord_pad_voicing_uses_selected_octave_range() {
    let chord = ChordEvent {
        root: 0,
        quality: ChordQuality::Major,
        degree: 0,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    };
    let narrow = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 2,
        max_octave: 2,
        ..GeneratorSettings::default()
    };
    let wide = GeneratorSettings {
        max_octave: 5,
        ..narrow
    };

    let mut rng = StdRng::seed_from_u64(1);
    let narrow_pitches = chord_pad_pitches(&narrow, &chord, &mut rng);
    let wide_pitches = chord_pad_pitches(&wide, &chord, &mut rng);

    assert!(narrow_pitches
        .iter()
        .all(|pitch| (narrow.low_pitch()..=narrow.high_pitch()).contains(pitch)));
    assert!(wide_pitches
        .iter()
        .all(|pitch| (wide.low_pitch()..=wide.high_pitch()).contains(pitch)));
    assert!(wide_pitches
        .iter()
        .any(|pitch| *pitch >= octave_to_midi_c(5)));
    assert!(wide_pitches.len() > narrow_pitches.len());
}

#[test]
fn zero_chord_inversion_preserves_spread_voicing() {
    let chord = ChordEvent {
        root: 0,
        quality: ChordQuality::Major,
        degree: 0,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    };
    let settings = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 3,
        max_octave: 5,
        chord_inversion_amount: 0,
        ..GeneratorSettings::default()
    };
    let candidates = chord_pitches_in_range(&chord, settings.low_pitch(), settings.high_pitch());
    let expected = spread_voicing(
        candidates,
        (chord.tones().len() + settings.max_octave.saturating_sub(settings.min_octave) as usize)
            .clamp(2, 8),
    );
    let mut rng = StdRng::seed_from_u64(8);

    assert_eq!(chord_pad_pitches(&settings, &chord, &mut rng), expected);
}

#[test]
fn max_chord_inversion_can_change_chord_pad_voicing() {
    let chord = ChordEvent {
        root: 0,
        quality: ChordQuality::Major,
        degree: 0,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    };
    let base = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 3,
        max_octave: 5,
        seed: 8,
        ..GeneratorSettings::default()
    };
    let inverted = GeneratorSettings {
        chord_inversion_amount: 100,
        ..base
    };
    let mut base_rng = StdRng::seed_from_u64(8);
    let mut inverted_rng = StdRng::seed_from_u64(8);

    assert_ne!(
        chord_pad_pitches(&base, &chord, &mut base_rng),
        chord_pad_pitches(&inverted, &chord, &mut inverted_rng)
    );
}

#[test]
fn inverted_chord_pad_notes_stay_in_octave_range() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 3,
        max_octave: 5,
        chord_inversion_amount: 100,
        seed: 21,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);

    assert!(!song.notes.is_empty());
    assert!(song
        .notes
        .iter()
        .all(|note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)));
}

#[test]
fn chord_pad_voice_leading_reduces_center_motion() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 3,
        max_octave: 6,
        ..GeneratorSettings::default()
    };
    let previous = vec![60, 64, 67, 72];
    let next = vec![79, 83, 86, 91];

    let led = voice_lead_chord_pad_voicing(&settings, next.clone(), &previous);

    assert!(
        (voicing_center(&led) - voicing_center(&previous)).abs()
            < (voicing_center(&next) - voicing_center(&previous)).abs()
    );
    assert!(led
        .iter()
        .all(|pitch| (settings.low_pitch()..=settings.high_pitch()).contains(pitch)));
}

#[test]
fn chord_pad_mode_changes_when_octave_range_changes() {
    let low = GeneratorSettings {
        mode: GeneratorMode::ChordPads,
        min_octave: 2,
        max_octave: 2,
        seed: 22,
        ..GeneratorSettings::default()
    };
    let high = GeneratorSettings {
        max_octave: 5,
        ..low
    };

    let low_song = generate_song(&low);
    let high_song = generate_song(&high);

    assert_ne!(
        note_signature(&low_song.notes),
        note_signature(&high_song.notes)
    );
    assert!(high_song
        .notes
        .iter()
        .any(|note| note.pitch >= octave_to_midi_c(5)));
}

#[test]
fn every_bassline_style_produces_notes() {
    for bassline_style in BasslineStyle::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(
            !song.notes.is_empty(),
            "{bassline_style} generated no notes"
        );
    }
}

#[test]
fn every_bassline_style_is_deterministic_for_fixed_seed() {
    for bassline_style in BasslineStyle::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style,
            seed: 1234,
            ..GeneratorSettings::default()
        };
        let first = generate_song(&settings);
        let second = generate_song(&settings);
        assert_eq!(first.notes.len(), second.notes.len(), "{bassline_style}");
        assert!(first.notes.iter().zip(second.notes.iter()).all(|(a, b)| {
            a.pitch == b.pitch
                && a.start_ticks == b.start_ticks
                && a.duration_ticks == b.duration_ticks
                && a.velocity == b.velocity
        }));
    }
}

#[test]
fn every_bassline_style_respects_global_octave_range() {
    for bassline_style in BasslineStyle::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style,
            min_octave: 2,
            max_octave: 3,
            density: 100,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(!song.notes.is_empty(), "{bassline_style}");
        assert!(
            song.notes
                .iter()
                .all(|note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)),
            "{bassline_style} generated a note outside the selected octave range"
        );
    }
}

#[test]
fn bassline_high_accent_creates_accent_velocity() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        bassline_style: BasslineStyle::Techno,
        density: 100,
        bassline_accent: 100,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    assert!(song.notes.iter().any(|note| note.velocity >= 116));
}

#[test]
fn drill_high_slide_creates_legato_overlap() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        bassline_style: BasslineStyle::Drill,
        density: 100,
        bassline_slide: 100,
        bassline_mutation: 100,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let step_ticks = PPQN as u32 / 4;
    assert!(song
        .notes
        .iter()
        .any(|note| note.duration_ticks > step_ticks));
}

#[test]
fn techno_density_controls_note_count() {
    let sparse = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        bassline_style: BasslineStyle::Techno,
        density: 20,
        seed: 77,
        ..GeneratorSettings::default()
    };
    let dense = GeneratorSettings {
        density: 95,
        ..sparse
    };
    assert!(generate_song(&dense).notes.len() > generate_song(&sparse).notes.len());
}

#[test]
fn preset_applies_related_generator_settings() {
    let mut settings = GeneratorSettings::default();
    settings.apply_preset(GeneratorPreset::TechnoBass);
    assert_eq!(settings.preset, GeneratorPreset::TechnoBass);
    assert_eq!(settings.mode, GeneratorMode::Bassline);
    assert_eq!(settings.bassline_style, BasslineStyle::Techno);
    assert_eq!(settings.chord_style, ChordStyle::AcidMinimal);
    assert_eq!(settings.rhythm_style, RhythmStyle::Syncopated);
    assert!(settings.bassline_accent > 70);
}

#[test]
fn bassline_presets_apply_expected_styles_and_tempos() {
    let cases = [
        (GeneratorPreset::TechnoBass, BasslineStyle::Techno, 128),
        (GeneratorPreset::HouseBass, BasslineStyle::House, 124),
        (GeneratorPreset::Drill808, BasslineStyle::Drill, 140),
        (GeneratorPreset::HipHop808, BasslineStyle::HipHop, 92),
        (GeneratorPreset::UkGarageBass, BasslineStyle::UkGarage, 127),
        (
            GeneratorPreset::DrumAndBass,
            BasslineStyle::DrumAndBass,
            174,
        ),
    ];

    for (preset, bassline_style, tempo) in cases {
        let mut settings = GeneratorSettings::default();
        settings.apply_preset(preset);
        assert_eq!(settings.mode, GeneratorMode::Bassline);
        assert_eq!(settings.bassline_style, bassline_style);
        assert_eq!(settings.tempo, tempo);
    }
}

#[test]
fn boc_chord_pads_preset_applies_expected_settings() {
    let mut settings = GeneratorSettings::default();
    settings.apply_preset(GeneratorPreset::BocChordPads);
    assert_eq!(settings.mode, GeneratorMode::ChordPads);
    assert_eq!(settings.chord_style, ChordStyle::BoardsOfCanada);
    assert_eq!(settings.scale, Scale::Dorian);
    assert_eq!(settings.tempo, 88);
    assert_eq!(settings.bars, 8);
}

#[test]
fn uk_garage_creates_swung_start_times() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        bassline_style: BasslineStyle::UkGarage,
        density: 100,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let step_ticks = PPQN as u32 / 4;
    assert!(song
        .notes
        .iter()
        .any(|note| note.start_ticks % step_ticks != 0));
}

#[test]
fn hiphop_is_sparser_than_drum_and_bass() {
    let hiphop = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        bassline_style: BasslineStyle::HipHop,
        density: 60,
        seed: 19,
        ..GeneratorSettings::default()
    };
    let dnb = GeneratorSettings {
        bassline_style: BasslineStyle::DrumAndBass,
        ..hiphop
    };
    assert!(generate_song(&dnb).notes.len() > generate_song(&hiphop).notes.len());
}

#[test]
fn chord_style_uses_expected_degree_pattern() {
    assert_eq!(chord_style_degree(ChordStyle::Pop, 0, 7), 0);
    assert_eq!(chord_style_degree(ChordStyle::Pop, 1, 7), 4);
    assert_eq!(chord_style_degree(ChordStyle::Jazz, 0, 7), 1);
    assert_eq!(chord_style_degree(ChordStyle::Jazz, 2, 7), 0);
}

#[test]
fn scale_quality_tables_match_expected_character() {
    assert_eq!(
        quality_for_degree(Scale::Major, 6),
        ChordQuality::Diminished
    );
    assert_eq!(
        quality_for_degree(Scale::Mixolydian, 6),
        ChordQuality::Major
    );
    assert_eq!(quality_for_degree(Scale::Dorian, 1), ChordQuality::Minor);
    assert_eq!(
        quality_for_degree(Scale::HarmonicMinor, 4),
        ChordQuality::Dominant
    );
    assert_eq!(
        quality_for_degree(Scale::MinorPentatonic, 0),
        ChordQuality::Minor
    );
}

#[test]
fn high_cadence_shapes_final_approach() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::Balanced,
        cadence: 100,
        surprise: 0,
        bars: 4,
        seed: 12,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);

    assert_eq!(chords.last().map(|chord| chord.degree), Some(0));
    assert!(matches!(chords[chords.len() - 2].degree, 1 | 3 | 4 | 6));
}

#[test]
fn borrowed_surprise_chords_have_intentional_quality() {
    let mut rng = StdRng::seed_from_u64(3);
    let settings = GeneratorSettings {
        surprise: 100,
        ..GeneratorSettings::default()
    };
    let borrowed = (0..32)
        .find_map(|_| borrowed_chord(0, &settings, &mut rng))
        .expect("high surprise should eventually borrow a chord");

    assert!(matches!(
        borrowed.1,
        ChordQuality::Major | ChordQuality::Minor | ChordQuality::Dominant
    ));
}

#[test]
fn high_surprise_perturbs_fixed_chord_styles() {
    let low = GeneratorSettings {
        chord_style: ChordStyle::Pop,
        surprise: 0,
        cadence: 0,
        bars: 8,
        seed: 44,
        ..GeneratorSettings::default()
    };
    let high = GeneratorSettings {
        surprise: 100,
        ..low
    };

    let low_chords = generate_song(&low).chords;
    let high_chords = generate_song(&high).chords;
    let expected_pattern: Vec<usize> = (0..high_chords.len())
        .map(|index| chord_style_degree(ChordStyle::Pop, index, high.scale.degree_count()))
        .collect();
    let high_pattern: Vec<usize> = high_chords.iter().map(|chord| chord.degree).collect();

    assert_ne!(high_pattern, expected_pattern);
    assert_ne!(high_chords, low_chords);
}

#[test]
fn high_surprise_can_use_borrowed_chromatic_roots() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::Pop,
        surprise: 100,
        cadence: 0,
        bars: 8,
        seed: 3,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let diatonic_roots: Vec<u8> = settings
        .scale
        .intervals()
        .iter()
        .map(|interval| ((settings.key.semitone() + *interval) as i16).rem_euclid(12) as u8)
        .collect();

    assert!(song
        .chords
        .iter()
        .any(|chord| !diatonic_roots.contains(&chord.root)));
}

#[test]
fn rhythm_style_adjusts_density() {
    let base = GeneratorSettings {
        density: 50,
        rhythm_style: RhythmStyle::Straight,
        ..GeneratorSettings::default()
    };
    let sparse = GeneratorSettings {
        rhythm_style: RhythmStyle::Sparse,
        ..base
    };
    let busy = GeneratorSettings {
        rhythm_style: RhythmStyle::Busy,
        ..base
    };
    assert!(rhythm_density(&sparse) < rhythm_density(&base));
    assert!(rhythm_density(&busy) > rhythm_density(&base));
}

#[test]
fn phrase_memory_repeats_first_phrase_when_fully_enabled() {
    let settings = GeneratorSettings {
        bars: 4,
        phrase_length: 1,
        repeat_amount: 100,
        variation_amount: 0,
        ..GeneratorSettings::default()
    };
    let notes = vec![NoteEvent {
        pitch: 60,
        start_ticks: 0,
        duration_ticks: 120,
        velocity: 90,
    }];
    let mut rng = StdRng::seed_from_u64(5);
    let repeated = apply_phrase_memory(&settings, notes, &mut rng);
    let starts: Vec<u32> = repeated.iter().map(|note| note.start_ticks).collect();
    assert_eq!(starts, vec![0, 1920, 3840, 5760]);
}

#[test]
fn phrase_length_clamps_to_bar_count() {
    let mut settings = GeneratorSettings {
        bars: 4,
        ..GeneratorSettings::default()
    };
    settings.set_phrase_length(8);
    assert_eq!(settings.phrase_length, 4);
    settings.set_phrase_length(0);
    assert_eq!(settings.phrase_length, 1);
}
