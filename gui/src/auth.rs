use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};
use web_sys::window;

use gloo_net::http::Request;
use serde::Deserialize;


pub const CLIENT_ID: &str = env!("CLIENT_ID");
pub const DOMAIN: &str = env!("DOMAIN");
pub const REDIRECT_URI: &str = env!("REDIRECT_URI");
pub const AUDIENCE: &str = env!("AUDIENCE");

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    // token_type, expires_in, id_token, refresh_token
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

    log::debug!("Generated code_verifier: {}", verifier);

    if let Some(storage) = web_sys::window()
        .and_then(|w| w.session_storage().ok().flatten())
    {
        match storage.set_item("code_verifier", &verifier) {
            Ok(_) => log::debug!("code_verifier saved to sessionStorage"),
            Err(e) => log::error!("Failed to save code_verifier: {:?}", e),
        }
    } else {
        log::error!("sessionStorage not available");
    }

    let url = format!(
        "https://{domain}/authorize?response_type=code\
         &client_id={client_id}&redirect_uri={redirect_uri}\
         &scope=openid%20profile%20email\
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
    client_id: &str,
    domain: &str,
    redirect_uri: &str,
    audience: &str,
) -> Result<TokenResponse, String> {
    let verifier = web_sys::window()
        .and_then(|w| w.session_storage().ok().flatten())
        .and_then(|s| s.get_item("code_verifier").ok().flatten())
        .ok_or("missing code_verifier in sessionStorage")?;

    let audience_url_encoding =  urlencoding::encode(audience);
    let redirect_url_url_encoding = urlencoding::encode(redirect_uri);
    let body = format!(
        "grant_type=authorization_code\
        &client_id={client_id}\
        &code={code}\
        &redirect_uri={redirect_url_url_encoding}\
        &code_verifier={verifier}\
        &audience={audience_url_encoding}",
    );

    let url = format!("https://{domain}/oauth/token");

    let builder = Request::post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| format!("{e}"))?;

    let response = builder
        .send()
        .await
        .map_err(|e| format!("{e}"))?;

    if !response.ok() {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "empty".into());
        return Err(format!("status: {status}: {text}"));
    }

    let token: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("JSON: {e}"))?;

    Ok(token)
}