use adw::prelude::*;
use adw::{
    Application, ApplicationWindow, Breakpoint, BreakpointCondition, HeaderBar, OverlaySplitView,
};
use gtk::{Box, Label, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("com.github.lopesmarcello.dispatch")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let sidebar_content = Box::new(Orientation::Vertical, 0);
    let sidebar_header = HeaderBar::new();
    sidebar_header.set_show_end_title_buttons(false);
    sidebar_header.set_show_start_title_buttons(false);
    sidebar_content.append(&sidebar_header);

    let list_placeholder = Label::new(Some("History & Collections"));
    list_placeholder.set_vexpand(true);
    sidebar_content.append(&list_placeholder);

    let main_content = Box::new(Orientation::Vertical, 0);
    let main_header = HeaderBar::new();
    main_content.append(&main_header);

    let editor_placeholder = Label::new(Some("Request Editor Area"));
    editor_placeholder.set_vexpand(true);
    main_content.append(&editor_placeholder);

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
        .default_width(800)
        .default_width(600)
        .content(&split_view)
        .build();

    window.add_breakpoint(breakpoint);

    window.present();
}
