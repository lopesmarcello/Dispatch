use gtk::{Box, Orientation};
use gtk::{ScrolledWindow, prelude::*};
use sourceview5::prelude::*;
use sourceview5::{Buffer, StyleSchemeManager, View};

pub fn build() -> (Box, Buffer) {
    let container = Box::new(Orientation::Vertical, 0);

    let buffer = Buffer::new(None);
    buffer.set_text("{\n  \"status\": \"ready\",\n  \"message\": \"Hit Send to fetch data...\"\n}");

    let style_manager = StyleSchemeManager::default();
    let scheme = style_manager
        .scheme("Adwaita-dark")
        .or_else(|| style_manager.scheme("oblivion"));

    if let Some(s) = scheme {
        buffer.set_style_scheme(Some(&s));
    }

    let view = View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_show_line_numbers(true);
    view.set_editable(false);
    view.set_top_margin(12);
    view.set_bottom_margin(12);
    view.set_left_margin(12);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .vexpand(true)
        .build();

    container.append(&scrolled_window);

    (container, buffer)
}
