use clipboard::ClipboardProvider;

#[cfg(not(target_os = "linux"))]
use clipboard::ClipboardContext;
#[cfg(target_os = "linux")]
use clipboard_ext::x11_fork::ClipboardContext;

pub fn clipboard_set(text: String) {
    ClipboardContext::new().unwrap().set_contents(text).unwrap();
}
