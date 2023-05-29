use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub summary: Option<bool> // optional property, set true to summarize
}

#[function_component(Learnera)]
pub fn learnera(props: &Props) -> Html {

    let summary = match props.summary {
        Some(v) => v,
        None => false
    };

    html! {
        <div class={classes!("content")}>
            <Link<Route> to={Route::ProjectsPageName { name: "learnera".to_owned() }}><h4>{"Learnera"}</h4></Link<Route>>
            <p>
            {"
                Project Learnera, originally named Project BFS, is a group project I worked on from May 2021 to August 2021.
                The name of our development team was Brute Force Solutions hence BFS, but we later changed the product name 
                to Learnera.
            "}
            </p>
            
            <p>
            {"
                Project Learnera is a collaboration project with The Bridge from "} 
                <a href="https://www.utsc.utoronto.ca/">{"UTSC"}</a>
                {". The goal of the project is to tackle the African Impact Challenge by 
                creating a web platform that combines a community and an E-Learning platform. 
                The goal of the African Impact Challenge is to build the Africa we want to 
                see, by investing in the continent's aspiring entrepreneurs-to-be. 
                The goal is to enable them to build market-creating innovations, 
                which tackle their country's biggest challenges with technology.                    
            "}
            </p>

            if !summary {
                <p>{"Features requested at start of development by the product owner:"}</p>
                <ul>
                    <li>{"Individual profile creations which use the data from the applications/registrations in an automated way, to guide each person’s user journey and recommended actions."}</li>
                    <li>{"Company profile creations with consent-provided access to uploaded information by the founding teams on pitch decks, financials, MCs, founding team, etc."}</li>
                    <li>{"Partner profile creations (investors, service providers, corporations) with relevant information about them."}</li>
                    <li>{"Enabling organic and guided interaction within this community."}</li>
                    <li>{"Virtual pre-recorded content (video/audio and visual) that can be consumed in a self-paced manner."}</li>
                    <li>{"Deliverable completion and submission."}</li>
                    <li>{"Calendar and scheduling features for participants in the incubation program."}</li>
                    <li>{"Video conferencing integration for live workshops/sessions."}</li>
                </ul>
            }

            <a href="https://github.com/asdwhy/learnera">{"Github Repository for Learnera"}</a>

        </div>
    }
}