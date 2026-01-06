use crate::config;
use gtk::{Label, Notebook};
use gtk::{ScrolledWindow, prelude::*};
use sourceview5::prelude::*;
use sourceview5::{Buffer, StyleSchemeManager, View};

pub fn build() -> (Notebook, Buffer, Buffer) {
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    fn create_view(lang_id: Option<&str>) -> (ScrolledWindow, Buffer) {
        let buffer = Buffer::new(None);

        let style_manager = StyleSchemeManager::default();
        let scheme = style_manager
            .scheme(config::EDITOR_SCHEME_PREF_1)
            .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_2))
            .or_else(|| style_manager.scheme(config::EDITOR_SCHEME_PREF_3));

        if let Some(s) = scheme {
            buffer.set_style_scheme(Some(&s));
        };

        if let Some(id) = lang_id {
            if let Some(lang) = sourceview5::LanguageManager::default().language(id) {
                buffer.set_language(Some(&lang));
            }
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

        (scrolled_window, buffer)
    }

    let (body_scroll, body_buffer) = create_view(Some("json"));
    let body_label = Label::new(Some("Body"));
    notebook.append_page(&body_scroll, Some(&body_label));

    let (headers_scroll, headers_buffer) = create_view(None);
    let headers_label = Label::new(Some("Headers"));
    notebook.append_page(&headers_scroll, Some(&headers_label));

    (notebook, body_buffer, headers_buffer)
}
