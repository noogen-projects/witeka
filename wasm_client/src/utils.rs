use wasm_bindgen::JsValue;
use web_sys::{Document, Window};

pub fn document() -> Document {
    window().document().expect("Can't get document")
}

pub fn window() -> Window {
    web_sys::window().expect("Can't get window")
}

pub fn location_path() -> String {
    window().location().pathname().expect("Getting path expect")
}

pub struct PageData {
    pub user: Option<String>,
    pub wiki: Option<String>,
    pub path: String,
}

impl PageData {
    pub fn from_location_path() -> Self {
        let path = location_path();
        let parts: Vec<_> = path.splitn(3, |c| c == '/').collect();
        Self {
            user: parts.get(1).map(|user| user.to_string()),
            wiki: parts.get(2).map(|wiki| wiki.to_string()),
            path,
        }
    }
}

pub fn console_log(message: impl AsRef<str>) {
    web_sys::console::log_1(&JsValue::from_str(message.as_ref()))
}
