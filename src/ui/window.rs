use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use glib;
use gtk::{Box, Orientation};
use std::thread;

use super::{request_bar, response_view, sidebar};

pub fn build(app: &Application) {
    let sidebar_content = sidebar::build();
    let main_content = Box::new(Orientation::Vertical, 0);

    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let (req_bar_container, url_entry, method_dropdown, send_button) = request_bar::build();
    main_content.append(&req_bar_container);

    let (resp_view_container, response_buffer) = response_view::build();
    main_content.append(&resp_view_container);

    let buffer = response_buffer.clone();
    let entry = url_entry.clone();

    send_button.connect_clicked(move |_| {
        let url = entry.text().to_string();

        if url.is_empty() {
            return;
        }

        let (sender, receiver) = glib::MainContext::channel(glib::Priority::DEFAULT);

        thread::spawn(move || {
            let result = reqwest::blocking::get(&url);

            let text_to_show = match result {
                Ok(response) => match response.json::<serde_json::Value>() {
                    Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_default(),
                    Err(_) => "Error: could not parse JSON".to_string(),
                },
                Err(e) => format!("Request Failed: {}", e),
            };

            let _ = sender.send(text_to_show);
        });

        let buffer_clone = buffer.clone();
        receiver.attach(None, move |text: String| {
            buffer_clone.set_text(&text);
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
