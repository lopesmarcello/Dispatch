// Window Geometry
pub const WINDOW_DEFAULT_WIDTH: i32 = 950;
pub const WINDOW_DEFAULT_HEIGHT: i32 = 650;
pub const SIDEBAR_WIDTH_FRACTION: f64 = 0.25;
pub const MIN_SIDEBAR_WIDTH: f64 = 220.0;
pub const BREAKPOINT_WIDTH: f64 = 650.0;
pub const REQUEST_PANE_POSITION: i32 = 250;
pub const SIDEBAR_HISTORY_MIN_HEIGHT: i32 = 400;

// Editor Styles
pub const EDITOR_SCHEME_PREF_1: &str = "Adwaita-Dark";
pub const EDITOR_SCHEME_PREF_2: &str = "oblivion";
pub const EDITOR_SCHEME_PREF_3: &str = "classic";

// CSS Classes
pub const CLASS_BADGE_GET: &str = "badge-get";
pub const CLASS_BADGE_POST: &str = "badge-post";
pub const CLASS_BADGE_PUT: &str = "badge-put";
pub const CLASS_BADGE_DELETE: &str = "badge-delete";
pub const CLASS_BADGE_PATCH: &str = "badge-patch";
pub const CLASS_BADGE_DEFAULT: &str = "badge-default";
pub const CLASS_SUCCESS: &str = "success";
pub const CLASS_ERROR: &str = "error";

// Sizing
pub const SPACING_NONE: i32 = 0;
pub const SPACING_EXTRA_SMALL: i32 = 6;
pub const SPACING_SMALL: i32 = 8;
pub const SPACING_MEDIUM: i32 = 12;

// CSS values
pub const COLOR_GET: &str = "#61affe";
pub const COLOR_POST: &str = "#49cc90";
pub const COLOR_PUT: &str = "#fca130";
pub const COLOR_DELETE: &str = "#f93e3e";
pub const COLOR_PATCH: &str = "#50e3c2";
pub const COLOR_DEFAULT: &str = "#999999";
pub const COLOR_FG_DARK: &str = "black";
pub const COLOR_FG_LIGHT: &str = "white";

pub const BORDER_RADIUS_SMALL: i32 = 4;
pub const PADDING_VERTICAL_SMALL: i32 = 2;
pub const PADDING_HORIZONTAL_SMALL: i32 = 6;

pub const FONT_WEIGHT_BOLD: &str = "bold";
pub const FONT_WEIGHT_HEADING: i32 = 800;
pub const FONT_SIZE_HEADING: i32 = 14;

pub const OPACITY_HEADING: f64 = 0.8;

pub fn get_badge_class(method: &str) -> &'static str {
    match method {
        "GET" => CLASS_BADGE_GET,
        "POST" => CLASS_BADGE_POST,
        "PATCH" => CLASS_BADGE_PATCH,
        "PUT" => CLASS_BADGE_PUT,
        "DELETE" => CLASS_BADGE_DELETE,
        _ => CLASS_BADGE_DEFAULT,
    }
}
