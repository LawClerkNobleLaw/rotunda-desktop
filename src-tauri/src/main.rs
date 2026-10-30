// Rotunda for the desktop: one window onto https://myrotunda.com.
//
// Everything real (sign-in, the firm's data, new features) lives on the
// website; this shell only gives it an icon and a window of its own. So it
// almost never needs a new release: a change to the site shows up here the
// next time the window loads.
//
// Three things a browser does that a bare window doesn't, handled below:
// links to other sites (leginfo, LegiScan, CAL-ACCESS, email) go to the
// person's own browser or mail app; "open in new tab" links open in this
// window, since there are no tabs; and downloads land in the Downloads folder.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use tauri::webview::{DownloadEvent, NewWindowResponse};
use tauri::{Manager, Runtime, Url, Webview, WebviewUrl, WebviewWindowBuilder};

// The dashboard sends anyone not signed in to the sign-in page first.
const HOME: &str = "https://myrotunda.com/dashboard";

fn is_rotunda(url: &Url) -> bool {
    matches!(url.host_str(), Some("myrotunda.com") | Some("www.myrotunda.com"))
}

fn opens_outside(url: &Url) -> bool {
    match url.scheme() {
        "http" | "https" => !is_rotunda(url),
        "mailto" | "tel" => true,
        // about:, blob: and data: pages are the site's own (e.g. an export).
        _ => false,
    }
}

fn open_outside(url: &Url) {
    let _ = open::that_detached(url.as_str());
}

// An app window has no toolbar, so the browser's back / forward / reload
// shortcuts are wired up here: Cmd+[ Cmd+] Cmd+R on a Mac, Alt+Left
// Alt+Right Ctrl+R (or F5) elsewhere.
const SHORTCUTS: &str = r#"
(function () {
  var mac = /Mac/.test(navigator.platform);
  document.addEventListener('keydown', function (e) {
    var key = e.key.toLowerCase();
    if (mac ? e.metaKey && key === '[' : e.altKey && key === 'arrowleft') history.back();
    else if (mac ? e.metaKey && key === ']' : e.altKey && key === 'arrowright') history.forward();
    else if ((mac ? e.metaKey : e.ctrlKey) && key === 'r' || key === 'f5') location.reload();
    else return;
    e.preventDefault();
  });
})();
"#;

// Keeps the name the site gave the file, adding " (1)", " (2)"… rather than
// overwriting something already in Downloads.
fn save_to_downloads<R: Runtime>(webview: &Webview<R>, url: &Url, destination: &mut PathBuf) {
    let Ok(dir) = webview.path().download_dir() else {
        return;
    };
    let name = destination
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .or_else(|| url.path_segments()?.last().filter(|s| !s.is_empty()).map(String::from))
        .unwrap_or_else(|| "Rotunda download".into());
    let first = dir.join(&name);
    let stem = Path::new(&name).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or(name.clone());
    let ext = Path::new(&name).extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let mut path = first;
    let mut n = 1;
    while path.exists() {
        path = dir.join(format!("{stem} ({n}){ext}"));
        n += 1;
    }
    *destination = path;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(HOME.parse()?))
                .title("Rotunda")
                .inner_size(1280.0, 860.0)
                .min_inner_size(400.0, 500.0)
                .initialization_script(SHORTCUTS)
                .on_navigation(|url| {
                    if opens_outside(url) {
                        open_outside(url);
                        return false;
                    }
                    true
                })
                .on_new_window(move |url, _features| {
                    if opens_outside(&url) {
                        open_outside(&url);
                    } else if let Some(main) = handle.get_webview_window("main") {
                        let _ = main.navigate(url);
                    }
                    NewWindowResponse::Deny
                })
                .on_download(|webview, event| {
                    if let DownloadEvent::Requested { url, destination } = event {
                        save_to_downloads(&webview, &url, destination);
                    }
                    true
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Rotunda could not start");
}
