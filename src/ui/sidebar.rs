use adw::{ViewStack, ViewSwitcher};
use gtk::{Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, prelude::*};

use crate::{config, ui::helpers::add_box_margins};

pub struct SidebarWidgets {
    pub history_list: ListBox,
    pub collections_list: ListBox,
    pub clear_history_btn: Button,
    pub new_request_btn: Button,
    pub new_collection_btn: Button,
}

pub fn build() -> (Box, SidebarWidgets) {
    let container = Box::new(Orientation::Vertical, config::SPACING_NONE);

    let stack = ViewStack::new();
    stack.set_vexpand(true);

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
    container.append(&history_toolbar);

    let history_list = ListBox::new();
    history_list.add_css_class("navigation-sidebar");

    let history_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&history_list)
        .vexpand(true)
        .build();

    history_box.append(&history_toolbar);
    history_box.append(&history_scrolled);

    let history_page = stack.add_titled(&history_box, Some("history"), "History");
    history_page.set_icon_name(Some("document-open-recent-symbolic"));

    // Collections
    let collections_box = Box::new(Orientation::Vertical, 0);

    let col_toolbar = Box::new(Orientation::Horizontal, config::SPACING_SMALL);
    add_box_margins(&col_toolbar, config::SPACING_MEDIUM);

    let col_label = Label::new(Some("Folders"));
    col_label.add_css_class("heading");
    col_label.set_hexpand_set(true);
    col_label.set_xalign(0.0);

    let new_collection_btn = Button::builder()
        .icon_name("folder-new-symbolic")
        .css_classes(vec!["flat".to_string()])
        .tooltip_markup("New Collections")
        .build();

    col_toolbar.append(&col_label);
    col_toolbar.append(&new_collection_btn);

    let collections_list = ListBox::new();
    collections_list.add_css_class("navigation-sidebar");

    let col_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&collections_list)
        .vexpand_set(true)
        .build();

    collections_box.append(&col_toolbar);
    collections_box.append(&col_scroll);

    let collections_page = stack.add_titled(&collections_box, Some("collections"), "Collections");
    collections_page.set_icon_name(Some("folder-symbolic"));

    let switcher = ViewSwitcher::builder()
        .stack(&stack)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();

    container.append(&switcher);
    container.append(&stack);

    (
        container,
        SidebarWidgets {
            history_list,
            collections_list,
            clear_history_btn,
            new_request_btn,
            new_collection_btn,
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

pub fn add_collection_row(list: &ListBox, name: &str, id: i64) {
    let row = ListBoxRow::new();
    let row_box = Box::new(Orientation::Horizontal, config::SPACING_MEDIUM);
    add_box_margins(&row_box, config::SPACING_MEDIUM);

    let icon = gtk::Image::from_icon_name("folder-symbolic");

    let name_label = Label::new(Some(name));
    name_label.set_hexpand(true);
    name_label.set_xalign(0.0);

    row_box.append(&icon);
    row_box.append(&name_label);

    row.set_child(Some(&row_box));
    row.set_widget_name(&id.to_string());

    list.append(&row);
}
