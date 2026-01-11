use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use glib;
use gtk::{Box, Orientation};
use sourceview5::Buffer;
use std::rc::Rc;
use std::thread;

use super::{request_bar, request_tabs, response_view, sidebar, status_bar};
use crate::models::Method;
use crate::state::Action;
use crate::ui::helpers::{set_syntax_highlightin, show_input_dialog};
use crate::ui::key_value_editor::KeyValueEditor;
use crate::ui::window;
use crate::{api, config, database};

#[derive(Clone)]
struct WindowWidgets {
    url_entry: gtk::Entry,
    method_dropdown: gtk::DropDown,
    request_body_buffer: Buffer,
    response_buffer: Buffer,
    response_headers_buffer: Buffer,
    status_label: gtk::Label,
    time_label: gtk::Label,
    size_label: gtk::Label,
    spinner: gtk::Spinner,
    headers_editor: KeyValueEditor,
    history_list: gtk::ListBox,
    collections_list: gtk::ListBox,
}

#[allow(deprecated)]
pub fn build(app: &Application) {
    // --- Layout Setup ---
    let (sidebar_content, sidebar_widgets) = sidebar::build();
    let main_content = Box::new(Orientation::Vertical, 0);

    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let (req_bar_container, url_entry, method_dropdown, send_button) = request_bar::build();
    main_content.append(&req_bar_container);

    let (req_tabs_widget, request_body_buffer, headers_editor) = request_tabs::build();

    let status_widget = status_bar::build();

    let (resp_view_container, response_buffer, response_headers_buffer) = response_view::build();

    let response_area = Box::new(Orientation::Vertical, 0);
    response_area.append(&status_widget.container);
    response_area.append(&resp_view_container);

    let paned = gtk::Paned::new(Orientation::Vertical);
    paned.set_start_child(Some(&req_tabs_widget));
    paned.set_end_child(Some(&response_area));
    paned.set_position(config::REQUEST_PANE_POSITION);
    paned.set_vexpand(true);

    main_content.append(&paned);

    let widgets = WindowWidgets {
        url_entry,
        method_dropdown,
        request_body_buffer,
        response_buffer,
        response_headers_buffer,
        status_label: status_widget.status_label,
        time_label: status_widget.time_label,
        size_label: status_widget.size_label,
        spinner: status_widget.spinner,
        headers_editor,
        history_list: sidebar_widgets.history_list.clone(),
        collections_list: sidebar_widgets.collections_list.clone(),
    };

    // --- Database Setup ---
    let db = Rc::new(database::Database::new().expect("Failed to init DB"));

    // Load history
    match db.get_history() {
        Ok(history) => {
            for item in history.iter().rev() {
                sidebar::add_history_row(&widgets.history_list, &item.method, &item.url, item.id);
            }
        }
        Err(e) => eprintln!("CRITICAL DB ERROR: {}", e),
    }

    // Load Collections
    if let Ok(cols) = db.get_collections() {
        for col in cols {
            sidebar::add_collection_row(&widgets.collections_list, &col.name, col.id);
        }
    }

    // --- Signal Handlers (All at top level!) ---
    let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

    // Sidebar Click (Load Request)
    let w = widgets.clone();
    let db_clone = db.clone();
    let sender_clone = sender.clone();

    // ---  Window Creation---
    let split_view = OverlaySplitView::builder()
        .sidebar(&sidebar_content)
        .content(&main_content)
        .sidebar_width_fraction(config::SIDEBAR_WIDTH_FRACTION)
        .min_sidebar_width(config::MIN_SIDEBAR_WIDTH)
        .build();

    let breakpoint = Breakpoint::new(BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        config::BREAKPOINT_WIDTH,
        adw::LengthUnit::Px,
    ));
    breakpoint.add_setter(&split_view, "collapsed", Some(&true.to_value()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dispatch")
        .default_width(config::WINDOW_DEFAULT_WIDTH)
        .default_height(config::WINDOW_DEFAULT_HEIGHT)
        .content(&split_view)
        .build();

    window.add_breakpoint(breakpoint);

    // Reducer
    receiver.attach(None, move |action| {
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
                let _ = db_clone.clear_history();
                while let Some(row) = w.history_list.first_child() {
                    w.history_list.remove(&row);
                }
            }

            Action::LoadHistoryItem(id) => {
                if let Ok(item) = db_clone.get_request_by_id(id) {
                    sender_clone.send(Action::UpdateUrl(item.url)).unwrap();
                    if let Ok(m) = item.method.parse::<Method>() {
                        sender_clone.send(Action::UpdateMethod(m)).unwrap();
                    }
                    sender_clone
                        .send(Action::UpdateBody(item.request_body))
                        .unwrap();

                    if let Ok(h) = serde_json::from_str(&item.request_headers) {
                        sender_clone.send(Action::UpdateHeaders(h)).unwrap();
                    }

                    w.response_buffer.set_text(&item.response_body);
                    w.response_headers_buffer.set_text(&item.response_headers);
                    w.status_label.set_text(&item.status);
                    w.time_label.set_text(&item.time);
                    w.size_label.set_text(&item.size);

                    set_syntax_highlightin(&w.response_buffer, &item.response_headers);

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
                    return glib::ControlFlow::Continue;
                }

                let method = Method::from_index(w.method_dropdown.selected());
                let (start, end) = w.request_body_buffer.bounds();
                let body = w.request_body_buffer.text(&start, &end, true).to_string();
                let headers = w.headers_editor.get_data();

                sender_clone.send(Action::RequestStarted).unwrap();

                let tx = sender_clone.clone();
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

                        if let Ok(id) = db_clone.save_exchange(
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
                            sender_clone.send(Action::HistorySaved(id)).unwrap();
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

            Action::CreateCollection(name) => {
                if let Ok(id) = db_clone.create_collection(&name) {
                    sidebar::add_collection_row(&w.collections_list, &name, id);
                }
            }
            _ => {}
        }
        glib::ControlFlow::Continue
    });

    // --- Connect Signals --

    let s = sender.clone();
    send_button.connect_clicked(move |_| {
        s.send(Action::SendRequest).unwrap();
    });

    let s = sender.clone();
    sidebar_widgets.new_request_btn.connect_clicked(move |_| {
        s.send(Action::NewRequest).unwrap();
    });

    let s = sender.clone();
    sidebar_widgets.clear_history_btn.connect_clicked(move |_| {
        s.send(Action::ClearHistory).unwrap();
    });

    let s = sender.clone();
    sidebar_widgets
        .history_list
        .connect_row_activated(move |_, row| {
            let id_str = row.widget_name();
            if let Ok(id) = id_str.parse::<i64>() {
                s.send(Action::LoadHistoryItem(id)).unwrap();
            }
        });

    let s = sender.clone();
    let window_clone = window.clone();
    sidebar_widgets
        .new_collection_btn
        .connect_clicked(move |_| {
            let s_inner = s.clone();
            show_input_dialog(&window_clone, "New Collection", move |name| {
                s_inner.send(Action::CreateCollection(name)).unwrap()
            });
        });

    window.present();
}
