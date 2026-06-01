use std::fmt::{Display, Formatter};

use super::common::octave_to_midi_c;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorSettings {
    pub preset: GeneratorPreset,
    pub key: Key,
    pub scale: Scale,
    pub mode: GeneratorMode,
    pub drop_type: DropType,
    pub bars: u16,
    pub tempo: u16,
    pub seed: u64,
    pub seed_behavior: SeedBehavior,
    pub chord_style: ChordStyle,
    pub rhythm_style: RhythmStyle,
    pub hook_type: HookType,
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
            drop_type: DropType::BassDrop,
            bars: 4,
            tempo: 110,
            seed: 42,
            seed_behavior: SeedBehavior::Locked,
            chord_style: ChordStyle::Balanced,
            rhythm_style: RhythmStyle::Straight,
            hook_type: HookType::FourNoteLoop,
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

#[derive(Debug, Clone, Copy)]
struct PresetDefinition {
    preset: GeneratorPreset,
    settings: PresetSettings,
}

#[derive(Debug, Clone, Copy)]
struct PresetSettings {
    mode: Option<GeneratorMode>,
    drop_type: Option<DropType>,
    scale: Option<Scale>,
    bars: Option<u16>,
    tempo: Option<u16>,
    chord_style: Option<ChordStyle>,
    rhythm_style: Option<RhythmStyle>,
    hook_type: Option<HookType>,
    tension: Option<u8>,
    surprise: Option<u8>,
    cadence: Option<u8>,
    chord_inversion_amount: Option<u8>,
    density: Option<u8>,
    note_length: Option<u8>,
    repeat_amount: Option<u8>,
    variation_amount: Option<u8>,
    min_octave: Option<u8>,
    max_octave: Option<u8>,
    arp_note_count: Option<u8>,
    arp_pattern: Option<ArpPattern>,
    arp_rotate_slot: Option<u8>,
    arp_rotation: Option<ArpRotation>,
    bassline_style: Option<BasslineStyle>,
    bassline_accent: Option<u8>,
    bassline_slide: Option<u8>,
    bassline_octave_jump: Option<u8>,
    bassline_mutation: Option<u8>,
    velocity_mode: Option<VelocityMode>,
}

impl PresetSettings {
    const EMPTY: Self = Self {
        mode: None,
        drop_type: None,
        scale: None,
        bars: None,
        tempo: None,
        chord_style: None,
        rhythm_style: None,
        hook_type: None,
        tension: None,
        surprise: None,
        cadence: None,
        chord_inversion_amount: None,
        density: None,
        note_length: None,
        repeat_amount: None,
        variation_amount: None,
        min_octave: None,
        max_octave: None,
        arp_note_count: None,
        arp_pattern: None,
        arp_rotate_slot: None,
        arp_rotation: None,
        bassline_style: None,
        bassline_accent: None,
        bassline_slide: None,
        bassline_octave_jump: None,
        bassline_mutation: None,
        velocity_mode: None,
    };

    fn apply_to(self, settings: &mut GeneratorSettings) {
        if let Some(value) = self.mode {
            settings.mode = value;
        }
        if let Some(value) = self.drop_type {
            settings.drop_type = value;
        }
        if let Some(value) = self.scale {
            settings.scale = value;
        }
        if let Some(value) = self.bars {
            settings.bars = value;
        }
        if let Some(value) = self.tempo {
            settings.tempo = value;
        }
        if let Some(value) = self.chord_style {
            settings.chord_style = value;
        }
        if let Some(value) = self.rhythm_style {
            settings.rhythm_style = value;
        }
        if let Some(value) = self.hook_type {
            settings.hook_type = value;
        }
        if let Some(value) = self.tension {
            settings.tension = value;
        }
        if let Some(value) = self.surprise {
            settings.surprise = value;
        }
        if let Some(value) = self.cadence {
            settings.cadence = value;
        }
        if let Some(value) = self.chord_inversion_amount {
            settings.chord_inversion_amount = value;
        }
        if let Some(value) = self.density {
            settings.density = value;
        }
        if let Some(value) = self.note_length {
            settings.note_length = value;
        }
        if let Some(value) = self.repeat_amount {
            settings.repeat_amount = value;
        }
        if let Some(value) = self.variation_amount {
            settings.variation_amount = value;
        }
        if let Some(value) = self.min_octave {
            settings.min_octave = value;
        }
        if let Some(value) = self.max_octave {
            settings.max_octave = value;
        }
        if let Some(value) = self.arp_note_count {
            settings.arp_note_count = value;
        }
        if let Some(value) = self.arp_pattern {
            settings.arp_pattern = value;
        }
        if let Some(value) = self.arp_rotate_slot {
            settings.arp_rotate_slot = value;
        }
        if let Some(value) = self.arp_rotation {
            settings.arp_rotation = value;
        }
        if let Some(value) = self.bassline_style {
            settings.bassline_style = value;
        }
        if let Some(value) = self.bassline_accent {
            settings.bassline_accent = value;
        }
        if let Some(value) = self.bassline_slide {
            settings.bassline_slide = value;
        }
        if let Some(value) = self.bassline_octave_jump {
            settings.bassline_octave_jump = value;
        }
        if let Some(value) = self.bassline_mutation {
            settings.bassline_mutation = value;
        }
        if let Some(value) = self.velocity_mode {
            settings.velocity_mode = value;
        }
    }
}

const PRESET_DEFINITIONS: &[PresetDefinition] = &[
    PresetDefinition {
        preset: GeneratorPreset::TechnoBass,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::Techno),
            scale: Some(Scale::MinorPentatonic),
            tempo: Some(128),
            min_octave: Some(1),
            max_octave: Some(3),
            chord_style: Some(ChordStyle::AcidMinimal),
            rhythm_style: Some(RhythmStyle::Syncopated),
            density: Some(78),
            note_length: Some(22),
            repeat_amount: Some(72),
            variation_amount: Some(35),
            bassline_accent: Some(82),
            bassline_slide: Some(45),
            bassline_octave_jump: Some(46),
            bassline_mutation: Some(42),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::HouseBass,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::House),
            scale: Some(Scale::Dorian),
            tempo: Some(124),
            min_octave: Some(2),
            max_octave: Some(4),
            chord_style: Some(ChordStyle::Pop),
            rhythm_style: Some(RhythmStyle::Syncopated),
            density: Some(66),
            note_length: Some(48),
            repeat_amount: Some(64),
            variation_amount: Some(24),
            bassline_accent: Some(58),
            bassline_slide: Some(10),
            bassline_octave_jump: Some(22),
            bassline_mutation: Some(35),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::Drill808,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::Drill),
            scale: Some(Scale::NaturalMinor),
            tempo: Some(140),
            min_octave: Some(1),
            max_octave: Some(3),
            chord_style: Some(ChordStyle::MinorCinematic),
            rhythm_style: Some(RhythmStyle::Sparse),
            density: Some(52),
            note_length: Some(88),
            repeat_amount: Some(55),
            variation_amount: Some(30),
            bassline_accent: Some(76),
            bassline_slide: Some(78),
            bassline_octave_jump: Some(45),
            bassline_mutation: Some(48),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::HipHop808,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::HipHop),
            scale: Some(Scale::Blues),
            tempo: Some(92),
            min_octave: Some(1),
            max_octave: Some(3),
            chord_style: Some(ChordStyle::MinorCinematic),
            rhythm_style: Some(RhythmStyle::Sparse),
            density: Some(42),
            note_length: Some(72),
            repeat_amount: Some(60),
            variation_amount: Some(18),
            bassline_accent: Some(72),
            bassline_slide: Some(18),
            bassline_octave_jump: Some(36),
            bassline_mutation: Some(28),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::UkGarageBass,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::UkGarage),
            scale: Some(Scale::NaturalMinor),
            tempo: Some(127),
            min_octave: Some(2),
            max_octave: Some(4),
            chord_style: Some(ChordStyle::Modal),
            rhythm_style: Some(RhythmStyle::Syncopated),
            density: Some(70),
            note_length: Some(34),
            repeat_amount: Some(62),
            variation_amount: Some(38),
            bassline_accent: Some(64),
            bassline_slide: Some(24),
            bassline_octave_jump: Some(40),
            bassline_mutation: Some(55),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::DrumAndBass,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Bassline),
            bassline_style: Some(BasslineStyle::DrumAndBass),
            scale: Some(Scale::NaturalMinor),
            tempo: Some(174),
            min_octave: Some(1),
            max_octave: Some(3),
            chord_style: Some(ChordStyle::MinorCinematic),
            rhythm_style: Some(RhythmStyle::Busy),
            density: Some(74),
            note_length: Some(28),
            repeat_amount: Some(45),
            variation_amount: Some(48),
            bassline_accent: Some(80),
            bassline_slide: Some(20),
            bassline_octave_jump: Some(34),
            bassline_mutation: Some(58),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::BocChordPads,
        settings: PresetSettings {
            mode: Some(GeneratorMode::ChordPads),
            scale: Some(Scale::Dorian),
            tempo: Some(88),
            bars: Some(8),
            min_octave: Some(2),
            max_octave: Some(5),
            chord_style: Some(ChordStyle::BoardsOfCanada),
            rhythm_style: Some(RhythmStyle::Sparse),
            tension: Some(62),
            surprise: Some(42),
            cadence: Some(45),
            density: Some(38),
            note_length: Some(88),
            repeat_amount: Some(72),
            variation_amount: Some(12),
            chord_inversion_amount: Some(35),
            velocity_mode: Some(VelocityMode::Humanized),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::DreamyArp,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Arp),
            scale: Some(Scale::MajorPentatonic),
            tempo: Some(96),
            min_octave: Some(3),
            max_octave: Some(6),
            chord_style: Some(ChordStyle::Modal),
            rhythm_style: Some(RhythmStyle::Straight),
            density: Some(55),
            note_length: Some(68),
            repeat_amount: Some(58),
            variation_amount: Some(18),
            arp_note_count: Some(5),
            arp_pattern: Some(ArpPattern::UpDown),
            arp_rotate_slot: Some(5),
            arp_rotation: Some(ArpRotation::Up),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::ChipLead,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Chiptune),
            scale: Some(Scale::Mixolydian),
            tempo: Some(140),
            min_octave: Some(4),
            max_octave: Some(7),
            chord_style: Some(ChordStyle::ChiptuneLoop),
            rhythm_style: Some(RhythmStyle::Busy),
            density: Some(82),
            note_length: Some(18),
            repeat_amount: Some(65),
            variation_amount: Some(30),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::SparseMotif,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Melodic),
            scale: Some(Scale::Dorian),
            tempo: Some(88),
            min_octave: Some(3),
            max_octave: Some(5),
            chord_style: Some(ChordStyle::Modal),
            rhythm_style: Some(RhythmStyle::Sparse),
            density: Some(35),
            note_length: Some(72),
            repeat_amount: Some(68),
            variation_amount: Some(15),
            ..PresetSettings::EMPTY
        },
    },
    PresetDefinition {
        preset: GeneratorPreset::BusySequence,
        settings: PresetSettings {
            mode: Some(GeneratorMode::Euclidean),
            scale: Some(Scale::NaturalMinor),
            tempo: Some(124),
            min_octave: Some(3),
            max_octave: Some(6),
            chord_style: Some(ChordStyle::MinorCinematic),
            rhythm_style: Some(RhythmStyle::Busy),
            density: Some(88),
            note_length: Some(36),
            repeat_amount: Some(42),
            variation_amount: Some(45),
            ..PresetSettings::EMPTY
        },
    },
];

impl GeneratorSettings {
    pub fn apply_preset(&mut self, preset: GeneratorPreset) {
        self.preset = preset;

        if preset == GeneratorPreset::Custom {
            return;
        }

        if let Some(definition) = PRESET_DEFINITIONS
            .iter()
            .find(|definition| definition.preset == preset)
        {
            definition.settings.apply_to(self);
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
    PopDescent,
    Modal,
    Jazz,
    MinorCinematic,
    AcidMinimal,
    ChiptuneLoop,
    BoardsOfCanada,
}

impl ChordStyle {
    pub const ALL: [Self; 9] = [
        Self::Balanced,
        Self::Pop,
        Self::PopDescent,
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
            Self::PopDescent => "Pop descent",
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
    Hook,
    CounterMelody,
    BuildupDrop,
    Euclidean,
    Arp,
    Chiptune,
    Bassline,
    ChordPads,
}

impl GeneratorMode {
    pub const ALL: [Self; 9] = [
        Self::Melodic,
        Self::Hook,
        Self::CounterMelody,
        Self::BuildupDrop,
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
            Self::Hook => "Hook",
            Self::CounterMelody => "Counter",
            Self::BuildupDrop => "Drop",
            Self::Euclidean => "Euclidean",
            Self::Arp => "Arp",
            Self::Chiptune => "Chiptune",
            Self::Bassline => "Bassline",
            Self::ChordPads => "Chord pads",
        })
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropType {
    BassDrop,
    SupersawDrop,
    HalfTimeDrop,
    FillDrop,
    VocalDrop,
}

impl DropType {
    pub const ALL: [Self; 5] = [
        Self::BassDrop,
        Self::SupersawDrop,
        Self::HalfTimeDrop,
        Self::FillDrop,
        Self::VocalDrop,
    ];
}

impl Display for DropType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::BassDrop => "Bass drop",
            Self::SupersawDrop => "Supersaw drop",
            Self::HalfTimeDrop => "Half-time drop",
            Self::FillDrop => "Fill drop",
            Self::VocalDrop => "Vocal drop",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    FourNoteLoop,
    CallResponse,
    MotifDevelop,
    StutterHook,
    DescendingBass,
}

impl HookType {
    pub const ALL: [Self; 5] = [
        Self::FourNoteLoop,
        Self::CallResponse,
        Self::MotifDevelop,
        Self::StutterHook,
        Self::DescendingBass,
    ];
}

impl Display for HookType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::FourNoteLoop => "Four-note loop",
            Self::CallResponse => "Call & response",
            Self::MotifDevelop => "Motif develop",
            Self::StutterHook => "Stutter hook",
            Self::DescendingBass => "Descending bass",
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
