use http::Method;
use axum_extra::extract::CookieJar;
use axum::extract::Host;
use openapi::server;
use openapi::apis::default::{Default, HelloGetResponse};
use openapi::models::HelloGet200Response;

#[derive(Clone)]
struct MyApi;


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
}

impl AsRef<MyApi> for MyApi {
    fn as_ref(&self) -> &MyApi {
        self
    }
}

#[tokio::main]
async fn main() {

    let api_impl = MyApi;

    let app = server::new(api_impl);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("server error: {}", e);
    }
}