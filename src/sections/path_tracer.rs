use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub summary: Option<bool> // optional property, set true to summarize
}

#[function_component(PathTracer)]
pub fn path_tracer(props: &Props) -> Html {
    let summary = match props.summary {
        Some(v) => v,
        None => false
    };

    html! {
        <div class={classes!("content")}>
            <Link<Route> to={Route::ProjectsPageName { name: "pathtracer".to_owned() }}><h4>{"Path Tracer"}</h4></Link<Route>>
            <p>
            {"
                When I was learning Rust, I made a path tracer as a fun project.

                This is a brute force path tracer, not meant to produce images in real time, but rather
                realistic images.
            "}
            </p>

            <p>
            {"
                In university I took a computer graphics course and I really enjoyed it! I wanted
                to take it to the next level and re-wrote the project in Rust and 
                added some features from the classic "} 
                <a href="https://raytracing.github.io/">{"Ray Tracing in One Weekend Series"}</a>
                {"."}
            </p>
            
            if !summary {
                <p>{"Here are some images that I rendered:"}</p>
                <img src="/assets/scene.png"/>
                <img src="/assets/monkey+cornell.png"/>
                <br/>
            }

            <a href="https://github.com/asdwhy/jrpt">{"Github Repository for the path tracer"}</a>
        </div>
    }
}