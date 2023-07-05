use crate::composite::nav_bar::NavBar;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");


#[styled_component(AboutPage)]
pub fn about() -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    html! {
        <>
            <NavBar/>
            <div class={classes!(styles)}>
                <section class="section content">
                    <h1 class="title">{"About Me"}</h1>
                    <p>
                        {"
                            My name is Joshua Carrasco and I graduated from the University of Toronto
                            with a bachelors in Computer Science. I like music, computers, games, and making software!
                        "} 
                    </p>
                    <br/>
                    <strong>{"Statement of Originality"}</strong>
                    <p>
                        {"
                            I, Joshua E. Carrasco, certify that this portfolio and the included 
                            works are that of my own and any ideas or quotations from the work of 
                            others have been fully acknowledged. Please do not copy without my 
                            permission.
                        "}
                    </p>
                </section>
            </div>
        </>
    }
}