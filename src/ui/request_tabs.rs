use crate::{config, ui::key_value_editor::KeyValueEditor};
use gtk::{Box, Label, Notebook, Orientation, ScrolledWindow, prelude::*};
use sourceview5::{Buffer, StyleSchemeManager, View, prelude::*};

pub fn build() -> (Notebook, Buffer, KeyValueEditor) {
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    let body_box = Box::new(Orientation::Vertical, config::SPACING_NONE);

    let buffer = Buffer::new(None);
    let style_manager = StyleSchemeManager::default();
    if let Some(s) = style_manager
        .scheme(config::EDITOR_SCHEME_PREF_1)
        .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_2))
        .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_3))
    {
        buffer.set_style_scheme(Some(&s));
    }

    let view = View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_show_line_numbers(true);
    view.set_top_margin(config::SPACING_MEDIUM);
    view.set_left_margin(config::SPACING_MEDIUM);
    view.set_bottom_margin(config::SPACING_MEDIUM);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .vexpand(true)
        .build();

    body_box.append(&scrolled);

    let tab_label = Label::new(Some("Body (JSON)"));
    notebook.append_page(&body_box, Some(&tab_label));

    let headers_editor = KeyValueEditor::new();
    let headers_label = Label::new(Some("Headers"));

    notebook.append_page(&headers_editor.container, Some(&headers_label));

    (notebook, buffer, headers_editor)
}
