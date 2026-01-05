use crate::config;
use crate::ui::helpers::add_box_margins;
use gtk::{Box, Label, Spinner};
use gtk::{Separator, prelude::*};

pub struct StatusBarWidgets {
    pub container: Box,
    pub status_label: Label,
    pub time_label: Label,
    pub size_label: Label,
    pub spinner: Spinner,
}
pub fn build() -> StatusBarWidgets {
    let container = Box::new(gtk::Orientation::Horizontal, config::SPACING_MEDIUM);
    add_box_margins(&container, config::SPACING_MEDIUM);
    container.set_margin_top(config::SPACING_SMALL);
    container.set_margin_bottom(config::SPACING_SMALL);

    let spinner = Spinner::new();
    spinner.set_visible(false);
    container.append(&spinner);

    // "200 OK"
    let status_label = Label::new(Some("-"));
    status_label.add_css_class("title-4");

    // "150ms"
    let time_label = Label::new(Some("- ms"));

    // "200 KB"
    let size_label = Label::new(Some("- KB"));

    // "[Status: 200 OK] | [Time: 150 ms"] | [Size: 200 KB]"

    fn add_pair(box_container: &Box, title: &str, value_label: &Label) {
        let label_title = Label::new(Some(&title));
        label_title.add_css_class("dim-label");
        box_container.append(&label_title);
        box_container.append(value_label);
    }

    add_pair(&container, "Status: ", &status_label);
    container.append(&Separator::new(gtk::Orientation::Vertical));

    add_pair(&container, "Time: ", &time_label);
    container.append(&Separator::new(gtk::Orientation::Vertical));

    add_pair(&container, "Size: ", &size_label);

    StatusBarWidgets {
        container,
        status_label,
        time_label,
        size_label,
        spinner,
    }
}
