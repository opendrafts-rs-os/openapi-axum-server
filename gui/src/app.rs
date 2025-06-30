use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Home, Callback};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/callback")]
    Callback,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Callback => html! { <Callback /> },
        Route::NotFound => html! { <p>{ "404 - not found" }</p> },
    }
}
