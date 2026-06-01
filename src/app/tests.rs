use super::widgets::active_note_colors;
use super::*;
use iced::Application;
use std::path::{Path, PathBuf};

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

    assert_eq!(app.music.settings.preset, GeneratorPreset::TechnoBass);
}

#[test]
fn manual_music_edit_marks_preset_custom() {
    let mut app = app();
    apply(
        &mut app,
        Message::PresetChanged(GeneratorPreset::TechnoBass),
    );
    apply(&mut app, Message::KeyChanged(Key::D));

    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
}

#[test]
fn manual_rhythm_edit_marks_preset_custom() {
    let mut app = app();
    apply(
        &mut app,
        Message::PresetChanged(GeneratorPreset::TechnoBass),
    );
    apply(&mut app, Message::RhythmStyleChanged(RhythmStyle::Busy));

    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
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

    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
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

    assert_eq!(app.music.settings.preset, GeneratorPreset::TechnoBass);
}

#[test]
fn random_velocity_range_edit_marks_preset_custom() {
    let mut app = app();
    apply(
        &mut app,
        Message::PresetChanged(GeneratorPreset::TechnoBass),
    );
    apply(&mut app, Message::RandomVelocityMinChanged(80));

    assert_eq!(app.music.settings.random_velocity_min, 80);
    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
}

#[test]
fn chord_inversion_edit_marks_preset_custom() {
    let mut app = app();
    apply(
        &mut app,
        Message::PresetChanged(GeneratorPreset::BocChordPads),
    );

    apply(&mut app, Message::ChordInversionChanged(80));

    assert_eq!(app.music.settings.chord_inversion_amount, 80);
    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
}

#[test]
fn selected_export_directory_prefixes_current_filename() {
    let mut app = app();
    app.export.filename = "example.mid".to_string();
    let directory = PathBuf::from("/tmp/melody-browser-test");

    apply(
        &mut app,
        Message::ExportDirectorySelected(Some(directory.clone())),
    );

    assert_eq!(app.export.filename, "example.mid");
    assert_eq!(
        app.current_export_path(),
        directory.join("example.mid").display().to_string()
    );
    assert_eq!(app.export.directory, Some(directory));
    assert!(app.export.path_auto);
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
    assert!(app.export.filename.ends_with(".mid"));
}

#[test]
fn cancelled_export_directory_keeps_existing_path() {
    let mut app = app();
    app.export.filename = "example.mid".to_string();

    apply(&mut app, Message::ExportDirectorySelected(None));

    assert_eq!(app.export.filename, "example.mid");
    assert_eq!(app.export.directory, None);
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

    assert_eq!(app.export.directory, Some(directory.clone()));
    assert_eq!(app.export.filename, "manual-name");
    assert_eq!(
        app.current_export_path(),
        directory.join("manual-name").display().to_string()
    );
    assert!(!app.export.path_auto);
}

#[test]
fn mode_segmented_action_marks_preset_custom() {
    let mut app = app();
    apply(
        &mut app,
        Message::PresetChanged(GeneratorPreset::TechnoBass),
    );

    apply(&mut app, Message::ModeChanged(GeneratorMode::Arp));

    assert_eq!(app.music.settings.mode, GeneratorMode::Arp);
    assert_eq!(app.music.settings.preset, GeneratorPreset::Custom);
}

#[test]
fn velocity_segmented_action_updates_velocity_mode() {
    let mut app = app();

    apply(&mut app, Message::VelocityModeChanged(VelocityMode::Random));

    assert_eq!(app.music.settings.velocity_mode, VelocityMode::Random);
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
