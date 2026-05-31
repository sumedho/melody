# AGENTS.md — Melody Codebase Guide

## What this project is

**Melody** is a Rust desktop GUI application that procedurally generates musical melodies, basslines, arpeggios, chord pads, and sequences — then exports them as MIDI files. It uses the `iced` GUI toolkit for a dark-themed window with a sidebar of controls and a piano-roll-style timeline preview.

## Project structure

```
src/
├── main.rs       — Entry point. Calls app::run().
├── app.rs        — iced Application impl: MelodyApp struct, Message enum, update/view loops, all UI widgets, styling, sidebar sections, export logic. Contains unit tests.
├── generator.rs  — Core music engine. GeneratorSettings, all enums, chord generation, note generation per mode (melodic, euclidean, arp, chiptune, bassline, chord pads), velocity, phrase memory, cleanup. Contains unit tests.
├── midi.rs       — MIDI file writing (midly Smf), export path normalization, unique filename generation, parent directory creation. Contains unit tests.
├── music.rs      — Helpers: note_name(), pitch_class_name(), roman_degree().
└── ui.rs         — PreviewNoteIndex, PreviewStep, grid_line_for_step. Builds the piano-roll grid. Contains unit tests.
```

## Key types

### `GeneratorSettings` (generator.rs)
The single struct holding all user-facing parameters. Passed to `generate_song()` to produce deterministic output. Key fields:
- `mode: GeneratorMode` — which generator algorithm to use
- `key: Key`, `scale: Scale` — tonal center and scale
- `bars: u16`, `tempo: u16` — structure
- `seed: u64`, `seed_behavior: SeedBehavior` — determinism
- `chord_style: ChordStyle`, `tension`, `surprise`, `cadence` — harmony parameters
- `rhythm_style: RhythmStyle`, `density`, `note_length` — rhythm parameters
- `min_octave`, `max_octave` — pitch range
- Bassline-specific: `bassline_style`, `bassline_accent/slide/octave_jump/mutation`
- Arp-specific: `arp_note_count`, `arp_pattern`, `arp_rotation`, `arp_rotate_slot`
- `velocity_mode: VelocityMode`, `random_velocity_min/max`

### `GeneratedSong` (generator.rs)
Output of the generator: `notes: Vec<NoteEvent>` and `chords: Vec<ChordEvent>`.

### `Message` (app.rs)
All UI events. Each maps to a field update or a generate/export action. The `update` method matches on `Message` and mutates `MelodyApp.settings`, then optionally calls `generate_current_song()`.

### `MelodyApp` (app.rs)
The `iced::Application` implementation. Contains:
- `settings: GeneratorSettings` — current config
- `output: GeneratedSong` — most recently generated song
- `locked_chords: Option<Vec<ChordEvent>>` — chord lock state
- `sidebar: SidebarState` — which sections are open
- `export_filename`, `export_directory` — export config
- `status: String` — bottom status bar text

## Generation pipeline

```
GeneratorSettings
    │
    ▼
generate_song_with_chords()
    │
    ├── generate_chords() → Vec<ChordEvent>
    │   └── chord style patterns, cadence, surprise, tension, borrowed chords
    │
    ├── mode dispatch → Vec<NoteEvent>
    │   ├── generate_melodic()  — rhythm patterns + chord-aware pitch selection
    │   ├── generate_euclidean() — euclidean rhythm distribution
    │   ├── generate_arp()       — chord tone arpeggios with patterns
    │   ├── generate_chiptune()  — motif-based gated leads
    │   ├── generate_bassline()  — genre-specific patterns (6 styles)
    │   └── generate_chord_pads() — stacked chord notes with voice leading
    │
    ├── apply_phrase_memory() — repeat/variation across phrase bars
    ├── apply_velocity_range() — velocity mode application
    └── cleanup_notes() — filter, sort, dedup
    │
    ▼
GeneratedSong { notes, chords }
```

### Determinism
Uses `rand::rngs::StdRng::seed_from_u64(seed)` throughout. Same seed + same settings = identical output. The `locked_chords` feature bypasses chord generation entirely and repeats a chord cycle across the song length.

### Chord generation highlights
- `generate_chords()` iterates tick by tick, assigning chord events
- `chord_style_degree()` maps style → degree progression patterns
- `choose_next_degree()` uses tension/surprise to decide functional moves vs. leaps
- `borrowed_chord()` introduces chromatic mediants at high surprise
- `tension_quality()` upgrades to dominant/sus/add9 at high tension
- Boards of Canada style uses special `generate_boards_of_canada_chords()` with grounded/wandering progressions

### Bassline generation highlights
Each style has its own pattern array and probability model:
- **Techno**: 16-step grid with downbeat/syncopation bonuses
- **House**: Fixed pattern `[2,4,6,10,12,14]` per bar
- **Drill**: 4-bar group patterns with slide pickups
- **Hip-hop**: 4-bar group sparse patterns with mutation
- **UK Garage**: Swing offset on odd steps
- **Drum & Bass**: Sparse pattern with mutation on off-beats

### Phrase memory
`apply_phrase_memory()` copies a phrase template (first N bars) into subsequent phrase blocks, with probabilistic repeat and pitch/velocity variation.

## UI layout (app.rs)

```
┌──────────────────────────────────────────────────────────┐
│  Melody                          │  Generate  │  ...     │  ← Top bar
│  {mode} generator                │  Randomize │  Browse  │
│                                  │            │  Export  │
│  Directory: [exports]  Filename: [melody.mid]           │
├──────────────────┬───────────────────────────────────────┤
│  Sidebar (286px) │  Preview panel                        │
│                  │                                       │
│  [>] Mode        │  Preview  │  C 4 bars               │
│    Preset: ...   │  ─────────┼───────────────────        │
│    Generator: ...│  Summary: melodic C Major ...       │
│    (help text)   │  ─────────┼───┬───┬───┬───          │
│                  │  Bars    │ 1 │ 2 │ 3 │ 4             │
│  [>] Music       │  Chords  │ I │ IV│ V │ I             │
│    Key: ...      │  ────────┼───┴───┴───┴───            │
│    Scale: ...    │  Bb3     │ ░░│░░░│░░│░░░            │
│    Bars: ...     │  A3      │ ░ │░░░│░│░░░             │
│    ...           │  G3      │ ░░│░░│░░░│░               │
│                  │  ...     │ ...                       │
│  [>] Harmony     │  C3      │░░░│░░│░░░│░░             │
│    Lock chords   │  ────────┴───┴───┴───┴───            │
│    Chord style   │                                       │
│    Tension: ...  │                                       │
│    ...           │                                       │
│  [>] Rhythm      │                                       │
│    ...           │                                       │
│  [>] Phrase      │                                       │
│    ...           │                                       │
│  [>] Velocity    │                                       │
│    ...           │                                       │
│  [>] Seed        │                                       │
│    ...           │                                       │
├──────────────────┴───────────────────────────────────────┤
│  Generated 48 notes across 4 chord changes.              │  ← Status bar
└──────────────────────────────────────────────────────────┘
```

## Styling (app.rs)

Uses custom `theme::Container` styles via the `|_theme: &Theme| ContainerAppearance { ... }` closure pattern. All panels use dark Tokyo Night colors:
- `panel_style()` — dark background (24,28,42) with subtle borders
- `chord_style()` — purple-tinted chord lane
- `summary_style()` — slightly lighter summary bar
- `field_style()` — input field background
- `group_style()` — sidebar section group
- `timeline_cell_style(velocity, grid_line)` — per-cell coloring with velocity-brightened notes

## Adding a new generator mode

1. Add enum variant to `GeneratorMode` in `generator.rs`
2. Add `Display` impl case
3. Add to `GeneratorMode::ALL`
4. Implement `generate_{mode}()` function in `generator.rs`
5. Add dispatch match arm in `generate_song_with_chords()`
6. Add help text in `app.rs` `controls()` → `mode_help`
7. Add to sidebar in `app.rs` `controls()` if mode-specific settings needed
8. Add tests: `every_generator_produces_notes`, `every_generator_respects_octave_range`, determinism test

## Adding a new preset

1. Add variant to `GeneratorPreset` enum in `generator.rs`
2. Add to `GeneratorPreset::ALL`
3. Add `Display` impl case
4. Add match arm in `GeneratorSettings::apply_preset()` setting all relevant fields
5. Update `GeneratorPreset::ALL` array length
6. Add a test in `app.rs` or `generator.rs` verifying preset application

## Adding a new chord style

1. Add variant to `ChordStyle` enum in `generator.rs`
2. Add to `ChordStyle::ALL`
3. Add `Display` impl case
4. Add degree pattern in `chord_style_degree()`
5. If special generation is needed (like Boards of Canada), add a check in `generate_chords()`

## Adding a new bassline style

1. Add variant to `BasslineStyle` enum in `generator.rs`
2. Add to `BasslineStyle::ALL`
3. Add `Display` impl case
4. Implement `generate_{style}_bassline()` function
5. Add dispatch arm in `generate_bassline()`
6. Add tests: style produces notes, determinism, respects octave range

## Adding a new rhythm style

1. Add variant to `RhythmStyle` enum in `generator.rs`
2. Add to `RhythmStyle::ALL`
3. Add `Display` impl case
4. Add rhythm pattern array in `melodic_rhythm_patterns()`
5. Add density adjustment in `rhythm_density()`

## Adding a new scale

1. Add variant to `Scale` enum in `generator.rs`
2. Add to `Scale::ALL`
3. Add `Display` impl case
4. Add `intervals()` return in `Scale::intervals()`
5. Add `quality_for_degree()` mapping
6. If minor-ish, add to `Scale::is_minorish()` match

## Key conventions

- **PPQN = 480** (pulses per quarter note), ticks per bar = 1920
- All enums are `Copy + Clone + PartialEq + Eq + Debug` for iced compatibility
- `GeneratorSettings` fields are public; setters use `set_*` methods with clamping
- The `preset` field tracks whether the user has manually tweaked settings (auto-sets to `Custom` on manual edit)
- MIDI export uses `midly::Smf` with Format::SingleTrack and Metrical timing
- Export paths: auto-generated paths allow overwrite; manual paths refuse overwrite
- All modules have `#[cfg(test)]` modules with comprehensive tests
- Status messages communicate generation results, export results, and errors at the bottom of the window

## Testing

```bash
cargo test
```

Tests cover:
- Preset application and custom-mode tracking
- Locked chord reuse across seeds
- Every generator mode produces notes within octave range
- Bassline styles: production, determinism, octave range, accent/slide behavior
- Velocity modes: range clamping, per-mode enforcement
- Arp ordering: up/down/UpDown/randomWalk patterns
- Chord pad voicing: spread, inversion, voice leading
- Export: path normalization, parent creation, overwrite protection
- Preview grid: sustained notes, adjacent notes, duplicate handling, clipping
