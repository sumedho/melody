use super::*;

#[test]
fn locked_chords_are_reused_by_every_bassline_style() {
    let locked_chords = vec![
        ChordEvent {
            root: 3,
            quality: ChordQuality::Minor,
            slash_bass: None,
            degree: 0,
            start_ticks: 0,
            duration_ticks: ticks_per_bar() * 2,
            tension: 35,
        },
        ChordEvent {
            root: 10,
            quality: ChordQuality::Suspended,
            slash_bass: None,
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
fn generator_modes_include_bassline() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::Bassline));
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
fn bass_degree_pitches_follow_current_chord_tones() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Bassline,
        key: Key::C,
        scale: Scale::Major,
        min_octave: 5,
        max_octave: 6,
        bassline_octave_jump: 0,
        ..GeneratorSettings::default()
    };
    let chord = ChordEvent {
        root: 5,
        quality: ChordQuality::Major,
        slash_bass: None,
        degree: 3,
        start_ticks: 0,
        duration_ticks: ticks_per_bar(),
        tension: 0,
    };
    let mut rng = StdRng::seed_from_u64(9);

    let root = choose_bass_degree_pitch(&settings, &chord, 0, &mut rng);
    let third = choose_bass_degree_pitch(&settings, &chord, 2, &mut rng);
    let fifth = choose_bass_degree_pitch(&settings, &chord, 4, &mut rng);

    assert_eq!(root % 12, 5);
    assert_eq!(third % 12, 9);
    assert_eq!(fifth % 12, 0);
    assert!([root, third, fifth]
        .iter()
        .all(|pitch| (settings.low_pitch()..=settings.high_pitch()).contains(pitch)));
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
