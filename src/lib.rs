mod atomic {
    pub mod text_input;
    pub mod button;
    pub mod text_area;
    pub mod feedback;
}

mod composite {
    pub mod content_list;
    pub mod nav_bar;
    pub mod section_list;
}

mod pages {
    pub mod root;
    pub mod projects;
    pub mod about;
    pub mod contact;
    pub mod experience;
}

mod sections {
    pub mod caseware;
    pub mod learnera;
    pub mod pepes_bubbles;
    pub mod path_tracer;
    pub mod other;
}

mod router;

use crate::router::{Route,switch};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {    
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    } 
}