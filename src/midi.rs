use midly::num::{u15, u24, u28, u4, u7};
use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::DEFAULT_EXPORT_DIR;
use crate::generator::{BasslineStyle, GeneratedSong, GeneratorMode, GeneratorSettings, PPQN};

static EXPORT_NAME_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportResult {
    pub path: PathBuf,
    pub created_parent: bool,
}

pub fn export_midi(
    song: &GeneratedSong,
    tempo: u16,
    requested_path: &str,
    allow_overwrite: bool,
) -> Result<ExportResult, String> {
    let path = normalized_midi_path(requested_path)?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    let mut created_parent = false;

    if let Some(parent) = parent {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
            created_parent = true;
        }
    }

    if path.exists() && !allow_overwrite {
        return Err(format!(
            "{} already exists. Choose a new name or generate again.",
            path.display()
        ));
    }

    let bytes = midi_bytes(song, tempo)?;
    fs::write(&path, bytes).map_err(|error| error.to_string())?;
    Ok(ExportResult {
        path,
        created_parent,
    })
}

pub fn normalized_midi_path(requested_path: &str) -> Result<PathBuf, String> {
    let trimmed = requested_path.trim();
    if trimmed.is_empty() {
        return Err("export path cannot be empty".to_string());
    }

    let path = PathBuf::from(trimmed);
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mid"))
    {
        Ok(path)
    } else {
        let mut value = path.into_os_string();
        value.push(".mid");
        Ok(PathBuf::from(value))
    }
}

pub fn midi_bytes(song: &GeneratedSong, tempo: u16) -> Result<Vec<u8>, String> {
    let mut absolute_events = Vec::new();
    absolute_events.push((
        0,
        TrackEventKind::Meta(MetaMessage::Tempo(u24::new(60_000_000 / tempo as u32))),
    ));

    for note in &song.notes {
        absolute_events.push((
            note.start_ticks,
            TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOn {
                    key: u7::new(note.pitch),
                    vel: u7::new(note.velocity),
                },
            },
        ));
        absolute_events.push((
            note.start_ticks + note.duration_ticks.max(1),
            TrackEventKind::Midi {
                channel: u4::new(0),
                message: MidiMessage::NoteOff {
                    key: u7::new(note.pitch),
                    vel: u7::new(0),
                },
            },
        ));
    }

    absolute_events.sort_by_key(|(tick, kind)| {
        let priority = match kind {
            TrackEventKind::Meta(_) => 0,
            TrackEventKind::Midi {
                message: MidiMessage::NoteOff { .. },
                ..
            } => 1,
            _ => 2,
        };
        (*tick, priority)
    });

    let mut last_tick = 0;
    let mut track = Vec::new();
    for (tick, kind) in absolute_events {
        track.push(TrackEvent {
            delta: u28::new(tick - last_tick),
            kind,
        });
        last_tick = tick;
    }
    track.push(TrackEvent {
        delta: u28::new(0),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    let smf = Smf {
        header: Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(u15::new(PPQN)),
        },
        tracks: vec![track],
    };

    let mut bytes = Vec::new();
    smf.write_std(&mut bytes)
        .map_err(|error| error.to_string())?;
    Ok(bytes)
}

pub fn unique_midi_filename(settings: &GeneratorSettings) -> String {
    unique_midi_path(settings, Path::new(DEFAULT_EXPORT_DIR))
        .display()
        .to_string()
}

pub fn unique_midi_path(settings: &GeneratorSettings, directory: &Path) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    let counter = EXPORT_NAME_COUNTER.fetch_add(1, Ordering::Relaxed);

    directory.join(format!(
        "melody-{}-{}-{}-{}.mid",
        generator_slug(settings),
        timestamp,
        settings.seed,
        counter
    ))
}

fn generator_slug(settings: &GeneratorSettings) -> &'static str {
    if settings.mode == GeneratorMode::Bassline {
        return bassline_style_slug(settings.bassline_style);
    }
    generator_mode_slug(settings.mode)
}

fn generator_mode_slug(mode: GeneratorMode) -> &'static str {
    match mode {
        GeneratorMode::Melodic => "melodic",
        GeneratorMode::Hook => "hook",
        GeneratorMode::CounterMelody => "counter-melody",
        GeneratorMode::BuildupDrop => "buildup-drop",
        GeneratorMode::Euclidean => "euclidean",
        GeneratorMode::Arp => "arp",
        GeneratorMode::Chiptune => "chiptune",
        GeneratorMode::Bassline => "bassline",
        GeneratorMode::ChordPads => "chord-pads",
    }
}

fn bassline_style_slug(style: BasslineStyle) -> &'static str {
    match style {
        BasslineStyle::Techno => "bassline-techno",
        BasslineStyle::House => "bassline-house",
        BasslineStyle::Drill => "bassline-drill",
        BasslineStyle::HipHop => "bassline-hiphop",
        BasslineStyle::UkGarage => "bassline-uk-garage",
        BasslineStyle::DrumAndBass => "bassline-drum-and-bass",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{generate_song, GeneratorSettings};

    #[test]
    fn empty_export_path_is_rejected() {
        assert!(normalized_midi_path("  ").is_err());
    }

    #[test]
    fn export_path_appends_mid_extension() {
        assert_eq!(
            normalized_midi_path("exports/example").unwrap(),
            PathBuf::from("exports/example.mid")
        );
    }

    #[test]
    fn export_creates_parent_directory() {
        let settings = GeneratorSettings::default();
        let song = generate_song(&settings);
        let dir = std::env::temp_dir().join(format!(
            "melody_export_parent_{}",
            unique_midi_filename(&settings).replace('/', "_")
        ));
        let path = dir.join("nested").join("song");

        let result = export_midi(&song, settings.tempo, path.to_str().unwrap(), false).unwrap();
        assert!(result.created_parent);
        assert!(result.path.exists());

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn export_refuses_to_overwrite_manual_path() {
        let settings = GeneratorSettings::default();
        let song = generate_song(&settings);
        let path = std::env::temp_dir().join("melody_overwrite_test.mid");
        fs::write(&path, b"existing").unwrap();

        let error = export_midi(&song, settings.tempo, path.to_str().unwrap(), false)
            .expect_err("manual exports should not overwrite");
        assert!(error.contains("already exists"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn auto_generated_export_path_may_overwrite() {
        let settings = GeneratorSettings::default();
        let song = generate_song(&settings);
        let path = std::env::temp_dir().join("melody_auto_overwrite_test.mid");
        fs::write(&path, b"existing").unwrap();

        export_midi(&song, settings.tempo, path.to_str().unwrap(), true).unwrap();
        assert!(fs::metadata(&path).unwrap().len() > 32);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn midi_bytes_are_parseable() {
        let settings = GeneratorSettings::default();
        let song = generate_song(&settings);

        let bytes = midi_bytes(&song, settings.tempo).unwrap();

        Smf::parse(&bytes).expect("MIDI bytes should parse");
    }

    #[test]
    fn unique_midi_filename_has_mid_extension_and_mode_slug() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::Bassline,
            bassline_style: BasslineStyle::House,
            seed: 123,
            ..GeneratorSettings::default()
        };
        let filename = unique_midi_filename(&settings);
        assert!(filename.starts_with("exports/melody-bassline-house-"));
        assert!(filename.ends_with(".mid"));
        assert!(filename.contains("-123-"));
    }

    #[test]
    fn counter_melody_filename_uses_mode_slug() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::CounterMelody,
            seed: 321,
            ..GeneratorSettings::default()
        };
        let filename = unique_midi_filename(&settings);

        assert!(filename.starts_with("exports/melody-counter-melody-"));
        assert!(filename.contains("-321-"));
    }

    #[test]
    fn buildup_drop_filename_uses_mode_slug() {
        let settings = GeneratorSettings {
            mode: GeneratorMode::BuildupDrop,
            seed: 654,
            ..GeneratorSettings::default()
        };
        let filename = unique_midi_filename(&settings);

        assert!(filename.starts_with("exports/melody-buildup-drop-"));
        assert!(filename.contains("-654-"));
    }

    #[test]
    fn unique_midi_filename_changes_between_calls() {
        let settings = GeneratorSettings::default();
        let first = unique_midi_filename(&settings);
        let second = unique_midi_filename(&settings);
        assert_ne!(first, second);
    }

    #[test]
    fn unique_midi_path_uses_selected_directory() {
        let settings = GeneratorSettings::default();
        let path = unique_midi_path(&settings, Path::new("/tmp/melody-target"));

        assert!(path.starts_with("/tmp/melody-target"));
        assert!(path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("melody-"));
    }
}
