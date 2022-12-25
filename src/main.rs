use yew::prelude::*;
use yew_router::prelude::*;
use router::{ Route, switch };

mod router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch <Route> render = { Switch::render(switch) } />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
