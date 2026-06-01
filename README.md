# Melody

A desktop GUI application for procedurally generating musical melodies, basslines, arpeggios, chord pads, and more — exported as MIDI files.

Built in Rust with the [iced](https://github.com/iced-rs/iced) GUI toolkit and [midly](https://github.com/Emilgardis/midly) for MIDI I/O.

## Features

- **6 generator modes**
  - **Melodic** — Pattern grammar with chord-aware melodic contour
  - **Euclidean** — Evenly distributed pulses with rotation and accents (Rhythmbox-style)
  - **Arp** — Chord tones unfolded as musical arpeggios with configurable patterns
  - **Chiptune** — Gated leads, octave jumps, motifs, and pulse-bass flavor
  - **Bassline** — Genre-shaped monophonic basslines for dance and beat production
  - **Chord Pads** — Sustained playable chords with gentle timing and velocity drift

- **6 bassline sub-styles** — Techno, House, Drill, Hip-hop, UK garage, Drum & bass (each with accent, slide, octave jump, and mutation controls)

- **8 chord styles** — Balanced, Pop, Modal, Jazz ii-V, Minor cinematic, Acid minimal, Chiptune loop, Boards of Canada

- **5 rhythm styles** — Straight, Syncopated, Sparse, Busy, Dotted

- **Deterministic generation** via seeded PRNG (`StdRng`), with a "Lock chords" feature to reuse chord progressions across seed variations

- **12 curated presets** — Techno bass, House bass, Drill 808, Hip-hop 808, UK garage bass, Drum & bass, BoC chord pads, Dreamy arp, Chip lead, Sparse motif, Busy sequence

- **Visual timeline preview** — Piano-roll-style grid showing notes, chords, and bar/beat markers

- **MIDI export** — Single-track SMF files with configurable tempo, directory, and filename

## Architecture

```
src/
├── main.rs       — Entry point, launches the iced Application
├── app.rs        — Main window, sidebar controls, preview panel, styling
├── generator.rs  — Core music generation engine (chords + notes for all modes)
├── midi.rs       — MIDI file writing and export path management
├── music.rs      — Note name formatting, pitch class, Roman numeral helpers
└── ui.rs         — Timeline preview grid, note indexing, step rendering
```

### Data flow

1. `GeneratorSettings` holds all user-facing parameters
2. `generate_song()` creates a `GeneratedSong` (notes + chords) deterministically from the seed
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
| **Mode** | Preset selector, generator mode (6 options), arp-specific settings, bassline-specific settings |
| **Music** | Key (12 options), scale (8 options), bars (1–16), tempo (60–180 bpm), min/max octave (1–8) |
| **Harmony** | Lock chords toggle, chord style (8 options), tension, surprise, resolution (cadence), chord inversion |
| **Rhythm** | Rhythm style (5 options), note density, gate/overlap length |
| **Phrase** | Phrase length, repeat amount, variation amount |
| **Velocity** | Mode (Fixed, Random, Accented, Humanized), random velocity range |
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

Comprehensive test suite covering every generator mode, every bassline style, preset behavior, chord locking, velocity ranges, octave clamping, arp ordering, and export path handling.

## Naming convention

Exported MIDI files follow the pattern:
```
melody-{generator}-{timestamp}-{seed}-{counter}.mid
```

Examples:
- `melody-bassline-techno-1780235993359-1070742-17.mid`
- `melody-chiptune-1780234833384-1681930-70.mid`
