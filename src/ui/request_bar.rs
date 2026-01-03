use gtk::prelude::*;
use gtk::{Box, Button, DropDown, Entry, Orientation, StringList};

pub fn build() -> Box {
    // a horizontal box with 12px spacing
    let container = Box::new(Orientation::Horizontal, 12);
    container.set_margin_top(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

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

    container
}
