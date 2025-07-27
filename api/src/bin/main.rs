
use http::{Method};
use axum_extra::extract::CookieJar;
use axum::extract::{Host, Query};
use clap::Parser;
use openapi::server;
use openapi::apis::default::{Default, HelloGetResponse, TestauthGetResponse};
use openapi::models::{HelloGet200Response};
use std::sync::OnceLock;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{Router};
use axum::routing::get;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation, TokenData};
use serde_json::{Value};
use tracing::{info, error, debug};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;
use josekit::jwe::Dir;
use josekit::jwt::decode_with_decrypter;
use base64::Engine;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::str;

#[derive(Parser, Debug)]
#[command(name = "Auth0 CLI", about = "")]
struct ArgsAuth0 {
    #[arg(long)]
    auth0_domain: String,
    #[arg(long)]
    pub auth0_client_secret: String,
    #[arg(long)]
    auth0_client_id: String,
    #[arg(long, default_value = "http://localhost:3000/callback")]
    auth0_redirect_uri: String,
    #[arg(long)]
    auth0_scope: String,

}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    //pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub azp: Option<String>,
    pub scope: Option<String>,
}

static AUTH0: OnceLock<ArgsAuth0> = OnceLock::new();

#[derive(Clone)]
struct MyApi;

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub async fn decode_access_token(token: &str, jwk_url: &str) -> Result<TokenData<Claims>, String> {

    let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;
    let kid = header.kid.ok_or("Missing `kid` in token header")?;

    let jwks_url = format!("https://{}/.well-known/jwks.json", jwk_url);
    let jwks: Value = Client::new()
        .get(&jwks_url)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;

    let keys = jwks["keys"].as_array().ok_or("No 'keys' in JWKS")?;

    let jwk = keys.iter().find(|k| k["kid"] == kid)
        .ok_or_else(|| "Matching JWK not found".to_string())?;

    let n = jwk["n"].as_str().ok_or("Missing 'n'")?;
    let e = jwk["e"].as_str().ok_or("Missing 'e'")?;

    let decoding_key = DecodingKey::from_rsa_components(n, e)
        .map_err(|e| format!("Invalid decoding key: {}", e))?;

    decode::<Claims>(
        token,
        &decoding_key,
        &Validation::new(Algorithm::RS256),
    ).map_err(|e| format!("Token validation failed: {}", e))
}

#[async_trait::async_trait]
impl Default for MyApi {
    async fn hello_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<HelloGetResponse, String> {
        let hello = HelloGet200Response { message: Some("hello".to_string()) };
        info!("{:?}",hello);
        Ok(HelloGetResponse::Status200_AJSONObjectWithAGreetingMessage(hello))
    }
    async fn testauth_get(&self, method: Method, host: Host, cookies: CookieJar) -> Result<TestauthGetResponse, String> {
        todo!()
    }
}

impl AsRef<MyApi> for MyApi {
    fn as_ref(&self) -> &MyApi {
        self
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("App started");

    let args = ArgsAuth0::parse();
    AUTH0.set(args).unwrap();

    let api_impl = MyApi;
    let app_open_api = server::new(api_impl);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app_open_api).await {
        info!("server error: {}", e);
    }
}
