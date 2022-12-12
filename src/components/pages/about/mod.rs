use crate::components::composite::nav_bar::NavBar;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");


#[styled_component(AboutPage)]
pub fn about() -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    

    html! {
        <div class={classes!(styles, "content")}>
            <NavBar/>
            <section class="section">
                <h1 class="title">{"About Me"}</h1>
                <p>
                    {"
                        My name is Joshua Carrasco Sousa and I'm a 4th year student at the 
                        University of Toronto in the Computer Science Co-op program and I
                        like making software!
                    "} 
                </p>
                <br/>
                <strong>{"Statement of originality"}</strong>
                <p>
                    {"
                        I, Joshua E. Carrasco Sousa, certify that this portfolio and the included 
                        works are that of my own and any ideas or quotations from the work of 
                        others have been fully acknowledged. Please do not copy without my 
                        permission.
                    "}
                </p>
            </section>
        </div>
    }
}