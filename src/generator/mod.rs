mod arp;
mod bassline;
mod buildup_drop;
mod chiptune;
mod chord_pads;
mod chords;
mod common;
mod counter_melody;
mod euclidean;
mod hook;
mod melody;
mod pipeline;
mod settings;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::music::{pitch_class_name, roman_degree};

pub use crate::constants::PPQN;
pub use settings::*;

#[allow(dead_code)]
pub fn ticks_per_bar() -> u32 {
    common::ticks_per_bar()
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
    pub slash_bass: Option<u8>,
    pub degree: usize,
    pub start_ticks: u32,
    pub duration_ticks: u32,
    pub tension: u8,
}

impl ChordEvent {
    pub fn label(&self) -> String {
        let suffix = match self.quality {
            ChordQuality::Major => "",
            ChordQuality::Minor => "m",
            ChordQuality::Dominant => "7",
            ChordQuality::Diminished => "dim",
            ChordQuality::Suspended => "sus",
            ChordQuality::MinorDyad => "m(no5)",
            ChordQuality::Minor7 => "m7",
            ChordQuality::Sus2 => "sus2",
            ChordQuality::Add9 => "add9",
            ChordQuality::Maj7 => "maj7",
            ChordQuality::Maj9 => "maj9",
            ChordQuality::Min9 => "m9",
            ChordQuality::Sus4 => "sus4",
            ChordQuality::Add11 => "add11",
            ChordQuality::Add13 => "add13",
        };
        let slash = self
            .slash_bass
            .map(|bass| format!("/{}", pitch_class_name(bass)))
            .unwrap_or_default();

        format!(
            "{}{}{} {}",
            pitch_class_name(self.root),
            suffix,
            slash,
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
            ChordQuality::Maj7 => &[0, 4, 7, 11],
            ChordQuality::Maj9 => &[0, 4, 7, 11, 14],
            ChordQuality::Min9 => &[0, 3, 7, 10, 14],
            ChordQuality::Sus4 => &[0, 5, 7],
            ChordQuality::Add11 => &[0, 4, 7, 17],
            ChordQuality::Add13 => &[0, 4, 7, 21],
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
    Maj7,
    Maj9,
    Min9,
    Sus4,
    Add11,
    Add13,
}

pub fn generate_song(settings: &GeneratorSettings) -> GeneratedSong {
    generate_song_with_chords(settings, None)
}

pub fn generate_song_with_chords(
    settings: &GeneratorSettings,
    locked_chords: Option<&[ChordEvent]>,
) -> GeneratedSong {
    let rng = StdRng::seed_from_u64(settings.seed);
    pipeline::SongPipeline::new(settings, rng)
        .with_chords(locked_chords)
        .generate_mode()
        .apply_phrase_memory()
        .apply_velocity_range()
        .build()
}

#[cfg(test)]
mod tests;
