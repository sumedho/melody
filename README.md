# Melody

A desktop GUI application for procedurally generating musical melodies, hooks, basslines, arpeggios, chord pads, and more — exported as MIDI files.

Built in Rust with the [iced](https://github.com/iced-rs/iced) GUI toolkit and [midly](https://github.com/Emilgardis/midly) for MIDI I/O.

## Features

- **7 generator modes**
  - **Melodic** — Pattern grammar with chord-aware melodic contour
  - **Hook** — Short repeating riffs with 5 pop and dance hook shapes (four-note loop, call & response, motif develop, stutter hook, descending bass)
  - **Euclidean** — Evenly distributed pulses with rotation and accents (Bjorklund-style)
  - **Arp** — Chord tones unfolded as musical arpeggios with configurable patterns and rotation
  - **Chiptune** — Gated leads, octave jumps, motifs, and pulse-bass flavor
  - **Bassline** — Genre-shaped monophonic basslines for dance and beat production
  - **Chord Pads** — Sustained playable chords with spread voicing, voice leading, and gentle timing drift

- **6 bassline sub-styles** — Techno, House, Drill, Hip-hop, UK garage, Drum & bass (each with accent, slide, octave jump, and mutation controls)

- **9 chord styles** — Balanced, Pop, Pop descent, Modal, Jazz ii-V, Minor cinematic, Acid minimal, Chiptune loop, Boards of Canada

- **5 rhythm styles** — Straight, Syncopated, Sparse, Busy, Dotted

- **Deterministic generation** via seeded PRNG (`StdRng`), with a "Lock chords" feature to reuse chord progressions across seed variations

- **12 curated presets** — Techno bass, House bass, Drill 808, Hip-hop 808, UK garage bass, Drum & bass, BoC chord pads, Dreamy arp, Chip lead, Sparse motif, Busy sequence

- **Visual timeline preview** — Piano-roll-style grid showing notes, chords, and bar/beat markers

- **MIDI export** — Single-track SMF files with configurable tempo, directory, and filename

## Architecture

```
src/
├── main.rs          — Entry point, launches the iced Application
├── constants.rs     — PPQN, tempo range, octave range, note gate ratios, swing factor
├── music.rs         — Note name formatting, pitch class, Roman numeral helpers
├── midi.rs          — MIDI file writing and export path management
├── ui.rs            — Timeline preview grid, note indexing, step rendering
├── app/
│   ├── mod.rs       — MelodyApp, MusicState, ExportState, UIState, Message enum
│   ├── update.rs    — All Message handling logic
│   ├── view.rs      — All UI rendering: top bar, sidebar, preview, timeline
│   ├── sidebar.rs   — SidebarSection enum, SidebarState, expandable_group
│   ├── widgets.rs   — UI components and all styling functions
│   └── tests.rs     — App-level integration tests
└── generator/
    ├── mod.rs       — GeneratedSong, NoteEvent, ChordEvent, ChordQuality, entry functions
    ├── settings.rs  — GeneratorSettings, all enums, preset definitions
    ├── pipeline.rs  — SongPipeline builder pattern
    ├── common.rs    — Shared helpers (rhythm density, phrase memory, velocity, scale math)
    ├── chords.rs    — Chord generation with styles, cadence, surprise, borrowed chords
    ├── melody.rs    — Melodic mode with rhythm patterns
    ├── hook.rs      — Hook mode with 5 hook types
    ├── euclidean.rs — Euclidean rhythm distribution
    ├── arp.rs       — Arpeggio generation with patterns and rotation
    ├── chiptune.rs  — Chiptune motif-based generation
    ├── bassline.rs  — Bassline generation with 6 genre styles
    ├── chord_pads.rs — Chord pad generation with voicing and voice leading
    └── tests.rs     — Comprehensive generator tests
```

### Data flow

1. `GeneratorSettings` holds all user-facing parameters
2. `SongPipeline` processes the pipeline: chords → mode notes → phrase memory → velocity → `GeneratedSong`
3. `export_midi()` writes the song as a single-track SMF file
4. The UI renders a piano-roll preview using `PreviewNoteIndex` from `ui.rs`

## Installation

```bash
# Requires Rust 1.70+ (edition 2021)
cargo build --release
```

## Running

```bash
cargo run
```

The app window opens at 1180×760 pixels with the Tokyo Night theme.

## Controls

| Section | Controls |
|---------|----------|
| **Mode** | Preset selector, generator mode (7 options), hook type (conditional), arp-specific settings, bassline-specific settings |
| **Music** | Key (12 options), scale (8 options), bars (1–16), tempo (60–180 bpm), min/max octave (1–8) |
| **Harmony** | Lock chords toggle, chord style (9 options), tension, chord surprise, resolution (cadence), chord inversion |
| **Rhythm** | Rhythm style (5 options), note density, gate / overlap |
| **Phrase** | Phrase bars, repeat amount, variation amount |
| **Velocity** | Mode (Fixed, Random, Accented, Humanized), random velocity range lower/upper |
| **Seed** | Seed mode (Locked / Randomize on generate), seed value input |

Top toolbar: **Generate**, **Randomize**, **Browse** (export directory), **Export**.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `iced` 0.12 | Cross-platform GUI |
| `midly` 0.5 | MIDI file read/write |
| `rand` 0.8 | Seeded PRNG (deterministic generation) |
| `rfd` 0.14 | Native file dialog for export directory |

## Tests

```bash
cargo test
```

Comprehensive test suite covering every generator mode, every hook type, every bassline style, preset behavior, chord locking, velocity ranges, octave clamping, arp ordering, chord pad voicing, phrase memory, rhythm density, and export path handling.

## Naming convention

Exported MIDI files follow the pattern:
```
melody-{generator}-{timestamp}-{seed}-{counter}.mid
```

For bassline presets, the generator slug includes the sub-style:
```
melody-bassline-{style}-{timestamp}-{seed}-{counter}.mid
```

Examples:
- `melody-hook-1780273918081-11582712957124492886-7.mid`
- `melody-bassline-house-1780235993359-1070742-17.mid`
- `melody-chiptune-1780234833384-1681930-70.mid`
