use crate::generator::GeneratedSong;
use crate::midi;
use iced::window;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum DragExportResult {
    Started(PathBuf),
    Unavailable(PathBuf),
    Failed(String),
}

pub fn write_drag_midi(song: &GeneratedSong, tempo: u16) -> Result<PathBuf, String> {
    let path = drag_midi_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    }
    fs::write(&path, midi::midi_bytes(song, tempo)?).map_err(|error| error.to_string())?;
    Ok(path)
}

fn drag_midi_path() -> PathBuf {
    std::env::temp_dir().join("melody").join("melody.mid")
}

#[cfg(target_os = "macos")]
pub fn begin_native_file_drag(
    handle: &window::raw_window_handle::WindowHandle<'_>,
    path: &Path,
) -> Result<(), String> {
    macos::begin_native_file_drag(handle, path)
}

#[cfg(not(target_os = "macos"))]
pub fn begin_native_file_drag(
    _handle: &window::raw_window_handle::WindowHandle<'_>,
    _path: &Path,
) -> Result<(), String> {
    Err(String::from("Native MIDI drag is only available on macOS."))
}

#[cfg(target_os = "macos")]
mod macos {
    use objc2::class;
    use objc2::msg_send;
    use objc2::runtime::{AnyClass, AnyObject, AnyProtocol, ClassBuilder, Sel};
    use objc2::sel;
    use objc2_foundation::{CGPoint, CGRect, CGSize, NSString};
    use raw_window_handle::RawWindowHandle;
    use std::path::Path;
    use std::sync::OnceLock;

    pub fn begin_native_file_drag(
        handle: &raw_window_handle::WindowHandle<'_>,
        path: &Path,
    ) -> Result<(), String> {
        let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
            return Err(String::from("Window is not an AppKit window."));
        };

        let path = path
            .to_str()
            .ok_or_else(|| String::from("MIDI path is not valid UTF-8."))?;
        let ns_view = appkit.ns_view.as_ptr().cast::<AnyObject>();

        unsafe {
            let app: *mut AnyObject = msg_send![class!(NSApplication), sharedApplication];
            let event: *mut AnyObject = msg_send![app, currentEvent];
            if event.is_null() {
                return Err(String::from("AppKit current event is unavailable."));
            }

            let filename = NSString::from_str(path);
            let url: *mut AnyObject = msg_send![class!(NSURL), fileURLWithPath: &*filename];
            if url.is_null() {
                return Err(String::from("Could not create MIDI file URL."));
            }

            let item_alloc: *mut AnyObject = msg_send![class!(NSDraggingItem), alloc];
            let item: *mut AnyObject = msg_send![item_alloc, initWithPasteboardWriter: url];
            if item.is_null() {
                return Err(String::from("Could not create dragging item."));
            }

            let workspace: *mut AnyObject = msg_send![class!(NSWorkspace), sharedWorkspace];
            let image: *mut AnyObject = msg_send![workspace, iconForFile: &*filename];
            if image.is_null() {
                return Err(String::from("Could not create drag image."));
            }
            let _: () = msg_send![
                image,
                setSize: CGSize {
                    width: 48.0,
                    height: 48.0
                }
            ];

            let frame = CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize {
                    width: 48.0,
                    height: 48.0,
                },
            };
            let _: () = msg_send![item, setDraggingFrame: frame contents: image];

            let items: *mut AnyObject = msg_send![class!(NSArray), arrayWithObject: item];
            let session: *mut AnyObject = msg_send![
                ns_view,
                beginDraggingSessionWithItems: items
                event: event
                source: dragging_source()
            ];
            if session.is_null() {
                return Err(String::from("Could not start AppKit dragging session."));
            }

            let _: () = msg_send![item, release];
        }

        Ok(())
    }

    fn dragging_source() -> *mut AnyObject {
        static SOURCE: OnceLock<usize> = OnceLock::new();

        *SOURCE.get_or_init(|| unsafe {
            let source: *mut AnyObject = msg_send![dragging_source_class(), new];
            source as usize
        }) as *mut AnyObject
    }

    fn dragging_source_class() -> &'static AnyClass {
        static CLASS: OnceLock<usize> = OnceLock::new();

        let class = *CLASS.get_or_init(|| {
            let mut builder = ClassBuilder::new("MelodyDraggingSource", class!(NSObject))
                .expect("MelodyDraggingSource class should be registered once");

            if let Some(protocol) = AnyProtocol::get("NSDraggingSource") {
                builder.add_protocol(protocol);
            }

            unsafe {
                let source_operation_mask: extern "C" fn(_, _, _, _) -> _ = source_operation_mask;
                builder.add_method(
                    sel!(draggingSession:sourceOperationMaskForDraggingContext:),
                    source_operation_mask,
                );
            }

            builder.register() as *const AnyClass as usize
        });

        unsafe { &*(class as *const AnyClass) }
    }

    extern "C" fn source_operation_mask(
        _this: &AnyObject,
        _cmd: Sel,
        _session: *mut AnyObject,
        _context: isize,
    ) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{generate_song, GeneratorSettings};
    use midly::Smf;

    #[test]
    fn drag_midi_export_writes_parseable_temp_file() {
        let settings = GeneratorSettings::default();
        let song = generate_song(&settings);

        let path = write_drag_midi(&song, settings.tempo).expect("drag MIDI writes");
        let bytes = fs::read(path).expect("drag MIDI can be read");
        Smf::parse(&bytes).expect("drag MIDI parses");
    }
}
