use crate::composite::nav_bar::NavBar;
use crate::composite::section_list::SectionList;
use crate::sections::caseware::CasewarePage;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub query: String
}

#[derive(PartialEq)]
enum ExperienceName {
    Caseware,
    All
}

#[styled_component(ExperiencePage)]
pub fn experience(props: &Props) -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    let experience_name = parse_query(&props.query);

    let title = "Work Experiences";
    let subtitle = if experience_name == ExperienceName::All { "Comments on some my work experiences" } else { "" };
    let list = get_experiences(experience_name);

    html! {
        <>
            <NavBar/>

            <div class={classes!(styles, "content")}>
                <SectionList    title={title}
                                subtitle={subtitle}
                                sections={list}/>
            </div>
        </>
    }
}


fn parse_query(query: &String) -> ExperienceName {
    if query == "caseware" {
        ExperienceName::Caseware
    } else {
        ExperienceName::All
    }
}

fn get_experiences(query: ExperienceName) -> Vec<Html> {
    let mut ret: Vec<Html> = vec![];

    match query {
        ExperienceName::Caseware => {ret.push(html!{ <CasewarePage/> }); ret},
        ExperienceName::All => {
            ret.push(html!{ <CasewarePage summary={true}/> });
            ret
        }
    }
}