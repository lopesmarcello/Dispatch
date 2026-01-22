use crate::{config, ui::helpers, ui::key_value_editor::KeyValueEditor};
use gtk::{prelude::*, Box, Label, Notebook, Orientation, ScrolledWindow};
use sourceview5::prelude::*;

pub fn build() -> (Notebook, sourceview5::Buffer, KeyValueEditor) {
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    fn create_json_view() -> (ScrolledWindow, sourceview5::Buffer) {
        let buffer = sourceview5::Buffer::new(None);

        let style_manager = sourceview5::StyleSchemeManager::default();
        let scheme = style_manager
            .scheme(config::EDITOR_SCHEME_PREF_1)
            .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_2))
            .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_3));

        if let Some(s) = scheme {
            buffer.set_style_scheme(Some(&s));
        }

        // Set JSON syntax highlighting
        helpers::set_syntax_highlighting(&buffer, "application/json");

        let view = sourceview5::View::with_buffer(&buffer);
        view.set_monospace(true);
        view.set_show_line_numbers(true);
        view.set_top_margin(config::SPACING_MEDIUM);
        view.set_bottom_margin(config::SPACING_MEDIUM);
        view.set_left_margin(config::SPACING_MEDIUM);

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&view)
            .vexpand(true)
            .build();

        (scrolled_window, buffer)
    }

    let body_box = Box::new(Orientation::Vertical, config::SPACING_NONE);
    let (scrolled, buffer) = create_json_view();
    body_box.append(&scrolled);

    let tab_label = Label::new(Some("Body (JSON)"));
    notebook.append_page(&body_box, Some(&tab_label));

    let headers_editor = KeyValueEditor::new();
    let headers_label = Label::new(Some("Headers"));

    notebook.append_page(&headers_editor.container, Some(&headers_label));

    (notebook, buffer, headers_editor)
}
