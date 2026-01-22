use gtk::prelude::*;
use std::{rc::Rc, thread};

use glib::Sender;

use crate::{
    api, config,
    database::Database,
    models::Method,
    state::Action,
    ui::{helpers::set_syntax_highlighting, sidebar, widgets::WindowWidgets},
};

pub fn handle_action(
    action: Action,
    w: &WindowWidgets,
    db: &Rc<Database>,
    sender: &Sender<Action>,
) {
    match action {
        Action::UpdateUrl(url) => w.url_entry.set_text(&url),
        Action::UpdateMethod(method) => w.method_dropdown.set_selected(method.to_index()),
        Action::UpdateBody(body) => w.request_body_buffer.set_text(&body),
        Action::UpdateHeaders(headers) => w.headers_editor.set_data(headers),

        Action::NewRequest => {
            w.url_entry.set_text("");
            w.method_dropdown.set_selected(0);
            w.request_body_buffer.set_text("");
            w.headers_editor.clear();
            w.response_buffer.set_text("");
            w.response_headers_buffer.set_text("");
            w.status_label.set_text("-");
            w.status_label.remove_css_class(config::CLASS_SUCCESS);
            w.status_label.remove_css_class(config::CLASS_ERROR);
        }

        Action::ClearHistory => {
            let _ = db.clear_history();
            while let Some(row) = w.history_list.first_child() {
                w.history_list.remove(&row);
            }
        }

        Action::LoadHistoryItem(id) => {
            if let Ok(item) = db.get_request_by_id(id) {
                sender.send(Action::UpdateUrl(item.url)).unwrap();
                if let Ok(m) = item.method.parse::<Method>() {
                    sender.send(Action::UpdateMethod(m)).unwrap();
                }
                sender.send(Action::UpdateBody(item.request_body)).unwrap();

                if let Ok(h) = serde_json::from_str(&item.request_headers) {
                    sender.send(Action::UpdateHeaders(h)).unwrap();
                }

                w.response_buffer.set_text(&item.response_body);
                w.response_headers_buffer.set_text(&item.response_headers);
                w.status_label.set_text(&item.status);
                w.time_label.set_text(&item.time);
                w.size_label.set_text(&item.size);

                set_syntax_highlighting(&w.response_buffer, &item.response_headers);

                if item.status.starts_with("2") {
                    w.status_label.add_css_class(config::CLASS_SUCCESS);
                    w.status_label.remove_css_class(config::CLASS_ERROR);
                } else {
                    w.status_label.add_css_class(config::CLASS_ERROR);
                    w.status_label.remove_css_class(config::CLASS_SUCCESS);
                }
            }
        }

        Action::SendRequest => {
            let url = w.url_entry.text().to_string();
            if url.is_empty() {
                return;
            }

            let method = Method::from_index(w.method_dropdown.selected());
            let (start, end) = w.request_body_buffer.bounds();
            let body = w.request_body_buffer.text(&start, &end, true).to_string();
            let headers = w.headers_editor.get_data();

            sender.send(Action::RequestStarted).unwrap();

            let tx = sender.clone();
            let method_clone = method.clone();
            let url_clone = url.clone();
            let body_clone = body.clone();
            let headers_clone = headers.clone();

            thread::spawn(move || {
                let result =
                    api::perform_request(method_clone, &url_clone, &body_clone, headers_clone);
                tx.send(Action::RequestCompleted(result)).unwrap();
            });
        }

        Action::RequestStarted => {
            w.spinner.set_visible(true);
            w.spinner.start();
            w.status_label.set_text("Sending...");
            w.status_label.remove_css_class(config::CLASS_ERROR);
            w.status_label.remove_css_class(config::CLASS_SUCCESS);
        }

        Action::RequestCompleted(result) => {
            w.spinner.stop();
            w.spinner.set_visible(false);

            match result {
                Ok(res) => {
                    w.response_buffer.set_text(&res.body);
                    w.response_headers_buffer.set_text(&res.headers);
                    w.status_label.set_text(&res.status);
                    w.time_label.set_text(&res.time);
                    w.size_label.set_text(&res.size);

                    if res.status_code >= 200 && res.status_code < 300 {
                        w.status_label.add_css_class(config::CLASS_SUCCESS);
                        w.status_label.remove_css_class(config::CLASS_ERROR);
                    } else {
                        w.status_label.add_css_class(config::CLASS_ERROR);
                        w.status_label.remove_css_class(config::CLASS_SUCCESS);
                    }

                    let url = w.url_entry.text().to_string();
                    let method = Method::from_index(w.method_dropdown.selected());
                    let (start, end) = w.request_body_buffer.bounds();
                    let body = w.request_body_buffer.text(&start, &end, true).to_string();
                    let headers = w.headers_editor.get_data();
                    let headers_json = serde_json::to_string(&headers).unwrap_or_default();

                    if let Ok(id) = db.save_exchange(
                        method.as_str(),
                        &url,
                        &body,
                        &headers_json,
                        &res.body,
                        &res.headers,
                        &res.status,
                        &res.time,
                        &res.size,
                    ) {
                        sender.send(Action::HistorySaved(id)).unwrap();
                    }
                }
                Err(api::ApiError::RequestFailed(msg)) => {
                    w.status_label.set_text("Error");
                    w.status_label.add_css_class(config::CLASS_ERROR);
                    w.response_buffer.set_text(&msg);
                }
            }
        }

        Action::HistorySaved(id) => {
            // Add to sidebar
            let url = w.url_entry.text().to_string();
            let method = Method::from_index(w.method_dropdown.selected());
            sidebar::add_history_row(&w.history_list, method.as_str(), &url, id);
        }
    }
}
