use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub summary: Option<bool> // optional property, set true to summarize
}

#[function_component(Other)]
pub fn other(props: &Props) -> Html {
    let summary = match props.summary {
        Some(v) => v,
        None => false
    };

    html! {
        <div class={classes!("content")}>
            <Link<Route> to={Route::ProjectsPageName { name: "other".to_owned() }}><h4>{"Other Projects"}</h4></Link<Route>>
            <p>
                {"Some other projects I have worked on. I have worked on a couple smaller projects related to computer graphics,
                school projects, and more web applications that are unfortunately private and can't share."}
            </p>

            if !summary {
                <b>{"Terminal Instant Chat Messenger"}</b>
                <p>{
                    "One school project I worked on is a terminal based instant chat
                    messenger written entirely in C with e2e encryption using the OpenSSL library. I worked on this
                    project with 1 peer."
                }</p>
                <a href="https://github.com/asdwhy/cscd58-project">{"Github Repository for the project"}</a>
            }

            if !summary {
                <br/><br/>
                <b>{"Simple Holomorphic Dynamics Renderer"}</b>
                <p>{
                    "One small computer graphics related project I worked on is a renderer for some simple
                    holomorphic dynamical systems. I originally wanted to render any generic system but then
                    simplified the project to only render the Mandlebrot set, and Julia sets. 
                    Here are some videos that I rendered in this project:"}
                    <ul>
                        <li><a href="https://youtube.com/shorts/vWEBXcXfI4E?feature=share">{"Mandlebrot Set Animation"}</a></li>
                        <li><a href="https://youtu.be/4rgisnCUsd0">{"Julia Set Animation"}</a></li>
                    </ul>
                </p>
                <a href="https://github.com/asdwhy/holomorphic_dynamics">{"Github Repository for the project"}</a>
            }

            if !summary {
                <br/><br/>
                <b>{"Terminal Parametric Renderer"}</b>
                <p>{
                    "Another small graphics related project I worked on is a renderer for surfaces modelled by parametric 
                    equations. The renderer animates a rotating surface in the terminal. The program currently renders
                    a torus (donut shaped surface), but can render any surface with a given parametrization, 
                    unit normal vector field, and domain."}
                </p>
                <a href="https://github.com/asdwhy/parametric_renderer">{"Github Repository for the project"}</a>
            }
        </div>
    }
}