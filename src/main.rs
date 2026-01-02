use adw::prelude::*;
use adw::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.github.lopesmarcello.dispatch")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dispatch")
        .default_width(800)
        .default_width(600)
        .build();

    window.present();
}
