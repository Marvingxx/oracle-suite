//! Carries out what the Library tab asks for: choosing its folder, dragging a file out of the
//! window to another app, opening a file in an Oracle app, and showing it in the file manager.

use std::path::Path;

use oracle_suite_ui_egui::oracle_library_ui::Request;

use crate::DesktopHost;

/// The drag picture when a file has no preview: the Oracle mark.
#[cfg(any(target_os = "windows", target_os = "macos"))]
const DRAG_ICON: &[u8] = include_bytes!("../../../docs/brand/oracle-mark-64.png");

/// Handles the Library tab's pending requests. `frame` gives the window a native drag starts from.
pub fn handle(ui: &mut oracle_suite_ui_egui::LauncherUi, host: &mut DesktopHost, frame: &eframe::Frame) {
    let requests = match ui.library.as_mut() {
        Some(library) => library.take_requests(),
        None => return,
    };
    for request in requests {
        match request {
            Request::PickFolder => {
                if let Some(folder) = oracle_suite_ui_egui::Host::pick_folder(host, None)
                    && let Some(library) = ui.library.as_mut()
                {
                    library.set_root(folder.into());
                }
            }
            Request::Drag { file, preview } => {
                if start_drag(frame, &file, preview.as_deref())
                    && let Some(library) = ui.library.as_mut()
                {
                    library.used(&file);
                }
            }
            Request::OpenWith { app, file } => {
                let result = match ui.installed_location(app) {
                    Some(exe) => oracle_suite_native::process::launch_binary(Path::new(&exe), &[&file.to_string_lossy()]),
                    // Not installed: the system's default app still opens the file.
                    None => oracle_suite_native::process::open(file.as_os_str()),
                };
                report(ui, &file, result);
            }
            Request::Open(file) => {
                let result = oracle_suite_native::process::open(file.as_os_str());
                report(ui, &file, result);
            }
            Request::Reveal(file) => {
                if let Err(e) = oracle_suite_native::process::reveal(&file) {
                    log::warn!("couldn't show {}: {e}", file.display());
                }
            }
        }
    }
}

fn report(ui: &mut oracle_suite_ui_egui::LauncherUi, file: &Path, result: Result<(), String>) {
    let Some(library) = ui.library.as_mut() else { return };
    match result {
        Ok(()) => library.used(file),
        Err(e) => {
            log::warn!("couldn't open {}: {e}", file.display());
            library.say(e);
        }
    }
}

/// Starts a native drag of `file` from the window. Blocks until it is dropped or cancelled;
/// true when it was dropped somewhere.
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn start_drag(frame: &eframe::Frame, file: &Path, preview: Option<&Path>) -> bool {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let Ok(file) = std::path::absolute(file) else { return false };
    let image = match preview.filter(|p| p.is_file() && p.extension().is_some_and(|e| matches!(e.to_string_lossy().to_ascii_lowercase().as_str(), "png" | "jpg" | "jpeg"))) {
        Some(p) => drag::Image::File(p.to_path_buf()),
        None => drag::Image::Raw(DRAG_ICON.to_vec()),
    };
    let dropped = Arc::new(AtomicBool::new(false));
    let flag = dropped.clone();
    let result = drag::start_drag(
        frame,
        drag::DragItem::Files(vec![file]),
        image,
        move |result, _| flag.store(matches!(result, drag::DragResult::Dropped), Ordering::SeqCst),
        drag::Options::default(),
    );
    if let Err(e) = result {
        log::warn!("couldn't start the drag: {e}");
        return false;
    }
    dropped.load(Ordering::SeqCst)
}

/// No native drag out of a winit window on Linux and the BSDs yet.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn start_drag(_frame: &eframe::Frame, _file: &Path, _preview: Option<&Path>) -> bool {
    false
}
