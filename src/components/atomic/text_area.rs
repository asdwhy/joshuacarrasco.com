use yew::prelude::*;
use stylist::{yew::styled_component};
use web_sys::{HtmlInputElement};
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
    pub onchange: Callback<String>
}

#[styled_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let onchange_handler = props.onchange.clone();

    let onchange_cb = Callback::from(move |event: Event| {
        let input = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>();

        onchange_handler.emit(input.value());
    });

    html! {
        <textarea           class="textarea"
                            onchange={onchange_cb} 
                            placeholder={props.placeholder.clone()}
                            />
    }
}