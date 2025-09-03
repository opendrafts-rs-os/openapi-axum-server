use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetHelloResponse {
    /// A JSON object with a greeting message.
    Status200_AJSONObjectWithAGreetingMessage
    (models::GetHello200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTestauthResponse {
    /// Successfully retrieved user information.
    Status200_SuccessfullyRetrievedUserInformation
    (models::GetTestauth200Response)
    ,
    /// Unauthorized, missing or invalid token.
    Status401_Unauthorized
    ,
    /// Server error while retrieving user information.
    Status500_ServerErrorWhileRetrievingUserInformation
}


/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default {
    /// Returns a greeting.
    ///
    /// GetHello - GET /hello
    async fn get_hello(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<GetHelloResponse, String>;

    /// Get test if I logged.
    ///
    /// GetTestauth - GET /testauth
    async fn get_testauth(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<GetTestauthResponse, String>;
}
