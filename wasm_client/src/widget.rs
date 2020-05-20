use yew::{html, Callback, Html, MouseEvent};

pub use self::table::*;

pub mod table;

pub fn button(id: impl AsRef<str>, text: impl AsRef<str>, onclick: Callback<MouseEvent>) -> Html {
    let id = id.as_ref();
    let text = text.as_ref();
    let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id);

    html! {
        <button id = id class = "mdc-button" onclick = onclick>
            <span class = "mdc-button__ripple"></span>
            { text }
            <script>{ mdc_init }</script>
        </button>
    }
}
