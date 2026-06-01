use super::*;

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
