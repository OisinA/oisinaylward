use chrono::prelude::*;
use experience::Experience;
use yew::prelude::*;

#[macro_use]
extern crate lazy_static;

use crate::bio::Bio;
use crate::contact::Contact;
use crate::experience::ExperienceList;

mod bio;
mod contact;
mod experience;

lazy_static! {
    static ref EXPERIENCE: Vec<Experience<'static>> = vec![
        Experience {
            title: "Software Development Engineer in DevOps",
            company: "Workday",
            company_link: "https://www.workday.com/",
            start_date: Utc.ymd(2021, 9, 27),
            end_date: None,
        },
        Experience {
            title: "Ex-Officio",
            company: "Netsoc",
            company_link: "https://netsoc.co",
            start_date: Utc.ymd(2021, 6, 1),
            end_date: None,
        },
        Experience {
            title: "Chairperson",
            company: "Netsoc",
            company_link: "https://netsoc.co",
            start_date: Utc.ymd(2020, 6, 1),
            end_date: Some(Utc.ymd(2021, 5, 31)),
        },
        Experience {
            title: "Technical Intern",
            company: "Teamwork",
            company_link: "https://teamwork.com",
            start_date: Utc.ymd(2020, 4, 1),
            end_date: Some(Utc.ymd(2021, 5, 31)),
        },
        Experience {
            title: "SysAdmin",
            company: "Netsoc",
            company_link: "https://netsoc.co",
            start_date: Utc.ymd(2018, 6, 1),
            end_date: Some(Utc.ymd(2020, 5, 31)),
        },
    ];
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="m-10">
            <Bio />

            <ExperienceList experiences={EXPERIENCE.to_vec()}/>

            <Contact />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
