use adw::HeaderBar;
use adw::prelude::*;
use gtk::{Box, Label, Orientation, prelude::BoxExt};

pub fn build() -> Box {
    let container = Box::new(Orientation::Vertical, 0);

    let header = HeaderBar::new();
    header.set_show_end_title_buttons(false);
    header.set_show_start_title_buttons(false);

    container.append(&header);

    let label = Label::new(Some("History & Collections"));
    label.set_vexpand(true);
    container.append(&label);

    container
}
