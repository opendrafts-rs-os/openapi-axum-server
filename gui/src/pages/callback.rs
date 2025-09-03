use log::debug;
use web_sys::Url;
use crate::auth::{exchange_code_for_token, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_AUDIENCE, AuthContext};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument};
use crate::app::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Callback)]
pub fn callback() -> Html {

    let auth_ctx = use_context::<AuthContext>().expect("err");

    let navigator = use_navigator();

    {
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let href = window.location().href().unwrap();
            let url = Url::new(&href).unwrap();
            let code = url.search_params().get("code");

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

            if let Some(code) = code {
                wasm_bindgen_futures::spawn_local(async move {
                    match exchange_code_for_token(&code, &verifier, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_REDIRECT_URI, AUTH0_AUDIENCE).await {
                        Ok(token) => {
                            let access_token = &token.access_token.clone();
                            let _id_token = &token.id_token.clone();

                            auth_ctx.set_token(Some(access_token.clone()));

                            if let Some(nav) = navigator {
                                nav.push(&Route::Home);
                            } else {
                                debug!("No navigator");
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
            <p>{"Redirecting..."}</p>

        </>
    }
}
