use gtk::{Box, prelude::WidgetExt};

pub fn add_box_margins(target_box: &Box, size: i32) {
    target_box.set_margin_top(size);
    target_box.set_margin_bottom(size);
    target_box.set_margin_start(size);
    target_box.set_margin_end(size);
}
