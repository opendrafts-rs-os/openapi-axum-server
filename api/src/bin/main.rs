use http::Method;

use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
        header::{AUTHORIZATION},
    },
    response::Response,
    extract::Host,
    Router,
    extract::Extension
};
use tower::{Layer, Service};
use axum_extra::extract::CookieJar;
use clap::Parser;
use openapi::apis::default::{Default, GetHelloResponse, GetTestauthResponse};
use openapi::models::{GetHello200Response, GetTestauth200Response};
use openapi::server;

use reqwest::Client;

use jsonwebtoken::{
    decode,
    decode_header,
    Algorithm,
    DecodingKey,
    Validation,
    TokenData
};

use serde_json::{Value};
use serde::{Deserialize, Serialize};

use tracing::info;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

use std::{
    str,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    sync::OnceLock,
};
use std::collections::HashSet;

#[derive(Clone)]
pub struct AuthLayer {
    pub public_paths: Vec<&'static str>,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            public_paths: self.public_paths.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    public_paths: Vec<&'static str>,
}

#[derive(Parser, Debug)]
#[command(name = "Auth0 CLI", about = "")]
struct ArgsAuth0 {
    #[arg(long)]
    auth0_audience: String,
    #[arg(long)]
    auth0_jwks: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: Value,
    pub exp: usize,
    pub iat: usize,
    pub azp: Option<String>,
    pub scope: Option<String>,
}

static AUTH0: OnceLock<ArgsAuth0> = OnceLock::new();

#[derive(Clone)]
struct MyApi;

impl<S, ReqBody> Service<Request<ReqBody>> for AuthMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let path = req.uri().path().to_string();
        let public_paths = self.public_paths.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            if public_paths.iter().any(|p| path.starts_with(p)) {
                return inner.call(req).await;
            }


            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");


            if let Some(token) = auth_header.strip_prefix("Bearer ") {

                match verify_access_token(token, &*get_cfg().auth0_jwks, &*get_cfg().auth0_audience).await {
                    Ok(_td) => {
                        return inner.call(req).await;
                    }
                    Err(_e) => {
                        // fallthrough 401
                    }
                }
            }

            // 401
            let resp = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Ok(resp)
        })
    }
}

fn get_cfg() -> &'static ArgsAuth0 {
    match AUTH0.get() {
        Some(auth_cfg) => auth_cfg,
        None => panic!("Auth0 config not initialized"),
    }
}

async fn verify_access_token(
    token: &str,
    jwks_url: &str,
    expected_audience: &str,
) -> Result<TokenData<Claims>, String> {

    let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;
    let kid = header.kid.ok_or_else(|| "Missing `kid` in token header".to_string())?;

    let jwks: Value = Client::new()
        .get(jwks_url)
        .send().await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;

    let keys = jwks["keys"].as_array().ok_or("No 'keys' in JWKS".to_string())?;

    let jwk = keys.iter().find(|k| k["kid"] == kid)
        .ok_or_else(|| "Matching JWK not found".to_string())?;

    let n = jwk["n"].as_str().ok_or("Missing 'n'".to_string())?;
    let e = jwk["e"].as_str().ok_or("Missing 'e'".to_string())?;

    let decoding_key = DecodingKey::from_rsa_components(n, e)
        .map_err(|e| format!("Invalid decoding key: {}", e))?;

    let mut validation = Validation::new(Algorithm::RS256);

    validation.aud = Some(HashSet::from([expected_audience.to_string()]));

    let token_data = decode::<Claims>(
        token,
        &decoding_key,
        &validation,
    ).map_err(|e| {
        tracing::error!("Token validation failed: {}", e);
        format!("Token validation failed: {}", e)
    })?;

    Ok(token_data)
}

#[async_trait::async_trait]
impl Default for MyApi {
    async fn get_hello(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetHelloResponse, String> {
        let body = GetHello200Response {
            message: Some("hello".into()),
        };
        info!("{:?}",body);
        Ok(GetHelloResponse::Status200_AJSONObjectWithAGreetingMessage(body))
    }

    async fn get_testauth(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        ) -> Result<GetTestauthResponse, String> {
        let body = GetTestauth200Response {
            login: Some("test".into()),
            sub: Some("test sub".into()),
        };
        info!("{:?}", body);
        Ok(GetTestauthResponse::Status200_SuccessfullyRetrievedUserInformation(body))
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
    let app_open_api: Router = server::new(api_impl);

    let app = app_open_api.layer(AuthLayer {
        public_paths: vec!["/hello", "/health"],
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app).await {
        info!("server error: {}", e);
    }

    //TODO
    // cache JWKS, leeway in time, (403, 401 ).
    // production         Cache public kyes (JWKS)??
    // logs metrics
    // Integrate (prepare) error logs with SIEM and review regularly
    // inject x-auth
}