use crate::router::Route;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    html! {
        <nav class={classes!(styles, "navbar")} role="navigation" aria-label="main navigation">
            <div class="navbar-menu">
                <div class="navbar-start">
                    <Link<Route> to={Route::RootPage} classes="navbar-item jc-identity">
                        <img class="jc-logo" src="/assets/favicon.png"/ >
                        <p>{"Joshua Carrasco Sousa"}</p>
                    </Link<Route>>
                </div>
            </div>

            <div class="navbar-end">
                <div class="navbar-item">
                    <Link<Route> to={Route::AboutPage}>{ "About" }</Link<Route>>
                </div>
                <div class="navbar-item">
                    {"/"}
                </div>
                <div class="navbar-item">
                    <Link<Route> to={Route::ProjectsPage}>{ "Projects" }</Link<Route>>
                </div>
                <div class="navbar-item">
                    {"/"}
                </div>
                <div class="navbar-item">
                    <Link<Route> to={Route::ExperiencePage}>{ "Experience" }</Link<Route>>
                </div>
                <div class="navbar-item">
                    {"/"}
                </div>
                <div class="navbar-item">
                    <Link<Route> to={Route::ContactPage}>{ "Contact Me" }</Link<Route>>
                </div>
            </div>
        </nav>
    }
}