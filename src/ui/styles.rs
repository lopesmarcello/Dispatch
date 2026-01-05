use gtk::gdk::Display;
use gtk::{CssProvider, StyleContext};

const APP_CSS: &str = "
    .badge-get { background-color: #61affe; color: black; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    .badge-post { background-color: #49cc90; color: black; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    .badge-put { background-color: #fca130; color: black; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    .badge-delete { background-color: #f93e3e; color: white; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    .badge-patch { background-color: #50e3c2; color: black; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    .badge-default { background-color: #999999; color: black; border-radius: 4px; padding: 2px 6px; font-weight: bold; }
    
    .sidebar .heading { font-weight: 800; font-size: 14px; opacity: 0.8; }
";

pub fn load() {
    let provider = CssProvider::new();
    provider.load_from_data(APP_CSS);

    if let Some(display) = Display::default() {
        StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
