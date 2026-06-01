mod arp;
mod bassline;
mod chiptune;
mod chord_pads;
mod chords;
mod common;
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
