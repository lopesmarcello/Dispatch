use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use glib;
use gtk::{Box, Orientation};
use sourceview5::Buffer;

use super::dispatcher::{AppAction, Dispatcher};
use super::{request_bar, request_tabs, response_view, sidebar, status_bar};
use crate::api;

#[derive(Clone)]
struct WindowWidgets {
    url_entry: gtk::Entry,
    method_dropdown: gtk::DropDown,
    request_body_buffer: Buffer,
    response_buffer: Buffer,
    status_label: gtk::Label,
    time_label: gtk::Label,
    size_label: gtk::Label,
}

pub fn build(app: &Application) {
    let sidebar_content = sidebar::build();
    let main_content = Box::new(Orientation::Vertical, 0);

    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let (req_bar_container, url_entry, method_dropdown, send_button) = request_bar::build();
    main_content.append(&req_bar_container);

    let (req_tabs_widget, request_body_buffer) = request_tabs::build();

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

    let widgets = WindowWidgets {
        url_entry,
        method_dropdown,
        request_body_buffer,
        response_buffer,
        status_label: status_widget.status_label,
        time_label: status_widget.time_label,
        size_label: status_widget.size_label,
    };

    let w = widgets.clone();

    send_button.connect_clicked(move |_| {
        let url = w.url_entry.text().to_string();

        if url.is_empty() {
            return;
        }

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

        let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

        Dispatcher::dispatch(AppAction::SendRequest {
            method: method_str,
            url,
            body: body_text,
            sender,
        });

        let w_inner = w.clone();

        receiver.attach(None, move |res: api::RequestResult| {
            w_inner.response_buffer.set_text(&res.body);
            w_inner.status_label.set_text(&res.status);
            w_inner.time_label.set_text(&res.time);
            w_inner.size_label.set_text(&res.size);

            if res.is_error {
                w_inner.status_label.add_css_class("error");
                w_inner.status_label.remove_css_class("sucess");
            } else {
                w_inner.status_label.add_css_class("sucess");
                w_inner.status_label.remove_css_class("error");
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
