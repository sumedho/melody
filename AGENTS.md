# AGENTS.md — Melody Codebase Guide

## What this project is

**Melody** is a Rust desktop GUI application that procedurally generates musical melodies, hooks, basslines, arpeggios, chord pads, and sequences — then exports them as MIDI files. It uses the `iced` GUI toolkit for a dark-themed window with a sidebar of collapsible controls and a piano-roll-style timeline preview.

## Project structure

```
src/
├── main.rs          — Entry point. Calls app::run().
├── constants.rs     — PPQN, tempo range, octave range, note gate ratios, swing factor, and other tuning constants.
├── music.rs         — Helpers: note_name(), pitch_class_name(), roman_degree().
├── midi.rs          — MIDI file writing (midly Smf), export path normalization, unique filename generation. Contains unit tests.
├── ui.rs            — PreviewNoteIndex, PreviewStep, NoteSegment, GridLine. Builds the piano-roll grid. Contains unit tests.
├── app/
│   ├── mod.rs       — MelodyApp, MusicState, ExportState, UIState structs. Message enum. Application trait impl.
│   ├── update.rs    — All Message handling logic. Export directory picker (async).
│   ├── view.rs      — All UI rendering: top bar, sidebar sections, preview panel, timeline cells.
│   ├── sidebar.rs   — SidebarSection enum, SidebarState, expandable_group widget.
│   ├── widgets.rs   — UI components: labeled_pick, segmented_control, toolbar_button, sliders. All styling functions (panel_style, chord_style, summary_style, field_style, group_style, timeline_cell_style, active_note_colors).
│   └── tests.rs     — App-level integration tests: preset tracking, export path behavior, segmented controls.
└── generator/
    ├── mod.rs       — GeneratedSong, NoteEvent, ChordEvent, ChordQuality enum. generate_song(), generate_song_with_chords().
    ├── settings.rs  — GeneratorSettings struct, all enums (GeneratorMode, GeneratorPreset, Key, Scale, ChordStyle, RhythmStyle, BasslineStyle, ArpPattern, ArpRotation, HookType, VelocityMode, SeedBehavior). Preset definitions array.
    ├── pipeline.rs  — SongPipeline builder: new(), with_chords(), generate_mode(), apply_phrase_memory(), apply_velocity_range(), build(). locked_chords_for_song().
    ├── common.rs    — Shared helpers: rhythm_density, apply_phrase_memory, vary_pitch_by_scale_step, cleanup_notes, apply_velocity_range, choose_chord_or_scale_pitch, chord_pitches_in_range, chord_at, velocity_for, note_duration, quality_for_degree, scale_pitch, scale_pitches_in_range, nearest_scale_pitch, octave_to_midi_c, ticks_per_bar.
    ├── chords.rs    — generate_chords(), generate_boards_of_canada_chords(), boc_progression_pattern, chord_style_degree, choose_next_degree, surprising_degree, cadence_approach_degree, borrowed_chord, surprise_quality, tension_quality.
    ├── melody.rs    — generate_melodic(), melodic_rhythm_patterns(), choose_melodic_pitch().
    ├── hook.rs      — generate_hook() dispatch, four_note_loop, call_response, motif_develop, stutter_hook, descending_bass, four_note_seed, developed_motif, nearby_scale_pitch.
    ├── euclidean.rs — generate_euclidean(), euclidean_pattern().
    ├── arp.rs       — generate_arp(), arp_pattern_pitches(), arp_order(), random_walk_order(), rotating_arp_pitch().
    ├── chiptune.rs  — generate_chiptune() with motif-based gated leads and octave jumps.
    ├── bassline.rs  — generate_bassline() dispatch, techno/house/drill/hip-hop/uk-garage/drum-and-bass generators, choose_bassline_pitch, choose_bass_degree_pitch, bassline_chord_candidates, is_bassline_accented, should_bassline_slide.
    ├── chord_pads.rs — generate_chord_pads(), chord_pad_pitches(), spread_voicing(), maybe_invert_chord_pad_voicing(), invert_chord_pad_voicing(), voice_lead_chord_pad_voicing(), voicing_center().
    └── tests.rs     — Comprehensive generator tests: every mode, every bassline style, preset application, locked chords, arp ordering, chord pad voicing, velocity ranges, phrase memory, rhythm density.
```

## Key types

### `GeneratorSettings` (generator/settings.rs)
The single struct holding all user-facing parameters. Passed to `generate_song()` to produce deterministic output. Key fields:
- `preset: GeneratorPreset` — tracks whether user has manually tweaked (auto-sets to `Custom` on edit)
- `mode: GeneratorMode` — which generator algorithm to use (7 modes)
- `key: Key`, `scale: Scale` — tonal center and scale
- `bars: u16`, `tempo: u16` — structure
- `seed: u64`, `seed_behavior: SeedBehavior` — determinism
- `chord_style: ChordStyle`, `tension`, `surprise`, `cadence`, `chord_inversion_amount` — harmony parameters
- `rhythm_style: RhythmStyle`, `density`, `note_length` — rhythm parameters
- `hook_type: HookType` — hook-specific pattern type (5 types)
- `phrase_length`, `repeat_amount`, `variation_amount` — phrase memory
- `min_octave`, `max_octave` — pitch range (clamped via `set_min_octave`/`set_max_octave`)
- Bassline-specific: `bassline_style`, `bassline_accent/slide/octave_jump/mutation`
- Arp-specific: `arp_note_count`, `arp_pattern`, `arp_rotation`, `arp_rotate_slot` (clamped via `set_arp_note_count`/`set_arp_rotate_slot`)
- `velocity_mode: VelocityMode`, `random_velocity_min/max` (clamped via `set_random_velocity_min`/`set_random_velocity_max`)
- `low_pitch()` / `high_pitch()` — computed convenience methods

### `GeneratedSong` (generator/mod.rs)
Output of the generator: `notes: Vec<NoteEvent>` and `chords: Vec<ChordEvent>`.

### `NoteEvent` (generator/mod.rs)
Individual note: `pitch`, `start_ticks`, `duration_ticks`, `velocity`.

### `ChordEvent` (generator/mod.rs)
Individual chord: `root`, `quality: ChordQuality`, `degree`, `start_ticks`, `duration_ticks`, `tension`. Has `label()` and `tones()` methods.

### `Message` (app/mod.rs)
All UI events. Each maps to a field update or a generate/export action. `update_setting()` helper auto-sets `preset = Custom` on music/harmony/rhythm edits but NOT on seed/export edits.

### `MelodyApp` (app/mod.rs)
The `iced::Application` implementation, composed of three state structs:
- `MusicState` — `settings`, `output`, `locked_chords`
- `ExportState` — `filename`, `directory`, `path_auto`
- `UIState` — `sidebar` (SidebarState), `seed_input`, `status`
- `theme()` returns `Theme::TokyoNight`
- Window size: 1180×760

### `SongPipeline` (generator/pipeline.rs)
Builder-pattern pipeline for generation:
```
SongPipeline::new(settings, rng)
    .with_chords(locked_chords)    → generates or reuses chords
    .generate_mode()               → dispatches to mode-specific generator
    .apply_phrase_memory()         → copies phrase template with variation
    .apply_velocity_range()        → clamps velocities for Random mode
    .build()                       → returns GeneratedSong
```

## Generation pipeline

```
GeneratorSettings + seed
    │
    ▼
SongPipeline::new(settings, rng)
    │
    ├── with_chords(locked_chords)
    │   ├── locked? → locked_chords_for_song() cycles chord template
    │   └── not locked? → chords::generate_chords()
    │       ├── BoardsOfCanada → boc_progression_pattern (grounded/wandering)
    │       └── others → chord_style_degree() + cadence/surprise/borrowed logic
    │
    ├── generate_mode()
    │   ├── Melodic → melody::generate_melodic() (rhythm patterns + chord-aware pitch)
    │   ├── Hook    → hook::generate_hook() (5 hook types)
    │   ├── Euclidean → euclidean::generate_euclidean() (Bjorklund distribution)
    │   ├── Arp     → arp::generate_arp() (chord tones + patterns + rotation)
    │   ├── Chiptune → chiptune::generate_chiptune() (motif + octave jumps)
    │   ├── Bassline → bassline::generate_bassline() (6 genre patterns)
    │   └── ChordPads → chord_pads::generate_chord_pads() (spread voicing + voice leading)
    │
    ├── apply_phrase_memory() — repeat/variation across phrase bars
    ├── apply_velocity_range() — velocity mode enforcement
    └── build() → GeneratedSong { notes, chords }
```

### Determinism
Uses `rand::rngs::StdRng::seed_from_u64(seed)` throughout. Same seed + same settings = identical output. The `locked_chords` feature bypasses chord generation entirely and repeats a chord cycle across the song length (with clipping or expansion as needed).

### Chord generation highlights
- `generate_chords()` iterates tick by tick, assigning chord events
- `chord_style_degree()` maps style → degree progression patterns
- `choose_next_degree()` uses tension/surprise to decide functional moves vs. leaps
- `borrowed_chord()` introduces chromatic mediants at high surprise (surprise > 35)
- `tension_quality()` upgrades to dominant/sus/add9 at high tension
- Boards of Canada style uses special `generate_boards_of_canada_chords()` with grounded/wandering progressions, MinorDyad/Sus2/Minor7 qualities

### Bassline generation highlights
Each style has its own pattern array and probability model:
- **Techno**: 16-step grid with downbeat/syncopation bonuses, accent/slide logic
- **House**: Fixed pattern `[2,4,6,10,12,14]` per bar, degree-based pitch selection
- **Drill**: 4-bar group patterns `[0,6,11,16,24,30,42,48,54,59]` with slide pickups
- **Hip-hop**: 4-bar group sparse patterns `[0,7,12,22,32,38,44,55]` with mutation
- **UK Garage**: Swing offset (`UKG_SWING_FACTOR = 0.42`) on odd steps
- **Drum & Bass**: Sparse pattern `[0,3,7,10,14]` with mutation on off-beats

### Hook generation highlights
Five hook types, each with a 4-note seed derived from chord tones:
- **Four-note loop**: Repeats seed motif per bar, with probabilistic variation
- **Call & response**: Two-note call + two-note response per bar, with gap on beat 2
- **Motif develop**: Gradually adds notes across 4-bar cycles (1→2→4→5 notes)
- **Stutter hook**: Increasing repeats per beat (1, 2, 3, 4), with density gating
- **Descending bass**: Root motion on degrees [5, 3, 0, 4] with fifth/octave pattern

### Phrase memory
`apply_phrase_memory()` copies a phrase template (first N bars) into subsequent phrase blocks, with probabilistic repeat and pitch/velocity variation. `set_phrase_length()` clamps to `bars.min(8)`.

### Chord pad voicing
- `spread_voicing()` distributes candidates evenly across range
- `maybe_invert_chord_pad_voicing()` lifts lowest notes by octave based on inversion amount
- `voice_lead_chord_pad_voicing()` selects voicing with minimum center-of-gravity motion from previous chord (tries shifts ±12 and inversions)

## UI layout (app/view.rs)

```
┌──────────────────────────────────────────────────────────┐
│  Melody                          │  Generate  │  ...     │  ← Top bar
│  {mode} generator                │  Randomize │  Browse  │
│                                  │            │  Export  │
│  Directory: [exports]  Filename: [melody.mid]           │
├──────────────────┬───────────────────────────────────────┤
│  Sidebar (286px) │  Preview panel                        │
│                  │                                       │
│  [v] Mode        │  Preview  │  C 4 bars               │
│    Preset: ...   │  ─────────┼───────────────────        │
│    Generator: ...│  Summary: melodic C Major ...       │
│    (help text)   │  ─────────┼───┬───┬───┬───          │
│    [Hook] Type   │  Bars    │ 1 │ 2 │ 3 │ 4             │
│    [Arp] Notes   │  Chords  │ I │ IV│ V │ I             │
│    [Bass] Style  │  ────────┼───┴───┴───┴───            │
│                  │  Bb3     │ ░░│░░░│░░│░░░            │
│  [v] Music       │  A3      │ ░ │░░░│░│░░░             │
│    Key: ...      │  G3      │ ░░│░░│░░░│░               │
│    Scale: ...    │  ...     │ ...                       │
│    Bars: ...     │  C3      │░░░│░░│░░░│░░             │
│    ...           │  ────────┴───┴───┴───┴───            │
│  [>] Harmony     │                                       │
│    Lock chords   │                                       │
│    Chord style   │                                       │
│    Tension: ...  │                                       │
│    ...           │                                       │
│  [v] Rhythm      │                                       │
│    ...           │                                       │
│  [>] Phrase      │                                       │
│    ...           │                                       │
│  [v] Velocity    │                                       │
│    ...           │                                       │
│  [>] Seed        │                                       │
│    ...           │                                       │
├──────────────────┴───────────────────────────────────────┤
│  Generated 48 notes across 4 chord changes.              │  ← Status bar
└──────────────────────────────────────────────────────────┘
```

## Sidebar sections

| Section | Default Open | Controls |
|---------|-------------|----------|
| **Mode** | ✓ | Preset, Generator (segmented 3-across), help text, Hook type (conditional), Arp settings (conditional), Bassline settings (conditional) |
| **Music** | ✓ | Key (12), Scale (8), Bars (1–16), Tempo (60–180), Min/Max octave (1–8) |
| **Harmony** | ✗ | Lock chords toggle, Chord style (9), Tension, Chord surprise, Resolution (cadence), Chord inversion |
| **Rhythm** | ✓ | Rhythm style (5), Note density, Gate / overlap |
| **Phrase** | ✗ | Phrase bars, Repeat, Variation |
| **Velocity** | ✓ | Velocity mode (segmented 4-across), Random range lower/upper |
| **Seed** | ✗ | Seed mode (Locked / Randomize on generate), Seed input |

## Styling (app/widgets.rs)

Uses custom `theme::Container` styles via the `|_theme: &Theme| ContainerAppearance { ... }` closure pattern. All panels use dark Tokyo Night colors:
- `panel_style()` — dark background (24,28,42) with subtle borders
- `chord_style()` — purple-tinted chord lane (38,48,79)
- `summary_style()` — slightly lighter summary bar (29,36,53)
- `field_style()` — input field background (20,25,38)
- `group_style()` — sidebar section group (28,34,50)
- `timeline_cell_style(velocity, grid_line)` — per-cell coloring with velocity-brightened notes (blue-green tones shaped by `VELOCITY_SHAPING_POWER = 0.75`)
- `active_note_colors(velocity)` — powf-shaped color mapping from velocity to RGB

## Enums reference

| Enum | Values | Location |
|------|--------|----------|
| `GeneratorMode` | Melodic, Hook, Euclidean, Arp, Chiptune, Bassline, ChordPads (7) | settings.rs |
| `GeneratorPreset` | Custom, TechnoBass, HouseBass, Drill808, HipHop808, UkGarageBass, DrumAndBass, BocChordPads, DreamyArp, ChipLead, SparseMotif, BusySequence (12) | settings.rs |
| `ChordStyle` | Balanced, Pop, PopDescent, Modal, Jazz, MinorCinematic, AcidMinimal, ChiptuneLoop, BoardsOfCanada (9) | settings.rs |
| `ChordQuality` | Major, Minor, Dominant, Diminished, Suspended, MinorDyad, Minor7, Sus2, Add9 (9) | mod.rs |
| `RhythmStyle` | Straight, Syncopated, Sparse, Busy, Dotted (5) | settings.rs |
| `BasslineStyle` | Techno, House, Drill, HipHop, UkGarage, DrumAndBass (6) | settings.rs |
| `HookType` | FourNoteLoop, CallResponse, MotifDevelop, StutterHook, DescendingBass (5) | settings.rs |
| `ArpPattern` | Up, Down, UpDown, RandomWalk (4) | settings.rs |
| `ArpRotation` | Off, Up, Down (3) | settings.rs |
| `VelocityMode` | Fixed, Random, Accented, Humanized (4) | settings.rs |
| `SeedBehavior` | Locked, RandomizeOnGenerate (2) | settings.rs |
| `Key` | C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B (12) | settings.rs |
| `Scale` | Major, NaturalMinor, HarmonicMinor, MajorPentatonic, MinorPentatonic, Blues, Dorian, Mixolydian (8) | settings.rs |
| `GridLine` | Bar, Beat, Step | ui.rs |

## Adding a new generator mode

1. Add enum variant to `GeneratorMode` in `generator/settings.rs`
2. Add to `GeneratorMode::ALL`
3. Add `Display` impl case
4. Implement `generate_{mode}()` function in a new file `generator/{mode}.rs`
5. Add `mod {mode};` in `generator/mod.rs`
6. Add dispatch match arm in `SongPipeline::generate_mode()` (generator/pipeline.rs)
7. Add help text in `app/view.rs` `controls()` → `mode_help`
8. Add to sidebar in `app/view.rs` `controls()` if mode-specific settings needed
9. Add tests in `generator/tests.rs`: produces notes, respects octave range, determinism

## Adding a new preset

1. Add variant to `GeneratorPreset` enum in `generator/settings.rs`
2. Add to `GeneratorPreset::ALL` array
3. Add `Display` impl case
4. Add `PresetDefinition` to `PRESET_DEFINITIONS` array using `PresetSettings`
5. Update `GeneratorPreset::ALL` length
6. Add a test in `generator/tests.rs` verifying preset application

## Adding a new chord style

1. Add variant to `ChordStyle` enum in `generator/settings.rs`
2. Add to `ChordStyle::ALL`
3. Add `Display` impl case
4. Add degree pattern in `chord_style_degree()` (generator/chords.rs)
5. If special generation is needed (like Boards of Canada), add a check in `generate_chords()`

## Adding a new bassline style

1. Add variant to `BasslineStyle` enum in `generator/settings.rs`
2. Add to `BasslineStyle::ALL`
3. Add `Display` impl case
4. Implement `generate_{style}_bassline()` function in `generator/bassline.rs`
5. Add dispatch arm in `generate_bassline()`
6. Add tests: style produces notes, determinism, respects octave range

## Adding a new rhythm style

1. Add variant to `RhythmStyle` enum in `generator/settings.rs`
2. Add to `RhythmStyle::ALL`
3. Add `Display` impl case
4. Add rhythm pattern array in `melodic_rhythm_patterns()` (generator/melody.rs)
5. Add density adjustment in `rhythm_density()` (generator/common.rs)

## Adding a new scale

1. Add variant to `Scale` enum in `generator/settings.rs`
2. Add to `Scale::ALL`
3. Add `Display` impl case
4. Add `intervals()` return in `Scale::intervals()`
5. Add `quality_for_degree()` mapping in `common.rs`
6. If minor-ish, add to `Scale::is_minorish()` match

## Key conventions

- **PPQN = 480** (pulses per quarter note), ticks per bar = 1920 (BEATS_PER_BAR = 4, STEPS_PER_BEAT = 4)
- All enums are `Copy + Clone + PartialEq + Eq + Debug` for iced compatibility
- `GeneratorSettings` fields are public; setters use `set_*` methods with clamping
- The `preset` field tracks whether the user has manually tweaked settings (auto-sets to `Custom` on music/harmony/rhythm edits)
- MIDI export uses `midly::Smf` with Format::SingleTrack and Metrical timing
- Export paths: auto-generated paths allow overwrite; manual paths refuse overwrite
- Export filenames use a slug from `generator_slug()`: mode slug, or bassline sub-style slug for bassline mode
- All modules have `#[cfg(test)]` modules with comprehensive tests
- Status messages communicate generation results, export results, and errors at the bottom of the window
- Default sidebar open state: Mode ✓, Music ✓, Harmony ✗, Rhythm ✓, Phrase ✗, Velocity ✓, Seed ✗

## Testing

```bash
cargo test
```

Tests cover:
- Preset application and custom-mode tracking
- Locked chord reuse across seeds (with clipping and expansion)
- Every generator mode produces notes within octave range
- Every hook type produces notes, is deterministic, respects octave range and velocity
- Every bassline style produces notes, is deterministic, respects octave range, accent/slide behavior
- Velocity modes: range clamping, per-mode enforcement for all generators and basslines
- Arp ordering: up/down/UpDown/randomWalk patterns, rotation, note count clamping
- Chord pad voicing: spread, inversion, voice leading, octave range
- Export: path normalization, parent creation, overwrite protection, unique naming
- Preview grid: sustained notes, adjacent notes, duplicate handling, out-of-range clipping
- Chord generation: style degree patterns, cadence, borrowed chords, surprise quality, Boards of Canada
- Phrase memory: repeat, clamping, variation
- Rhythm density adjustments per style
