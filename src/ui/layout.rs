use adw::{
    prelude::*, Application, ApplicationWindow, Breakpoint, BreakpointCondition, OverlaySplitView,
};
use gtk::{Box, HeaderBar, Orientation};

use crate::{
    config,
    ui::{request_bar, request_tabs, response_view, sidebar, status_bar, widgets::WindowWidgets},
};

pub fn build_ui(app: &Application) -> (ApplicationWindow, WindowWidgets) {
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
        send_button,
        new_request_btn: sidebar_widgets.new_request_btn,
        clear_history_btn: sidebar_widgets.clear_history_btn,
    };

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

    (window, widgets)
}
