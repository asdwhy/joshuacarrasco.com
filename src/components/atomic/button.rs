use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Option<Callback<()>>
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let mut button_onclick = Callback::from(|_| ());

    if let Some(cb) = props.onclick.clone() {

        button_onclick = Callback::from(move |_| {
            cb.emit(())
        });
    }

    html! {
        <button class={classes!("button")} onclick={button_onclick}>{&props.label}</button>
    }
}