pub const PPQN: u16 = 480;
pub const BEATS_PER_BAR: u32 = 4;
pub const STEPS_PER_BEAT: u32 = 4;

pub const MIN_TEMPO: u16 = 60;
pub const MAX_TEMPO: u16 = 180;
pub const MIN_OCTAVE: u8 = 1;
pub const MAX_OCTAVE: u8 = 8;

pub const DEFAULT_EXPORT_DIR: &str = "exports";
pub const DEFAULT_EXPORT_FILENAME: &str = "melody.mid";

pub const MIN_NOTE_GATE_RATIO: f32 = 0.35;
pub const NOTE_GATE_RANGE_RATIO: f32 = 1.45;
pub const NOTE_DURATION_VARIATION_RATIO: f32 = 0.35;

pub const CHIPTUNE_OCTAVE_JUMP_CHANCE: f64 = 0.22;
pub const CHIPTUNE_OCTAVE_INTERVAL: usize = 12;
pub const MELODIC_STRONG_NOTE_CHANCE: f64 = 0.42;
pub const DEGREE_STABILITY_PROBABILITY: f64 = 0.65;
pub const UKG_SWING_FACTOR: f32 = 0.42;
pub const VELOCITY_SHAPING_POWER: f32 = 0.75;
