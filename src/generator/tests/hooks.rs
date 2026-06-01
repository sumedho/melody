use super::*;

#[test]
fn every_hook_type_produces_notes() {
    for hook_type in HookType::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Hook,
            hook_type,
            repeat_amount: 0,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(!song.notes.is_empty(), "{hook_type} generated no notes");
    }
}

#[test]
fn hook_generation_is_deterministic_for_fixed_seed() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::FourNoteLoop,
        seed: 901,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };

    let first = generate_song(&settings);
    let second = generate_song(&settings);

    assert_eq!(note_signature(&first.notes), note_signature(&second.notes));
    assert_eq!(first.chords, second.chords);
}

#[test]
fn every_hook_type_respects_octave_range() {
    for hook_type in HookType::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Hook,
            hook_type,
            min_octave: 2,
            max_octave: 3,
            repeat_amount: 0,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);

        assert!(
            song.notes
                .iter()
                .all(|note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)),
            "{hook_type} generated a note outside the selected octave range"
        );
    }
}

#[test]
fn every_hook_type_honors_random_velocity_range() {
    for hook_type in HookType::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Hook,
            hook_type,
            velocity_mode: VelocityMode::Random,
            random_velocity_min: 31,
            random_velocity_max: 36,
            repeat_amount: 0,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);

        assert!(!song.notes.is_empty());
        assert!(
            song.notes
                .iter()
                .all(|note| (31..=36).contains(&note.velocity)),
            "{hook_type} ignored the configured random velocity range"
        );
    }
}

#[test]
fn four_note_hook_repeats_seed_motif() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::FourNoteLoop,
        bars: 2,
        variation_amount: 0,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let bar_ticks = ticks_per_bar();
    let first_bar: Vec<u8> = song
        .notes
        .iter()
        .filter(|note| note.start_ticks < bar_ticks)
        .map(|note| note.pitch)
        .collect();
    let second_bar: Vec<u8> = song
        .notes
        .iter()
        .filter(|note| note.start_ticks >= bar_ticks && note.start_ticks < bar_ticks * 2)
        .map(|note| note.pitch)
        .collect();

    assert_eq!(first_bar.len(), 4);
    assert_eq!(first_bar, second_bar);
}

#[test]
fn call_response_hook_leaves_mid_phrase_gap() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::CallResponse,
        bars: 1,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let gap_start = PPQN as u32;
    let gap_end = PPQN as u32 * 2;

    assert!(song
        .notes
        .iter()
        .all(|note| note.start_ticks < gap_start || note.start_ticks >= gap_end));
}

#[test]
fn motif_develop_hook_adds_notes_across_repeats() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::MotifDevelop,
        bars: 4,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let bar_ticks = ticks_per_bar();
    let counts: Vec<usize> = (0..4)
        .map(|bar| {
            let start = bar * bar_ticks;
            let end = start + bar_ticks;
            song.notes
                .iter()
                .filter(|note| note.start_ticks >= start && note.start_ticks < end)
                .count()
        })
        .collect();

    assert_eq!(counts, vec![1, 2, 4, 5]);
}

#[test]
fn stutter_hook_increases_density_within_bar() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::StutterHook,
        bars: 1,
        density: 100,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let counts: Vec<usize> = (0..4)
        .map(|beat| {
            let start = beat * PPQN as u32;
            let end = start + PPQN as u32;
            song.notes
                .iter()
                .filter(|note| note.start_ticks >= start && note.start_ticks < end)
                .count()
        })
        .collect();

    assert_eq!(counts, vec![1, 2, 3, 4]);
}

#[test]
fn descending_bass_hook_follows_pop_root_motion() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::Hook,
        hook_type: HookType::DescendingBass,
        bars: 4,
        min_octave: 2,
        max_octave: 4,
        repeat_amount: 0,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let expected_degrees = [5usize, 3, 0, 4];
    let actual_roots: Vec<u8> = (0..4)
        .map(|bar| {
            let start = bar * ticks_per_bar();
            song.notes
                .iter()
                .find(|note| note.start_ticks == start)
                .map(|note| note.pitch % 12)
                .expect("bar root note")
        })
        .collect();
    let expected_roots: Vec<u8> = expected_degrees
        .iter()
        .map(|degree| pitch_class_for_degree(settings.key, settings.scale, *degree))
        .collect();

    assert_eq!(actual_roots, expected_roots);
}
