use crate::components::composite::nav_bar::NavBar;
use crate::components::composite::section_list::SectionList;
use crate::components::sections::learnera::Learnera;
use crate::components::sections::pepes_bubbles::PepesBubbles;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub query: String
}

#[derive(PartialEq)]
enum ProjectName {
    ProjectPepesBubbles,
    ProjectLearnera,
    All
}

#[styled_component(ProjectsPage)]
pub fn projects(props: &Props) -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    let project_name = parse_query(&props.query);

    let title = "Projects";
    let subtitle = if project_name == ProjectName::All { "A little bit about some projects I've worked on" } else { "" };
    let list = get_projects(project_name);

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

fn parse_query(query: &String) -> ProjectName {
    if query == "pepesbubbles" {
        ProjectName::ProjectPepesBubbles
    } else if query == "learnera" {
        ProjectName::ProjectLearnera
    } else {
        ProjectName::All
    }
}

fn get_projects(query: ProjectName) -> Vec<Html> {
    let mut ret: Vec<Html> = vec![];

    match query {
        ProjectName::ProjectPepesBubbles => {ret.push(html!{ <PepesBubbles/> }); ret} ,
        ProjectName::ProjectLearnera => {ret.push(html!{ <Learnera/> }); ret},
        ProjectName::All => {
            ret.push(html!{ <PepesBubbles summary={true}/> });
            ret.push(html!{ <Learnera summary={true}/> });
            ret
        }
    }
}