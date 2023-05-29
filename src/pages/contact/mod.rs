use crate::composite::nav_bar::NavBar;
use crate::atomic::text_input::TextInput;
use crate::atomic::text_area::TextArea;
use crate::atomic::feedback::{Feedback,FeedbackType};

use reqwasm::http::Request;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use serde_json::json;
use std::{ops::Deref};

const STYLE_FILE: &str = include_str!("styles.css");

#[derive(Default, Clone)]
struct FormData {
    name: String,
    email: String,
    message: String
}

#[styled_component(ContactPage)]
pub fn contact() -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    let feedback_visibility = use_state(|| FeedbackType::NotVisible);
    let filled_flag = use_state(|| false);
    let state = use_state(|| FormData::default());

    let name_changed = {
        let filled_flag = filled_flag.clone();
        let state = state.clone();
        Callback::from(move |name: String| {
            
            state.set(
                FormData {
                    name: name.clone(), 
                    ..state.deref().clone()
                }
            );

            filled_flag.set(!name.is_empty() && !state.email.is_empty() && !state.message.is_empty());
        })
    };

    let email_changed = {
        let filled_flag = filled_flag.clone();
        let state = state.clone();
        Callback::from(move |email: String| {
            state.set(
                FormData {
                    email: email.clone(), 
                    ..state.deref().clone()
                }
            );

            filled_flag.set(!state.name.is_empty() && !email.is_empty() && !state.message.is_empty());
        })
    };
    
    let message_changed = {
        let filled_flag = filled_flag.clone();
        let state = state.clone();
        Callback::from(move |message: String| {
            state.set(
                FormData {
                    message: message.clone(), 
                    ..state.deref().clone()
                }
            );

            filled_flag.set(!state.name.is_empty() && !state.email.is_empty() && !message.is_empty());
        })
    };

    let onsubmit = {
        let state = state.clone();
        let feedback_visibility = feedback_visibility.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let body = json!({
                "username": state.deref().name,
                "email": state.deref().email,
                "message": state.deref().message
            });

            let feedback_visibility = feedback_visibility.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("https://fdxbjr1bvl.execute-api.us-east-2.amazonaws.com/default/SendEmail")
                .body(body.to_string())
                .send()
                .await
                .unwrap();
                
                if response.status() == 200 {
                    feedback_visibility.set(FeedbackType::Success);
                } else {
                    feedback_visibility.set(FeedbackType::Error);
                }
            });
        })
    };

    let on_close_feedback = {
        let feedback_visibility = feedback_visibility.clone();

        Callback::from(move |_: ()| {
            feedback_visibility.set(FeedbackType::NotVisible);
        })
    };

    html! {
        <>
            <NavBar/>
            <div class={classes!(styles, "content")}>
                <section class="section">
                    <h1 class="title">{"Contact Me"}</h1>
                    
                    <form {onsubmit}>
                        <div class="field">
                            <label class="label">{"Name"}</label>
                            <div class="control">
                                <TextInput onchange={name_changed} input_type="text" placeholder="John Smith"/>
                            </div>
                        </div>

                        <div class="field">
                            <label class="label">{"Email"}</label>
                            <div class="control">
                                <TextInput onchange={email_changed} input_type="email" placeholder="myemail@email.com"/>
                            </div>
                        </div>

                        <div class="field">
                            <label class="label">{"Message"}</label>
                            <div class="control">
                                <TextArea onchange={message_changed} placeholder="write anything"/>
                            </div>
                        </div>

                        <fieldset disabled={!*filled_flag}>
                        <div class="field">
                            <div class="control">
                                <button class="button is-link">{"Submit"}</button>
                            </div>
                        </div>
                        </fieldset>
                    </form>

                    if *feedback_visibility == FeedbackType::Success {
                        <Feedback title="Success" feedback_type={FeedbackType::Success} close_button_callback={on_close_feedback} label="I will get back to you as soon as possible"/>
                    } else if *feedback_visibility == FeedbackType::Error {
                        <Feedback title="Failure" feedback_type={FeedbackType::Error} close_button_callback={on_close_feedback} label="Something went wrong and your message couldn't send..."/>
                    }
                    
                </section>
            </div>
        </>
    }
}