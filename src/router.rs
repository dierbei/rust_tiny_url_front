use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::LinkTo;
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

    #[at("/404")]
    NotFound,

    #[at("/:code")]
    LinkTo { code: String },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <h1 class="text-3xl font-bold underline">
                { "Hello world!" }
            </h1>
        },
        Route::Create => html! {
            <Create />
        },
        Route::Links => html! {
            <Links />
        },
        Route::LinkTo { code } => html! {<LinkTo code = {code.clone()} />},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}