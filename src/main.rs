use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <h1> { "Hello Yew" } </h1>
    }
}

fn main() {
    yew::start_app::<App>();
}