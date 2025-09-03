use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{window};
use yew::UseStateHandle;

pub const AUTH0_CLIENT_ID: &str = env!("AUTH0_CLIENT_ID");
pub const AUTH0_DOMAIN: &str = env!("AUTH0_DOMAIN");
pub const AUTH0_REDIRECT_URI: &str = env!("AUTH0_REDIRECT_URI");
pub const AUTH0_AUDIENCE: &str = env!("AUTH0_AUDIENCE");
pub const AUTH0_LOGOUT_REDIRECT_URI: &str = env!("AUTH0_LOGOUT_REDIRECT_URI");

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub token: UseStateHandle<Option<String>>,
}

impl AuthContext {
    pub fn is_sing_in(&self) -> bool {
        self.token.is_some()
    }
    pub fn set_token(&self, value: Option<String>) {
        self.token.set(value);
    }
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

    if let Some(document) = window().and_then(|w| w.document()) {
        let cookie_str = format!(
            "pkce_verifier={}; Path=/; Max-Age=300; SameSite=Lax",
            verifier
        );

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
         &code_challenge={challenge}&code_challenge_method=S256\
         &audience={audience}",
        audience = AUTH0_AUDIENCE,
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

pub fn logout(client_id: &str, domain: &str, post_logout_redirect_uri: &str) {
    let logout_url = format!(
        "https://{domain}/v2/logout?client_id={client_id}&returnTo={redirect_uri}",
        domain = domain,
        client_id = client_id,
        redirect_uri = urlencoding::encode(post_logout_redirect_uri),
    );

    web_sys::window()
        .unwrap()
        .location()
        .set_href(&logout_url)
        .unwrap();
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

    Ok(token)
}