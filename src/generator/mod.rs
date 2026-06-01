mod arp;
mod bassline;
mod chiptune;
mod chord_pads;
mod chords;
mod common;
mod euclidean;
mod melody;
mod settings;

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::music::{pitch_class_name, roman_degree};

pub use crate::constants::PPQN;
pub use common::ticks_per_bar;
pub use settings::*;

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
        .unwrap_or_else(|| chords::generate_chords(settings, &mut rng));
    let notes = match settings.mode {
        GeneratorMode::Melodic => melody::generate_melodic(settings, &chords, &mut rng),
        GeneratorMode::Euclidean => euclidean::generate_euclidean(settings, &chords, &mut rng),
        GeneratorMode::Arp => arp::generate_arp(settings, &chords, &mut rng),
        GeneratorMode::Chiptune => chiptune::generate_chiptune(settings, &chords, &mut rng),
        GeneratorMode::Bassline => bassline::generate_bassline(settings, &chords, &mut rng),
        GeneratorMode::ChordPads => chord_pads::generate_chord_pads(settings, &chords, &mut rng),
    };
    let notes = common::apply_velocity_range(
        settings,
        common::apply_phrase_memory(settings, notes, &mut rng),
    );

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

#[cfg(test)]
mod tests;
