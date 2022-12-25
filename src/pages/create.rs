use yew::prelude::*;

#[function_component(Create)]
pub fn create() -> Html {
    log::info!("this is create page");

    html! {
        <h1>
            { "Create New" }
        </h1>
    }
}