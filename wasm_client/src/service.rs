use gloo::events::EventListener;
use wasm_bindgen::JsValue;
use web_sys::Event;

use crate::utils::window;

pub struct HistoryService;

impl HistoryService {
    pub fn change_url(url: impl AsRef<str>) {
        window()
            .history()
            .expect("Can't get history")
            .push_state_with_url(&JsValue::NULL, "", Some(url.as_ref()))
            .expect("Can't push state with url");
    }

    #[must_use]
    pub fn subscribe_to_back(callback: impl FnMut(&Event) + 'static) -> EventListener {
        EventListener::new(&window(), "popstate", callback)
    }
}
