use crate::router::URL;
use crate::router::Route;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;


const STYLE_FILE: &str = include_str!("styles.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub content: Vec<(String, URL)>
}

#[styled_component(ContentList)]
pub fn content_list(props: &Props) -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={classes!(styles, "columns")}>
            <div class="column is-one-third jc-content-list-title">
                <p>{props.title.clone()}</p>
            </div>
            <div class="column">
                <ul>
                    { content_to_list(props.content.clone()) }
                </ul>
            </div>
        </div>
    }
}


fn content_to_list(content: Vec<(String, URL)>) -> Html {
    let l = content.iter().map(|(text, url)| {
        
        html! {
            <li>
                if let URL::Internal(route) = url.clone() {
                    <Link<Route> to={route}>{ text }</Link<Route>>    
                }

                if let URL::External(route) = url.clone() {
                    <a href={route}>{text}</a>
                }

                if let URL::ExternalTarget { url, target } = url.clone() {
                    <a href={url} target={target}>{text}</a>
                }
            </li>
        }
    }).collect();

    l
}