use yew::prelude::*;

#[function_component(Bio)]
pub fn bio() -> Html {
    html! {
        <div class="bg-white w-full mb-10" id="about">
            <h1 class="text-3xl">{ "Hi! My name is Oisin. 👋"}</h1>
            <p class="">{ "I'm a software engineer from Ireland." }</p>
            <p class="">{ "I'm interested in DevOps, programming languages & game development." }</p>
        </div>
    }
}
