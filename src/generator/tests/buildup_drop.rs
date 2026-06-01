use super::*;

#[test]
fn generator_modes_include_buildup_drop() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::BuildupDrop));
}

#[test]
fn drop_types_have_display_labels() {
    assert_eq!(DropType::ALL.len(), 5);
    assert_eq!(DropType::BassDrop.to_string(), "Bass drop");
    assert_eq!(DropType::SupersawDrop.to_string(), "Supersaw drop");
    assert_eq!(DropType::HalfTimeDrop.to_string(), "Half-time drop");
    assert_eq!(DropType::FillDrop.to_string(), "Fill drop");
    assert_eq!(DropType::VocalDrop.to_string(), "Vocal drop");
}

#[test]
fn every_drop_type_produces_notes_and_is_deterministic() {
    for drop_type in DropType::ALL {
        let settings = GeneratorSettings {
            mode: GeneratorMode::BuildupDrop,
            drop_type,
            bars: 8,
            density: 80,
            seed: 440,
            ..GeneratorSettings::default()
        };
        let first = generate_song(&settings);
        let second = generate_song(&settings);

        assert!(!first.notes.is_empty(), "{drop_type}");
        assert_eq!(note_signature(&first.notes), note_signature(&second.notes));
        assert!(first
            .notes
            .iter()
            .all(|note| { (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch) }));
    }
}

#[test]
fn buildup_drop_increases_buildup_density() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        bars: 8,
        density: 90,
        seed: 441,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let sections = buildup_drop_sections(&settings);
    let first_bar = song
        .notes
        .iter()
        .filter(|note| note.start_ticks < ticks_per_bar())
        .count();
    let later_bar_start = sections.silence_start.saturating_sub(ticks_per_bar());
    let later_bar = song
        .notes
        .iter()
        .filter(|note| {
            note.start_ticks >= later_bar_start && note.start_ticks < sections.silence_start
        })
        .count();

    assert!(later_bar > first_bar);
}

#[test]
fn buildup_drop_has_ascending_louder_riser() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        bars: 8,
        seed: 442,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let sections = buildup_drop_sections(&settings);
    let riser_start = sections.silence_start.saturating_sub(ticks_per_bar());
    let riser: Vec<&NoteEvent> = song
        .notes
        .iter()
        .filter(|note| note.start_ticks >= riser_start && note.start_ticks < sections.silence_start)
        .collect();

    assert!(riser.len() >= 4);
    assert!(riser.last().unwrap().pitch > riser.first().unwrap().pitch);
    assert!(riser.last().unwrap().velocity > riser.first().unwrap().velocity);
    assert!(riser.last().unwrap().duration_ticks <= riser.first().unwrap().duration_ticks);
}

#[test]
fn buildup_drop_leaves_pre_drop_silence() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        bars: 8,
        seed: 443,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let sections = buildup_drop_sections(&settings);
    let silence_start = sections.drop_start - PPQN as u32 / 2;

    assert!(song.notes.iter().all(|note| {
        !(note.start_ticks >= silence_start && note.start_ticks < sections.drop_start)
    }));
}

#[test]
fn buildup_drop_impact_lands_on_drop_start() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        bars: 8,
        seed: 444,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let sections = buildup_drop_sections(&settings);
    let impact: Vec<&NoteEvent> = song
        .notes
        .iter()
        .filter(|note| note.start_ticks == sections.drop_start)
        .collect();
    let chord = chord_at(&song.chords, sections.drop_start);

    assert!(impact
        .iter()
        .any(|note| note.pitch <= settings.low_pitch() + 12));
    assert!(
        impact
            .iter()
            .filter(|note| chord.tones().contains(&(note.pitch % 12)))
            .count()
            >= 2
    );
}

#[test]
fn half_time_drop_has_fewer_longer_notes_than_bass_drop() {
    let base = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        bars: 8,
        density: 100,
        seed: 445,
        ..GeneratorSettings::default()
    };
    let bass = generate_song(&GeneratorSettings {
        drop_type: DropType::BassDrop,
        ..base
    });
    let half = generate_song(&GeneratorSettings {
        drop_type: DropType::HalfTimeDrop,
        ..base
    });
    let sections = buildup_drop_sections(&base);
    let bass_drop_notes: Vec<&NoteEvent> = bass
        .notes
        .iter()
        .filter(|note| note.start_ticks >= sections.drop_start)
        .collect();
    let half_drop_notes: Vec<&NoteEvent> = half
        .notes
        .iter()
        .filter(|note| note.start_ticks >= sections.drop_start)
        .collect();
    let bass_avg = bass_drop_notes
        .iter()
        .map(|note| note.duration_ticks)
        .sum::<u32>() as f32
        / bass_drop_notes.len() as f32;
    let half_avg = half_drop_notes
        .iter()
        .map(|note| note.duration_ticks)
        .sum::<u32>() as f32
        / half_drop_notes.len() as f32;

    assert!(half_drop_notes.len() < bass_drop_notes.len());
    assert!(half_avg > bass_avg);
}

#[test]
fn vocal_drop_uses_short_upper_repeated_notes() {
    let settings = GeneratorSettings {
        mode: GeneratorMode::BuildupDrop,
        drop_type: DropType::VocalDrop,
        bars: 8,
        min_octave: 2,
        max_octave: 6,
        seed: 446,
        ..GeneratorSettings::default()
    };
    let song = generate_song(&settings);
    let sections = buildup_drop_sections(&settings);
    let upper_floor = settings.low_pitch() + (settings.high_pitch() - settings.low_pitch()) / 2;
    let vocal_notes: Vec<&NoteEvent> = song
        .notes
        .iter()
        .filter(|note| {
            note.start_ticks >= sections.drop_start
                && note.duration_ticks <= PPQN as u32 / 8
                && note.pitch >= upper_floor
        })
        .collect();

    assert!(vocal_notes.len() >= 8);
}
