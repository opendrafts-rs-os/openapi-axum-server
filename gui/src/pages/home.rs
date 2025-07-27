use yew::prelude::*;
pub use crate::auth::{login,  AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI};

#[function_component(Home)]
pub fn home() -> Html {
    let onclick = Callback::from(|_| login(AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI));

    html! {
        <div>
            <h1>{ "Strona główna" }</h1>
            <button onclick={onclick}>{ "Zaloguj przez Auth0" }</button>
        </div>
    }
}