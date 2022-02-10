use chrono::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Experience<'a> {
    pub title: &'a str,
    pub company: &'a str,
    pub company_link: &'a str,
    pub start_date: Date<Utc>,
    pub end_date: Option<Date<Utc>>,
}

#[derive(Properties, PartialEq)]
pub struct ExperienceProps<'a> {
    pub experiences: Vec<Experience<'a>>,
}

#[derive(Properties, PartialEq)]
pub struct ExperienceItemProps<'a> {
    pub experience: Experience<'a>,
}

#[function_component(ExperienceList)]
pub fn experience_list(ExperienceProps { experiences }: &ExperienceProps<'static>) -> Html {
    let present: Html = experiences
        .iter()
        .filter(|experience| experience.end_date.is_none())
        .map(|experience| {
            html! {
                <ExperienceItem experience={experience.clone()} />
            }
        })
        .collect();

    let past: Html = experiences
        .iter()
        .filter(|experience| experience.end_date.is_some())
        .map(|experience| {
            html! {
                <ExperienceItem experience={experience.clone()} />
            }
        })
        .collect();

    html! {
        <div class="bg-white w-full mb-10" id="experience">
            <h1 class="text-3xl mb-5 border-b-2 pb-2">{ "🚀 Experience"}</h1>

            <h2 class="text-xl pb-2">{ "Present" }</h2>
            <ul class="ml-5">
                { present }
            </ul>

            <h2 class="text-xl pb-2">{ "Past" }</h2>
            <ul class="ml-5">
                { past }
            </ul>
        </div>
    }
}

#[function_component(ExperienceItem)]
pub fn experience_item(ExperienceItemProps { experience }: &ExperienceItemProps<'static>) -> Html {
    let end_date = if let Some(date) = experience.end_date {
        date
    } else {
        Utc::now().date()
    };

    let duration = end_date - experience.start_date;
    let mut duration_str = String::new();
    if duration.num_days() >= 364 {
        let years = duration.num_days() / 364;
        duration_str += &format!("{} year{}", years, if years > 1 { "s" } else { "" });
        let months = (duration.num_days() % 364) / 30;
        if months != 0 {
            duration_str += &format!(
                " {} month{}",
                (duration.num_days() % 364) / 30,
                if months > 1 { "s" } else { "" }
            );
        }
    } else {
        let months = (duration.num_days() % 364) / 30;
        duration_str += &format!(
            "{} month{}",
            duration.num_days() / 30,
            if months > 1 { "s" } else { "" }
        );
    }

    html! {
    <li class="mb-5">
        <h2 class="text-2xl bold">{ experience.title } { " @ " } <a class="text-blue-600" href={ experience.company_link }>{ experience.company }</a></h2>
        <p>{ experience.start_date.format("%B %Y") } { " - " } { if experience.end_date.is_none() {
            "Present".to_string()
        } else {
            format!("{}", experience.end_date.unwrap().format("%B %Y"))
        }}</p>
        <p class="text-xs text-slate-500"> { duration_str } </p>
    </li>
    }
}
