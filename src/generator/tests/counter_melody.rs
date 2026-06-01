use super::*;

#[test]
fn generator_modes_include_counter_melody() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::CounterMelody));
}

#[test]
fn counter_melody_is_deterministic_for_fixed_seed() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        seed: 2024,
        ..GeneratorSettings::default()
    };
    let first = generate_song(&settings);
    let second = generate_song(&settings);

    assert_eq!(note_signature(&first.notes), note_signature(&second.notes));
}

#[test]
fn counter_melody_produces_main_and_counter_parts() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 80,
        repeat_amount: 0,
        seed: 91,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let parts = generate_counter_melody_parts(&settings, &chords, &mut rng);

    assert!(!parts.main.is_empty());
    assert!(!parts.counter.is_empty());
    assert_eq!(song.notes.len(), parts.main.len() + parts.counter.len());
}

#[test]
fn counter_melody_uses_complementary_lower_register() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 80,
        repeat_amount: 0,
        seed: 92,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let parts = generate_counter_melody_parts(&settings, &chords, &mut rng);
    let highest_counter = parts.counter.iter().map(|note| note.pitch).max().unwrap();
    let lowest_main = parts.main.iter().map(|note| note.pitch).min().unwrap();

    assert!(highest_counter < lowest_main);
}

#[test]
fn counter_melody_fills_main_melody_gaps() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 80,
        repeat_amount: 0,
        seed: 93,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let parts = generate_counter_melody_parts(&settings, &chords, &mut rng);

    assert!(!parts.counter.is_empty());
    assert!(parts
        .counter
        .iter()
        .all(|note| !note_active_at(&parts.main, note.start_ticks)));
}

#[test]
fn counter_melody_uses_contrary_motion_when_main_moves() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 85,
        repeat_amount: 0,
        seed: 94,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let parts = generate_counter_melody_parts(&settings, &chords, &mut rng);

    let contrary = parts.counter.iter().any(|counter| {
        let Some(previous_main) = parts
            .main
            .iter()
            .rev()
            .find(|note| note.start_ticks < counter.start_ticks)
        else {
            return false;
        };
        let Some(next_main) = parts
            .main
            .iter()
            .find(|note| note.start_ticks > counter.start_ticks)
        else {
            return false;
        };
        let main_motion = next_main.pitch as i16 - previous_main.pitch as i16;
        let counter_motion = counter.pitch as i16 - previous_main.pitch as i16;
        (main_motion > 0 && counter_motion < 0) || (main_motion < 0 && counter_motion > 0)
    });

    assert!(contrary);
}

#[test]
fn counter_melody_resolves_to_chord_tones_on_strong_beats() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 100,
        repeat_amount: 0,
        seed: 95,
        ..GeneratorSettings::default()
    };
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = generate_chords(&settings, &mut rng);
    let parts = generate_counter_melody_parts(&settings, &chords, &mut rng);
    let strong_counter_notes: Vec<&NoteEvent> = parts
        .counter
        .iter()
        .filter(|note| note.start_ticks.is_multiple_of(PPQN as u32))
        .collect();

    assert!(!strong_counter_notes.is_empty());
    assert!(strong_counter_notes.iter().all(|note| {
        let chord = chord_at(&chords, note.start_ticks);
        chord.tones().contains(&(note.pitch % 12))
    }));
}

#[test]
fn counter_melody_respects_combined_per_bar_density() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::CounterMelody,
        min_octave: 3,
        max_octave: 6,
        density: 45,
        repeat_amount: 100,
        seed: 96,
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
        assert!(count <= max_per_bar);
    }
}
