use crate::ui::key_value_editor::KeyValueEditor;
use gtk::{DropDown, Entry, Label, ListBox, Spinner};
use sourceview5::Buffer;

#[derive(Clone)]
pub struct WindowWidgets {
    pub url_entry: Entry,
    pub method_dropdown: DropDown,
    pub request_body_buffer: Buffer,
    pub response_buffer: Buffer,
    pub response_headers_buffer: Buffer,
    pub status_label: Label,
    pub time_label: Label,
    pub size_label: Label,
    pub spinner: Spinner,
    pub headers_editor: KeyValueEditor,
    pub history_list: ListBox,
    pub collections_list: ListBox,
    pub send_button: gtk::Button,
    pub new_request_btn: gtk::Button,
    pub clear_history_btn: gtk::Button,
    pub new_collection_btn: gtk::Button,
}
