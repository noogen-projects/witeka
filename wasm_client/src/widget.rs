use yew::{html, Callback, Html, MouseEvent};

pub use self::table::*;

pub mod table;

pub enum ButtonStyle {
    Text,
    Outlined,
    Raised,
    Unelevated,
}

impl ButtonStyle {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonStyle::Text => "mdc-button",
            ButtonStyle::Outlined => "mdc-button mdc-button--outlined",
            ButtonStyle::Raised => "mdc-button mdc-button--raised",
            ButtonStyle::Unelevated => "mdc-button mdc-button--unelevated",
        }
    }
}

pub fn button(id: impl AsRef<str>, text: impl AsRef<str>, style: ButtonStyle, onclick: Callback<MouseEvent>) -> Html {
    let id = id.as_ref();
    let text = text.as_ref();
    let mdc_init = format!("mdc.ripple.MDCRipple.attachTo(document.getElementById('{}'))", id);

    html! {
        <button id = id class = style.class() onclick = onclick>
            <span class = "mdc-button__ripple"></span>
            { text }
            <script>{ mdc_init }</script>
        </button>
    }
}

pub fn text_field(id: impl AsRef<str>, label: impl AsRef<str>) -> Html {
    let id = id.as_ref();
    let input_id = format!("{}-input", id);
    let label = label.as_ref();
    let mdc_init = format!("mdc.textField.MDCTextField.attachTo(document.getElementById('{}'))", id);

    html! {
        <div id = id class = "mdc-text-field mdc-text-field--outlined">
            <input id = input_id class = "mdc-text-field__input"/>
            <div class = "mdc-notched-outline">
                <div class = "mdc-notched-outline__leading"></div>
                <div class = "mdc-notched-outline__notch">
                    <label for = input_id class = "mdc-floating-label">{ label }</label>
                </div>
                <div class = "mdc-notched-outline__trailing"></div>
            </div>
            <script>{ mdc_init }</script>
        </div>
    }
}
