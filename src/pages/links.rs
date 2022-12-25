use yew::prelude::*;

#[function_component(Links)]
pub fn links() -> Html {
    log::info!("this is links page");

    html! {
        <h1>
            { "Link New" }
        </h1>
    }
}