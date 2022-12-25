use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::Links;
use crate::pages::Create;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/create")]
    Create,

    #[at("/links")]
    Links,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <h1>
                { "Home" }
            </h1>
        },
        Route::Create => html! {
            <Create>
            </Create>
        },
        Route::Links => html! {
            <Links>
            </Links>
        },
    }
}