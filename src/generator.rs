use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fmt::{Display, Formatter};

use crate::music::{pitch_class_name, roman_degree};

pub const PPQN: u16 = 480;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorSettings {
    pub preset: GeneratorPreset,
    pub key: Key,
    pub scale: Scale,
    pub mode: GeneratorMode,
    pub bars: u16,
    pub tempo: u16,
    pub seed: u64,
    pub seed_behavior: SeedBehavior,
    pub chord_style: ChordStyle,
    pub rhythm_style: RhythmStyle,
    pub tension: u8,
    pub surprise: u8,
    pub cadence: u8,
    pub chord_inversion_amount: u8,
    pub density: u8,
    pub note_length: u8,
    pub phrase_length: u8,
    pub repeat_amount: u8,
    pub variation_amount: u8,
    pub min_octave: u8,
    pub max_octave: u8,
    pub arp_note_count: u8,
    pub arp_pattern: ArpPattern,
    pub arp_rotate_slot: u8,
    pub arp_rotation: ArpRotation,
    pub bassline_style: BasslineStyle,
    pub bassline_accent: u8,
    pub bassline_slide: u8,
    pub bassline_octave_jump: u8,
    pub bassline_mutation: u8,
    pub velocity_mode: VelocityMode,
    pub random_velocity_min: u8,
    pub random_velocity_max: u8,
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            preset: GeneratorPreset::Custom,
            key: Key::C,
            scale: Scale::Major,
            mode: GeneratorMode::Melodic,
            bars: 4,
            tempo: 110,
            seed: 42,
            seed_behavior: SeedBehavior::Locked,
            chord_style: ChordStyle::Balanced,
            rhythm_style: RhythmStyle::Straight,
            tension: 35,
            surprise: 20,
            cadence: 75,
            chord_inversion_amount: 0,
            density: 60,
            note_length: 45,
            phrase_length: 2,
            repeat_amount: 35,
            variation_amount: 20,
            min_octave: 3,
            max_octave: 6,
            arp_note_count: 4,
            arp_pattern: ArpPattern::Up,
            arp_rotate_slot: 4,
            arp_rotation: ArpRotation::Off,
            bassline_style: BasslineStyle::Techno,
            bassline_accent: 55,
            bassline_slide: 35,
            bassline_octave_jump: 35,
            bassline_mutation: 25,
            velocity_mode: VelocityMode::Humanized,
            random_velocity_min: 56,
            random_velocity_max: 116,
        }
    }
}

impl GeneratorSettings {
    pub fn apply_preset(&mut self, preset: GeneratorPreset) {
        self.preset = preset;
        match preset {
            GeneratorPreset::Custom => {}
            GeneratorPreset::TechnoBass => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::Techno;
                self.scale = Scale::MinorPentatonic;
                self.tempo = 128;
                self.min_octave = 1;
                self.max_octave = 3;
                self.chord_style = ChordStyle::AcidMinimal;
                self.rhythm_style = RhythmStyle::Syncopated;
                self.density = 78;
                self.note_length = 22;
                self.repeat_amount = 72;
                self.variation_amount = 35;
                self.bassline_accent = 82;
                self.bassline_slide = 45;
                self.bassline_octave_jump = 46;
                self.bassline_mutation = 42;
            }
            GeneratorPreset::HouseBass => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::House;
                self.scale = Scale::Dorian;
                self.tempo = 124;
                self.min_octave = 2;
                self.max_octave = 4;
                self.chord_style = ChordStyle::Pop;
                self.rhythm_style = RhythmStyle::Syncopated;
                self.density = 66;
                self.note_length = 48;
                self.repeat_amount = 64;
                self.variation_amount = 24;
                self.bassline_accent = 58;
                self.bassline_slide = 10;
                self.bassline_octave_jump = 22;
                self.bassline_mutation = 35;
            }
            GeneratorPreset::Drill808 => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::Drill;
                self.scale = Scale::NaturalMinor;
                self.tempo = 140;
                self.min_octave = 1;
                self.max_octave = 3;
                self.chord_style = ChordStyle::MinorCinematic;
                self.rhythm_style = RhythmStyle::Sparse;
                self.density = 52;
                self.note_length = 88;
                self.repeat_amount = 55;
                self.variation_amount = 30;
                self.bassline_accent = 76;
                self.bassline_slide = 78;
                self.bassline_octave_jump = 45;
                self.bassline_mutation = 48;
            }
            GeneratorPreset::HipHop808 => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::HipHop;
                self.scale = Scale::Blues;
                self.tempo = 92;
                self.min_octave = 1;
                self.max_octave = 3;
                self.chord_style = ChordStyle::MinorCinematic;
                self.rhythm_style = RhythmStyle::Sparse;
                self.density = 42;
                self.note_length = 72;
                self.repeat_amount = 60;
                self.variation_amount = 18;
                self.bassline_accent = 72;
                self.bassline_slide = 18;
                self.bassline_octave_jump = 36;
                self.bassline_mutation = 28;
            }
            GeneratorPreset::UkGarageBass => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::UkGarage;
                self.scale = Scale::NaturalMinor;
                self.tempo = 127;
                self.min_octave = 2;
                self.max_octave = 4;
                self.chord_style = ChordStyle::Modal;
                self.rhythm_style = RhythmStyle::Syncopated;
                self.density = 70;
                self.note_length = 34;
                self.repeat_amount = 62;
                self.variation_amount = 38;
                self.bassline_accent = 64;
                self.bassline_slide = 24;
                self.bassline_octave_jump = 40;
                self.bassline_mutation = 55;
            }
            GeneratorPreset::DrumAndBass => {
                self.mode = GeneratorMode::Bassline;
                self.bassline_style = BasslineStyle::DrumAndBass;
                self.scale = Scale::NaturalMinor;
                self.tempo = 174;
                self.min_octave = 1;
                self.max_octave = 3;
                self.chord_style = ChordStyle::MinorCinematic;
                self.rhythm_style = RhythmStyle::Busy;
                self.density = 74;
                self.note_length = 28;
                self.repeat_amount = 45;
                self.variation_amount = 48;
                self.bassline_accent = 80;
                self.bassline_slide = 20;
                self.bassline_octave_jump = 34;
                self.bassline_mutation = 58;
            }
            GeneratorPreset::BocChordPads => {
                self.mode = GeneratorMode::ChordPads;
                self.scale = Scale::Dorian;
                self.tempo = 88;
                self.bars = 8;
                self.min_octave = 2;
                self.max_octave = 5;
                self.chord_style = ChordStyle::BoardsOfCanada;
                self.rhythm_style = RhythmStyle::Sparse;
                self.tension = 62;
                self.surprise = 42;
                self.cadence = 45;
                self.density = 38;
                self.note_length = 88;
                self.repeat_amount = 72;
                self.variation_amount = 12;
                self.chord_inversion_amount = 35;
                self.velocity_mode = VelocityMode::Humanized;
            }
            GeneratorPreset::DreamyArp => {
                self.mode = GeneratorMode::Arp;
                self.scale = Scale::MajorPentatonic;
                self.tempo = 96;
                self.min_octave = 3;
                self.max_octave = 6;
                self.chord_style = ChordStyle::Modal;
                self.rhythm_style = RhythmStyle::Straight;
                self.density = 55;
                self.note_length = 68;
                self.repeat_amount = 58;
                self.variation_amount = 18;
                self.arp_note_count = 5;
                self.arp_pattern = ArpPattern::UpDown;
                self.arp_rotate_slot = 5;
                self.arp_rotation = ArpRotation::Up;
            }
            GeneratorPreset::ChipLead => {
                self.mode = GeneratorMode::Chiptune;
                self.scale = Scale::Mixolydian;
                self.tempo = 140;
                self.min_octave = 4;
                self.max_octave = 7;
                self.chord_style = ChordStyle::ChiptuneLoop;
                self.rhythm_style = RhythmStyle::Busy;
                self.density = 82;
                self.note_length = 18;
                self.repeat_amount = 65;
                self.variation_amount = 30;
            }
            GeneratorPreset::SparseMotif => {
                self.mode = GeneratorMode::Melodic;
                self.scale = Scale::Dorian;
                self.tempo = 88;
                self.min_octave = 3;
                self.max_octave = 5;
                self.chord_style = ChordStyle::Modal;
                self.rhythm_style = RhythmStyle::Sparse;
                self.density = 35;
                self.note_length = 72;
                self.repeat_amount = 68;
                self.variation_amount = 15;
            }
            GeneratorPreset::BusySequence => {
                self.mode = GeneratorMode::Euclidean;
                self.scale = Scale::NaturalMinor;
                self.tempo = 124;
                self.min_octave = 3;
                self.max_octave = 6;
                self.chord_style = ChordStyle::MinorCinematic;
                self.rhythm_style = RhythmStyle::Busy;
                self.density = 88;
                self.note_length = 36;
                self.repeat_amount = 42;
                self.variation_amount = 45;
            }
        }
        self.set_phrase_length(self.phrase_length);
        self.set_arp_note_count(self.arp_note_count);
    }

    pub fn set_phrase_length(&mut self, value: u8) {
        self.phrase_length = value.clamp(1, self.bars.min(8) as u8);
    }

    pub fn set_min_octave(&mut self, value: u8) {
        self.min_octave = value;
        if self.max_octave < value {
            self.max_octave = value;
        }
    }

    pub fn set_max_octave(&mut self, value: u8) {
        self.max_octave = value;
        if self.min_octave > value {
            self.min_octave = value;
        }
    }

    pub fn set_arp_note_count(&mut self, value: u8) {
        self.arp_note_count = value;
        if self.arp_rotate_slot > value {
            self.arp_rotate_slot = value;
        }
    }

    pub fn set_arp_rotate_slot(&mut self, value: u8) {
        self.arp_rotate_slot = value.clamp(1, self.arp_note_count);
    }

    pub fn set_random_velocity_min(&mut self, value: u8) {
        self.random_velocity_min = value.clamp(1, 127);
        if self.random_velocity_max < self.random_velocity_min {
            self.random_velocity_max = self.random_velocity_min;
        }
    }

    pub fn set_random_velocity_max(&mut self, value: u8) {
        self.random_velocity_max = value.clamp(1, 127);
        if self.random_velocity_min > self.random_velocity_max {
            self.random_velocity_min = self.random_velocity_max;
        }
    }

    pub fn low_pitch(self) -> u8 {
        octave_to_midi_c(self.min_octave)
    }

    pub fn high_pitch(self) -> u8 {
        octave_to_midi_c(self.max_octave)
            .saturating_add(11)
            .min(127)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorPreset {
    Custom,
    TechnoBass,
    HouseBass,
    Drill808,
    HipHop808,
    UkGarageBass,
    DrumAndBass,
    BocChordPads,
    DreamyArp,
    ChipLead,
    SparseMotif,
    BusySequence,
}

impl GeneratorPreset {
    pub const ALL: [Self; 12] = [
        Self::Custom,
        Self::TechnoBass,
        Self::HouseBass,
        Self::Drill808,
        Self::HipHop808,
        Self::UkGarageBass,
        Self::DrumAndBass,
        Self::BocChordPads,
        Self::DreamyArp,
        Self::ChipLead,
        Self::SparseMotif,
        Self::BusySequence,
    ];
}

impl Display for GeneratorPreset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Custom => "Custom",
            Self::TechnoBass => "Techno bass",
            Self::HouseBass => "House bass",
            Self::Drill808 => "Drill 808",
            Self::HipHop808 => "Hip-hop 808",
            Self::UkGarageBass => "UK garage bass",
            Self::DrumAndBass => "Drum & bass",
            Self::BocChordPads => "BoC chord pads",
            Self::DreamyArp => "Dreamy arp",
            Self::ChipLead => "Chip lead",
            Self::SparseMotif => "Sparse motif",
            Self::BusySequence => "Busy sequence",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedBehavior {
    Locked,
    RandomizeOnGenerate,
}

impl SeedBehavior {
    pub const ALL: [Self; 2] = [Self::Locked, Self::RandomizeOnGenerate];
}

impl Display for SeedBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Locked => "Locked",
            Self::RandomizeOnGenerate => "Randomize on generate",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordStyle {
    Balanced,
    Pop,
    Modal,
    Jazz,
    MinorCinematic,
    AcidMinimal,
    ChiptuneLoop,
    BoardsOfCanada,
}

impl ChordStyle {
    pub const ALL: [Self; 8] = [
        Self::Balanced,
        Self::Pop,
        Self::Modal,
        Self::Jazz,
        Self::MinorCinematic,
        Self::AcidMinimal,
        Self::ChiptuneLoop,
        Self::BoardsOfCanada,
    ];
}

impl Display for ChordStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Balanced => "Balanced",
            Self::Pop => "Pop",
            Self::Modal => "Modal",
            Self::Jazz => "Jazz ii-V",
            Self::MinorCinematic => "Minor cinematic",
            Self::AcidMinimal => "Acid minimal",
            Self::ChiptuneLoop => "Chiptune loop",
            Self::BoardsOfCanada => "Boards of Canada",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhythmStyle {
    Straight,
    Syncopated,
    Sparse,
    Busy,
    Dotted,
}

impl RhythmStyle {
    pub const ALL: [Self; 5] = [
        Self::Straight,
        Self::Syncopated,
        Self::Sparse,
        Self::Busy,
        Self::Dotted,
    ];
}

impl Display for RhythmStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Straight => "Straight",
            Self::Syncopated => "Syncopated",
            Self::Sparse => "Sparse",
            Self::Busy => "Busy",
            Self::Dotted => "Dotted",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasslineStyle {
    Techno,
    House,
    Drill,
    HipHop,
    UkGarage,
    DrumAndBass,
}

impl BasslineStyle {
    pub const ALL: [Self; 6] = [
        Self::Techno,
        Self::House,
        Self::Drill,
        Self::HipHop,
        Self::UkGarage,
        Self::DrumAndBass,
    ];
}

impl Display for BasslineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Techno => "Techno",
            Self::House => "House",
            Self::Drill => "Drill",
            Self::HipHop => "Hip-hop",
            Self::UkGarage => "UK garage",
            Self::DrumAndBass => "Drum & bass",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpPattern {
    Up,
    Down,
    UpDown,
    RandomWalk,
}

impl ArpPattern {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::UpDown, Self::RandomWalk];
}

impl Display for ArpPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::UpDown => "Up/down",
            Self::RandomWalk => "Random walk",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpRotation {
    Off,
    Up,
    Down,
}

impl ArpRotation {
    pub const ALL: [Self; 3] = [Self::Off, Self::Up, Self::Down];
}

impl Display for ArpRotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Off => "Off",
            Self::Up => "Up",
            Self::Down => "Down",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Key {
    pub const ALL: [Self; 12] = [
        Self::C,
        Self::Db,
        Self::D,
        Self::Eb,
        Self::E,
        Self::F,
        Self::Gb,
        Self::G,
        Self::Ab,
        Self::A,
        Self::Bb,
        Self::B,
    ];

    fn semitone(self) -> i8 {
        match self {
            Self::C => 0,
            Self::Db => 1,
            Self::D => 2,
            Self::Eb => 3,
            Self::E => 4,
            Self::F => 5,
            Self::Gb => 6,
            Self::G => 7,
            Self::Ab => 8,
            Self::A => 9,
            Self::Bb => 10,
            Self::B => 11,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::C => "C",
            Self::Db => "Db",
            Self::D => "D",
            Self::Eb => "Eb",
            Self::E => "E",
            Self::F => "F",
            Self::Gb => "Gb",
            Self::G => "G",
            Self::Ab => "Ab",
            Self::A => "A",
            Self::Bb => "Bb",
            Self::B => "B",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MajorPentatonic,
    MinorPentatonic,
    Blues,
    Dorian,
    Mixolydian,
}

impl Scale {
    pub const ALL: [Self; 8] = [
        Self::Major,
        Self::NaturalMinor,
        Self::HarmonicMinor,
        Self::MajorPentatonic,
        Self::MinorPentatonic,
        Self::Blues,
        Self::Dorian,
        Self::Mixolydian,
    ];

    fn intervals(self) -> &'static [i8] {
        match self {
            Self::Major => &[0, 2, 4, 5, 7, 9, 11],
            Self::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            Self::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            Self::MajorPentatonic => &[0, 2, 4, 7, 9],
            Self::MinorPentatonic => &[0, 3, 5, 7, 10],
            Self::Blues => &[0, 3, 5, 6, 7, 10],
            Self::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            Self::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
        }
    }

    fn degree_count(self) -> usize {
        self.intervals().len()
    }

    fn is_minorish(self) -> bool {
        matches!(
            self,
            Self::NaturalMinor
                | Self::HarmonicMinor
                | Self::MinorPentatonic
                | Self::Blues
                | Self::Dorian
        )
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Major => "Major",
            Self::NaturalMinor => "Natural minor",
            Self::HarmonicMinor => "Harmonic minor",
            Self::MajorPentatonic => "Major pentatonic",
            Self::MinorPentatonic => "Minor pentatonic",
            Self::Blues => "Blues",
            Self::Dorian => "Dorian",
            Self::Mixolydian => "Mixolydian",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorMode {
    Melodic,
    Euclidean,
    Arp,
    Chiptune,
    Bassline,
    ChordPads,
}

impl GeneratorMode {
    pub const ALL: [Self; 6] = [
        Self::Melodic,
        Self::Euclidean,
        Self::Arp,
        Self::Chiptune,
        Self::Bassline,
        Self::ChordPads,
    ];
}

impl Display for GeneratorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Melodic => "Melodic",
            Self::Euclidean => "Euclidean",
            Self::Arp => "Arp",
            Self::Chiptune => "Chiptune",
            Self::Bassline => "Bassline",
            Self::ChordPads => "Chord pads",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VelocityMode {
    Fixed,
    Random,
    Accented,
    Humanized,
}

impl VelocityMode {
    pub const ALL: [Self; 4] = [Self::Fixed, Self::Random, Self::Accented, Self::Humanized];
}

impl Display for VelocityMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Fixed => "Fixed",
            Self::Random => "Random",
            Self::Accented => "Accented",
            Self::Humanized => "Humanized",
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedSong {
    pub notes: Vec<NoteEvent>,
    pub chords: Vec<ChordEvent>,
}

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub pitch: u8,
    pub start_ticks: u32,
    pub duration_ticks: u32,
    pub velocity: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordEvent {
    pub root: u8,
    pub quality: ChordQuality,
    pub degree: usize,
    pub start_ticks: u32,
    pub duration_ticks: u32,
    pub tension: u8,
}

impl ChordEvent {
    pub fn label(&self) -> String {
        format!(
            "{}{} {}",
            pitch_class_name(self.root),
            match self.quality {
                ChordQuality::Major => "",
                ChordQuality::Minor => "m",
                ChordQuality::Dominant => "7",
                ChordQuality::Diminished => "dim",
                ChordQuality::Suspended => "sus",
                ChordQuality::MinorDyad => "m(no5)",
                ChordQuality::Minor7 => "m7",
                ChordQuality::Sus2 => "sus2",
                ChordQuality::Add9 => "add9",
            },
            roman_degree(self.degree, self.quality, self.tension)
        )
    }

    pub fn tones(&self) -> Vec<u8> {
        let intervals: &[u8] = match self.quality {
            ChordQuality::Major => &[0, 4, 7],
            ChordQuality::Minor => &[0, 3, 7],
            ChordQuality::Dominant => &[0, 4, 7, 10],
            ChordQuality::Diminished => &[0, 3, 6],
            ChordQuality::Suspended => &[0, 5, 7],
            ChordQuality::MinorDyad => &[0, 3],
            ChordQuality::Minor7 => &[0, 3, 7, 10],
            ChordQuality::Sus2 => &[0, 2, 7],
            ChordQuality::Add9 => &[0, 4, 7, 14],
        };
        intervals
            .iter()
            .map(|interval| (self.root + interval) % 12)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordQuality {
    Major,
    Minor,
    Dominant,
    Diminished,
    Suspended,
    MinorDyad,
    Minor7,
    Sus2,
    Add9,
}

pub fn generate_song(settings: &GeneratorSettings) -> GeneratedSong {
    generate_song_with_chords(settings, None)
}

pub fn generate_song_with_chords(
    settings: &GeneratorSettings,
    locked_chords: Option<&[ChordEvent]>,
) -> GeneratedSong {
    let mut rng = StdRng::seed_from_u64(settings.seed);
    let chords = locked_chords
        .and_then(|chords| locked_chords_for_song(settings, chords))
        .unwrap_or_else(|| generate_chords(settings, &mut rng));
    let notes = match settings.mode {
        GeneratorMode::Melodic => generate_melodic(settings, &chords, &mut rng),
        GeneratorMode::Euclidean => generate_euclidean(settings, &chords, &mut rng),
        GeneratorMode::Arp => generate_arp(settings, &chords, &mut rng),
        GeneratorMode::Chiptune => generate_chiptune(settings, &chords, &mut rng),
        GeneratorMode::Bassline => generate_bassline(settings, &chords, &mut rng),
        GeneratorMode::ChordPads => generate_chord_pads(settings, &chords, &mut rng),
    };
    let notes = apply_velocity_range(settings, apply_phrase_memory(settings, notes, &mut rng));

    GeneratedSong { notes, chords }
}

fn locked_chords_for_song(
    settings: &GeneratorSettings,
    locked_chords: &[ChordEvent],
) -> Option<Vec<ChordEvent>> {
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    if total_ticks == 0 {
        return None;
    }

    let mut source = locked_chords.to_vec();
    source.retain(|chord| chord.duration_ticks > 0);
    source.sort_by_key(|chord| (chord.start_ticks, chord.duration_ticks));

    let cycle_ticks = source
        .iter()
        .map(|chord| chord.start_ticks + chord.duration_ticks)
        .max()?;
    if cycle_ticks == 0 {
        return None;
    }

    let mut chords = Vec::new();
    let mut cycle_start = 0;
    while cycle_start < total_ticks {
        for chord in &source {
            let start_ticks = cycle_start + chord.start_ticks;
            if start_ticks >= total_ticks {
                continue;
            }

            let end_ticks = (start_ticks + chord.duration_ticks).min(total_ticks);
            if end_ticks <= start_ticks {
                continue;
            }

            let mut copied = chord.clone();
            copied.start_ticks = start_ticks;
            copied.duration_ticks = end_ticks - start_ticks;
            chords.push(copied);
        }
        cycle_start += cycle_ticks;
    }

    if chords.is_empty() {
        None
    } else {
        Some(chords)
    }
}

fn generate_chords(settings: &GeneratorSettings, rng: &mut StdRng) -> Vec<ChordEvent> {
    if settings.chord_style == ChordStyle::BoardsOfCanada {
        return generate_boards_of_canada_chords(settings, rng);
    }

    let bars_per_chord = if settings.density > 75 || settings.surprise > 80 {
        1
    } else {
        2
    };
    let chord_ticks = ticks_per_bar() * bars_per_chord;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut start = 0;
    let mut degree = 0;
    let mut chord_index = 0usize;
    let mut chords = Vec::new();

    while start < total_ticks {
        let remaining = total_ticks - start;
        let duration = chord_ticks.min(remaining);
        let is_final = start + duration >= total_ticks;
        let is_penultimate = !is_final && start + duration * 2 >= total_ticks;
        let patterned_degree = if settings.chord_style != ChordStyle::Balanced {
            Some(chord_style_degree(
                settings.chord_style,
                chord_index,
                settings.scale.degree_count(),
            ))
        } else {
            None
        };
        let effective_cadence = settings.cadence.saturating_sub(settings.surprise / 2);
        let next_degree = if let Some(patterned_degree) = patterned_degree {
            if is_penultimate && rng.gen_range(0..100) < effective_cadence {
                cadence_approach_degree(settings, rng)
            } else if is_final && rng.gen_range(0..100) < effective_cadence {
                0
            } else if rng.gen_range(0..100) < settings.surprise {
                surprising_degree(patterned_degree, settings.scale.degree_count(), rng)
            } else {
                patterned_degree
            }
        } else if is_penultimate && rng.gen_range(0..100) < effective_cadence {
            cadence_approach_degree(settings, rng)
        } else if is_final && rng.gen_range(0..100) < effective_cadence {
            0
        } else {
            choose_next_degree(degree, settings, rng)
        };
        degree = next_degree;
        let mut root = pitch_class_for_degree(settings.key, settings.scale, degree);
        let mut quality = quality_for_degree(settings.scale, degree);
        if let Some((borrowed_root, borrowed_quality)) = borrowed_chord(root, settings, rng) {
            root = borrowed_root;
            quality = borrowed_quality;
        }

        if should_surprise_quality(settings, rng) {
            quality = surprise_quality(quality, rng);
        }

        quality = tension_quality(settings, degree, quality, is_penultimate, rng);

        chords.push(ChordEvent {
            root,
            quality,
            degree,
            start_ticks: start,
            duration_ticks: duration,
            tension: settings.tension,
        });
        start += duration;
        chord_index += 1;
    }

    chords
}

fn generate_boards_of_canada_chords(
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> Vec<ChordEvent> {
    let pattern = boc_progression_pattern(settings, rng);
    let bars_per_chord = if settings.bars >= 8 { 2 } else { 1 };
    let chord_ticks = ticks_per_bar() * bars_per_chord;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    let mut chords = Vec::new();
    let mut start = 0;
    let mut index = 0usize;

    while start < total_ticks {
        let remaining = total_ticks - start;
        let duration = chord_ticks.min(remaining);
        let is_final = start + duration >= total_ticks;
        let mut offset = pattern[index % pattern.len()];
        if is_final && rng.gen_range(0..100) < settings.cadence {
            offset = 0;
        }

        let root = boc_root_for_offset(settings, offset);
        chords.push(ChordEvent {
            root,
            quality: boc_chord_quality(settings, index, rng),
            degree: boc_degree_label(offset, settings.scale.degree_count()),
            start_ticks: start,
            duration_ticks: duration,
            tension: settings.tension,
        });

        start += duration;
        index += 1;
    }

    chords
}

fn boc_progression_pattern(settings: &GeneratorSettings, rng: &mut StdRng) -> &'static [i8] {
    let grounded: &[&[i8]] = &[&[0, 3, 7, 0], &[0, 3, 5, 0], &[0, 8, 5, 3]];
    let wandering: &[&[i8]] = &[
        &[0, 3, 7, 0],
        &[0, 3, 5, 0],
        &[0, 8, 5, 3],
        &[0, 7, 3, 5],
        &[0, 3, 10, 5],
    ];
    let pool = if settings.surprise > 50 {
        wandering
    } else {
        grounded
    };
    pool[rng.gen_range(0..pool.len())]
}

fn boc_root_for_offset(settings: &GeneratorSettings, offset: i8) -> u8 {
    ((settings.key.semitone() + offset) as i16).rem_euclid(12) as u8
}

fn boc_degree_label(offset: i8, scale_degree_count: usize) -> usize {
    let normalized = offset.rem_euclid(12);
    match normalized {
        0 => 0,
        2 => 1,
        3 => 2,
        5 => 3,
        7 => 4,
        8 => 5,
        10 => 6,
        _ => normalized as usize % scale_degree_count,
    }
}

fn boc_chord_quality(settings: &GeneratorSettings, index: usize, rng: &mut StdRng) -> ChordQuality {
    if settings.tension > 70 && rng.gen_range(0..100) < settings.tension / 2 {
        if settings.surprise > 70 && rng.gen_bool(0.25) {
            ChordQuality::Add9
        } else if rng.gen_bool(0.55) {
            ChordQuality::Minor7
        } else {
            ChordQuality::Sus2
        }
    } else if settings.tension > 45 && index % 4 == 2 && rng.gen_bool(0.5) {
        ChordQuality::Minor7
    } else if settings.surprise > 65 && rng.gen_range(0..100) < settings.surprise / 3 {
        ChordQuality::Sus2
    } else {
        ChordQuality::MinorDyad
    }
}

fn chord_style_degree(style: ChordStyle, index: usize, scale_degree_count: usize) -> usize {
    let pattern: &[usize] = match style {
        ChordStyle::Balanced => &[0, 3, 4, 0],
        ChordStyle::Pop => &[0, 4, 5, 3],
        ChordStyle::Modal => &[0, 3, 0, 6],
        ChordStyle::Jazz => &[1, 4, 0, 5],
        ChordStyle::MinorCinematic => &[0, 5, 2, 6],
        ChordStyle::AcidMinimal => &[0, 0, 6, 0],
        ChordStyle::ChiptuneLoop => &[0, 4, 5, 3],
        ChordStyle::BoardsOfCanada => &[0, 2, 4, 0],
    };

    pattern[index % pattern.len()] % scale_degree_count
}

fn choose_next_degree(current: usize, settings: &GeneratorSettings, rng: &mut StdRng) -> usize {
    let count = settings.scale.degree_count();
    let stable_targets = if settings.scale.is_minorish() {
        [0, 2, 3, 4, 5]
    } else {
        [0, 1, 3, 4, 5]
    };
    let functional_moves = [3, 4, 5, 1, 0];

    if rng.gen_range(0..100) < settings.surprise {
        return surprising_degree(current, count, rng);
    }

    if rng.gen_range(0..100) < settings.tension {
        functional_moves[rng.gen_range(0..functional_moves.len())] % count
    } else {
        let step = if rng.gen_bool(0.5) { 1 } else { count - 1 };
        let candidate = (current + step) % count;
        if rng.gen_bool(0.65) {
            candidate
        } else {
            stable_targets[rng.gen_range(0..stable_targets.len())] % count
        }
    }
}

fn surprising_degree(current: usize, count: usize, rng: &mut StdRng) -> usize {
    if count <= 1 {
        return 0;
    }

    let leaps = [2usize, 3, 4, 5];
    let offset = leaps[rng.gen_range(0..leaps.len())] % count;
    let candidate = (current + offset) % count;
    if candidate == current {
        (current + 1) % count
    } else {
        candidate
    }
}

fn cadence_approach_degree(settings: &GeneratorSettings, rng: &mut StdRng) -> usize {
    let count = settings.scale.degree_count();
    let candidates: &[usize] = if settings.scale.is_minorish() {
        &[4, 3, 1, 6]
    } else {
        &[4, 3, 1]
    };
    candidates[rng.gen_range(0..candidates.len())] % count
}

fn borrowed_chord(
    root: u8,
    settings: &GeneratorSettings,
    rng: &mut StdRng,
) -> Option<(u8, ChordQuality)> {
    if settings.surprise <= 35 {
        return None;
    }
    if rng.gen_range(0..100) >= (settings.surprise - 35) / 2 {
        return None;
    }

    let colors = [
        (1u8, ChordQuality::Major),
        (3, ChordQuality::Minor),
        (6, ChordQuality::Dominant),
        (8, ChordQuality::Major),
        (10, ChordQuality::Major),
    ];
    let (offset, quality) = colors[rng.gen_range(0..colors.len())];
    Some(((root + offset) % 12, quality))
}

fn should_surprise_quality(settings: &GeneratorSettings, rng: &mut StdRng) -> bool {
    settings.surprise > 30 && rng.gen_range(0..100) < settings.surprise / 3
}

fn surprise_quality(current: ChordQuality, rng: &mut StdRng) -> ChordQuality {
    let colors = [
        ChordQuality::Dominant,
        ChordQuality::Suspended,
        ChordQuality::Minor7,
        ChordQuality::Sus2,
        ChordQuality::Add9,
    ];
    let picked = colors[rng.gen_range(0..colors.len())];
    if picked == current {
        ChordQuality::Suspended
    } else {
        picked
    }
}

fn tension_quality(
    settings: &GeneratorSettings,
    degree: usize,
    current: ChordQuality,
    is_penultimate: bool,
    rng: &mut StdRng,
) -> ChordQuality {
    if settings.tension <= 55 || rng.gen_range(0..100) >= settings.tension / 2 {
        return current;
    }

    let scale_degree = degree % settings.scale.degree_count();
    if is_penultimate || scale_degree == 4 {
        ChordQuality::Dominant
    } else if matches!(scale_degree, 1 | 3) {
        if rng.gen_bool(0.6) {
            ChordQuality::Suspended
        } else {
            ChordQuality::Add9
        }
    } else if rng.gen_bool(0.35) {
        ChordQuality::Sus2
    } else {
        current
    }
}

fn generate_melodic(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let mut cursor = 0;
    let total = ticks_per_bar() * settings.bars as u32;
    let mut last_pitch = 60 + settings.key.semitone() as i32;

    while cursor < total {
        let patterns = melodic_rhythm_patterns(settings.rhythm_style);
        let pattern = patterns[rng.gen_range(0..patterns.len())];
        for duration in pattern {
            if cursor >= total {
                break;
            }
            let dur = (*duration).min(total - cursor);
            if rng.gen_range(0..100) <= rhythm_density(settings) {
                let chord = chord_at(chords, cursor);
                let strong = cursor % ticks_per_bar() == 0 || cursor % PPQN as u32 == 0;
                let pitch = choose_melodic_pitch(settings, chord, last_pitch, strong, rng);
                last_pitch = pitch as i32;
                notes.push(NoteEvent {
                    pitch,
                    start_ticks: cursor,
                    duration_ticks: note_duration(settings, dur, rng),
                    velocity: velocity_for(settings, cursor, rng),
                });
            }
            cursor += dur;
        }
    }
    notes
}

fn generate_euclidean(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let steps_per_bar = 16;
    let pulses = ((rhythm_density(settings) as usize * steps_per_bar) / 120).clamp(2, 14);
    let pattern = euclidean_pattern(
        steps_per_bar,
        pulses,
        settings.surprise as usize % steps_per_bar,
    );
    let mut notes = Vec::new();
    let step_ticks = ticks_per_bar() / steps_per_bar as u32;

    for bar in 0..settings.bars as u32 {
        for (step, active) in pattern.iter().enumerate() {
            if !active {
                continue;
            }
            let start = bar * ticks_per_bar() + step as u32 * step_ticks;
            let chord = chord_at(chords, start);
            let pitch = choose_chord_or_scale_pitch(settings, chord, rng);
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks, rng),
                velocity: velocity_for(settings, start, rng),
            });
        }
    }

    notes
}

fn melodic_rhythm_patterns(style: RhythmStyle) -> &'static [&'static [u32]] {
    match style {
        RhythmStyle::Straight => &[
            &[480, 480, 480, 480],
            &[960, 480, 480],
            &[720, 240, 480, 480],
        ],
        RhythmStyle::Syncopated => &[
            &[240, 720, 240, 720],
            &[360, 120, 480, 960],
            &[240, 240, 480, 960],
        ],
        RhythmStyle::Sparse => &[&[960, 960], &[1440, 480], &[1920]],
        RhythmStyle::Busy => &[
            &[240, 240, 240, 240, 480, 480],
            &[240; 8],
            &[360, 120, 240, 240, 480, 480],
        ],
        RhythmStyle::Dotted => &[&[720, 240, 720, 240], &[360, 120, 360, 120, 960]],
    }
}

fn rhythm_density(settings: &GeneratorSettings) -> u8 {
    let adjusted = match settings.rhythm_style {
        RhythmStyle::Straight => settings.density as i16,
        RhythmStyle::Syncopated => settings.density as i16 + 8,
        RhythmStyle::Sparse => settings.density as i16 - 24,
        RhythmStyle::Busy => settings.density as i16 + 18,
        RhythmStyle::Dotted => settings.density as i16 - 4,
    };

    adjusted.clamp(5, 100) as u8
}

fn generate_arp(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let rate = match settings.rhythm_style {
        RhythmStyle::Busy => 240,
        RhythmStyle::Sparse => 960,
        RhythmStyle::Dotted => 360,
        _ if settings.density > 70 => 240,
        _ => 480,
    };
    let mut cycle = 0usize;

    for chord in chords {
        let pattern_pitches = arp_pattern_pitches(settings, chord);
        let order = arp_order(settings.arp_pattern, pattern_pitches.len(), rng);
        let mut cursor = chord.start_ticks;
        let mut index = 0;
        while cursor < chord.start_ticks + chord.duration_ticks {
            let pattern_index = order[index % order.len()];
            let pitch = if settings.arp_rotation != ArpRotation::Off
                && pattern_index + 1 == settings.arp_rotate_slot as usize
            {
                rotating_arp_pitch(settings, cycle)
            } else {
                pattern_pitches[pattern_index]
            };
            notes.push(NoteEvent {
                pitch,
                start_ticks: cursor,
                duration_ticks: note_duration(settings, rate, rng),
                velocity: velocity_for(settings, cursor, rng),
            });
            cursor += rate;
            index += 1;
            if index % order.len() == 0 {
                cycle += 1;
            }
        }
    }

    notes
}

fn arp_pattern_pitches(settings: &GeneratorSettings, chord: &ChordEvent) -> Vec<u8> {
    let mut pitches = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    pitches.sort_unstable();
    pitches.dedup();

    let scale_pitches = scale_pitches_in_range(settings);
    for pitch in scale_pitches {
        if pitches.len() >= settings.arp_note_count as usize {
            break;
        }
        if !pitches.contains(&pitch) {
            pitches.push(pitch);
        }
    }

    pitches.sort_unstable();
    while pitches.len() < settings.arp_note_count as usize {
        let fallback = pitches
            .last()
            .copied()
            .or_else(|| scale_pitches_in_range(settings).first().copied())
            .unwrap_or_else(|| settings.low_pitch());
        pitches.push(fallback);
    }

    pitches.truncate(settings.arp_note_count as usize);
    pitches
}

fn arp_order(pattern: ArpPattern, note_count: usize, rng: &mut StdRng) -> Vec<usize> {
    match pattern {
        ArpPattern::Up => (0..note_count).collect(),
        ArpPattern::Down => (0..note_count).rev().collect(),
        ArpPattern::UpDown => {
            if note_count <= 2 {
                (0..note_count).collect()
            } else {
                (0..note_count).chain((1..note_count - 1).rev()).collect()
            }
        }
        ArpPattern::RandomWalk => random_walk_order(note_count, rng),
    }
}

fn random_walk_order(note_count: usize, rng: &mut StdRng) -> Vec<usize> {
    let steps = note_count.max(2) * 2;
    let mut current = rng.gen_range(0..note_count);
    let mut order = Vec::with_capacity(steps);
    for _ in 0..steps {
        order.push(current);
        let direction: isize = if rng.gen_bool(0.5) { 1 } else { -1 };
        current = (current as isize + direction).rem_euclid(note_count as isize) as usize;
    }
    order
}

fn rotating_arp_pitch(settings: &GeneratorSettings, cycle: usize) -> u8 {
    let pitches = scale_pitches_in_range(settings);
    if pitches.is_empty() {
        return settings.low_pitch();
    }

    let start = (settings.arp_rotate_slot as usize - 1) % pitches.len();
    let index = match settings.arp_rotation {
        ArpRotation::Off | ArpRotation::Up => (start + cycle) % pitches.len(),
        ArpRotation::Down => (start + pitches.len() - (cycle % pitches.len())) % pitches.len(),
    };
    pitches[index]
}

fn generate_chiptune(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let total_steps = settings.bars as u32 * 16;
    let motif = [0, 2, 4, 7, 4, 2, 0, 12];

    for step in 0..total_steps {
        let start = step * step_ticks;
        if step % 2 != 0 && rng.gen_range(0..100) > rhythm_density(settings) {
            continue;
        }
        let chord = chord_at(chords, start);
        let degree = motif[(step as usize + settings.seed as usize) % motif.len()];
        let base = scale_pitch(settings, degree, settings.min_octave as i8 + 1)
            .clamp(settings.low_pitch(), settings.high_pitch());
        let chord_tones = chord.tones();
        let pitch = if step % 4 == 0 {
            nearest_pitch_class(settings, base, &chord_tones)
        } else if rng.gen_bool(0.22) {
            base.saturating_add(12).min(settings.high_pitch())
        } else {
            base
        };

        notes.push(NoteEvent {
            pitch,
            start_ticks: start,
            duration_ticks: note_duration(settings, step_ticks, rng),
            velocity: velocity_for(settings, start, rng),
        });
    }

    notes
}

fn generate_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    match settings.bassline_style {
        BasslineStyle::Techno => generate_techno_bassline(settings, chords, rng),
        BasslineStyle::House => generate_house_bassline(settings, chords, rng),
        BasslineStyle::Drill => generate_drill_bassline(settings, chords, rng),
        BasslineStyle::HipHop => generate_hiphop_bassline(settings, chords, rng),
        BasslineStyle::UkGarage => generate_uk_garage_bassline(settings, chords, rng),
        BasslineStyle::DrumAndBass => generate_drum_and_bassline(settings, chords, rng),
    }
}

fn generate_chord_pads(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let mut previous_voicing: Option<Vec<u8>> = None;

    for chord in chords {
        let mut pitches = chord_pad_pitches(settings, chord, rng);
        if let Some(previous) = previous_voicing.as_deref() {
            pitches = voice_lead_chord_pad_voicing(settings, pitches, previous);
        }
        previous_voicing = Some(pitches.clone());
        for (index, pitch) in pitches.into_iter().enumerate() {
            let strum_ticks = (index as u32) * (12 + rng.gen_range(0..=18));
            if strum_ticks >= chord.duration_ticks {
                continue;
            }
            let start_ticks = chord.start_ticks + strum_ticks;
            let available = chord.duration_ticks - strum_ticks;
            let duration_ticks = note_duration(settings, available, rng)
                .min(available)
                .max(1);
            let base_velocity = velocity_for(settings, chord.start_ticks, rng);
            let velocity = (base_velocity as i16 - index as i16 * 5 + rng.gen_range(-4..=4))
                .clamp(36, 118) as u8;
            notes.push(NoteEvent {
                pitch,
                start_ticks,
                duration_ticks,
                velocity,
            });
        }
    }

    notes
}

fn chord_pad_pitches(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    rng: &mut StdRng,
) -> Vec<u8> {
    let candidates = chord_pitches_in_range(chord, settings.low_pitch(), settings.high_pitch());
    if candidates.is_empty() {
        return vec![settings.low_pitch()];
    }

    let octave_span = settings.max_octave.saturating_sub(settings.min_octave) as usize;
    let target_count = (chord.tones().len() + octave_span).clamp(2, 8);
    let voicing = spread_voicing(candidates, target_count);

    maybe_invert_chord_pad_voicing(settings, voicing, rng)
}

fn spread_voicing(candidates: Vec<u8>, target_count: usize) -> Vec<u8> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    candidates.dedup();

    if candidates.len() <= target_count {
        return candidates;
    }

    let last = candidates.len() - 1;
    let picks = target_count.max(2);
    let mut pitches = Vec::with_capacity(picks);
    for index in 0..picks {
        let candidate_index = ((index * last) + (picks - 1) / 2) / (picks - 1);
        let pitch = candidates[candidate_index];
        if !pitches.contains(&pitch) {
            pitches.push(pitch);
        }
    }

    pitches
}

fn maybe_invert_chord_pad_voicing(
    settings: &GeneratorSettings,
    voicing: Vec<u8>,
    rng: &mut StdRng,
) -> Vec<u8> {
    if settings.chord_inversion_amount == 0
        || voicing.len() < 3
        || rng.gen_range(0..100) >= settings.chord_inversion_amount
    {
        return voicing;
    }

    let inversion_depth = if voicing.len() >= 4 && rng.gen_bool(0.45) {
        2
    } else {
        1
    };
    invert_chord_pad_voicing(
        &voicing,
        inversion_depth,
        settings.low_pitch(),
        settings.high_pitch(),
    )
    .unwrap_or(voicing)
}

fn invert_chord_pad_voicing(
    voicing: &[u8],
    inversion_depth: usize,
    low_pitch: u8,
    high_pitch: u8,
) -> Option<Vec<u8>> {
    if voicing.len() < 3 || inversion_depth == 0 {
        return None;
    }

    let mut inverted = voicing.to_vec();
    inverted.sort_unstable();
    let moves = inversion_depth.min(inverted.len() - 1);
    for _ in 0..moves {
        let lowest = inverted.remove(0);
        let raised = lowest.checked_add(12)?;
        if raised > high_pitch {
            return None;
        }
        inverted.push(raised);
        inverted.sort_unstable();
    }
    inverted.dedup();

    if inverted.len() < 3
        || !inverted
            .iter()
            .all(|pitch| (low_pitch..=high_pitch).contains(pitch))
    {
        None
    } else {
        Some(inverted)
    }
}

fn voice_lead_chord_pad_voicing(
    settings: &GeneratorSettings,
    voicing: Vec<u8>,
    previous: &[u8],
) -> Vec<u8> {
    if voicing.is_empty() || previous.is_empty() {
        return voicing;
    }

    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let previous_center = voicing_center(previous);
    let mut candidates = vec![voicing.clone()];

    for shift in [-12i16, 12] {
        let shifted: Option<Vec<u8>> = voicing
            .iter()
            .map(|pitch| {
                let shifted = *pitch as i16 + shift;
                if (low as i16..=high as i16).contains(&shifted) {
                    Some(shifted as u8)
                } else {
                    None
                }
            })
            .collect();
        if let Some(shifted) = shifted {
            candidates.push(shifted);
        }
    }

    for depth in 1..=2 {
        if let Some(inverted) = invert_chord_pad_voicing(&voicing, depth, low, high) {
            candidates.push(inverted);
        }
    }

    candidates
        .into_iter()
        .min_by_key(|candidate| {
            ((voicing_center(candidate) - previous_center).abs() * 100.0).round() as i32
        })
        .unwrap_or(voicing)
}

fn voicing_center(voicing: &[u8]) -> f32 {
    let sum: u32 = voicing.iter().map(|pitch| *pitch as u32).sum();
    sum as f32 / voicing.len() as f32
}

fn generate_techno_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let total_steps = settings.bars as u32 * 16;
    let mut previous_was_rest = true;
    let mut previous_pitch = None;

    for step in 0..total_steps {
        let start = step * step_ticks;
        let beat_step = step % 16;
        let base_probability = rhythm_density(settings) as i16;
        let downbeat_bonus = if beat_step == 0 || beat_step == 8 {
            18
        } else {
            0
        };
        let syncopation_bonus = if matches!(beat_step, 3 | 6 | 10 | 14) {
            settings.bassline_mutation as i16 / 4
        } else {
            0
        };
        let active_probability =
            (base_probability + downbeat_bonus + syncopation_bonus).clamp(0, 100);

        if rng.gen_range(0..100) >= active_probability {
            previous_was_rest = true;
            continue;
        }

        let chord = chord_at(chords, start);
        let pitch = choose_bassline_pitch(settings, chord, step, rng);
        let accented = is_bassline_accented(settings, beat_step, previous_was_rest, rng);
        let sliding = should_bassline_slide(settings, previous_pitch, pitch, rng);
        let duration_ticks = if sliding {
            ((step_ticks as f32) * 1.35).round() as u32
        } else {
            note_duration(settings, step_ticks, rng).min(step_ticks)
        };
        let velocity = if accented {
            116
        } else {
            velocity_for(settings, start, rng).min(96)
        };

        notes.push(NoteEvent {
            pitch,
            start_ticks: start,
            duration_ticks: duration_ticks.max(1),
            velocity,
        });

        previous_was_rest = false;
        previous_pitch = Some(pitch);
    }

    notes
}

fn generate_house_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let pattern = [2_u32, 4, 6, 10, 12, 14];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if step != 2 && rng.gen_range(0..100) > rhythm_density(settings) {
                continue;
            }
            let start = bar * ticks_per_bar() + step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = match step {
                2 | 10 => 0,
                4 | 12 => 2,
                6 | 14 => 4,
                _ => 0,
            };
            let pitch = choose_bass_degree_pitch(settings, chord, degree, rng);
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if matches!(step, 2 | 10) {
                    108
                } else {
                    velocity_for(settings, start, rng)
                },
            });
        }
    }

    notes
}

fn generate_drill_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let total_bars = settings.bars as u32;
    let pattern = [0_u32, 6, 11, 16, 24, 30, 42, 48, 54, 59];

    for bar_group in (0..total_bars).step_by(4) {
        for step in pattern {
            let absolute_step = bar_group * 16 + step;
            if absolute_step >= total_bars * 16 {
                continue;
            }
            if step % 16 != 0 && rng.gen_range(0..100) > rhythm_density(settings) + 10 {
                continue;
            }

            let start = absolute_step * step_ticks;
            let chord = chord_at(chords, start);
            let slide_pick =
                matches!(step, 11 | 30 | 54) && rng.gen_range(0..100) < settings.bassline_slide;
            let degree = if slide_pick { 2 } else { 0 };
            let mut pitch = choose_bass_degree_pitch(settings, chord, degree, rng);
            if slide_pick {
                pitch = pitch.saturating_add(3).min(settings.high_pitch());
            }
            let duration = if slide_pick {
                step_ticks * 3
            } else {
                note_duration(settings, step_ticks * 4, rng).max(step_ticks * 2)
            };
            notes.push(NoteEvent {
                pitch,
                start_ticks: start,
                duration_ticks: duration,
                velocity: if slide_pick { 118 } else { 102 },
            });
        }
    }

    notes
}

fn generate_hiphop_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let pattern = [0_u32, 7, 12, 22, 32, 38, 44, 55];

    for bar_group in (0..settings.bars as u32).step_by(4) {
        for step in pattern {
            let absolute_step = bar_group * 16 + step;
            if absolute_step >= settings.bars as u32 * 16 {
                continue;
            }
            if step != 0 && rng.gen_range(0..100) > rhythm_density(settings) + 5 {
                continue;
            }

            let start = absolute_step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = if rng.gen_range(0..100) < settings.bassline_mutation {
                4
            } else {
                0
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 4, rng).max(step_ticks * 2),
                velocity: velocity_for(settings, start, rng).max(86),
            });
        }
    }

    notes
}

fn generate_uk_garage_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let swing_ticks = ((step_ticks as f32) * 0.42).round() as u32;
    let pattern = [0_u32, 5, 7, 10, 13, 15];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if !matches!(step, 0 | 7 | 13) && rng.gen_range(0..100) > rhythm_density(settings) {
                continue;
            }
            let unswung = bar * ticks_per_bar() + step * step_ticks;
            let start = if step % 2 == 1 {
                unswung + swing_ticks
            } else {
                unswung
            };
            let chord = chord_at(chords, start);
            let degree = match step {
                5 | 13 => 4,
                7 | 15 => 2,
                _ => 0,
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if step % 2 == 1 { 110 } else { 94 },
            });
        }
    }

    notes
}

fn generate_drum_and_bassline(
    settings: &GeneratorSettings,
    chords: &[ChordEvent],
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let mut notes = Vec::new();
    let step_ticks = PPQN as u32 / 4;
    let pattern = [0_u32, 3, 7, 10, 14];

    for bar in 0..settings.bars as u32 {
        for step in pattern {
            if matches!(step, 3 | 10) && rng.gen_range(0..100) > settings.bassline_mutation + 30 {
                continue;
            }
            let start = bar * ticks_per_bar() + step * step_ticks;
            let chord = chord_at(chords, start);
            let degree = match step {
                7 | 14 => 4,
                3 | 10 => rng.gen_range(0..settings.scale.degree_count()),
                _ => 0,
            };
            notes.push(NoteEvent {
                pitch: choose_bass_degree_pitch(settings, chord, degree, rng),
                start_ticks: start,
                duration_ticks: note_duration(settings, step_ticks * 2, rng).min(step_ticks * 2),
                velocity: if matches!(step, 0 | 14) { 116 } else { 92 },
            });
        }
    }

    notes
}

fn choose_bassline_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    step: u32,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let midpoint = low + (high - low) / 2;
    let mut candidates = bassline_chord_candidates(chord, low, midpoint.max(low));

    if candidates.is_empty() {
        candidates = scale_pitches_in_range(settings)
            .into_iter()
            .filter(|pitch| *pitch <= midpoint.max(low))
            .collect();
    }
    if candidates.is_empty() {
        candidates.push(low);
    }

    let root_class = chord.root % 12;
    let fifth_class = (chord.root + 7) % 12;
    let pitch = if step % 8 == 0 {
        candidates
            .iter()
            .copied()
            .find(|candidate| candidate % 12 == root_class)
            .unwrap_or(candidates[0])
    } else if rng.gen_range(0..100) < 40 {
        candidates
            .iter()
            .copied()
            .find(|candidate| candidate % 12 == fifth_class)
            .unwrap_or_else(|| candidates[rng.gen_range(0..candidates.len())])
    } else {
        candidates[rng.gen_range(0..candidates.len())]
    };

    if rng.gen_range(0..100) < settings.bassline_octave_jump && pitch + 12 <= high {
        pitch + 12
    } else {
        pitch
    }
}

fn choose_bass_degree_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    degree: usize,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let target_class = if degree == 0 {
        chord.root % 12
    } else {
        pitch_class_for_degree(settings.key, settings.scale, degree) % 12
    };
    let mut candidates: Vec<u8> = (low..=high)
        .filter(|pitch| *pitch <= low.saturating_add(24).min(high) && pitch % 12 == target_class)
        .collect();
    if candidates.is_empty() {
        candidates = bassline_chord_candidates(chord, low, low.saturating_add(24).min(high));
    }
    if candidates.is_empty() {
        candidates = scale_pitches_in_range(settings)
            .into_iter()
            .filter(|pitch| *pitch <= low.saturating_add(24).min(high))
            .collect();
    }
    let mut pitch = candidates
        .get(rng.gen_range(0..candidates.len().max(1)))
        .copied()
        .unwrap_or(low);
    if rng.gen_range(0..100) < settings.bassline_octave_jump && pitch + 12 <= high {
        pitch += 12;
    }
    pitch
}

fn bassline_chord_candidates(chord: &ChordEvent, low: u8, high: u8) -> Vec<u8> {
    let mut tone_classes = chord.tones();
    tone_classes.push((chord.root + 3) % 12);
    tone_classes.sort_unstable();
    tone_classes.dedup();

    (low..=high)
        .filter(|pitch| tone_classes.contains(&(pitch % 12)))
        .collect()
}

fn is_bassline_accented(
    settings: &GeneratorSettings,
    beat_step: u32,
    previous_was_rest: bool,
    rng: &mut StdRng,
) -> bool {
    let structural_bonus = if beat_step == 0 || beat_step == 8 {
        30
    } else if previous_was_rest {
        20
    } else if matches!(beat_step, 3 | 6 | 10 | 14) {
        12
    } else {
        0
    };
    rng.gen_range(0..100) < (settings.bassline_accent + structural_bonus).min(100)
}

fn should_bassline_slide(
    settings: &GeneratorSettings,
    previous_pitch: Option<u8>,
    pitch: u8,
    rng: &mut StdRng,
) -> bool {
    previous_pitch.is_some_and(|previous| {
        previous != pitch && rng.gen_range(0..100) < settings.bassline_slide
    })
}

fn apply_phrase_memory(
    settings: &GeneratorSettings,
    mut notes: Vec<NoteEvent>,
    rng: &mut StdRng,
) -> Vec<NoteEvent> {
    let phrase_ticks = ticks_per_bar() * settings.phrase_length as u32;
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    if settings.repeat_amount == 0 || phrase_ticks == 0 || phrase_ticks >= total_ticks {
        return cleanup_notes(settings, notes);
    }

    let template: Vec<NoteEvent> = notes
        .iter()
        .filter(|note| note.start_ticks < phrase_ticks)
        .cloned()
        .collect();
    if template.is_empty() {
        return cleanup_notes(settings, notes);
    }

    let mut phrase_start = phrase_ticks;
    while phrase_start < total_ticks {
        let phrase_end = (phrase_start + phrase_ticks).min(total_ticks);
        if rng.gen_range(0..100) < settings.repeat_amount {
            notes.retain(|note| note.start_ticks < phrase_start || note.start_ticks >= phrase_end);
            for source in &template {
                let start_ticks = phrase_start + source.start_ticks;
                if start_ticks >= phrase_end {
                    continue;
                }

                let mut copied = source.clone();
                copied.start_ticks = start_ticks;
                copied.duration_ticks = copied.duration_ticks.min(phrase_end - start_ticks).max(1);
                if rng.gen_range(0..100) < settings.variation_amount {
                    copied.pitch = vary_pitch_by_scale_step(settings, copied.pitch, rng);
                    copied.velocity = vary_velocity(copied.velocity, rng);
                }
                notes.push(copied);
            }
        }
        phrase_start += phrase_ticks;
    }

    cleanup_notes(settings, notes)
}

fn vary_pitch_by_scale_step(settings: &GeneratorSettings, pitch: u8, rng: &mut StdRng) -> u8 {
    let scale = scale_pitches_in_range(settings);
    if scale.is_empty() {
        return pitch.clamp(settings.low_pitch(), settings.high_pitch());
    }

    let index = scale
        .iter()
        .enumerate()
        .min_by_key(|(_, candidate)| (**candidate as i16 - pitch as i16).abs())
        .map(|(index, _)| index)
        .unwrap_or(0);
    let direction: isize = if rng.gen_bool(0.5) { 1 } else { -1 };
    let next = (index as isize + direction).clamp(0, scale.len() as isize - 1) as usize;
    scale[next]
}

fn vary_velocity(velocity: u8, rng: &mut StdRng) -> u8 {
    let offset: i16 = rng.gen_range(-8..=8);
    (velocity as i16 + offset).clamp(1, 127) as u8
}

fn cleanup_notes(settings: &GeneratorSettings, mut notes: Vec<NoteEvent>) -> Vec<NoteEvent> {
    let total_ticks = ticks_per_bar() * settings.bars as u32;
    notes.retain(|note| {
        note.start_ticks < total_ticks
            && note.duration_ticks > 0
            && (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)
    });
    notes.sort_by_key(|note| (note.start_ticks, note.pitch, note.duration_ticks));
    notes.dedup_by_key(|note| (note.start_ticks, note.pitch));
    notes
}

fn apply_velocity_range(settings: &GeneratorSettings, mut notes: Vec<NoteEvent>) -> Vec<NoteEvent> {
    if settings.velocity_mode != VelocityMode::Random {
        return notes;
    }

    let low = settings.random_velocity_min.min(settings.random_velocity_max);
    let high = settings.random_velocity_min.max(settings.random_velocity_max);
    for note in &mut notes {
        note.velocity = note.velocity.clamp(low, high);
    }
    notes
}

fn euclidean_pattern(steps: usize, pulses: usize, rotation: usize) -> Vec<bool> {
    (0..steps)
        .map(|step| {
            let rotated = (step + rotation) % steps;
            (rotated * pulses) % steps < pulses
        })
        .collect()
}

fn choose_melodic_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    last_pitch: i32,
    strong: bool,
    rng: &mut StdRng,
) -> u8 {
    if strong || rng.gen_bool(0.42) {
        choose_chord_or_scale_pitch(settings, chord, rng)
    } else {
        let step = if rng.gen_bool(0.55) { 1 } else { -1 };
        let candidate = last_pitch + step * rng.gen_range(1..=2);
        nearest_scale_pitch(settings, candidate) as u8
    }
}

fn choose_chord_or_scale_pitch(
    settings: &GeneratorSettings,
    chord: &ChordEvent,
    rng: &mut StdRng,
) -> u8 {
    let low = settings.low_pitch();
    let high = settings.high_pitch();
    let chord_tones = chord_pitches_in_range(chord, low, high);
    if rng.gen_bool(0.72) && !chord_tones.is_empty() {
        chord_tones[rng.gen_range(0..chord_tones.len())]
    } else {
        let octave = rng.gen_range(settings.min_octave..=settings.max_octave) as i8;
        let pitch = scale_pitch(
            settings,
            rng.gen_range(0..settings.scale.degree_count()),
            octave,
        );
        if (low..=high).contains(&pitch) {
            pitch
        } else {
            nearest_scale_pitch(settings, pitch as i32) as u8
        }
    }
}

fn chord_pitches_in_range(chord: &ChordEvent, low: u8, high: u8) -> Vec<u8> {
    let tones = chord.tones();
    (low..=high)
        .filter(|pitch| tones.contains(&(pitch % 12)))
        .collect()
}

fn chord_at(chords: &[ChordEvent], tick: u32) -> &ChordEvent {
    chords
        .iter()
        .find(|chord| tick >= chord.start_ticks && tick < chord.start_ticks + chord.duration_ticks)
        .unwrap_or_else(|| chords.last().expect("at least one chord"))
}

fn velocity_for(settings: &GeneratorSettings, start: u32, rng: &mut StdRng) -> u8 {
    match settings.velocity_mode {
        VelocityMode::Fixed => 92,
        VelocityMode::Random => {
            rng.gen_range(settings.random_velocity_min..=settings.random_velocity_max)
        }
        VelocityMode::Accented => {
            if start % ticks_per_bar() == 0 {
                116
            } else if start % PPQN as u32 == 0 {
                98
            } else {
                76
            }
        }
        VelocityMode::Humanized => {
            let base = if start % ticks_per_bar() == 0 {
                108
            } else if start % PPQN as u32 == 0 {
                92
            } else {
                74
            };
            (base + rng.gen_range(0..=12)).min(127)
        }
    }
}

fn note_duration(settings: &GeneratorSettings, slot_ticks: u32, rng: &mut StdRng) -> u32 {
    let fixed_gate = PPQN as u32 / 4;
    if settings.note_length == 0 {
        return fixed_gate.max(1);
    }

    let normalized = settings.note_length as f32 / 100.0;
    let base_multiplier = 0.35 + normalized * 1.45;
    let variation = if settings.note_length < 25 {
        0.0
    } else {
        let spread = normalized * 0.35;
        rng.gen_range(-spread..=spread)
    };

    ((slot_ticks as f32 * (base_multiplier + variation)).round() as u32).max(1)
}

fn quality_for_degree(scale: Scale, degree: usize) -> ChordQuality {
    let qualities: &[ChordQuality] = match scale {
        Scale::Major => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
        ],
        Scale::NaturalMinor => &[
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
        ],
        Scale::HarmonicMinor => &[
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Dominant,
            ChordQuality::Major,
            ChordQuality::Diminished,
        ],
        Scale::MajorPentatonic => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Minor,
        ],
        Scale::MinorPentatonic => &[
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Major,
        ],
        Scale::Blues => &[
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Suspended,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor7,
        ],
        Scale::Dorian => &[
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
        ],
        Scale::Mixolydian => &[
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
        ],
    };

    qualities[degree % qualities.len()]
}

fn pitch_class_for_degree(key: Key, scale: Scale, degree: usize) -> u8 {
    let intervals = scale.intervals();
    ((key.semitone() + intervals[degree % intervals.len()]) as i16).rem_euclid(12) as u8
}

fn scale_pitch(settings: &GeneratorSettings, degree: usize, octave: i8) -> u8 {
    let intervals = settings.scale.intervals();
    let octaves = degree / intervals.len();
    let interval = intervals[degree % intervals.len()];
    (12 * octave + settings.key.semitone() + interval + 12 * octaves as i8) as u8
}

fn scale_pitches_in_range(settings: &GeneratorSettings) -> Vec<u8> {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|pitch| {
            let pc = ((*pitch as i8 - settings.key.semitone()) as i16).rem_euclid(12) as i8;
            settings.scale.intervals().contains(&pc)
        })
        .collect()
}

fn nearest_scale_pitch(settings: &GeneratorSettings, pitch: i32) -> i32 {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|candidate| {
            let pc = ((*candidate as i8 - settings.key.semitone()) as i16).rem_euclid(12) as i8;
            settings.scale.intervals().contains(&pc)
        })
        .min_by_key(|candidate| (*candidate as i32 - pitch).abs())
        .map(i32::from)
        .unwrap_or_else(|| i32::from(settings.low_pitch()))
}

fn nearest_pitch_class(settings: &GeneratorSettings, pitch: u8, classes: &[u8]) -> u8 {
    (settings.low_pitch()..=settings.high_pitch())
        .filter(|candidate| classes.contains(&(candidate % 12)))
        .min_by_key(|candidate| (*candidate as i16 - pitch as i16).abs())
        .unwrap_or(pitch)
}

fn octave_to_midi_c(octave: u8) -> u8 {
    12 * (octave + 1)
}

pub fn ticks_per_bar() -> u32 {
    PPQN as u32 * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note_signature(notes: &[NoteEvent]) -> Vec<(u8, u32, u32, u8)> {
        notes
            .iter()
            .map(|note| {
                (
                    note.pitch,
                    note.start_ticks,
                    note.duration_ticks,
                    note.velocity,
                )
            })
            .collect()
    }

    #[test]
    fn euclidean_pattern_has_requested_pulses() {
        let pattern = euclidean_pattern(16, 5, 3);
        assert_eq!(pattern.len(), 16);
        assert_eq!(pattern.iter().filter(|active| **active).count(), 5);
    }

    #[test]
    fn chord_timeline_fills_requested_bars() {
        let settings = GeneratorSettings {
            bars: 5,
            ..GeneratorSettings::default()
        };
        let mut rng = StdRng::seed_from_u64(settings.seed);
        let chords = generate_chords(&settings, &mut rng);
        let total: u32 = chords.iter().map(|chord| chord.duration_ticks).sum();
        assert_eq!(total, ticks_per_bar() * settings.bars as u32);
        assert_eq!(chords.first().map(|chord| chord.start_ticks), Some(0));
    }

    #[test]
    fn locked_chords_are_reused_exactly_across_seeds() {
        let settings = GeneratorSettings {
            seed: 1,
            ..GeneratorSettings::default()
        };
        let source = generate_song(&settings);
        let regenerated = generate_song_with_chords(
            &GeneratorSettings {
                seed: 999,
                ..settings
            },
            Some(&source.chords),
        );

        assert_eq!(regenerated.chords, source.chords);
    }

    #[test]
    fn locked_chords_still_allow_seeded_note_changes() {
        let settings = GeneratorSettings::default();
        let source = generate_song(&settings);
        let first = generate_song_with_chords(
            &GeneratorSettings {
                seed: 11,
                ..settings
            },
            Some(&source.chords),
        );
        let second = generate_song_with_chords(
            &GeneratorSettings {
                seed: 12,
                ..settings
            },
            Some(&source.chords),
        );

        assert_eq!(first.chords, source.chords);
        assert_eq!(second.chords, source.chords);
        assert_ne!(note_signature(&first.notes), note_signature(&second.notes));
    }

    #[test]
    fn locked_chords_clip_when_bars_are_reduced() {
        let source_settings = GeneratorSettings {
            bars: 4,
            ..GeneratorSettings::default()
        };
        let source = generate_song(&source_settings);
        let clipped = generate_song_with_chords(
            &GeneratorSettings {
                bars: 1,
                ..source_settings
            },
            Some(&source.chords),
        );

        assert_eq!(clipped.chords.len(), 1);
        assert_eq!(clipped.chords[0].root, source.chords[0].root);
        assert_eq!(clipped.chords[0].quality, source.chords[0].quality);
        assert_eq!(clipped.chords[0].start_ticks, 0);
        assert_eq!(clipped.chords[0].duration_ticks, ticks_per_bar());
    }

    #[test]
    fn locked_chords_repeat_when_bars_are_expanded() {
        let source_settings = GeneratorSettings {
            bars: 4,
            ..GeneratorSettings::default()
        };
        let source = generate_song(&source_settings);
        let expanded = generate_song_with_chords(
            &GeneratorSettings {
                bars: 6,
                ..source_settings
            },
            Some(&source.chords),
        );

        assert_eq!(expanded.chords.len(), 3);
        assert_eq!(expanded.chords[0], source.chords[0]);
        assert_eq!(expanded.chords[1], source.chords[1]);
        assert_eq!(expanded.chords[2].root, source.chords[0].root);
        assert_eq!(expanded.chords[2].quality, source.chords[0].quality);
        assert_eq!(expanded.chords[2].start_ticks, ticks_per_bar() * 4);
        assert_eq!(expanded.chords[2].duration_ticks, ticks_per_bar() * 2);
    }

    #[test]
    fn locked_chords_are_reused_by_every_generator_mode() {
        let locked_chords = vec![
            ChordEvent {
                root: 1,
                quality: ChordQuality::Minor7,
                degree: 0,
                start_ticks: 0,
                duration_ticks: ticks_per_bar() * 2,
                tension: 70,
            },
            ChordEvent {
                root: 8,
                quality: ChordQuality::Dominant,
                degree: 4,
                start_ticks: ticks_per_bar() * 2,
                duration_ticks: ticks_per_bar() * 2,
                tension: 82,
            },
        ];

        for mode in GeneratorMode::ALL {
            let song = generate_song_with_chords(
                &GeneratorSettings {
                    mode,
                    bars: 4,
                    density: 100,
                    seed: 9876,
                    ..GeneratorSettings::default()
                },
                Some(&locked_chords),
            );

            assert_eq!(song.chords, locked_chords, "{mode} ignored locked chords");
            assert!(!song.notes.is_empty(), "{mode} generated no notes");
        }
    }

    #[test]
    fn locked_chords_are_reused_by_every_bassline_style() {
        let locked_chords = vec![
            ChordEvent {
                root: 3,
                quality: ChordQuality::Minor,
                degree: 0,
                start_ticks: 0,
                duration_ticks: ticks_per_bar() * 2,
                tension: 35,
            },
            ChordEvent {
                root: 10,
                quality: ChordQuality::Suspended,
                degree: 3,
                start_ticks: ticks_per_bar() * 2,
                duration_ticks: ticks_per_bar() * 2,
                tension: 60,
            },
        ];

        for bassline_style in BasslineStyle::ALL {
            let song = generate_song_with_chords(
                &GeneratorSettings {
                    mode: GeneratorMode::Bassline,
                    bassline_style,
                    bars: 4,
                    density: 100,
                    seed: 6789,
                    ..GeneratorSettings::default()
                },
                Some(&locked_chords),
            );

            assert_eq!(
                song.chords, locked_chords,
                "{bassline_style} bassline ignored locked chords"
            );
            assert!(
                !song.notes.is_empty(),
                "{bassline_style} bassline generated no notes"
            );
        }
    }

    #[test]
    fn unlocked_generation_can_change_chords_between_seeds() {
        let settings = GeneratorSettings {
            surprise: 100,
            cadence: 0,
            ..GeneratorSettings::default()
        };
        let first = generate_song(&GeneratorSettings {
            seed: 1,
            ..settings
        });
        let second = generate_song(&GeneratorSettings {
            seed: 2,
            ..settings
        });

        assert_ne!(first.chords, second.chords);
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
                song.notes.iter().all(
                    |note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)
                ),
                "{mode} generated a note outside the selected octave range"
            );
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
    fn random_velocity_range_is_honored_by_every_bassline_style() {
        for bassline_style in BasslineStyle::ALL {
            let settings = GeneratorSettings {
                mode: GeneratorMode::Bassline,
                bassline_style,
                velocity_mode: VelocityMode::Random,
                random_velocity_min: 96,
                random_velocity_max: 99,
                density: 100,
                seed: 2211,
                ..GeneratorSettings::default()
            };
            let song = generate_song(&settings);

            assert!(
                !song.notes.is_empty(),
                "{bassline_style} bassline generated no notes"
            );
            assert!(
                song.notes
                    .iter()
                    .all(|note| (96..=99).contains(&note.velocity)),
                "{bassline_style} bassline did not honor the random velocity range"
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

    #[test]
    fn generator_modes_include_bassline() {
        assert!(GeneratorMode::ALL.contains(&GeneratorMode::Bassline));
    }

    #[test]
    fn generator_modes_include_chord_pads() {
        assert!(GeneratorMode::ALL.contains(&GeneratorMode::ChordPads));
    }

    #[test]
    fn chord_styles_include_boards_of_canada() {
        assert!(ChordStyle::ALL.contains(&ChordStyle::BoardsOfCanada));
    }

    #[test]
    fn boards_of_canada_chords_are_deterministic_for_fixed_seed() {
        let settings = GeneratorSettings {
            chord_style: ChordStyle::BoardsOfCanada,
            seed: 808,
            ..GeneratorSettings::default()
        };
        let first = generate_song(&settings);
        let second = generate_song(&settings);
        assert_eq!(first.chords, second.chords);
    }

    #[test]
    fn boards_of_canada_chords_are_mostly_minor_colored() {
        let settings = GeneratorSettings {
            chord_style: ChordStyle::BoardsOfCanada,
            bars: 8,
            tension: 70,
            seed: 123,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(!song.chords.is_empty());
        assert!(song.chords.iter().all(|chord| matches!(
            chord.quality,
            ChordQuality::MinorDyad | ChordQuality::Minor7 | ChordQuality::Sus2
        )));
    }

    #[test]
    fn boards_of_canada_high_surprise_can_borrow_roots() {
        let settings = GeneratorSettings {
            chord_style: ChordStyle::BoardsOfCanada,
            surprise: 100,
            cadence: 0,
            bars: 8,
            seed: 2,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        let diatonic_roots: Vec<u8> = settings
            .scale
            .intervals()
            .iter()
            .map(|interval| ((settings.key.semitone() + *interval) as i16).rem_euclid(12) as u8)
            .collect();
        assert!(song
            .chords
            .iter()
            .any(|chord| !diatonic_roots.contains(&chord.root)));
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
        let candidates =
            chord_pitches_in_range(&chord, settings.low_pitch(), settings.high_pitch());
        let expected = spread_voicing(
            candidates,
            (chord.tones().len()
                + settings.max_octave.saturating_sub(settings.min_octave) as usize)
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
    fn every_bassline_style_produces_notes() {
        for bassline_style in BasslineStyle::ALL {
            let settings = GeneratorSettings {
                mode: GeneratorMode::Bassline,
                bassline_style,
                ..GeneratorSettings::default()
            };
            let song = generate_song(&settings);
            assert!(
                !song.notes.is_empty(),
                "{bassline_style} generated no notes"
            );
        }
    }

    #[test]
    fn every_bassline_style_is_deterministic_for_fixed_seed() {
        for bassline_style in BasslineStyle::ALL {
            let settings = GeneratorSettings {
                mode: GeneratorMode::Bassline,
                bassline_style,
                seed: 1234,
                ..GeneratorSettings::default()
            };
            let first = generate_song(&settings);
            let second = generate_song(&settings);
            assert_eq!(first.notes.len(), second.notes.len(), "{bassline_style}");
            assert!(first.notes.iter().zip(second.notes.iter()).all(|(a, b)| {
                a.pitch == b.pitch
                    && a.start_ticks == b.start_ticks
                    && a.duration_ticks == b.duration_ticks
                    && a.velocity == b.velocity
            }));
        }
    }

    #[test]
    fn every_bassline_style_respects_global_octave_range() {
        for bassline_style in BasslineStyle::ALL {
            let settings = GeneratorSettings {
                mode: GeneratorMode::Bassline,
                bassline_style,
                min_octave: 2,
                max_octave: 3,
                density: 100,
                ..GeneratorSettings::default()
            };
            let song = generate_song(&settings);
            assert!(!song.notes.is_empty(), "{bassline_style}");
            assert!(
                song.notes.iter().all(
                    |note| (settings.low_pitch()..=settings.high_pitch()).contains(&note.pitch)
                ),
                "{bassline_style} generated a note outside the selected octave range"
            );
        }
    }

    #[test]
    fn bassline_high_accent_creates_accent_velocity() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::Techno,
            density: 100,
            bassline_accent: 100,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        assert!(song.notes.iter().any(|note| note.velocity >= 116));
    }

    #[test]
    fn drill_high_slide_creates_legato_overlap() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::Drill,
            density: 100,
            bassline_slide: 100,
            bassline_mutation: 100,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        let step_ticks = PPQN as u32 / 4;
        assert!(song
            .notes
            .iter()
            .any(|note| note.duration_ticks > step_ticks));
    }

    #[test]
    fn techno_density_controls_note_count() {
        let sparse = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::Techno,
            density: 20,
            seed: 77,
            ..GeneratorSettings::default()
        };
        let dense = GeneratorSettings {
            density: 95,
            ..sparse
        };
        assert!(generate_song(&dense).notes.len() > generate_song(&sparse).notes.len());
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
    fn bassline_presets_apply_expected_styles_and_tempos() {
        let cases = [
            (GeneratorPreset::TechnoBass, BasslineStyle::Techno, 128),
            (GeneratorPreset::HouseBass, BasslineStyle::House, 124),
            (GeneratorPreset::Drill808, BasslineStyle::Drill, 140),
            (GeneratorPreset::HipHop808, BasslineStyle::HipHop, 92),
            (GeneratorPreset::UkGarageBass, BasslineStyle::UkGarage, 127),
            (
                GeneratorPreset::DrumAndBass,
                BasslineStyle::DrumAndBass,
                174,
            ),
        ];

        for (preset, bassline_style, tempo) in cases {
            let mut settings = GeneratorSettings::default();
            settings.apply_preset(preset);
            assert_eq!(settings.mode, GeneratorMode::Bassline);
            assert_eq!(settings.bassline_style, bassline_style);
            assert_eq!(settings.tempo, tempo);
        }
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

    #[test]
    fn uk_garage_creates_swung_start_times() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::UkGarage,
            density: 100,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        let step_ticks = PPQN as u32 / 4;
        assert!(song
            .notes
            .iter()
            .any(|note| note.start_ticks % step_ticks != 0));
    }

    #[test]
    fn hiphop_is_sparser_than_drum_and_bass() {
        let hiphop = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::HipHop,
            density: 60,
            seed: 19,
            ..GeneratorSettings::default()
        };
        let dnb = GeneratorSettings {
            bassline_style: BasslineStyle::DrumAndBass,
            ..hiphop
        };
        assert!(generate_song(&dnb).notes.len() > generate_song(&hiphop).notes.len());
    }

    #[test]
    fn chord_style_uses_expected_degree_pattern() {
        assert_eq!(chord_style_degree(ChordStyle::Pop, 0, 7), 0);
        assert_eq!(chord_style_degree(ChordStyle::Pop, 1, 7), 4);
        assert_eq!(chord_style_degree(ChordStyle::Jazz, 0, 7), 1);
        assert_eq!(chord_style_degree(ChordStyle::Jazz, 2, 7), 0);
    }

    #[test]
    fn scale_quality_tables_match_expected_character() {
        assert_eq!(
            quality_for_degree(Scale::Major, 6),
            ChordQuality::Diminished
        );
        assert_eq!(
            quality_for_degree(Scale::Mixolydian, 6),
            ChordQuality::Major
        );
        assert_eq!(quality_for_degree(Scale::Dorian, 1), ChordQuality::Minor);
        assert_eq!(
            quality_for_degree(Scale::HarmonicMinor, 4),
            ChordQuality::Dominant
        );
        assert_eq!(
            quality_for_degree(Scale::MinorPentatonic, 0),
            ChordQuality::Minor
        );
    }

    #[test]
    fn high_cadence_shapes_final_approach() {
        let settings = GeneratorSettings {
            chord_style: ChordStyle::Balanced,
            cadence: 100,
            surprise: 0,
            bars: 4,
            seed: 12,
            ..GeneratorSettings::default()
        };
        let mut rng = StdRng::seed_from_u64(settings.seed);
        let chords = generate_chords(&settings, &mut rng);

        assert_eq!(chords.last().map(|chord| chord.degree), Some(0));
        assert!(matches!(chords[chords.len() - 2].degree, 1 | 3 | 4 | 6));
    }

    #[test]
    fn borrowed_surprise_chords_have_intentional_quality() {
        let mut rng = StdRng::seed_from_u64(3);
        let settings = GeneratorSettings {
            surprise: 100,
            ..GeneratorSettings::default()
        };
        let borrowed = (0..32)
            .find_map(|_| borrowed_chord(0, &settings, &mut rng))
            .expect("high surprise should eventually borrow a chord");

        assert!(matches!(
            borrowed.1,
            ChordQuality::Major | ChordQuality::Minor | ChordQuality::Dominant
        ));
    }

    #[test]
    fn high_surprise_perturbs_fixed_chord_styles() {
        let low = GeneratorSettings {
            chord_style: ChordStyle::Pop,
            surprise: 0,
            cadence: 0,
            bars: 8,
            seed: 44,
            ..GeneratorSettings::default()
        };
        let high = GeneratorSettings {
            surprise: 100,
            ..low
        };

        let low_chords = generate_song(&low).chords;
        let high_chords = generate_song(&high).chords;
        let expected_pattern: Vec<usize> = (0..high_chords.len())
            .map(|index| chord_style_degree(ChordStyle::Pop, index, high.scale.degree_count()))
            .collect();
        let high_pattern: Vec<usize> = high_chords.iter().map(|chord| chord.degree).collect();

        assert_ne!(high_pattern, expected_pattern);
        assert_ne!(high_chords, low_chords);
    }

    #[test]
    fn high_surprise_can_use_borrowed_chromatic_roots() {
        let settings = GeneratorSettings {
            chord_style: ChordStyle::Pop,
            surprise: 100,
            cadence: 0,
            bars: 8,
            seed: 3,
            ..GeneratorSettings::default()
        };
        let song = generate_song(&settings);
        let diatonic_roots: Vec<u8> = settings
            .scale
            .intervals()
            .iter()
            .map(|interval| ((settings.key.semitone() + *interval) as i16).rem_euclid(12) as u8)
            .collect();

        assert!(song
            .chords
            .iter()
            .any(|chord| !diatonic_roots.contains(&chord.root)));
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
}
