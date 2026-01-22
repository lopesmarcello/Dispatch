use gtk::{Box, prelude::*};
use sourceview5::{Buffer, LanguageManager, prelude::BufferExt};

pub fn add_box_margins(target_box: &Box, size: i32) {
    target_box.set_margin_top(size);
    target_box.set_margin_bottom(size);
    target_box.set_margin_start(size);
    target_box.set_margin_end(size);
}

pub fn set_syntax_highlighting(buffer: &Buffer, headers_or_type: &str) {
    let lm = LanguageManager::default();
    let text = headers_or_type.to_lowercase();

    let lang_id = if text.contains("application/json") || text.contains("text/json") {
        "json"
    } else if text.contains("html") {
        "html"
    } else if text.contains("xml") {
        "xml"
    } else if text.contains("javascript") {
        "javascript"
    } else if text.contains("css") {
        "css"
    } else if text.contains("markdown") {
        "markdown"
    } else {
        "text"
    };

    if let Some(lang) = lm.language(lang_id) {
        buffer.set_language(Some(&lang));
    }
}
