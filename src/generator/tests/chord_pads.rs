use super::*;

#[test]
fn generator_modes_include_chord_pads() {
    assert!(GeneratorMode::ALL.contains(&GeneratorMode::ChordPads));
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
        slash_bass: None,
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
        slash_bass: None,
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
        slash_bass: None,
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
fn boc_chord_pads_preset_applies_expected_settings() {
    let mut settings = GeneratorSettings::default();
    settings.apply_preset(GeneratorPreset::BocChordPads);
    assert_eq!(settings.mode, GeneratorMode::ChordPads);
    assert_eq!(settings.chord_style, ChordStyle::BoardsOfCanada);
    assert_eq!(settings.scale, Scale::Dorian);
    assert_eq!(settings.tempo, 88);
    assert_eq!(settings.bars, 8);
}
