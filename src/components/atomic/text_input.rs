use yew::prelude::*;
use stylist::{yew::styled_component};
use web_sys::{HtmlInputElement};
use wasm_bindgen::JsCast;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_type: String,
    pub placeholder: String,
    pub onchange: Callback<String>
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange_handler = props.onchange.clone();

    let onchange_cb = Callback::from(move |event: Event| {
        let input = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>();

        onchange_handler.emit(input.value());
    });


    html! {
        <input              class="input"
                            type={props.input_type.clone()} 
                            onchange={onchange_cb} 
                            placeholder={props.placeholder.clone()}
                            />
    }
}