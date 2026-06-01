use super::*;

#[test]
fn euclidean_pattern_has_requested_pulses() {
    let pattern = euclidean_pattern(16, 5, 3);
    assert_eq!(pattern.len(), 16);
    assert_eq!(pattern.iter().filter(|active| **active).count(), 5);
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
fn density_caps_notes_per_bar_for_note_generators() {
    for mode in GeneratorMode::ALL {
        if matches!(mode, GeneratorMode::ChordPads | GeneratorMode::BuildupDrop) {
            continue;
        }

        let settings = GeneratorSettings {
            mode,
            hook_type: HookType::StutterHook,
            rhythm_style: RhythmStyle::Busy,
            bars: 4,
            density: 25,
            repeat_amount: 100,
            seed: 5150,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        let max_per_bar = density_notes_per_bar(&settings);

        for bar in 0..settings.bars as u32 {
            let start = bar * ticks_per_bar();
            let end = start + ticks_per_bar();
            let count = song
                .notes
                .iter()
                .filter(|note| note.start_ticks >= start && note.start_ticks < end)
                .count();
            assert!(
                count <= max_per_bar,
                "{mode} generated {count} notes in bar {} with a per-bar density limit of {max_per_bar}",
                bar + 1
            );
        }
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
fn high_tension_pop_and_jazz_generate_modern_extensions() {
    for chord_style in [ChordStyle::Pop, ChordStyle::Jazz] {
        let generated_extension = (0..16).any(|seed| {
            let song = generate_song(&GeneratorSettings {
                chord_style,
                tension: 100,
                surprise: 60,
                cadence: 0,
                bars: 8,
                seed,
                ..GeneratorSettings::default()
            });
            song.chords.iter().any(|chord| {
                matches!(
                    chord.quality,
                    ChordQuality::Maj7
                        | ChordQuality::Maj9
                        | ChordQuality::Min9
                        | ChordQuality::Sus4
                        | ChordQuality::Add11
                        | ChordQuality::Add13
                )
            })
        });

        assert!(
            generated_extension,
            "{chord_style} should generate modern chord extensions"
        );
    }
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
