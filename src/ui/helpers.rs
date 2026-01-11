use adw::ApplicationWindow;
use gtk::{Box, Orientation, prelude::*};
use sourceview5::{Buffer, LanguageManager, prelude::BufferExt};
use std::rc::Rc;

use crate::config;

pub fn add_box_margins(target_box: &Box, size: i32) {
    target_box.set_margin_top(size);
    target_box.set_margin_bottom(size);
    target_box.set_margin_start(size);
    target_box.set_margin_end(size);
}

pub fn set_syntax_highlighting(buffer: &Buffer, headers_or_type: &str) {
    let lm = LanguageManager::default();
    let text = headers_or_type.to_lowercase();

    let lang_id = if text.contains("application/json") || text.contains("text/json") {
        "json"
    } else if text.contains("html") {
        "html"
    } else if text.contains("xml") {
        "xml"
    } else if text.contains("javascript") {
        "javascript"
    } else if text.contains("css") {
        "css"
    } else if text.contains("markdown") {
        "markdown"
    } else {
        "text"
    };

    if let Some(lang) = lm.language(lang_id) {
        buffer.set_language(Some(&lang));
    }
}

pub fn show_input_dialog<F>(parent: &ApplicationWindow, title: &str, callback: F)
where
    F: Fn(String) + 'static,
{
    let window = gtk::Window::builder()
        .transient_for(parent)
        .modal(true)
        .title(title)
        .default_width(350)
        .resizable(false)
        .build();

    let content_box = gtk::Box::new(gtk::Orientation::Vertical, config::SPACING_LARGE);
    add_box_margins(&content_box, config::SPACING_EXTRA_LARGE);

    let label = gtk::Label::new(Some("Enter the name for the new collection:"));
    label.set_xalign(0.0);
    content_box.append(&label);

    let entry = gtk::Entry::builder()
        .placeholder_text("Collection Name")
        .activates_default(true)
        .build();
    content_box.append(&entry);

    let btn_box = gtk::Box::new(Orientation::Horizontal, 10);
    btn_box.set_halign(gtk::Align::End);

    let cancel_btn = gtk::Button::with_label("Cancel");
    let create_btn = gtk::Button::with_label("Create");
    create_btn.add_css_class("suggested-action");

    btn_box.append(&cancel_btn);
    btn_box.append(&create_btn);
    content_box.append(&btn_box);

    window.set_child(Some(&content_box));

    // Logic
    let window_clone = window.clone();
    cancel_btn.connect_clicked(move |_| {
        window_clone.close();
    });

    let window_clone = window.clone();
    let entry_clone = entry.clone();
    let callback = Rc::new(callback);

    let on_create = move || {
        let text = entry_clone.text().to_string();
        if !text.is_empty() {
            callback(text);
        }
        window_clone.close();
    };

    let on_create_clone = on_create.clone();
    create_btn.connect_clicked(move |_| on_create_clone());

    // Allow pressing "Enter" to submit
    entry.connect_activate(move |_| on_create());

    window.present();
}
