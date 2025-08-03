use yew::prelude::*;
use yew_router::prelude::*;
use crate::auth::{login, logout, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_LOGOUT_REDIRECT_URI, AuthContext};
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

    let token = use_state(|| None::<String>);
    let auth_ctx = AuthContext {
        token: token.clone(),
    };

    let login = yew::Callback::from(|_| login(AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI));
    let logout = yew::Callback::from(|_| logout(AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_LOGOUT_REDIRECT_URI));

    let button = if auth_ctx.is_sing_in() {
        html! { <button onclick={logout}>{"Sign out"}</button> }
    } else {
        html! { <button onclick={login}>{"Sign IN"}</button> }
    };

    html! {
        <ContextProvider<AuthContext> context={auth_ctx}>
            <p>{ "v0.1.1" }</p>
            { button }

            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AuthContext>>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Callback => html! { <Callback /> },
        Route::NotFound => html! { <p>{ "404 - not found" }</p> },
    }
}
