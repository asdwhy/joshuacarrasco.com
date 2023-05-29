use crate::composite::nav_bar::NavBar;
use crate::composite::section_list::SectionList;
use crate::sections::learnera::Learnera;
use crate::sections::pepes_bubbles::PepesBubbles;
use crate::sections::path_tracer::PathTracer;
use crate::sections::other::Other;

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
    ProjectPathTracer,
    ProjectOther,
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
    } else if query == "pathtracer" {
        ProjectName::ProjectPathTracer
    } else if query == "other" {
        ProjectName::ProjectOther
    } else {
        ProjectName::All
    }
}

fn get_projects(query: ProjectName) -> Vec<Html> {
    let mut ret: Vec<Html> = vec![];

    match query {
        ProjectName::ProjectPepesBubbles => {ret.push(html!{ <PepesBubbles/> }); ret} ,
        ProjectName::ProjectLearnera => {ret.push(html!{ <Learnera/> }); ret},
        ProjectName::ProjectPathTracer => {ret.push(html!{ <PathTracer/> }); ret},
        ProjectName::ProjectOther => {ret.push(html!{ <Other/> }); ret},
        ProjectName::All => {
            ret.push(html!{ <PepesBubbles summary={true}/> });
            ret.push(html!{ <Learnera summary={true}/> });
            ret.push(html!{ <PathTracer summary={true}/> });
            ret.push(html!{ <Other summary={true}/> });
            ret
        }
    }
}

