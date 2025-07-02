use log::debug;
use yew::prelude::*;
use web_sys::Url;
use crate::auth::{exchange_code_for_token, AUDIENCE, CLIENT_ID, DOMAIN, REDIRECT_URI};

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

            debug!(" my: {:?}", CLIENT_ID);
            debug!(" my: {:?}", DOMAIN);
            debug!(" my: {:?}", REDIRECT_URI);
            debug!(" my: {:?}", code);



            if let Some(code) = code {
                wasm_bindgen_futures::spawn_local(async move {
                    match exchange_code_for_token(&code, CLIENT_ID, DOMAIN, REDIRECT_URI, AUDIENCE).await {
                        Ok(token) => {

                            if let Some(storage) = web_sys::window()
                                .and_then(|w| w.session_storage().ok().flatten())
                            {
                                let verifier = storage.get_item("code_verifier").ok().flatten();
                                debug!("code_verifier found in callback: {:?}", verifier);
                            }

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
