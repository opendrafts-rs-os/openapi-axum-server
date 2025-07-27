use log::debug;
use yew::prelude::*;
use web_sys::Url;
use crate::auth::{exchange_code_for_token,  AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_AUDIENCE};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument};

#[function_component(Callback)]
pub fn callback() -> Html {
    let access_token_state = use_state(|| None::<String>);
    let id_token_state = use_state(|| None::<String>);

    {
        let access_token_state = access_token_state.clone();
        let id_token_state = id_token_state.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let href = window.location().href().unwrap();
            let url = Url::new(&href).unwrap();
            let code = url.search_params().get("code");

            debug!(" my client_id: {:?}", AUTH0_CLIENT_ID);
            debug!(" my domain: {:?}", AUTH0_DOMAIN);
            debug!(" my redirect_uri: {:?}", AUTH0_REDIRECT_URI);
            debug!(" my audience: {:?}", AUTH0_AUDIENCE);
            debug!(" my code : {:?}", code);

            let verifier = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| {
                    doc.dyn_ref::<HtmlDocument>()?.cookie().ok()
                })
                .and_then(|cookie_str| {
                    cookie_str
                        .split(';')
                        .find_map(|entry| {
                            let mut parts = entry.trim().splitn(2, '=');
                            let key = parts.next()?;
                            let value = parts.next()?;
                            if key == "pkce_verifier" {
                                Some(value.to_string())
                            } else {
                                None
                            }
                        })
                }).unwrap();

            debug!("code_verifier from cookie: {:?}", verifier);

            if let Some(code) = code {
                wasm_bindgen_futures::spawn_local(async move {
                    match exchange_code_for_token(&code, &verifier, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_AUDIENCE).await {
                        Ok(token) => {
                            let access_token = &token.access_token.clone();
                            let id_token = &token.id_token.clone();

                            access_token_state.set(Some(token.access_token));
                            id_token_state.set(Some(token.id_token));

                            if let Some(window) = web_sys::window() {
                                if let Ok(Some(storage)) = window.session_storage() {
                                    let _ = storage.set_item("access_token", access_token);
                                    let _ = storage.set_item("id_token", id_token);
                                }
                            }

                        },
                        Err(e) => log::error!("Err: {}", e),
                    }
                });
            }

            || ()
        });
    }

    html! {
        <>
            <h1>{ "Callback Auth0" }</h1>
            {
                if access_token_state.is_some() {
                    html! { <p>{ "I'm logged" }</p> }
                } else {
                    html! { <p>{ "Logowanie..." }</p> }
                }
            }
        </>
    }
}
