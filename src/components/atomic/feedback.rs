use yew::prelude::*;
use stylist::{yew::styled_component};

#[derive(PartialEq, Clone)]
pub enum FeedbackType {
    Success,
    Error,
    NotVisible
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub label: String,
    pub feedback_type: FeedbackType,
    pub close_button_callback: Option<Callback<()>>
}

#[styled_component(Feedback)]
pub fn feedback(props: &Props) -> Html {

    let mut onclick = Callback::from(|_| ());
    
    if let Some(cb) = props.close_button_callback.clone() {
        onclick = Callback::from(move |_| {
            cb.emit(())
        });
    }

    let class = if props.feedback_type == FeedbackType::Success {
        "is-success"
    } else if props.feedback_type == FeedbackType::Error {
        "is-danger"
    } else {""};
    
    html! {
        <article class={classes!("message", class)}>
            <div class="message-header">
                <p>{&props.title}</p>
                <button class="delete" aria-label="delete" {onclick}></button>
            </div>
            
            <div class="message-body">
                {&props.label}
            </div>
        </article>
    }
}