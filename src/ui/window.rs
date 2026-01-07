use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use glib;
use gtk::{Box, Orientation};
use sourceview5::Buffer;

use super::dispatcher::{AppAction, Dispatcher};
use super::{request_bar, request_tabs, response_view, sidebar, status_bar};
use crate::ui::key_value_editor::KeyValueEditor;
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
    sidebar_list: gtk::ListBox,
}

pub fn build(app: &Application) {
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
        sidebar_list: sidebar_widgets.list_box.clone(),
    };

    let db = match database::Database::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to init DB: {}", e);
            return;
        }
    };

    if let Ok(history) = db.get_history() {
        for item in history.iter().rev() {
            sidebar::add_history_row(&widgets.sidebar_list, &item.method, &item.url, item.id);
        }
    }

    let db_ref = std::rc::Rc::new(db);

    let w = widgets.clone();
    let db_clone = db_ref.clone();

    let w_sidebar = widgets.clone();
    sidebar_widgets
        .list_box
        .connect_row_activated(move |_, row| {
            let index = row.index();
            let (method, url) = match index {
                0 => ("GET", "https://httpbin.org/get"),
                1 => ("POST", "https://httpbin.org/post"),
                2 => ("DELETE", "https://httpbin.org/delete"),
                _ => ("GET", ""),
            };

            w_sidebar.url_entry.set_text(url);

            let method_index = match method {
                "GET" => 0,
                "POST" => 1,
                "PATCH" => 2,
                "PUT" => 3,
                "DELETE" => 4,
                _ => 0,
            };
            w_sidebar.method_dropdown.set_selected(method_index);
        });

    send_button.connect_clicked(move |_| {
        let url = w.url_entry.text().to_string();

        if url.is_empty() {
            return;
        }

        w.spinner.set_visible(true);
        w.spinner.start();
        w.status_label.set_text("Sending...");
        w.status_label.remove_css_class(config::CLASS_ERROR);
        w.status_label.remove_css_class(config::CLASS_SUCCESS);

        let selected_method = w.method_dropdown.selected();
        let method_str = match selected_method {
            0 => "GET",
            1 => "POST",
            2 => "PUT",
            3 => "PATCH",
            4 => "DELETE",
            _ => "GET",
        }
        .to_string();

        let (buffer_start, buffer_end) = w.request_body_buffer.bounds();
        let body_text = w
            .request_body_buffer
            .text(&buffer_start, &buffer_end, true)
            .to_string();

        let headers = w.headers_editor.get_data();

        let headers_json = serde_json::to_string(&headers).unwrap_or_default();
        if let Err(e) = db_clone.save_request(&method_str, &url, &body_text, &headers_json) {
            eprintln!("Failed to save history: {}", e);
        }
        sidebar::add_history_row(&w.sidebar_list, &method_str, &url, 0);

        let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

        Dispatcher::dispatch(AppAction::SendRequest {
            method: method_str,
            url,
            body: body_text,
            sender,
            headers,
        });

        let w_inner = w.clone();

        let w_sidebar = widgets.clone();
        let db_sidebar = db_ref.clone();

        sidebar_widgets
            .list_box
            .connect_row_activated(move |_, row| {
                let id_str = row.widget_name();
                if let Ok(id) = id_str.parse::<i64>() {
                    if let Ok(item) = db_sidebar.get_request_by_id(id) {
                        w_sidebar.url_entry.set_text(&item.url);

                        let method_idx = match item.method.as_str() {
                            "GET" => 0,
                            "POST" => 1,
                            "PATCH" => 2,
                            "PUT" => 3,
                            "DELETE" => 4,
                            _ => 0,
                        };

                        w_sidebar.method_dropdown.set_selected(method_idx);
                        w_sidebar.request_body_buffer.set_text(&item.body);

                        if let Ok(headers_vec) =
                            serde_json::from_str::<Vec<(String, String)>>(&item.headers)
                        {
                            w_sidebar.headers_editor.set_data(headers_vec);
                        } else {
                            w_sidebar.headers_editor.clear();
                        }
                    }
                }
            });

        let w_new = widgets.clone();
        sidebar_widgets.new_btn.connect_clicked(move |_| {
            w_new.url_entry.set_text("");
            w_new.method_dropdown.set_selected(0);
            w_new.request_body_buffer.set_text("");
            w_new.headers_editor.clear();

            w_new.response_buffer.set_text("");
            w_new.response_headers_buffer.set_text("");
            w_new.status_label.set_text("-");
            w_new.size_label.set_text("- KB");
            w_new.time_label.set_text("- ms");
            w_new.status_label.remove_css_class("success");
            w_new.status_label.remove_css_class("error");
        });

        let db_clear = db_ref.clone();
        let list_clear = widgets.sidebar_list.clone();
        sidebar_widgets.clear_btn.connect_clicked(move |_| {
            let _ = db_clear.clear_history();
            // Remove all rows
            while let Some(row) = list_clear.first_child() {
                list_clear.remove(&row);
            }
        });

        receiver.attach(None, move |res: api::RequestResult| {
            w_inner.spinner.stop();
            w_inner.spinner.set_visible(false);

            w_inner.response_buffer.set_text(&res.body);
            w_inner.response_headers_buffer.set_text(&res.headers);
            w_inner.status_label.set_text(&res.status);
            w_inner.time_label.set_text(&res.time);
            w_inner.size_label.set_text(&res.size);

            if res.is_error {
                w_inner.status_label.add_css_class(config::CLASS_ERROR);
                w_inner.status_label.remove_css_class(config::CLASS_SUCCESS);
            } else {
                w_inner.status_label.add_css_class(config::CLASS_SUCCESS);
                w_inner.status_label.remove_css_class(config::CLASS_ERROR);
            }

            glib::ControlFlow::Break
        });
    });

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
    window.present();
}
