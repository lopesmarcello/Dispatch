use crate::config;
use gtk::prelude::*;
use gtk::{Box, Button, DropDown, Entry, Orientation, StringList};

pub fn build() -> (Box, Entry, DropDown, Button) {
    // a horizontal box with 12px spacing
    let container = Box::new(Orientation::Horizontal, config::SPACING_MEDIUM);
    container.set_margin_top(config::SPACING_MEDIUM);
    container.set_margin_start(config::SPACING_MEDIUM);
    container.set_margin_end(config::SPACING_MEDIUM);
    container.set_margin_bottom(config::SPACING_MEDIUM);

    let methods = StringList::new(&["GET", "POST", "PUT", "PATCH", "DELETE"]);
    let method_dropdown = DropDown::new(Some(methods), gtk::Expression::NONE);

    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("https://api.example.com/endpoint"));
    url_entry.set_hexpand(true);

    let send_button = Button::with_label("Send");
    send_button.add_css_class("suggested-action");

    container.append(&method_dropdown);
    container.append(&url_entry);
    container.append(&send_button);

    (container, url_entry, method_dropdown, send_button)
}
