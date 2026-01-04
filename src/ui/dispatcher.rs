use crate::api;
use std::thread;

pub enum AppAction {
    SendRequest {
        method: String,
        url: String,
        body: String,
        sender: glib::Sender<api::RequestResult>,
    },
}

pub struct Dispatcher;

impl Dispatcher {
    pub fn dispatch(action: AppAction) {
        match action {
            AppAction::SendRequest {
                method,
                url,
                body,
                sender,
            } => thread::spawn(move || {
                let result = api::perform_request(&method, &url, &body);
                let _ = sender.send(result);
            }),
        };
    }
}
