use crate::pages::root::RootPage;
use crate::pages::about::AboutPage;
use crate::pages::projects::ProjectsPage;
use crate::pages::contact::ContactPage;
use crate::pages::experience::ExperiencePage;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/")]
    RootPage,
    #[at("/about")]
    AboutPage,
    #[at("/projects")]
    ProjectsPage,
    #[at("/projects/:name")]
    ProjectsPageName { name: String },
    #[at("/contact")]
    ContactPage,
    #[at("/experience")]
    ExperiencePage,
    #[at("/experience/:name")]
    ExperiencePageName { name: String }
}

#[derive(Clone, PartialEq)]
pub enum URL {
    Internal(Route),    // route internally using Route enum
    External(String),
    ExternalTarget {
        url: String,
        target: String
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::RootPage => html! { <RootPage/> },
        Route::AboutPage => html! { <AboutPage/> },
        Route::ProjectsPage => html! { <ProjectsPage query={""}/> },
        Route::ProjectsPageName { name } => html! { <ProjectsPage query={name}/> },
        Route::ContactPage => html! { <ContactPage/> },
        Route::ExperiencePage => html! { <ExperiencePage query={""}/> },
        Route::ExperiencePageName { name } => html! { <ExperiencePage query={name}/> }
    }
}