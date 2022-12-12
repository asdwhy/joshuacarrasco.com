use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub summary: Option<bool> // optional property, set true to summarize
}

#[function_component(CasewarePage)]
pub fn caseware(props: &Props) -> Html {
    let summary = match props.summary {
        Some(v) => v,
        None => false
    };

    html! {
        <div class={classes!("content")}>
            <Link<Route> to={Route::ExperiencePageName { name: "caseware".to_owned() }}><h4>{"CaseWare International"}</h4></Link<Route>>

            <p>
            {"
                I worked at CaseWare International, a software solutions company for accounting, 
                auditing, financial, risk and governance professionals from January 2020 to December 2020.
            "}
            </p>

            <p>
            {"
                I worked as a application developer, working on CaseWare's Audit Template product for their
                Working Papers application for about 3 months, then I was transferred to CaseWare's cloud platform
                team to work on their new Cloud Platform 2.0  product.
            "}
            </p>

            <p>
            {"
                While on CaseWare's cloud platform team I worked as a full stack developer, working on new 
                stories, and features. We used Java with the Spring Boot framework as our back-end, and Angular
                as our front-end framework.
            "}
            </p>

            if !summary {
                <br/>
                <p>{"Below are some artifacts from my time working at CaseWare,"}</p>
                
                <p><strong>{"Artifact 1"}</strong></p>
                <img src="/assets/contactUI.jpg"/>
                <p>
                {"
                    I am nearing the end of my 12 month co-op placement at CaseWare International, 
                    and I've been working hard on delivering the minimum viable product (MVP) for 
                    the new cloud product that CaseWare is investing in. As a fullstack developer 
                    there are a lot of technologies that I should have some level of understanding 
                    and competence in, and having that breadth I found was difficult. Advice that I 
                    acted on to try to overcome this difficulty is to try to become the subject 
                    matter expert on one area of our architecture. I decided to dive deep into the 
                    Angular framework and our front end technologies, and I was able to deliver 
                    several features into the product. 
                "}
                </p>

                <img src="/assets/dlq-architecture.jpg"/>
                <p>
                {"
                    Aside from the front end architecture that I have decided to specialized 
                    in, I have also been assigned architecture design tasks. I have presented 
                    such designs in meetings with the members of the wider CaseWare team. An 
                    example architecture that I have designed is the architecture shown in the 
                    first picture. It is a high level design and leaves out some implementation 
                    details. 
                "}
                </p>
                <p>
                {"
                    Now that I am mainly focusing on working on architecture design tasks, 
                    and so working on removing tech debt from our product. I am thinking back to 
                    the past 4 months of development to see if there's any instances I wrote code 
                    that could be considered tech debt. If I were to participate in the past 4 months 
                    of development again, I would try to be more prudent to ensure that the code I am 
                    writing would be easy to understand and not have to be changed to remove future tech 
                    debt. 
                "}
                </p>
                <p>
                {"Last updated 12/22/2020"}
                </p>

                <p><strong>{"Artifact 2"}</strong></p>

                <p>
                {"
                    I have been on my co-op placement at CaseWare International for 8 months now and a lot 
                    has changed in the industry due to Covid-19. Within the company I changed teams as a result 
                    of this change and my responsibilities have changed to that of a fullstack developer. 
                    The future seems to be cloud computing for CaseWare and I am now working as a part of the 
                    Cloud Platform team. CaseWare currently has a working cloud platform accounting solution 
                    but it has scalability issues with larger clients, due to this we are developing a new 
                    solution and I am a part development for the minimum viable product. Since the product is 
                    not yet released there are rules of confidentiality in place and I cannot share much about 
                    the product at this time. To solve the scalability issues, we are using a new software 
                    architecture described in the first picture. I am working on developing microservices to 
                    keep data and perform actions on the abstractions: users and entities. As a fullstack 
                    developer I also have to work with the client side, and I am working on UIs like that of 
                    the second picture. I am working developing UIs to interact with the server and 
                    microservices I am involved with. As a result of my involvements in development in 
                    the past several sprints, the team has picked up velocity and we are projected to 
                    finish the MVP ahead of schedule. Since my role has changed from that of a software 
                    developer to a fullstack developer, the technical stack that I work with has increased 
                    in difficulty and required level of understanding drastically. My skills of research 
                    and accelerated learning has increased as well. If I could participate in development 
                    made in these past 4 months again, I would try to learn one technology to level of 
                    competence that I am confident in, instead of trying to learn several technologies at 
                    the same time, but at a slower pace. As an onboarding fullstack developer it is common 
                    to be overwhelmed trying to learn all the different technologies, and with that 
                    change I would have been more confident in myself and that might have resulted in faster 
                    learning. 
                "}
                </p>
                <p>
                {"Last updated 8/31/2020"}
                </p>

                <p><strong>{"Artifact 3"}</strong></p>
                
                <div>
                    <img src="/assets/superdialog.jpg" height="400" width="400"/>
                    <img src="/assets/docprop.jpg" height="340" width="340"/>
                </div>
                
                <p>
                {"
                    After joining CaseWare International in the final phases of development for a new version of 
                    the Audit product, I fixed numerous bugs across several dialog boxes. The team had recently 
                    made a huge overhaul to the dialogs and as a result several bugs had been discovered. 
                    These screenshots showcase a few of the dialogs I worked on. I made several UI changes to 
                    these dialogs and fixed several logic errors in the code. In my efforts to fix bugs in 
                    these dialogs I took advantage of my research skills and was able to identify key 
                    information of internal or external documentation which allowed me to complete these 
                    assignments in my 2 week time restraints. The end result of my efforts was a product that 
                    worked properly after a huge code refactoring and overhaul. After fixing these several 
                    issues in time for the set release date of the new Audit product I developed my research 
                    and time management skills which will enable me to work more effectively in the future. 
                    I also learned new technologies and programming languages while working on these types of 
                    bugs. If I could go back in time and fix these same bugs now with the knowledge I now 
                    posses, I would try to make my code changes to a minimal level thus easier to read for 
                    code reviewers. 
                "}
                </p>
                <p>
                {"Last updated 4/25/2020"}
                </p>

            
            }
        </div>
    }
}