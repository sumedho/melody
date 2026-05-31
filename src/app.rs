use iced::alignment;
use iced::executor;
use iced::theme;
use iced::widget::container::Appearance as ContainerAppearance;
use iced::widget::{
    button, column, container, pick_list, row, scrollable, slider, text, text_input, toggler,
    Column,
};
use iced::{
    Application, Background, Border, Color, Command, Element, Length, Settings, Size, Theme,
};
use rand::Rng;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use crate::generator::*;
use crate::midi::{export_midi, unique_midi_filename};
use crate::music::note_name;
use crate::ui::{grid_line_for_step, GridLine, PreviewNoteIndex, PreviewStep};

const DEFAULT_EXPORT_FILENAME: &str = "melody.mid";
const DEFAULT_EXPORT_DIRECTORY: &str = "exports";

pub fn run() -> iced::Result {
    MelodyApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1180.0, 760.0),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSection(SidebarSection),
    PresetChanged(GeneratorPreset),
    KeyChanged(Key),
    ScaleChanged(Scale),
    ModeChanged(GeneratorMode),
    BarsChanged(u16),
    TempoChanged(u16),
    SeedChanged(String),
    RandomizeSeed,
    Generate,
    Export,
    BrowseExportDirectory,
    ExportDirectorySelected(Option<PathBuf>),
    ExportFilenameChanged(String),
    TensionChanged(u8),
    SurpriseChanged(u8),
    CadenceChanged(u8),
    ChordInversionChanged(u8),
    ChordStyleChanged(ChordStyle),
    ChordLockChanged(bool),
    RhythmStyleChanged(RhythmStyle),
    DensityChanged(u8),
    NoteLengthChanged(u8),
    PhraseLengthChanged(u8),
    RepeatAmountChanged(u8),
    VariationAmountChanged(u8),
    SeedBehaviorChanged(SeedBehavior),
    MinOctaveChanged(u8),
    MaxOctaveChanged(u8),
    ArpNoteCountChanged(u8),
    ArpPatternChanged(ArpPattern),
    ArpRotateSlotChanged(u8),
    ArpRotationChanged(ArpRotation),
    BasslineStyleChanged(BasslineStyle),
    BasslineAccentChanged(u8),
    BasslineSlideChanged(u8),
    BasslineOctaveJumpChanged(u8),
    BasslineMutationChanged(u8),
    VelocityModeChanged(VelocityMode),
    RandomVelocityMinChanged(u8),
    RandomVelocityMaxChanged(u8),
}

struct MelodyApp {
    settings: GeneratorSettings,
    sidebar: SidebarState,
    seed_input: String,
    export_filename: String,
    export_directory: Option<PathBuf>,
    export_path_auto: bool,
    output: GeneratedSong,
    locked_chords: Option<Vec<ChordEvent>>,
    status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidebarSection {
    Mode,
    Music,
    Harmony,
    Rhythm,
    Phrase,
    Velocity,
    Seed,
}

struct SidebarState {
    mode: bool,
    music: bool,
    harmony: bool,
    rhythm: bool,
    phrase: bool,
    velocity: bool,
    seed: bool,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            mode: true,
            music: true,
            harmony: false,
            rhythm: true,
            phrase: false,
            velocity: true,
            seed: false,
        }
    }
}

impl SidebarState {
    fn is_open(&self, section: SidebarSection) -> bool {
        match section {
            SidebarSection::Mode => self.mode,
            SidebarSection::Music => self.music,
            SidebarSection::Harmony => self.harmony,
            SidebarSection::Rhythm => self.rhythm,
            SidebarSection::Phrase => self.phrase,
            SidebarSection::Velocity => self.velocity,
            SidebarSection::Seed => self.seed,
        }
    }

    fn toggle(&mut self, section: SidebarSection) {
        let value = match section {
            SidebarSection::Mode => &mut self.mode,
            SidebarSection::Music => &mut self.music,
            SidebarSection::Harmony => &mut self.harmony,
            SidebarSection::Rhythm => &mut self.rhythm,
            SidebarSection::Phrase => &mut self.phrase,
            SidebarSection::Velocity => &mut self.velocity,
            SidebarSection::Seed => &mut self.seed,
        };
        *value = !*value;
    }
}

impl Application for MelodyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = GeneratorSettings::default();
        let output = generate_song(&settings);

        (
            Self {
                seed_input: settings.seed.to_string(),
                export_filename: generated_export_filename(&settings),
                export_directory: None,
                export_path_auto: true,
                settings,
                sidebar: SidebarState::default(),
                output,
                locked_chords: None,
                status: "Generated a starting melody.".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Melody - MIDI generator".to_string()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleSection(section) => self.sidebar.toggle(section),
            Message::PresetChanged(value) => {
                self.settings.apply_preset(value);
                self.seed_input = self.settings.seed.to_string();
                self.output = self.generate_current_song();
                self.export_filename = self.next_export_filename();
                self.export_path_auto = true;
                self.status = if self.locked_chords.is_some() {
                    format!("Applied {} preset and reused locked chords.", value)
                } else {
                    format!("Applied {} preset.", value)
                };
            }
            Message::KeyChanged(value) => {
                self.update_custom_settings(|settings| settings.key = value)
            }
            Message::ScaleChanged(value) => {
                self.update_custom_settings(|settings| settings.scale = value)
            }
            Message::ModeChanged(value) => {
                self.update_custom_settings(|settings| settings.mode = value)
            }
            Message::BarsChanged(value) => {
                self.update_custom_settings(|settings| {
                    settings.bars = value;
                    settings.set_phrase_length(settings.phrase_length);
                });
            }
            Message::TempoChanged(value) => {
                self.update_custom_settings(|settings| settings.tempo = value)
            }
            Message::SeedChanged(value) => {
                self.seed_input = value;
                if let Ok(seed) = self.seed_input.parse::<u64>() {
                    self.settings.seed = seed;
                    self.status = "Seed updated.".to_string();
                } else {
                    self.status = "Seed must be a positive integer.".to_string();
                }
            }
            Message::RandomizeSeed => {
                let seed = rand::thread_rng().gen::<u64>();
                self.settings.seed = seed;
                self.seed_input = seed.to_string();
                self.output = self.generate_current_song();
                self.export_filename = self.next_export_filename();
                self.export_path_auto = true;
                self.status = if self.locked_chords.is_some() {
                    "Randomized seed and generated with locked chords.".to_string()
                } else {
                    "Randomized seed and generated a new melody.".to_string()
                };
            }
            Message::Generate => {
                if self.settings.seed_behavior == SeedBehavior::RandomizeOnGenerate {
                    let seed = rand::thread_rng().gen::<u64>();
                    self.settings.seed = seed;
                    self.seed_input = seed.to_string();
                }
                self.output = self.generate_current_song();
                self.export_filename = self.next_export_filename();
                self.export_path_auto = true;
                self.status = if self.locked_chords.is_some() {
                    format!(
                        "Generated {} notes across {} locked chord changes.",
                        self.output.notes.len(),
                        self.output.chords.len()
                    )
                } else {
                    format!(
                        "Generated {} notes across {} chord changes.",
                        self.output.notes.len(),
                        self.output.chords.len()
                    )
                };
            }
            Message::Export => {
                match export_midi(
                    &self.output,
                    self.settings.tempo,
                    &self.current_export_path(),
                    self.export_path_auto,
                ) {
                    Ok(result) => {
                        self.export_filename = filename_from_path(&result.path);
                        self.export_path_auto = false;
                        let path = result.path.display().to_string();
                        self.status = if result.created_parent {
                            format!("Created export folder and exported MIDI to {}.", path)
                        } else {
                            format!("Exported MIDI to {}.", path)
                        };
                    }
                    Err(error) => self.status = format!("Export failed: {error}"),
                }
            }
            Message::BrowseExportDirectory => {
                return Command::perform(pick_export_directory(), Message::ExportDirectorySelected);
            }
            Message::ExportDirectorySelected(directory) => {
                if let Some(directory) = directory {
                    self.export_directory = Some(directory.clone());
                    self.status = format!("Export directory set to {}.", directory.display());
                } else {
                    self.status = "Export directory selection cancelled.".to_string();
                }
            }
            Message::ExportFilenameChanged(value) => {
                self.export_filename = value;
                self.export_path_auto = false;
            }
            Message::TensionChanged(value) => {
                self.update_custom_settings(|settings| settings.tension = value)
            }
            Message::SurpriseChanged(value) => {
                self.update_custom_settings(|settings| settings.surprise = value)
            }
            Message::CadenceChanged(value) => {
                self.update_custom_settings(|settings| settings.cadence = value)
            }
            Message::ChordInversionChanged(value) => {
                self.update_custom_settings(|settings| settings.chord_inversion_amount = value)
            }
            Message::ChordStyleChanged(value) => {
                self.update_custom_settings(|settings| settings.chord_style = value)
            }
            Message::ChordLockChanged(locked) => {
                if locked {
                    self.locked_chords = Some(self.output.chords.clone());
                    self.status =
                        format!("Locked {} current chord changes.", self.output.chords.len());
                } else {
                    self.locked_chords = None;
                    self.status = "Unlocked chord changes.".to_string();
                }
            }
            Message::RhythmStyleChanged(value) => {
                self.update_custom_settings(|settings| settings.rhythm_style = value)
            }
            Message::DensityChanged(value) => {
                self.update_custom_settings(|settings| settings.density = value)
            }
            Message::NoteLengthChanged(value) => {
                self.update_custom_settings(|settings| settings.note_length = value)
            }
            Message::PhraseLengthChanged(value) => {
                self.update_custom_settings(|settings| settings.set_phrase_length(value))
            }
            Message::RepeatAmountChanged(value) => {
                self.update_custom_settings(|settings| settings.repeat_amount = value)
            }
            Message::VariationAmountChanged(value) => {
                self.update_custom_settings(|settings| settings.variation_amount = value)
            }
            Message::SeedBehaviorChanged(value) => self.settings.seed_behavior = value,
            Message::MinOctaveChanged(value) => {
                self.update_custom_settings(|settings| settings.set_min_octave(value))
            }
            Message::MaxOctaveChanged(value) => {
                self.update_custom_settings(|settings| settings.set_max_octave(value))
            }
            Message::ArpNoteCountChanged(value) => {
                self.update_custom_settings(|settings| settings.set_arp_note_count(value))
            }
            Message::ArpPatternChanged(value) => {
                self.update_custom_settings(|settings| settings.arp_pattern = value)
            }
            Message::ArpRotateSlotChanged(value) => {
                self.update_custom_settings(|settings| settings.set_arp_rotate_slot(value))
            }
            Message::ArpRotationChanged(value) => {
                self.update_custom_settings(|settings| settings.arp_rotation = value)
            }
            Message::BasslineStyleChanged(value) => {
                self.update_custom_settings(|settings| settings.bassline_style = value)
            }
            Message::BasslineAccentChanged(value) => {
                self.update_custom_settings(|settings| settings.bassline_accent = value)
            }
            Message::BasslineSlideChanged(value) => {
                self.update_custom_settings(|settings| settings.bassline_slide = value)
            }
            Message::BasslineOctaveJumpChanged(value) => {
                self.update_custom_settings(|settings| settings.bassline_octave_jump = value)
            }
            Message::BasslineMutationChanged(value) => {
                self.update_custom_settings(|settings| settings.bassline_mutation = value)
            }
            Message::VelocityModeChanged(value) => {
                self.update_custom_settings(|settings| settings.velocity_mode = value)
            }
            Message::RandomVelocityMinChanged(value) => {
                self.update_custom_settings(|settings| settings.set_random_velocity_min(value))
            }
            Message::RandomVelocityMaxChanged(value) => {
                self.update_custom_settings(|settings| settings.set_random_velocity_max(value))
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = self.controls();
        let preview = self.preview();

        container(
            column![
                self.top_bar(),
                row![controls, preview].spacing(14).height(Length::Fill),
                text(&self.status).size(14)
            ]
            .padding(14)
            .spacing(12),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

impl MelodyApp {
    fn generate_current_song(&self) -> GeneratedSong {
        generate_song_with_chords(&self.settings, self.locked_chords.as_deref())
    }

    fn next_export_filename(&self) -> String {
        generated_export_filename(&self.settings)
    }

    fn current_export_directory(&self) -> &Path {
        self.export_directory
            .as_deref()
            .unwrap_or_else(|| Path::new(DEFAULT_EXPORT_DIRECTORY))
    }

    fn current_export_path(&self) -> String {
        self.current_export_directory()
            .join(&self.export_filename)
            .display()
            .to_string()
    }

    fn export_directory_label(&self) -> String {
        self.current_export_directory().display().to_string()
    }

    fn update_custom_settings(&mut self, update: impl FnOnce(&mut GeneratorSettings)) {
        update(&mut self.settings);
        self.settings.preset = GeneratorPreset::Custom;
    }

    fn top_bar(&self) -> Element<'_, Message> {
        container(
            column![
                row![
                    column![
                        text("Melody").size(22),
                        text(format!("{} generator", self.settings.mode)).size(12)
                    ]
                    .spacing(1)
                    .width(Length::Fill),
                    toolbar_button("Generate", Message::Generate, true),
                    toolbar_button("Randomize", Message::RandomizeSeed, false),
                    toolbar_button("Browse", Message::BrowseExportDirectory, false),
                    toolbar_button("Export", Message::Export, false),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),
                row![
                    text("Directory").size(12).width(Length::Fixed(62.0)),
                    container(text(self.export_directory_label()).size(13))
                        .padding([7, 9])
                        .width(Length::FillPortion(2))
                        .style(field_style()),
                    text("Filename").size(12).width(Length::Fixed(58.0)),
                    text_input(DEFAULT_EXPORT_FILENAME, &self.export_filename)
                        .on_input(Message::ExportFilenameChanged)
                        .padding(7)
                        .width(Length::FillPortion(2)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center),
            ]
            .spacing(9),
        )
        .padding(10)
        .style(panel_style())
        .into()
    }

    fn controls(&self) -> Element<'_, Message> {
        let mode_help = match self.settings.mode {
            GeneratorMode::Melodic => "Pattern grammar with chord-aware melodic contour.",
            GeneratorMode::Euclidean => "Evenly distributed pulses with rotation and accents.",
            GeneratorMode::Arp => "Chord tones unfolded as musical arpeggios.",
            GeneratorMode::Chiptune => "Gated leads, octave jumps, motifs, and pulse-bass flavor.",
            GeneratorMode::Bassline => {
                "Genre-shaped monophonic basslines for dance and beat production."
            }
            GeneratorMode::ChordPads => {
                "Sustained playable chords with gentle timing and velocity drift."
            }
        };

        let mode_controls = column![
            labeled_pick(
                "Preset",
                GeneratorPreset::ALL.to_vec(),
                self.settings.preset,
                Message::PresetChanged
            ),
            text("Generator").size(13),
            segmented_control(
                &GeneratorMode::ALL,
                self.settings.mode,
                Message::ModeChanged,
                3
            ),
            text(mode_help).size(13),
            self.arp_controls(),
            self.bassline_controls(),
        ]
        .spacing(10);

        let music_controls = column![
            labeled_pick(
                "Key",
                Key::ALL.to_vec(),
                self.settings.key,
                Message::KeyChanged
            ),
            labeled_pick(
                "Scale",
                Scale::ALL.to_vec(),
                self.settings.scale,
                Message::ScaleChanged
            ),
            labeled_slider_u16("Bars", self.settings.bars, 1..=16, Message::BarsChanged),
            labeled_slider_u16(
                "Tempo",
                self.settings.tempo,
                60..=180,
                Message::TempoChanged
            ),
            labeled_slider_u8(
                "Min octave",
                self.settings.min_octave,
                1..=8,
                Message::MinOctaveChanged
            ),
            labeled_slider_u8(
                "Max octave",
                self.settings.max_octave,
                1..=8,
                Message::MaxOctaveChanged
            ),
        ]
        .spacing(10);

        let harmony_controls = column![
            toggler(
                Some("Lock chords".to_string()),
                self.locked_chords.is_some(),
                Message::ChordLockChanged
            ),
            labeled_pick(
                "Chord style",
                ChordStyle::ALL.to_vec(),
                self.settings.chord_style,
                Message::ChordStyleChanged
            ),
            labeled_slider_u8(
                "Tension",
                self.settings.tension,
                0..=100,
                Message::TensionChanged
            ),
            labeled_slider_u8(
                "Chord surprise",
                self.settings.surprise,
                0..=100,
                Message::SurpriseChanged
            ),
            labeled_slider_u8(
                "Resolution",
                self.settings.cadence,
                0..=100,
                Message::CadenceChanged
            ),
            labeled_slider_u8(
                "Chord inversion",
                self.settings.chord_inversion_amount,
                0..=100,
                Message::ChordInversionChanged
            ),
        ]
        .spacing(10);

        let rhythm_controls = column![
            labeled_pick(
                "Rhythm style",
                RhythmStyle::ALL.to_vec(),
                self.settings.rhythm_style,
                Message::RhythmStyleChanged
            ),
            labeled_slider_u8(
                "Note density",
                self.settings.density,
                10..=100,
                Message::DensityChanged
            ),
            labeled_slider_u8(
                "Gate / overlap",
                self.settings.note_length,
                0..=100,
                Message::NoteLengthChanged
            ),
        ]
        .spacing(10);

        let phrase_controls = column![
            labeled_slider_u8(
                "Phrase bars",
                self.settings.phrase_length,
                1..=self.settings.bars.min(8) as u8,
                Message::PhraseLengthChanged
            ),
            labeled_slider_u8(
                "Repeat",
                self.settings.repeat_amount,
                0..=100,
                Message::RepeatAmountChanged
            ),
            labeled_slider_u8(
                "Variation",
                self.settings.variation_amount,
                0..=100,
                Message::VariationAmountChanged
            ),
        ]
        .spacing(10);

        let velocity_controls = column![
            text("Velocity").size(13),
            segmented_control(
                &VelocityMode::ALL,
                self.settings.velocity_mode,
                Message::VelocityModeChanged,
                4
            ),
            row![
                text("Random range").size(14),
                text(format!(
                    "{}-{}",
                    self.settings.random_velocity_min, self.settings.random_velocity_max
                ))
                .size(14)
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center),
            labeled_slider_u8(
                "Lower",
                self.settings.random_velocity_min,
                1..=127,
                Message::RandomVelocityMinChanged
            ),
            labeled_slider_u8(
                "Upper",
                self.settings.random_velocity_max,
                1..=127,
                Message::RandomVelocityMaxChanged
            ),
        ]
        .spacing(10);

        let seed_controls = column![
            labeled_pick(
                "Seed mode",
                SeedBehavior::ALL.to_vec(),
                self.settings.seed_behavior,
                Message::SeedBehaviorChanged
            ),
            text_input("Seed", &self.seed_input)
                .on_input(Message::SeedChanged)
                .padding(8),
        ]
        .spacing(10);

        let controls = column![
            expandable_group(
                "Mode",
                SidebarSection::Mode,
                self.sidebar.is_open(SidebarSection::Mode),
                mode_controls.into()
            ),
            expandable_group(
                "Music",
                SidebarSection::Music,
                self.sidebar.is_open(SidebarSection::Music),
                music_controls.into()
            ),
            expandable_group(
                "Harmony",
                SidebarSection::Harmony,
                self.sidebar.is_open(SidebarSection::Harmony),
                harmony_controls.into()
            ),
            expandable_group(
                "Rhythm",
                SidebarSection::Rhythm,
                self.sidebar.is_open(SidebarSection::Rhythm),
                rhythm_controls.into()
            ),
            expandable_group(
                "Phrase",
                SidebarSection::Phrase,
                self.sidebar.is_open(SidebarSection::Phrase),
                phrase_controls.into()
            ),
            expandable_group(
                "Velocity",
                SidebarSection::Velocity,
                self.sidebar.is_open(SidebarSection::Velocity),
                velocity_controls.into()
            ),
            expandable_group(
                "Seed",
                SidebarSection::Seed,
                self.sidebar.is_open(SidebarSection::Seed),
                seed_controls.into()
            ),
        ]
        .spacing(8);

        container(scrollable(container(controls).padding([0, 14, 0, 0])).height(Length::Fill))
            .width(Length::Fixed(286.0))
            .height(Length::Fill)
            .padding(10)
            .style(panel_style())
            .into()
    }

    fn arp_controls(&self) -> Element<'_, Message> {
        if self.settings.mode != GeneratorMode::Arp {
            return column![].into();
        }

        column![
            section_label("Arp"),
            labeled_slider_u8(
                "Notes in arp",
                self.settings.arp_note_count,
                2..=8,
                Message::ArpNoteCountChanged
            ),
            labeled_pick(
                "Pattern",
                ArpPattern::ALL.to_vec(),
                self.settings.arp_pattern,
                Message::ArpPatternChanged
            ),
            labeled_slider_u8(
                "Rotating note",
                self.settings.arp_rotate_slot,
                1..=self.settings.arp_note_count,
                Message::ArpRotateSlotChanged
            ),
            labeled_pick(
                "Rotation",
                ArpRotation::ALL.to_vec(),
                self.settings.arp_rotation,
                Message::ArpRotationChanged
            ),
        ]
        .spacing(10)
        .into()
    }

    fn bassline_controls(&self) -> Element<'_, Message> {
        if self.settings.mode != GeneratorMode::Bassline {
            return column![].into();
        }

        column![
            section_label("Bassline"),
            labeled_pick(
                "Style",
                BasslineStyle::ALL.to_vec(),
                self.settings.bassline_style,
                Message::BasslineStyleChanged
            ),
            labeled_slider_u8(
                "Accent",
                self.settings.bassline_accent,
                0..=100,
                Message::BasslineAccentChanged
            ),
            labeled_slider_u8(
                "Slide",
                self.settings.bassline_slide,
                0..=100,
                Message::BasslineSlideChanged
            ),
            labeled_slider_u8(
                "Octave jump",
                self.settings.bassline_octave_jump,
                0..=100,
                Message::BasslineOctaveJumpChanged
            ),
            labeled_slider_u8(
                "Pattern mutation",
                self.settings.bassline_mutation,
                0..=100,
                Message::BasslineMutationChanged
            ),
        ]
        .spacing(10)
        .into()
    }

    fn preview(&self) -> Element<'_, Message> {
        let chord_lane = self.output.chords.iter().fold(
            row![text("Chords").width(Length::Fixed(64.0)).size(12)].spacing(0),
            |row, chord| {
                row.push(
                    container(
                        text(chord.label())
                            .size(13)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .width(Length::FillPortion(chord.duration_ticks as u16))
                    .padding(6)
                    .style(chord_style()),
                )
            },
        );

        let note_index = PreviewNoteIndex::new(
            &self.output.notes,
            self.settings.bars,
            self.settings.low_pitch(),
            self.settings.high_pitch(),
        );

        let rows = (self.settings.low_pitch()..=self.settings.high_pitch())
            .rev()
            .fold(Column::new().spacing(1), |column, pitch| {
                let cells = self.timeline_cells(pitch, &note_index);
                column.push(
                    row![
                        text(note_name(pitch)).width(Length::Fixed(64.0)).size(11),
                        cells
                    ]
                    .align_items(iced::Alignment::Center),
                )
            });

        let summary = self.settings_strip();
        let bar_lane = (0..self.settings.bars as u32).fold(
            row![text("Bars").width(Length::Fixed(64.0)).size(12)].spacing(0),
            |row, bar| {
                row.push(
                    container(text(format!("{}", bar + 1)).size(12))
                        .width(Length::FillPortion(16))
                        .padding(4)
                        .style(summary_style()),
                )
            },
        );

        container(
            column![
                row![
                    text("Preview").size(20),
                    text(format!("{} {} bars", self.settings.key, self.settings.bars)).size(13)
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center),
                summary,
                bar_lane,
                chord_lane,
                scrollable(rows).height(Length::Fill),
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(12)
        .style(panel_style())
        .into()
    }

    fn settings_strip(&self) -> Element<'_, Message> {
        let content = format!(
            "{} | {} {} | {} bars | {} bpm | {} notes | {}",
            self.settings.mode,
            self.settings.key,
            self.settings.scale,
            self.settings.bars,
            self.settings.tempo,
            self.output.notes.len(),
            self.export_filename
        );

        container(text(content).size(13))
            .padding([7, 9])
            .width(Length::Fill)
            .style(summary_style())
            .into()
    }

    fn timeline_cells(&self, pitch: u8, note_index: &PreviewNoteIndex) -> Element<'_, Message> {
        let steps = self.settings.bars as u32 * 16;

        let mut row = row![].spacing(0).width(Length::Fill);
        let mut step = 0;
        while step < steps {
            let grid_line = grid_line_for_step(step);
            let preview_step = note_index.step_at(pitch, step);
            let (active_velocity, span_steps) = match preview_step {
                PreviewStep::Empty | PreviewStep::NoteContinuation => (None, 1),
                PreviewStep::NoteStart(segment) => {
                    (Some(segment.velocity), segment.span_steps.min(steps - step))
                }
            };

            row = row.push(
                container(text("").size(1))
                    .width(Length::FillPortion(span_steps as u16))
                    .height(Length::Fixed(18.0))
                    .style(timeline_cell_style(active_velocity, grid_line)),
            );
            step += span_steps;
        }

        row.into()
    }
}

async fn pick_export_directory() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Choose export folder")
        .pick_folder()
}

fn generated_export_filename(settings: &GeneratorSettings) -> String {
    filename_from_path(Path::new(&unique_midi_filename(settings)))
}

fn filename_from_path(path: &Path) -> String {
    path.file_name()
        .filter(|filename| !filename.is_empty())
        .map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(|| DEFAULT_EXPORT_FILENAME.to_string())
}

fn labeled_pick<'a, T, F>(
    label: &'a str,
    options: Vec<T>,
    selected: T,
    on_select: F,
) -> Element<'a, Message>
where
    T: Display + Eq + Clone + 'a,
    F: Fn(T) -> Message + 'a,
{
    column![
        text(label).size(14),
        pick_list(options, Some(selected), on_select).width(Length::Fill),
    ]
    .spacing(4)
    .into()
}

fn segmented_control<'a, T, F>(
    options: &'a [T],
    selected: T,
    on_select: F,
    per_row: usize,
) -> Element<'a, Message>
where
    T: Display + Eq + Copy + 'a,
    F: Fn(T) -> Message + Copy + 'a,
{
    let mut rows = column![].spacing(4).width(Length::Fill);
    for chunk in options.chunks(per_row.max(1)) {
        let mut controls = row![].spacing(4).width(Length::Fill);
        for option in chunk {
            let active = *option == selected;
            let button = button(text(option.to_string()).size(12))
                .padding([6, 8])
                .width(Length::FillPortion(1))
                .style(if active {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                })
                .on_press(on_select(*option));
            controls = controls.push(button);
        }
        rows = rows.push(controls);
    }
    rows.into()
}

fn toolbar_button(
    label: &'static str,
    message: Message,
    primary: bool,
) -> Element<'static, Message> {
    button(text(label).size(13))
        .padding([8, 11])
        .style(if primary {
            theme::Button::Primary
        } else {
            theme::Button::Secondary
        })
        .on_press(message)
        .into()
}

fn section_label(label: &str) -> Element<'_, Message> {
    text(label).size(16).into()
}

fn expandable_group<'a>(
    title: &'static str,
    section: SidebarSection,
    open: bool,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let indicator = if open { "v" } else { ">" };
    let mut group = column![button(
        row![
            text(indicator).size(12).width(Length::Fixed(13.0)),
            text(title).size(14)
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .on_press(Message::ToggleSection(section))]
    .spacing(10);

    if open {
        group = group.push(content);
    }

    container(group)
        .padding(8)
        .style(group_style())
        .width(Length::Fill)
        .into()
}

fn labeled_slider_u16<'a, F>(
    label: &'a str,
    value: u16,
    range: std::ops::RangeInclusive<u16>,
    on_change: F,
) -> Element<'a, Message>
where
    F: Fn(u16) -> Message + 'a,
{
    column![
        row![text(label).size(14), text(value.to_string()).size(14)]
            .spacing(8)
            .align_items(iced::Alignment::Center),
        slider(range, value, on_change),
    ]
    .spacing(4)
    .into()
}

fn panel_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: None,
        background: Some(Background::Color(Color::from_rgb8(24, 28, 42))),
        border: Border {
            color: Color::from_rgb8(50, 58, 82),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

fn chord_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(224, 231, 255)),
        background: Some(Background::Color(Color::from_rgb8(38, 48, 79))),
        border: Border {
            color: Color::from_rgb8(79, 91, 128),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

fn summary_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(226, 232, 240)),
        background: Some(Background::Color(Color::from_rgb8(29, 36, 53))),
        border: Border {
            color: Color::from_rgb8(62, 74, 101),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

fn field_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(203, 213, 225)),
        background: Some(Background::Color(Color::from_rgb8(20, 25, 38))),
        border: Border {
            color: Color::from_rgb8(45, 55, 78),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}

fn group_style() -> theme::Container {
    theme::Container::from(|_theme: &Theme| ContainerAppearance {
        text_color: Some(Color::from_rgb8(226, 232, 240)),
        background: Some(Background::Color(Color::from_rgb8(28, 34, 50))),
        border: Border {
            color: Color::from_rgb8(48, 57, 80),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
}

fn timeline_cell_style(velocity: Option<u8>, grid_line: GridLine) -> theme::Container {
    theme::Container::from(move |_theme: &Theme| {
        let note_colors = velocity.map(active_note_colors);
        let background = match velocity {
            Some(_) => note_colors.expect("note colors exist for active note").0,
            None if grid_line == GridLine::Bar => Color::from_rgb8(47, 55, 76),
            None if grid_line == GridLine::Beat => Color::from_rgb8(31, 37, 52),
            None => Color::from_rgb8(22, 27, 39),
        };

        ContainerAppearance {
            text_color: None,
            background: Some(Background::Color(background)),
            border: Border {
                color: if let Some((_, border)) = note_colors {
                    border
                } else {
                    match grid_line {
                        GridLine::Bar => Color::from_rgb8(96, 111, 145),
                        GridLine::Beat => Color::from_rgb8(62, 73, 98),
                        GridLine::Step => Color::from_rgb8(35, 41, 56),
                    }
                },
                width: if velocity.is_some() || grid_line != GridLine::Step {
                    1.0
                } else {
                    0.5
                },
                radius: 2.0.into(),
            },
            ..Default::default()
        }
    })
}

fn active_note_colors(velocity: u8) -> (Color, Color) {
    let intensity = (velocity as f32 / 127.0).clamp(0.0, 1.0);
    let shaped = intensity.powf(0.75);
    let background = Color {
        r: 0.08 + shaped * 0.40,
        g: 0.28 + shaped * 0.55,
        b: 0.42 + shaped * 0.50,
        a: 1.0,
    };
    let border = Color {
        r: 0.18 + shaped * 0.55,
        g: 0.44 + shaped * 0.48,
        b: 0.62 + shaped * 0.36,
        a: 1.0,
    };

    (background, border)
}

fn labeled_slider_u8<'a, F>(
    label: &'a str,
    value: u8,
    range: std::ops::RangeInclusive<u8>,
    on_change: F,
) -> Element<'a, Message>
where
    F: Fn(u8) -> Message + 'a,
{
    column![
        row![text(label).size(14), text(value.to_string()).size(14)]
            .spacing(8)
            .align_items(iced::Alignment::Center),
        slider(range, value, on_change),
    ]
    .spacing(4)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> MelodyApp {
        MelodyApp::new(()).0
    }

    fn apply(app: &mut MelodyApp, message: Message) {
        let _ = app.update(message);
    }

    #[test]
    fn applying_preset_keeps_selected_preset() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );

        assert_eq!(app.settings.preset, GeneratorPreset::TechnoBass);
    }

    #[test]
    fn manual_music_edit_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );
        apply(&mut app, Message::KeyChanged(Key::D));

        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn manual_rhythm_edit_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );
        apply(&mut app, Message::RhythmStyleChanged(RhythmStyle::Busy));

        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn manual_bassline_edit_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );
        apply(
            &mut app,
            Message::BasslineStyleChanged(BasslineStyle::House),
        );

        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn seed_and_export_edits_do_not_mark_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );
        apply(&mut app, Message::SeedChanged("123".to_string()));
        apply(
            &mut app,
            Message::SeedBehaviorChanged(SeedBehavior::RandomizeOnGenerate),
        );
        apply(
            &mut app,
            Message::ExportFilenameChanged("manual.mid".to_string()),
        );

        assert_eq!(app.settings.preset, GeneratorPreset::TechnoBass);
    }

    #[test]
    fn random_velocity_range_edit_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );
        apply(&mut app, Message::RandomVelocityMinChanged(80));

        assert_eq!(app.settings.random_velocity_min, 80);
        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn chord_inversion_edit_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::BocChordPads),
        );

        apply(&mut app, Message::ChordInversionChanged(80));

        assert_eq!(app.settings.chord_inversion_amount, 80);
        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn selected_export_directory_prefixes_current_filename() {
        let mut app = app();
        app.export_filename = "example.mid".to_string();
        let directory = PathBuf::from("/tmp/melody-browser-test");

        apply(
            &mut app,
            Message::ExportDirectorySelected(Some(directory.clone())),
        );

        assert_eq!(app.export_filename, "example.mid");
        assert_eq!(
            app.current_export_path(),
            directory.join("example.mid").display().to_string()
        );
        assert_eq!(app.export_directory, Some(directory));
        assert!(app.export_path_auto);
    }

    #[test]
    fn selected_export_directory_prefixes_future_generated_names() {
        let mut app = app();
        let directory = PathBuf::from("/tmp/melody-browser-test");
        apply(
            &mut app,
            Message::ExportDirectorySelected(Some(directory.clone())),
        );

        apply(&mut app, Message::Generate);

        assert!(Path::new(&app.current_export_path()).starts_with(directory));
        assert!(app.export_filename.ends_with(".mid"));
    }

    #[test]
    fn cancelled_export_directory_keeps_existing_path() {
        let mut app = app();
        app.export_filename = "example.mid".to_string();

        apply(&mut app, Message::ExportDirectorySelected(None));

        assert_eq!(app.export_filename, "example.mid");
        assert_eq!(app.export_directory, None);
    }

    #[test]
    fn editing_filename_keeps_selected_directory() {
        let mut app = app();
        let directory = PathBuf::from("/tmp/melody-browser-test");
        apply(
            &mut app,
            Message::ExportDirectorySelected(Some(directory.clone())),
        );

        apply(
            &mut app,
            Message::ExportFilenameChanged("manual-name".to_string()),
        );

        assert_eq!(app.export_directory, Some(directory.clone()));
        assert_eq!(app.export_filename, "manual-name");
        assert_eq!(
            app.current_export_path(),
            directory.join("manual-name").display().to_string()
        );
        assert!(!app.export_path_auto);
    }

    #[test]
    fn mode_segmented_action_marks_preset_custom() {
        let mut app = app();
        apply(
            &mut app,
            Message::PresetChanged(GeneratorPreset::TechnoBass),
        );

        apply(&mut app, Message::ModeChanged(GeneratorMode::Arp));

        assert_eq!(app.settings.mode, GeneratorMode::Arp);
        assert_eq!(app.settings.preset, GeneratorPreset::Custom);
    }

    #[test]
    fn velocity_segmented_action_updates_velocity_mode() {
        let mut app = app();

        apply(&mut app, Message::VelocityModeChanged(VelocityMode::Random));

        assert_eq!(app.settings.velocity_mode, VelocityMode::Random);
    }

    #[test]
    fn active_note_colors_get_brighter_with_velocity() {
        let (low_background, low_border) = active_note_colors(20);
        let (high_background, high_border) = active_note_colors(120);

        assert!(high_background.r > low_background.r);
        assert!(high_background.g > low_background.g);
        assert!(high_background.b > low_background.b);
        assert!(high_border.g > low_border.g);
    }
}
