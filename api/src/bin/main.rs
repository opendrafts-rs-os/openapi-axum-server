use http::Method;
use axum_extra::extract::CookieJar;
use axum::extract::Host;
use clap::Parser;
use openapi::server;
use openapi::apis::default::{CallbackGetResponse, Default, HelloGetResponse, LogoutGetResponse, UserinfoGetResponse};
use openapi::models::{CallbackGetQueryParams, HelloGet200Response, LogoutGetQueryParams};
use std::sync::OnceLock;
use axum::response::{Redirect};
use axum::Router;
use axum::routing::get;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Parser, Debug)]
#[command(name = "Auth0 CLI", about = "")]
struct ArgsAuth0 {
    #[arg(long)]
    auth0_domain: String,

    #[arg(long)]
    auth0_client_id: String,

    #[arg(long, default_value = "http://localhost:3000/callback")]
    auth0_redirect_uri: String,

    #[arg(long)]
    auth0_response_type: String,

    #[arg(long)]
    auth0_scope: String,

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

    // TODO: Save csrf_state and nonce in (session or cookie), eg.:
    // session.insert("auth_csrf_state", &csrf_state);
    // session.insert("auth_nonce", &nonce);

    let auth_url = format!(
        "https://{}/authorize\
             ?response_type={}\
             &client_id={}\
             &redirect_uri={}\
             &scope={}\
             &state={}\
             &nonce={}",
        AUTH0.get().unwrap().auth0_domain,
        AUTH0.get().unwrap().auth0_response_type,
        AUTH0.get().unwrap().auth0_client_id,
        urlencoding::encode(&AUTH0.get().unwrap().auth0_redirect_uri),
        AUTH0.get().unwrap().auth0_scope,
        csrf_state,
        nonce
    );

    Redirect::to(&auth_url)

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
        Ok(HelloGetResponse::Status200_AJSONObjectWithAGreetingMessage(hello))
    }


    async fn callback_get(&self, _method: Method, _host: Host, _cookies: CookieJar, _query_params: CallbackGetQueryParams) -> Result<CallbackGetResponse, String> {
        todo!()
    }
    async fn userinfo_get(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<UserinfoGetResponse, String> {
        todo!()
    }
    async fn logout_get(&self, _method: Method, _host: Host, _cookies: CookieJar, _query_params: LogoutGetQueryParams) -> Result<LogoutGetResponse, String> {
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

    let args = ArgsAuth0::parse();

    AUTH0.set(args).unwrap();

    let api_impl = MyApi;

    let app_open_api = server::new(api_impl);

    let app_login = Router::new()
        .route("/login", get(login_get));

    let app = Router::new()
        .merge(app_open_api)
        .merge(app_login);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("server error: {}", e);
    }
}