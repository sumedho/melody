mod sidebar;
mod update;
mod view;
mod widgets;

#[cfg(test)]
mod tests;

use iced::executor;
use iced::{Application, Command, Element, Settings, Size, Theme};
use std::path::{Path, PathBuf};

use crate::constants::{DEFAULT_EXPORT_DIR, DEFAULT_EXPORT_FILENAME};
use crate::generator::*;
use crate::midi::unique_midi_filename;

use sidebar::{SidebarSection, SidebarState};

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
    music: MusicState,
    export: ExportState,
    ui: UIState,
}

struct MusicState {
    settings: GeneratorSettings,
    output: GeneratedSong,
    locked_chords: Option<Vec<ChordEvent>>,
}

struct ExportState {
    filename: String,
    directory: Option<PathBuf>,
    path_auto: bool,
}

struct UIState {
    sidebar: SidebarState,
    seed_input: String,
    status: String,
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
                music: MusicState {
                    settings,
                    output,
                    locked_chords: None,
                },
                export: ExportState {
                    filename: generated_export_filename(&settings),
                    directory: None,
                    path_auto: true,
                },
                ui: UIState {
                    sidebar: SidebarState::default(),
                    seed_input: settings.seed.to_string(),
                    status: "Generated a starting melody.".to_string(),
                },
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
        self.handle_update(message)
    }

    fn view(&self) -> Element<'_, Message> {
        self.view_content()
    }
}

impl MelodyApp {
    fn generate_current_song(&self) -> GeneratedSong {
        generate_song_with_chords(&self.music.settings, self.music.locked_chords.as_deref())
    }

    fn next_export_filename(&self) -> String {
        generated_export_filename(&self.music.settings)
    }

    fn current_export_directory(&self) -> &Path {
        self.export
            .directory
            .as_deref()
            .unwrap_or_else(|| Path::new(DEFAULT_EXPORT_DIR))
    }

    fn current_export_path(&self) -> String {
        self.current_export_directory()
            .join(&self.export.filename)
            .display()
            .to_string()
    }

    fn export_directory_label(&self) -> String {
        self.current_export_directory().display().to_string()
    }

    fn update_setting(&mut self, update: impl FnOnce(&mut GeneratorSettings)) {
        update(&mut self.music.settings);
        self.music.settings.preset = GeneratorPreset::Custom;
    }
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
