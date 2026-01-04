use gtk::{Box, Label, Notebook, Orientation, ScrolledWindow, prelude::*};
use sourceview5::{Buffer, StyleSchemeManager, View, prelude::*};

pub fn build() -> (Notebook, Buffer) {
    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    let body_box = Box::new(Orientation::Vertical, 0);

    let buffer = Buffer::new(None);
    let style_manager = StyleSchemeManager::default();
    if let Some(s) = style_manager
        .scheme("Adwaita-Dark")
        .or_else(|| style_manager.scheme("oblivion"))
    {
        buffer.set_style_scheme(Some(&s));
    }

    let view = View::with_buffer(&buffer);
    view.set_monospace(true);
    view.set_show_line_numbers(true);
    view.set_top_margin(12);
    view.set_left_margin(12);
    view.set_bottom_margin(12);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&view)
        .vexpand(true)
        .build();

    body_box.append(&scrolled);

    let tab_label = Label::new(Some("Body (JSON)"));
    notebook.append_page(&body_box, Some(&tab_label));

    let headers_label = Label::new(Some("Headers"));
    let headers_placeholder = Label::new(Some("Headers coming soon..."));
    notebook.append_page(&headers_placeholder, Some(&headers_label));

    (notebook, buffer)
}
