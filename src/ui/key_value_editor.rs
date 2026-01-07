use gtk::{Box, Entry, ScrolledWindow};
use gtk::{Button, prelude::*};
use std::{cell::RefCell, rc::Rc};

use crate::{config, ui::helpers::add_box_margins};

#[derive(Clone)]
pub struct KeyValueRow {
    pub container: Box,
    pub key_entry: Entry,
    pub value_entry: Entry,
}

#[derive(Clone)]
pub struct KeyValueEditor {
    pub container: Box,
    pub rows_box: Box,

    pub rows: Rc<RefCell<Vec<KeyValueRow>>>,
}

impl KeyValueEditor {
    pub fn new() -> Self {
        let container = Box::new(gtk::Orientation::Vertical, config::SPACING_NONE);

        let rows_box = Box::new(gtk::Orientation::Vertical, config::SPACING_EXTRA_SMALL);
        add_box_margins(&rows_box, config::SPACING_MEDIUM);

        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .child(&rows_box)
            .vexpand(true)
            .build();

        container.append(&scrolled);

        let add_btn = Button::with_label("Add Header");
        add_btn.add_css_class("suggested-action");
        add_btn.set_margin_bottom(config::SPACING_MEDIUM);
        add_btn.set_margin_start(config::SPACING_MEDIUM);
        add_btn.set_margin_end(config::SPACING_MEDIUM);

        container.append(&add_btn);

        let editor = KeyValueEditor {
            container,
            rows_box,
            rows: Rc::new(RefCell::new(Vec::new())),
        };

        let editor_clone = editor.clone();
        add_btn.connect_clicked(move |_| editor_clone.add_row("", ""));

        editor.add_row("", "");
        editor
    }

    pub fn add_row(&self, key: &str, value: &str) {
        let row_container = Box::new(gtk::Orientation::Horizontal, config::SPACING_EXTRA_SMALL);

        let key_entry = Entry::builder()
            .placeholder_text("Key")
            .hexpand(true)
            .text(key)
            .build();

        let value_entry = Entry::builder()
            .placeholder_text("Value")
            .hexpand(true)
            .text(value)
            .build();

        let del_btn = Button::builder()
            .icon_name("user-trash-symbolic")
            .css_classes(vec!["flat".to_string()])
            .build();

        row_container.append(&key_entry);
        row_container.append(&value_entry);
        row_container.append(&del_btn);

        self.rows_box.append(&row_container);

        let row_data = KeyValueRow {
            container: row_container.clone(),
            key_entry,
            value_entry,
        };

        self.rows.borrow_mut().push(row_data.clone());

        let rows_ref = self.rows.clone();
        let rows_box_ref = self.rows_box.clone();
        let container_ref = row_container.clone();

        del_btn.connect_clicked(move |_| {
            rows_box_ref.remove(&container_ref);

            rows_ref
                .borrow_mut()
                .retain(|row| row.container != container_ref)
        });
    }

    pub fn get_data(&self) -> Vec<(String, String)> {
        let mut data = Vec::new();

        for row in self.rows.borrow().iter() {
            let k = row.key_entry.text().to_string();
            let v = row.value_entry.text().to_string();
            if !k.is_empty() {
                data.push((k, v))
            }
        }
        data
    }

    pub fn set_data(&self, data: Vec<(String, String)>) {
        let mut rows = self.rows.borrow_mut();

        for row in rows.iter() {
            self.rows_box.remove(&row.container);
        }

        rows.clear();
        drop(rows);

        for (k, v) in data {
            self.add_row(&k, &v);
        }

        self.add_row("", "");
    }

    pub fn clear(&self) {
        self.set_data(Vec::new());
    }
}
