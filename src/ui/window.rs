use super::sidebar;
use crate::database;
use crate::state::Action;
use crate::ui::{layout, reducer};
use adw::{prelude::*, Application};
use glib;
use std::rc::Rc;

#[allow(deprecated)]
pub fn build(app: &Application) {
    // Build UI
    let (window, widgets) = layout::build_ui(app);

    // Init DB
    let db = Rc::new(database::Database::new().expect("Failed to init DB"));

    // Load Data
    if let Ok(history) = db.get_history() {
        for item in history.iter().rev() {
            sidebar::add_history_row(&widgets.history_list, &item.method, &item.url, item.id);
        }
    }

    // MVU Loop
    let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

    receiver.attach(
        None,
        glib::clone!(@strong widgets, @strong db, @strong sender => move |action| {
            reducer::handle_action(action, &widgets, &db, &sender);
            glib::ControlFlow::Continue
        }),
    );

    widgets
        .send_button
        .connect_clicked(glib::clone!(@strong sender => move |_| {
            sender.send(Action::SendRequest).unwrap();
        }));

    widgets
        .new_request_btn
        .connect_clicked(glib::clone!(@strong sender => move |_| {
            sender.send(Action::NewRequest).unwrap();
        }));

    widgets
        .clear_history_btn
        .connect_clicked(glib::clone!(@strong sender => move |_| {
            sender.send(Action::ClearHistory).unwrap();
        }));

    widgets
        .history_list
        .connect_row_activated(glib::clone!(@strong sender => move |_, row| {
            let id_str = row.widget_name();
            if let Ok(id) = id_str.parse::<i64>() {
                sender.send(Action::LoadHistoryItem(id)).unwrap();
            }
        }));

    window.present();
}
