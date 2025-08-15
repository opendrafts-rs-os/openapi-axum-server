use yew::prelude::*;
use yew_router::prelude::*;
use crate::auth::{login, logout, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_LOGOUT_REDIRECT_URI, AuthContext};
use crate::srv::testauth::testauth;
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

    let act_login = yew::Callback::from(|_| login(AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI));
    let act_logout = yew::Callback::from(|_| logout(AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_LOGOUT_REDIRECT_URI));

    let token_handle = token.clone();
    let act_testauth = yew::Callback::from(move |_| {
        let tok: Option<String> = token_handle.as_ref().cloned();
        wasm_bindgen_futures::spawn_local(async move {
            match testauth("https://localhost:8080/api", tok).await {
                Ok(msg) => {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!("OK: {}", msg))
                        .unwrap();
                }
                Err(err) => {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message(&format!("Błąd: {}", err))
                        .unwrap();
                }
            }
        });
    });

    let button = if auth_ctx.is_sing_in() {
        html! { <button onclick={act_logout}>{"Sign out"}</button> }
    } else {
        html! { <button onclick={act_login}>{"Sign IN"}</button> }
    };

    html! {
        <ContextProvider<AuthContext> context={auth_ctx}>
            <p>{ "v0.1.1" }</p>
            { button }
            <button onclick={act_testauth}>{"Test auth"}</button>

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
