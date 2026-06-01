use super::*;

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
fn extended_chord_qualities_have_expected_labels_and_tones() {
    let cases = [
        (ChordQuality::Maj7, "Cmaj7 I", vec![0, 4, 7, 11]),
        (ChordQuality::Maj9, "Cmaj9 I", vec![0, 4, 7, 11, 2]),
        (ChordQuality::Min9, "Cm9 i", vec![0, 3, 7, 10, 2]),
        (ChordQuality::Sus4, "Csus4 I", vec![0, 5, 7]),
        (ChordQuality::Add11, "Cadd11 I", vec![0, 4, 7, 5]),
        (ChordQuality::Add13, "Cadd13 I", vec![0, 4, 7, 9]),
    ];

    for (quality, label, tones) in cases {
        let chord = ChordEvent {
            root: 0,
            quality,
            slash_bass: None,
            degree: 0,
            start_ticks: 0,
            duration_ticks: ticks_per_bar(),
            tension: 0,
        };

        assert_eq!(chord.label(), label);
        assert_eq!(chord.tones(), tones);
    }

    let slash = ChordEvent {
        root: 0,
        quality: ChordQuality::Major,
        slash_bass: Some(4),
        degree: 0,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    };
    assert_eq!(slash.label(), "C/E I");
    assert_eq!(slash.tones(), vec![0, 4, 7]);
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
fn locked_chords_preserve_slash_bass_labels() {
    let locked_chords = vec![ChordEvent {
        root: 0,
        quality: ChordQuality::Major,
        slash_bass: Some(4),
        degree: 0,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    }];
    let song = generate_song_with_chords(
        &GeneratorSettings {
            bars: 2,
            ..GeneratorSettings::default()
        },
        Some(&locked_chords),
    );

    assert_eq!(song.chords.len(), 2);
    assert!(song
        .chords
        .iter()
        .all(|chord| chord.slash_bass == Some(4) && chord.label().starts_with("C/E")));
}

#[test]
fn high_tension_surprise_can_generate_slash_chord_labels() {
    let generated_slash = (0..32).any(|seed| {
        let song = generate_song(&GeneratorSettings {
            chord_style: ChordStyle::Pop,
            tension: 100,
            surprise: 100,
            cadence: 0,
            bars: 8,
            seed,
            ..GeneratorSettings::default()
        });

        song.chords
            .iter()
            .any(|chord| chord.slash_bass.is_some() && chord.label().contains('/'))
    });

    assert!(generated_slash);
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
            slash_bass: None,
            degree: 0,
            start_ticks: 0,
            duration_ticks: ticks_per_bar() * 2,
            tension: 70,
        },
        ChordEvent {
            root: 8,
            quality: ChordQuality::Dominant,
            slash_bass: None,
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
fn pop_descent_chord_style_uses_vi_iv_i_v_cycle() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::PopDescent,
        density: 100,
        bars: 4,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let degrees: Vec<usize> = chords.iter().map(|chord| chord.degree).collect();

    assert_eq!(degrees, vec![5, 3, 0, 4]);
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
        ChordQuality::MinorDyad | ChordQuality::Minor7 | ChordQuality::Min9 | ChordQuality::Sus2
    )));
}

#[test]
fn chiptune_loop_favors_simple_game_chord_colors() {
    let song = generate_song(&GeneratorSettings {
        chord_style: ChordStyle::ChiptuneLoop,
        bars: 8,
        seed: 64,
        ..GeneratorSettings::default()
    });

    assert!(!song.chords.is_empty());
    assert!(song.chords.iter().all(|chord| !matches!(
        chord.quality,
        ChordQuality::Maj9 | ChordQuality::Min9 | ChordQuality::Add11 | ChordQuality::Add13
    )));
}

#[test]
fn extension_quality_preserves_minor_chord_color() {
    let settings = GeneratorSettings {
        chord_style: ChordStyle::Pop,
        scale: Scale::NaturalMinor,
        tension: 100,
        surprise: 100,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(4);
    let quality = extension_quality(&settings, 0, ChordQuality::Minor, &mut rng);

    assert!(matches!(
        quality,
        ChordQuality::Minor
            | ChordQuality::Minor7
            | ChordQuality::Min9
            | ChordQuality::Add9
            | ChordQuality::Sus2
            | ChordQuality::Sus4
    ));
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
