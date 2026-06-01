use rand::rngs::StdRng;

use super::common::{apply_phrase_memory, apply_velocity_range, ticks_per_bar};
use super::{
    arp, bassline, chiptune, chord_pads, chords, euclidean, hook, melody, ChordEvent,
    GeneratedSong, GeneratorMode, GeneratorSettings, NoteEvent,
};

pub(crate) struct SongPipeline<'a> {
    settings: &'a GeneratorSettings,
    rng: StdRng,
    chords: Option<Vec<ChordEvent>>,
    notes: Option<Vec<NoteEvent>>,
}

impl<'a> SongPipeline<'a> {
    pub(crate) fn new(settings: &'a GeneratorSettings, rng: StdRng) -> Self {
        Self {
            settings,
            rng,
            chords: None,
            notes: None,
        }
    }

    pub(crate) fn with_chords(mut self, locked_chords: Option<&[ChordEvent]>) -> Self {
        self.chords = locked_chords
            .and_then(|chords| locked_chords_for_song(self.settings, chords))
            .or_else(|| Some(chords::generate_chords(self.settings, &mut self.rng)));
        self
    }

    pub(crate) fn generate_mode(mut self) -> Self {
        let chords = self.chords.as_deref().expect("pipeline chords generated");
        self.notes = Some(match self.settings.mode {
            GeneratorMode::Melodic => {
                melody::generate_melodic(self.settings, chords, &mut self.rng)
            }
            GeneratorMode::Hook => hook::generate_hook(self.settings, chords, &mut self.rng),
            GeneratorMode::Euclidean => {
                euclidean::generate_euclidean(self.settings, chords, &mut self.rng)
            }
            GeneratorMode::Arp => arp::generate_arp(self.settings, chords, &mut self.rng),
            GeneratorMode::Chiptune => {
                chiptune::generate_chiptune(self.settings, chords, &mut self.rng)
            }
            GeneratorMode::Bassline => {
                bassline::generate_bassline(self.settings, chords, &mut self.rng)
            }
            GeneratorMode::ChordPads => {
                chord_pads::generate_chord_pads(self.settings, chords, &mut self.rng)
            }
        });
        self
    }

    pub(crate) fn apply_phrase_memory(mut self) -> Self {
        let notes = self.notes.take().expect("pipeline notes generated");
        self.notes = Some(apply_phrase_memory(self.settings, notes, &mut self.rng));
        self
    }

    pub(crate) fn apply_velocity_range(mut self) -> Self {
        let notes = self.notes.take().expect("pipeline notes generated");
        self.notes = Some(apply_velocity_range(self.settings, notes));
        self
    }

    pub(crate) fn build(self) -> GeneratedSong {
        GeneratedSong {
            notes: self.notes.expect("pipeline notes generated"),
            chords: self.chords.expect("pipeline chords generated"),
        }
    }
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
