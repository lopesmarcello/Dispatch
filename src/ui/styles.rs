use crate::config;
use gtk::gdk::Display;
use gtk::{CssProvider, StyleContext};

fn get_app_css() -> String {
    format!(
        "
        .badge-get {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}
        .badge-post {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}
        .badge-put {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}
        .badge-delete {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}
        .badge-patch {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}
        .badge-default {{ background-color: {}; color: {}; border-radius: {}px; padding: {}px {}px; font-weight: {}; }}

        .sidebar .heading {{ font-weight: {}; font-size: {}px; opacity: {}; }}
    ",
        config::COLOR_GET, config::COLOR_FG_DARK, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::COLOR_POST, config::COLOR_FG_DARK, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::COLOR_PUT, config::COLOR_FG_DARK, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::COLOR_DELETE, config::COLOR_FG_LIGHT, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::COLOR_PATCH, config::COLOR_FG_DARK, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::COLOR_DEFAULT, config::COLOR_FG_DARK, config::BORDER_RADIUS_SMALL, config::PADDING_VERTICAL_SMALL, config::PADDING_HORIZONTAL_SMALL, config::FONT_WEIGHT_BOLD,
        config::FONT_WEIGHT_HEADING, config::FONT_SIZE_HEADING, config::OPACITY_HEADING
    )
}

pub fn load() {
    let provider = CssProvider::new();
    provider.load_from_data(&get_app_css());

    if let Some(display) = Display::default() {
        StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
