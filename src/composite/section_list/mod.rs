use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub subtitle: String,
    pub sections: Vec<Html>
}

#[styled_component(SectionList)]
pub fn section_list(props: &Props) -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={classes!(styles, "content")}>
            <section class="section section-list">
                <h1 class="title">{&props.title}</h1>
                
                if &props.subtitle != "" {
                    <h3 class="subtitle">
                        {&props.subtitle}
                    </h3>
                }

                {props.sections.clone()}
            </section>
        </div>
    }
}
