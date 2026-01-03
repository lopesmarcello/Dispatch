use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
    prelude::*,
};
use gtk::{Box, Label, Orientation};

use super::{request_bar, response_view, sidebar};

pub fn build(app: &Application) {
    let sidebar_content = sidebar::build();
    let main_content = Box::new(Orientation::Vertical, 0);

    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let request_bar_widget = request_bar::build();
    main_content.append(&request_bar_widget);

    let response_view_widget = response_view::build();
    main_content.append(&response_view_widget);

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
