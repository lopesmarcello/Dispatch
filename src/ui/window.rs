use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use glib;
use gtk::{Box, Orientation};
use std::thread;

use super::dispatcher::{AppAction, Dispatcher};
use super::{request_bar, request_tabs, response_view, sidebar, status_bar};
use crate::api;

pub fn build(app: &Application) {
    let sidebar_content = sidebar::build();
    let main_content = Box::new(Orientation::Vertical, 0);

    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let (req_bar_container, url_entry, method_dropdown, send_button) = request_bar::build();
    main_content.append(&req_bar_container);

    let (req_tabs_widget, req_body_buffer) = request_tabs::build();

    let status_widget = status_bar::build();

    let (resp_view_container, response_buffer) = response_view::build();

    let response_area = Box::new(Orientation::Vertical, 0);
    response_area.append(&status_widget.container);
    response_area.append(&resp_view_container);

    let paned = gtk::Paned::new(Orientation::Vertical);
    paned.set_start_child(Some(&req_tabs_widget));
    paned.set_end_child(Some(&response_area));
    paned.set_position(250);
    paned.set_vexpand(true);

    main_content.append(&paned);

    let req_body_buffer = req_body_buffer.clone();
    let buffer = response_buffer.clone();
    let entry = url_entry.clone();
    let method_dropdown = method_dropdown.clone();

    let status_label = status_widget.status_label.clone();
    let time_label = status_widget.time_label.clone();
    let size_label = status_widget.size_label.clone();

    send_button.connect_clicked(move |_| {
        let url = entry.text().to_string();

        if url.is_empty() {
            return;
        }

        let selected_method = method_dropdown.selected();
        let method_str = match selected_method {
            0 => "GET",
            1 => "POST",
            2 => "PUT",
            3 => "PATCH",
            4 => "DELETE",
            _ => "GET",
        }
        .to_string();

        let (buffer_start, buffer_end) = req_body_buffer.bounds();
        let body_text = req_body_buffer
            .text(&buffer_start, &buffer_end, true)
            .to_string();

        let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

        Dispatcher::dispatch(AppAction::SendRequest {
            method: method_str,
            url,
            body: body_text,
            sender,
        });

        let buffer_clone = buffer.clone();
        let status_lbl = status_label.clone();
        let time_lbl = time_label.clone();
        let size_lbl = size_label.clone();

        receiver.attach(None, move |res: api::RequestResult| {
            buffer_clone.set_text(&res.body);
            status_lbl.set_text(&res.status);
            time_lbl.set_text(&res.time);
            size_lbl.set_text(&res.size);

            if res.is_error {
                status_lbl.add_css_class("error");
                status_lbl.remove_css_class("sucess");
            } else {
                status_lbl.add_css_class("sucess");
                status_lbl.remove_css_class("error");
            }

            glib::ControlFlow::Break
        });
    });

    let split_view = OverlaySplitView::builder()
        .sidebar(&sidebar_content)
        .content(&main_content)
        .sidebar_width_fraction(0.25)
        .min_sidebar_width(200.0)
        .build();

    let breakpoint = Breakpoint::new(BreakpointCondition::new_length(
        adw::BreakpointConditionLengthType::MaxWidth,
        600.0,
        adw::LengthUnit::Px,
    ));
    breakpoint.add_setter(&split_view, "collapsed", Some(&true.to_value()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dispatch")
        .default_width(900)
        .default_height(600)
        .content(&split_view)
        .build();

    window.add_breakpoint(breakpoint);
    window.present();
}
