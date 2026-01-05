// Window Geometry
pub const WINDOW_DEFAULT_WIDTH: i32 = 950;
pub const WINDOW_DEFAULT_HEIGHT: i32 = 650;
pub const SIDEBAR_WIDTH_FRACTION: f64 = 0.25;
pub const MIN_SIDEBAR_WIDTH: f64 = 220.0;
pub const BREAKPOINT_WIDTH: f64 = 650.0;

// Editor Styles
pub const EDITOR_SCHEME_PREF_1: &str = "Adwaita-Dark";
pub const EDITOR_SCHEME_PREF_2: &str = "oblivion";
pub const EDITOR_SCHEME_PREF_3: &str = "classic";

// CSS Classes for Method Badges
pub const CLASS_BADGE_GET: &str = "badge-get";
pub const CLASS_BADGE_POST: &str = "badge-post";
pub const CLASS_BADGE_PUT: &str = "badge-put";
pub const CLASS_BADGE_DELETE: &str = "badge-delete";
pub const CLASS_BADGE_PATCH: &str = "badge-patch";
pub const CLASS_BADGE_DEFAULT: &str = "badge-default";

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
