use std::fmt::{Display, Formatter};

use super::common::octave_to_midi_c;

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

    pub(crate) fn semitone(self) -> i8 {
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

    pub(crate) fn intervals(self) -> &'static [i8] {
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

    pub(crate) fn degree_count(self) -> usize {
        self.intervals().len()
    }

    pub(crate) fn is_minorish(self) -> bool {
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
