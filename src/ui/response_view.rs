use crate::config;
use gtk::{Box, Orientation};
use gtk::{ScrolledWindow, prelude::*};
use sourceview5::prelude::*;
use sourceview5::{Buffer, StyleSchemeManager, View};

pub fn build() -> (Box, Buffer) {
    let container = Box::new(Orientation::Vertical, config::SPACING_NONE);

    let buffer = Buffer::new(None);
    buffer.set_text("{\n  \"status\": \"ready\",\n  \"message\": \"Hit Send to fetch data...\"\n}");

    let style_manager = StyleSchemeManager::default();
    let scheme = style_manager
        .scheme(config::EDITOR_SCHEME_PREF_1)
        .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_2))
        .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_3));


    if let Some(s) = scheme {
        buffer.set_style_scheme(Some(&s));
    }

    let view = View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_show_line_numbers(true);
    view.set_editable(false);
    view.set_top_margin(config::SPACING_MEDIUM);
    view.set_bottom_margin(config::SPACING_MEDIUM);
    view.set_left_margin(config::SPACING_MEDIUM);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .vexpand(true)
        .build();

    container.append(&scrolled_window);

    (container, buffer)
}
