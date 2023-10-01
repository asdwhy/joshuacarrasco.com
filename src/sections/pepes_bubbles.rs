use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub summary: Option<bool> // optional property, set true to summarize
}

#[function_component(PepesBubbles)]
pub fn pepes_bubbles(props: &Props) -> Html {
    let summary = match props.summary {
        Some(v) => v,
        None => false
    };

    html! {
        <div class={classes!("content")}>
            <Link<Route> to={Route::ProjectsPageName { name: "pepesbubbles".to_owned() }}><h4>{"Pepes Bubbles"}</h4></Link<Route>>
            <p>
            {"
                Pepes Bubbles is a group project I worked on from February 2022 to April 2022.
            "}
            </p>
            
            <p>
            {"
                Pepes Bubbles is a online storefront for a fictional Bubble Tea store called 'Pepes Bubbles'
                that we made up. The name/theme of the storefront was arbitrary, as the project was mainly to
                exercise and demonstrate our web-development skills.              
            "}
            </p>

            if !summary {
                <p>
                {"
                    During the development of this project I worked as a the sole back-end developer
                    for our application. I implemented the REST API and GraphQL API with Node.js, and also worked 
                    on deployment of our web-app using Dockerized containers running various technologies
                    such as Nginx, MongoDB, Memcached, and Graylog.
                "}
                </p>
                <p>{"See an extensive description of this project in the project repository."}</p>
            }

            <a href="https://github.com/asdwhy/pepes-bubbles">{"Github Repository for Pepes Bubbles"}</a>
        </div>
    }
}