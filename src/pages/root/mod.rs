use crate::composite::content_list::ContentList;
use crate::router::URL;
use crate::router::Route;

use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("styles.css");


#[styled_component(RootPage)]
pub fn root() -> Html {
    let styles = Style::new(STYLE_FILE).unwrap();

    let jcs: Vec<(String, URL)> = vec![
        ("About".to_owned(), URL::Internal(Route::AboutPage)),
        ("Projects".to_owned(), URL::Internal(Route::ProjectsPage)),
        ("Experiences".to_owned(), URL::Internal(Route::ExperiencePage)),
        ("Contact Me".to_owned(), URL::Internal(Route::ContactPage))
    ];

    let work: Vec<(String, URL)> = vec![
        ("CaseWare International".to_owned(), URL::Internal(Route::ExperiencePageName { name: "caseware".to_owned() }))
    ];

    let projects: Vec<(String, URL)> = vec![
        ("Learnera".to_owned(), URL::Internal(Route::ProjectsPageName { name: "learnera".to_owned() })),
        ("Pepes Bubbles (Online Storefront)".to_owned(), URL::Internal(Route::ProjectsPageName { name: "pepesbubbles".to_owned() })),
        ("Path Tracer".to_owned(), URL::Internal(Route::ProjectsPageName { name: "pathtracer".to_owned() })),
        ("Other Projects".to_owned(), URL::Internal(Route::ProjectsPageName { name: "other".to_owned() })),
    ];

    let other: Vec<(String, URL)> = vec![
        ("Resume".to_owned(), URL::ExternalTarget{ url: "/assets/resume.pdf".to_owned(), target: "_blank".to_owned()}),
        ("Github".to_owned(), URL::External("https://github.com/asdwhy".to_owned())),
        ("Source Code".to_owned(), URL::External("https://github.com/asdwhy/asdwhy.github.io".to_owned()))
    ];

    html! {
        <div class={styles}>
            <section class="section jc-root-top">
                <div class="hero-body">
                    <p class="title jc-root-identity is-size-5-mobile">
                        <img class="jc-logo" src="/assets/favicon.png"/ >
                        {"Joshua Carrasco"}
                    </p>

                    <p class="jc-root-subtitle">
                        {"Software Engineer"}
                    </p>
                </div>
            </section>
            
            <section class="section jc-root-bottom">
                <div class="columns">
                    <div class="column">
                        <ContentList title="JCS" content={jcs}/>
                        <ContentList title="EXPERIENCE" content={work}/>
                    </div>
                    <div class="column">
                        <ContentList title="PROJECTS" content={projects}/>
                    </div>
                    <div class="column">
                        <ContentList title="OTHER" content={other}/>
                    </div>
                </div>
            </section>
        </div>
    }
}