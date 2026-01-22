use gtk::{prelude::*, Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};

use crate::{config, ui::helpers::add_box_margins};

pub struct SidebarWidgets {
    pub history_list: ListBox,
    pub clear_history_btn: Button,
    pub new_request_btn: Button,
}

pub fn build() -> (Box, SidebarWidgets) {
    let container = Box::new(Orientation::Vertical, config::SPACING_NONE);

    let history_box = Box::new(Orientation::Vertical, 0);
    let history_toolbar = Box::new(Orientation::Horizontal, config::SPACING_SMALL);
    add_box_margins(&history_toolbar, config::SPACING_MEDIUM);

    let hist_label = Label::new(Some("Recent"));
    hist_label.add_css_class("heading");
    hist_label.set_hexpand(true);
    hist_label.set_xalign(0.0);

    let new_request_btn = Button::builder()
        .icon_name("document-new-symbolic")
        .css_classes(vec!["flat".to_string()])
        .tooltip_text("New Request")
        .build();

    let clear_history_btn = Button::builder()
        .icon_name("edit-delete-symbolic")
        .css_classes(vec!["flat".to_string()])
        .tooltip_text("Clear History")
        .build();

    history_toolbar.append(&hist_label);
    history_toolbar.append(&new_request_btn);
    history_toolbar.append(&clear_history_btn);

    let history_list = ListBox::new();
    history_list.add_css_class("navigation-sidebar");

    let history_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&history_list)
        .vexpand(true)
        .build();

    history_box.append(&history_toolbar);
    history_box.append(&history_scrolled);

    container.append(&history_box);

    (
        container,
        SidebarWidgets {
            history_list,
            clear_history_btn,
            new_request_btn,
        },
    )
}

pub fn add_history_row(list: &ListBox, method: &str, url: &str, id: i64) {
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

    row.set_widget_name(&id.to_string());

    list.append(&row);
}
