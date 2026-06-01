use iced::Command;
use rand::Rng;
use std::path::PathBuf;

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
}

async fn pick_export_directory() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Choose export folder")
        .pick_folder()
}
