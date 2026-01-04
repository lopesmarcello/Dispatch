use adw::Application;
use adw::prelude::*;

mod api;
mod ui;

fn main() {
    let app = Application::builder()
        .application_id("com.github.lopesmarcello.dispatch")
        .build();

    app.connect_activate(ui::window::build);
    app.run();
}
