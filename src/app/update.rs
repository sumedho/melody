use iced::Command;
use rand::Rng;
use std::path::PathBuf;

use crate::drag_export::{self, DragExportResult};
use crate::generator::*;
use crate::midi::export_midi;

use super::{filename_from_path, MelodyApp, Message};

impl MelodyApp {
    pub(super) fn handle_update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleSection(section) => self.ui.sidebar.toggle(section),
            Message::PresetChanged(value) => {
                self.music.settings.apply_preset(value);
                self.ui.seed_input = self.music.settings.seed.to_string();
                self.music.output = self.generate_current_song();
                self.export.filename = self.next_export_filename();
                self.export.path_auto = true;
                self.ui.status = if self.music.locked_chords.is_some() {
                    format!("Applied {} preset and reused locked chords.", value)
                } else {
                    format!("Applied {} preset.", value)
                };
            }
            Message::KeyChanged(value) => self.update_setting(|s| s.key = value),
            Message::ScaleChanged(value) => self.update_setting(|s| s.scale = value),
            Message::ModeChanged(value) => self.update_setting(|s| s.mode = value),
            Message::BarsChanged(value) => {
                self.update_setting(|s| {
                    s.bars = value;
                    s.set_phrase_length(s.phrase_length);
                });
            }
            Message::TempoChanged(value) => self.update_setting(|s| s.tempo = value),
            Message::SeedChanged(value) => {
                self.ui.seed_input = value;
                if let Ok(seed) = self.ui.seed_input.parse::<u64>() {
                    self.music.settings.seed = seed;
                    self.ui.status = "Seed updated.".to_string();
                } else {
                    self.ui.status = "Seed must be a positive integer.".to_string();
                }
            }
            Message::RandomizeSeed => {
                let seed = rand::thread_rng().gen::<u64>();
                self.music.settings.seed = seed;
                self.ui.seed_input = seed.to_string();
                self.music.output = self.generate_current_song();
                self.export.filename = self.next_export_filename();
                self.export.path_auto = true;
                self.ui.status = if self.music.locked_chords.is_some() {
                    "Randomized seed and generated with locked chords.".to_string()
                } else {
                    "Randomized seed and generated a new melody.".to_string()
                };
            }
            Message::Generate => {
                if self.music.settings.seed_behavior == SeedBehavior::RandomizeOnGenerate {
                    let seed = rand::thread_rng().gen::<u64>();
                    self.music.settings.seed = seed;
                    self.ui.seed_input = seed.to_string();
                }
                self.music.output = self.generate_current_song();
                self.export.filename = self.next_export_filename();
                self.export.path_auto = true;
                self.ui.status = if self.music.locked_chords.is_some() {
                    format!(
                        "Generated {} notes across {} locked chord changes.",
                        self.music.output.notes.len(),
                        self.music.output.chords.len()
                    )
                } else {
                    format!(
                        "Generated {} notes across {} chord changes.",
                        self.music.output.notes.len(),
                        self.music.output.chords.len()
                    )
                };
            }
            Message::Export => {
                match export_midi(
                    &self.music.output,
                    self.music.settings.tempo,
                    &self.current_export_path(),
                    self.export.path_auto,
                ) {
                    Ok(result) => {
                        self.export.filename = filename_from_path(&result.path);
                        self.export.path_auto = false;
                        let path = result.path.display().to_string();
                        self.ui.status = if result.created_parent {
                            format!("Created export folder and exported MIDI to {}.", path)
                        } else {
                            format!("Exported MIDI to {}.", path)
                        };
                    }
                    Err(error) => self.ui.status = format!("Export failed: {error}"),
                }
            }
            Message::DragMidi => {
                return self.drag_midi();
            }
            Message::DragMidiFinished(result) => {
                self.handle_drag_midi_result(result);
            }
            Message::BrowseExportDirectory => {
                return Command::perform(pick_export_directory(), Message::ExportDirectorySelected);
            }
            Message::ExportDirectorySelected(directory) => {
                if let Some(directory) = directory {
                    self.export.directory = Some(directory.clone());
                    self.ui.status = format!("Export directory set to {}.", directory.display());
                } else {
                    self.ui.status = "Export directory selection cancelled.".to_string();
                }
            }
            Message::ExportFilenameChanged(value) => {
                self.export.filename = value;
                self.export.path_auto = false;
            }
            Message::TensionChanged(value) => self.update_setting(|s| s.tension = value),
            Message::SurpriseChanged(value) => self.update_setting(|s| s.surprise = value),
            Message::CadenceChanged(value) => self.update_setting(|s| s.cadence = value),
            Message::ChordInversionChanged(value) => {
                self.update_setting(|s| s.chord_inversion_amount = value)
            }
            Message::ChordStyleChanged(value) => self.update_setting(|s| s.chord_style = value),
            Message::ChordLockChanged(locked) => {
                if locked {
                    self.music.locked_chords = Some(self.music.output.chords.clone());
                    self.ui.status = format!(
                        "Locked {} current chord changes.",
                        self.music.output.chords.len()
                    );
                } else {
                    self.music.locked_chords = None;
                    self.ui.status = "Unlocked chord changes.".to_string();
                }
            }
            Message::RhythmStyleChanged(value) => self.update_setting(|s| s.rhythm_style = value),
            Message::DropTypeChanged(value) => self.update_setting(|s| s.drop_type = value),
            Message::HookTypeChanged(value) => self.update_setting(|s| s.hook_type = value),
            Message::DensityChanged(value) => self.update_setting(|s| s.density = value),
            Message::NoteLengthChanged(value) => self.update_setting(|s| s.note_length = value),
            Message::PhraseLengthChanged(value) => {
                self.update_setting(|s| s.set_phrase_length(value))
            }
            Message::RepeatAmountChanged(value) => self.update_setting(|s| s.repeat_amount = value),
            Message::VariationAmountChanged(value) => {
                self.update_setting(|s| s.variation_amount = value)
            }
            Message::SeedBehaviorChanged(value) => self.music.settings.seed_behavior = value,
            Message::MinOctaveChanged(value) => self.update_setting(|s| s.set_min_octave(value)),
            Message::MaxOctaveChanged(value) => self.update_setting(|s| s.set_max_octave(value)),
            Message::ArpNoteCountChanged(value) => {
                self.update_setting(|s| s.set_arp_note_count(value))
            }
            Message::ArpPatternChanged(value) => self.update_setting(|s| s.arp_pattern = value),
            Message::ArpRotateSlotChanged(value) => {
                self.update_setting(|s| s.set_arp_rotate_slot(value))
            }
            Message::ArpRotationChanged(value) => self.update_setting(|s| s.arp_rotation = value),
            Message::BasslineStyleChanged(value) => {
                self.update_setting(|s| s.bassline_style = value)
            }
            Message::BasslineAccentChanged(value) => {
                self.update_setting(|s| s.bassline_accent = value)
            }
            Message::BasslineSlideChanged(value) => {
                self.update_setting(|s| s.bassline_slide = value)
            }
            Message::BasslineOctaveJumpChanged(value) => {
                self.update_setting(|s| s.bassline_octave_jump = value)
            }
            Message::BasslineMutationChanged(value) => {
                self.update_setting(|s| s.bassline_mutation = value)
            }
            Message::VelocityModeChanged(value) => self.update_setting(|s| s.velocity_mode = value),
            Message::RandomVelocityMinChanged(value) => {
                self.update_setting(|s| s.set_random_velocity_min(value))
            }
            Message::RandomVelocityMaxChanged(value) => {
                self.update_setting(|s| s.set_random_velocity_max(value))
            }
        }

        Command::none()
    }

    fn drag_midi(&mut self) -> Command<Message> {
        let path = match drag_export::write_drag_midi(&self.music.output, self.music.settings.tempo)
        {
            Ok(path) => path,
            Err(error) => {
                self.ui.status = format!("MIDI drag export failed: {error}");
                return Command::none();
            }
        };

        self.last_drag_midi_path = Some(path.clone());

        let Some(window_id) = self.window_id else {
            self.ui.status = format!(
                "Prepared MIDI for drag at {}, but the app window is not ready.",
                path.display()
            );
            return Command::none();
        };

        self.ui.status = format!("Prepared MIDI drag file: {}.", path.display());

        iced::window::run_with_handle(window_id, move |handle| {
            match drag_export::begin_native_file_drag(handle, &path) {
                Ok(()) => DragExportResult::Started(path),
                Err(_) if !cfg!(target_os = "macos") => DragExportResult::Unavailable(path),
                Err(error) => DragExportResult::Failed(error),
            }
        })
        .map(Message::DragMidiFinished)
    }

    fn handle_drag_midi_result(&mut self, result: DragExportResult) {
        match result {
            DragExportResult::Started(path) => {
                self.ui.status = format!("Started MIDI drag from {}.", path.display());
            }
            DragExportResult::Unavailable(path) => {
                self.ui.status = format!(
                    "Prepared MIDI at {}. Native drag is unavailable on this platform.",
                    path.display()
                );
            }
            DragExportResult::Failed(error) => {
                self.ui.status = format!("MIDI drag failed: {error}");
            }
        }
    }
}

async fn pick_export_directory() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Choose export folder")
        .pick_folder()
}
