
use http::{Method, StatusCode};
use axum_extra::extract::CookieJar;
use axum::extract::{Host, Query};
use clap::Parser;
use openapi::server;
use openapi::apis::default::{Default, HelloGetResponse, UserinfoGetResponse};
use openapi::models::{HelloGet200Response, UserinfoGet200Response};
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
//use tracing_subscriber::fmt::format::FmtSpan;

use josekit::jwe::Dir;
use josekit::jwt::decode_with_decrypter;


use base64::Engine;

use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `aes_gcm::Aes256GcmSiv`
use aes_gcm::aead::{Aead, KeyInit};
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
    auth0_response_type: String,
    #[arg(long)]
    auth0_scope: String,
    #[arg(long)]
    auth0_audience: String,
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
#[derive(Debug, Deserialize)]
pub struct IdTokenClaims {
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
    pub exp: usize,
    pub iss: Option<String>,
    pub aud: Option<String>,
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
    // Add more fields if needed
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

pub async fn login_get( _method: Method, _host: Host, _cookies: CookieJar) -> Redirect {

    let csrf_state = generate_random_string(32);
    let nonce = generate_random_string(32);

    debug!("state: {}", csrf_state);

    let redirect = match AUTH0.get() {
        Some(auth_cfg) => {
            let auth_url = format!(
                "https://{}/authorize\
                    ?response_type={}\
                    &client_id={}\
                    &redirect_uri={}\
                    &scope={}\
                    &state={}\
                    &nonce={}",
                auth_cfg.auth0_domain,
                auth_cfg.auth0_response_type,
                auth_cfg.auth0_client_id,
                urlencoding::encode(&auth_cfg.auth0_redirect_uri),
                auth_cfg.auth0_scope,
                csrf_state,
                nonce
            );
            Redirect::to(&auth_url)
        }
        None => {
            Redirect::to(&"")
        }
    };
    redirect
}

pub async fn callback_get(_method: Method, _host: Host, cookies: CookieJar, query_params: Query<CallbackQuery>) -> Response {
    let affter_callback = "http://localhost:8080";
    match AUTH0.get() {
        Some(auth_cfg) => {
            let jwks = format!("https://{}/.well-known/jwks.json", &auth_cfg.auth0_domain);
            debug!("query params: {:?}", query_params);
            match exchange_code_for_token(&query_params.code).await {
                Ok(token_response) => {
                    match decode_id_token(&token_response.id_token, &jwks).await {
                        Ok(_user_info) => {
                            let cookie_at =
                                Cookie::build((
                                                    "DEMO_ACCESS_TOKEN",
                                                    token_response.access_token.to_owned()))
                                    .path("/")
                                    .http_only(true)
                                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                                    .build();
                            let cookie_it =
                                Cookie::build((
                                    "DEMO_ID_TOKEN",
                                    token_response.id_token.to_owned()))
                                    .path("/")
                                    .http_only(true)
                                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                                    .build();

                            let cookie_is_logged_in =
                                Cookie::build((
                                    "DEMO_IS_LOGGED_IN",
                                    token_response.id_token.to_owned()))
                                    .path("/")
                                    .http_only(false)
                                    .same_site(axum_extra::extract::cookie::SameSite::Lax)
                                    .build();

                            (cookies.add(cookie_at).add(cookie_it).add(cookie_is_logged_in), Redirect::temporary(affter_callback)).into_response()
                        },
                        Err(err) => {
                            error!("Failed to decode ID token: {}", err);
                            (cookies, Redirect::temporary(affter_callback)).into_response()
                        }
                    }
                },
                Err(err) => {
                    error!("Token exchange failed: {}", err);
                    (cookies, Redirect::temporary(affter_callback)).into_response()
                }
            }
        }
        None => {
            error!("Token exchange failed: problem with configuration");
            (cookies, Redirect::temporary(affter_callback)).into_response()
        }
    }
}

async fn exchange_code_for_token(code: &str) -> Result<TokenResponse, String> {
    debug!("code: {}", code);
    let client = Client::new();
    let auth0_config = AUTH0.get().ok_or("Auth0 configuration not initialized")?;
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", code);
    params.insert("client_id", &auth0_config.auth0_client_id);
    params.insert("client_secret", &auth0_config.auth0_client_secret);
    params.insert("redirect_uri", &auth0_config.auth0_redirect_uri);
    params.insert("audience", &auth0_config.auth0_audience);
    params.insert("scope", "openid profile email");

    // curl --request POST \
    // --url https://dev-iord0jalg17zkw36.eu.auth0.com/oauth/token \
    // --header 'content-type: application/json' \
    // --data '{
    // "client_id":"SFYyLef5Lbq8c9D8UFRzeoWQN5gMZALN",
    // "client_secret":"UZ_F-uNQLB9qGvnEufFpuNGh0D0RcZ6K7k8E_3zifsmzS2tXkomNGsjkYB_il48b",
    // "audience":"https://auth01.local/api",
    // "grant_type":"client_credentials"
    // }'

    //params.insert("scope", "openid profile email");
    //debug!("audience: {}", &auth0_config.auth0_audience);
    //params.insert("audience", &auth0_config.auth0_audience);

    let token_url = format!("https://{}/oauth/token", auth0_config.auth0_domain);

    let res = client
        .post(&token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    if !status.is_success() {
        let error_text = res.text().await.map_err(|e| e.to_string())?;
        return Err(format!("Token exchange failed with status {}: {}",
                           StatusCode::from(status),
                           error_text));
    }
    let token_response = res.json::<TokenResponse>().await.map_err(|e| e.to_string())?;
    debug!("token_response: {:?}", token_response);
    Ok(token_response)
}

async fn decode_id_token(id_token: &str, jwks_url: &str) -> Result<IdTokenClaims, String> {

    let header = decode_header(id_token).map_err(|e| e.to_string())?;
    let kid = header.kid.ok_or("Missing kid in JWT header")?;

    let jwks: Value = reqwest::get(jwks_url).await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;

    let jwk = jwks["keys"]
        .as_array()
        .ok_or("Missing keys in JWK")?
        .iter()
        .find(|k| k["kid"] == kid)
        .ok_or("Key with the specified kid not found")?;

    let n = jwk["n"].as_str().ok_or("Missing 'n' in JWK")?;
    let e = jwk["e"].as_str().ok_or("Missing 'e' in JWK")?;

    let decoding_key = DecodingKey::from_rsa_components(n, e).map_err(|e| e.to_string())?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    validation.set_audience(&[&AUTH0.get().unwrap().auth0_client_id]);

    let token_data: TokenData<IdTokenClaims> = decode::<IdTokenClaims>(
        id_token,
        &decoding_key,
        &validation,
    ).map_err(|e| e.to_string())?;

    Ok(token_data.claims)
}

pub async fn decode_access_token(token: &str, jwk_url: &str) -> Result<TokenData<Claims>, String> {

    let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;
    let kid = header.kid.ok_or("Missing `kid` in token header")?;


    let jwks_url = format!("https://{}/.well-known/jwks.json", jwk_url);
    let jwks: serde_json::Value = Client::new()
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

    async fn userinfo_get(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<UserinfoGetResponse, String> {
        let access_token = _cookies.get("DEMO_ACCESS_TOKEN");
        let id_token = _cookies.get("DEMO_ID_TOKEN");
        match AUTH0.get() {
            Some(auth_cfg) => {
                let jwks = format!("https://{}/.well-known/jwks.json", &auth_cfg.auth0_domain);
                let id_token = id_token.ok_or("Missing id token")?.value();

                let id_claim = match decode_id_token(id_token, &jwks).await {
                    Ok(claim) => claim,
                    Err(e) => {
                        return Err(e.to_string())
                    }
                };
                debug!("IdTokenClaim: {:?}", id_claim);

                let access_token = access_token.ok_or("Missing id token")?.value();

                debug!("access token: {:?}", access_token);



                // let access_claim = match decode_access_token(access_token, &jwks).await {
                //     Ok(claim) => claim,
                //     Err(e) => {
                //         return Err(e.to_string())
                //     }
                // };
                // debug!("AccessClaim: {:?}", access_claim);

                // let key = "FXH44XySBH-qUIFMBdi_jX_-IeBQ_c65uCtrXmCcxr_RHokyY7jz6qp0LXi6BwW4";
                //
                // debug!("key: {:?}", key);
                // assert_eq!(key.len(), 32, "Klucz musi mieć dokładnie 32 bajty");
                //
                // let decrypter = Dir.decrypter_from_bytes(&key).expect("Nie udało się utworzyć decryptora");
                //
                // let access_token =  access_token.ok_or("Missing access token")?.value();
                // debug!("AccessToken: {:?}", access_token);
                //
                // let result = decode_with_decrypter(access_token.as_bytes(), &decrypter);
                //
                // match result {
                //     Ok((payload, _header)) => {
                //         let claims_map = payload.claims_set().clone(); // Clonuemy Map<String, Value>
                //         let claims_json = serde_json::Value::Object(claims_map);
                //         debug!("Claims: {:?}", claims_json);
                //     }
                //     Err(err) => {
                //         eprintln!("❌ Błąd odszyfrowania JWT: {:?}", err);
                //     }
                // }

                // to check exp
                // to validate access token
            }
            _ => { }
        };

        debug!("access_token: {:?}", access_token);
        let user_info = UserinfoGet200Response {
            login: Some("test".to_string()),
            sub: Some("subtest".to_string()),
        };

        let user = UserinfoGetResponse::Status200_SuccessfullyRetrievedUserInformation(user_info);
        Ok(user)
    }
}

impl AsRef<MyApi> for MyApi {
    fn as_ref(&self) -> &MyApi {
        self
    }
}

// async fn log_query_middleware<B>(req: Request<Body>, next: Next) -> Response {
//     if let Some(query) = req.uri().query() {
//         debug!("Incoming query params: {}", query);
//     }
//     next.run(req).await
// }

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .init();

    info!("App started");

    let args = ArgsAuth0::parse();

    AUTH0.set(args).unwrap();

    let api_impl = MyApi;

    let app_open_api = server::new(api_impl);

    let app_auth = Router::new()
        .route("/login", get(|method, host, cookies| async move {
            login_get(method, host, cookies).await
        }))
        .route("/callback", get(|method, host, cookies, query_params | async move {
            callback_get(method, host, cookies, query_params).await
        }));

    let app = Router::new()
        .merge(app_open_api)
        .merge(app_auth);
        //.layer(middleware::from_fn(log_query_middleware::<Body>));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app).await {
        info!("server error: {}", e);
    }
}
