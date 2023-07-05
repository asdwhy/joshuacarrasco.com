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
                        <p class="is-size-6">{"Joshua Carrasco"}</p>
                    </Link<Route>>
                </div>
            </div>
            
            <nav class="breadcrumb navbar-item is-size-7-mobile" aria-label="breadcrumbs">
                <ul>
                    <li><Link<Route> to={Route::AboutPage}>{ "About" }</Link<Route>></li>
                    <li><Link<Route> to={Route::ProjectsPage}>{ "Projects" }</Link<Route>></li>
                    <li><Link<Route> to={Route::ExperiencePage}>{ "Experience" }</Link<Route>></li>
                    <li><Link<Route> to={Route::ContactPage}>{ "Contact Me" }</Link<Route>></li>
                </ul>
          </nav>

            
        </nav>
    }
}