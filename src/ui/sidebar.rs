use gtk::{Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, prelude::*};

use crate::{config, ui::helpers::add_box_margins};

pub struct SidebarWidgets {
    pub list_box: ListBox,
}

pub fn build() -> (Box, SidebarWidgets) {
    let container = Box::new(Orientation::Vertical, config::SPACING_NONE);

    let header_box = Box::new(Orientation::Horizontal, config::SPACING_NONE);
    add_box_margins(&header_box, config::SPACING_MEDIUM);

    let title = Label::new(Some("History"));
    title.add_css_class("heading");
    title.set_hexpand(true);
    title.set_xalign(0.0);

    let clear_btn = Button::builder()
        .icon_name("edit-delete-symbolic")
        .css_classes(vec!["flat".to_string()])
        .tooltip_text("Clear History")
        .build();

    header_box.append(&title);
    header_box.append(&clear_btn);
    container.append(&header_box);

    let list_box = ListBox::new();
    list_box.add_css_class("navigation-sidebar");

    add_history_row(&list_box, "GET", "https://httpbin.org/get");
    add_history_row(&list_box, "POST", "https://httpbin.org/post");
    add_history_row(&list_box, "DELETE", "https://httpbin.org/delete");

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(config::SIDEBAR_HISTORY_MIN_HEIGHT)
        .child(&list_box)
        .vexpand(true)
        .build();

    container.append(&scrolled);

    (container, SidebarWidgets { list_box })
}

fn add_history_row(list: &ListBox, method: &str, url: &str) {
    let row = ListBoxRow::new();
    let row_box = Box::new(Orientation::Horizontal, config::SPACING_MEDIUM);
    add_box_margins(&row_box, config::SPACING_MEDIUM);

    let method_label = Label::new(Some(method));
    method_label.add_css_class(config::get_badge_class(method));

    let url_label = Label::new(Some(url));
    url_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    url_label.set_hexpand_set(true);
    url_label.set_xalign(0.0);

    row_box.append(&method_label);
    row_box.append(&url_label);

    row.set_child(Some(&row_box));

    list.append(&row);
}
