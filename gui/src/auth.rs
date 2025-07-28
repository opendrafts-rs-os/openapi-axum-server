use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

use gloo_net::http::Request;
use log::debug;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::{window};

pub const AUTH0_CLIENT_ID: &str = env!("AUTH0_CLIENT_ID");
pub const AUTH0_DOMAIN: &str = env!("AUTH0_DOMAIN");
pub const AUTH0_REDIRECT_URI: &str = env!("AUTH0_REDIRECT_URI");
pub const AUTH0_AUDIENCE: &str = env!("AUTH0_AUDIENCE");

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: Option<String>,
}

pub(crate) fn generate_code_verifier() -> String {
    let verifier: [u8; 32] = rand::thread_rng().gen();
    URL_SAFE_NO_PAD.encode(verifier)
}

pub(crate) fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}

pub fn login(client_id: &str, domain: &str, redirect_uri: &str) {
    let verifier = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);

    //let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    //let challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    log::debug!("Generated code_verifier: {}", verifier);

    if let Some(document) = window().and_then(|w| w.document()) {
        let cookie_str = format!(
            "pkce_verifier={}; Path=/; Max-Age=300; SameSite=Lax",
            verifier
        );

        // `cookie` jest property na `document`
        document
            .dyn_ref::<web_sys::HtmlDocument>()
            .expect("Should be an HtmlDocument")
            .set_cookie(&cookie_str)
            .unwrap_or_else(|e| {
                log::error!("Failed to set cookie: {:?}", e);
            });
    }


    let url = format!(
        "https://{domain}/authorize?response_type=code\
         &client_id={client_id}&redirect_uri={redirect_uri}\
         &scope=openid%20name%20email%20nickname\
         &code_challenge={challenge}&code_challenge_method=S256"
    );

    if let Some(window) = window() {
        if let Ok(_location) = window.location().set_href(&url) {
            // OK, redirect succeeded
        } else {
            log::error!("Redirect failed");
        }
    } else {
        log::error!("window() not available");
    }
}

pub async fn exchange_code_for_token(
    code: &str,
    verifier: &str,
    client_id: &str,
    domain: &str,
    redirect_uri: &str,
    audience: &str,
) -> Result<TokenResponse, String> {
    let redirect_url_encoded = urlencoding::encode(redirect_uri);
    let audience_encoded = urlencoding::encode(audience);

    let body = format!(
        "grant_type=authorization_code\
        &client_id={client_id}\
        &code={code}\
        &redirect_uri={redirect_uri}\
        &code_verifier={verifier}\
        &audience={audience}",
        redirect_uri = redirect_url_encoded,
        audience = audience_encoded
    );
    debug!("body: {}", body);

    let url = format!("https://{}/oauth/token", domain);

    let builder = Request::post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| format!("Request build error: {e}"))?;

    let response = builder
        .send()
        .await
        .map_err(|e| format!("Request send error: {e}"))?;

    if !response.ok() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "empty".into());
        return Err(format!("Token exchange failed: status {status}: {text}"));
    }

    let token: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {e}"))?;

    debug!("{:?}", token);
    Ok(token)
}