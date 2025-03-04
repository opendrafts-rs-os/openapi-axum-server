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
pub enum CallbackGetResponse {
    /// Redirects the user to the application after successful authentication.
    Status302_RedirectsTheUserToTheApplicationAfterSuccessfulAuthentication
    ,
    /// Bad request, missing or invalid parameters.
    Status400_BadRequest
    ,
    /// Server error during token exchange.
    Status500_ServerErrorDuringTokenExchange
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum HelloGetResponse {
    /// A JSON object with a greeting message.
    Status200_AJSONObjectWithAGreetingMessage
    (models::HelloGet200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LogoutGetResponse {
    /// Redirects the user to Auth0 for logout and then to the specified return URL.
    Status302_RedirectsTheUserToAuth
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UserinfoGetResponse {
    /// Successfully retrieved user information.
    Status200_SuccessfullyRetrievedUserInformation
    (models::UserinfoGet200Response)
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
    /// Auth0 callback endpoint.
    ///
    /// CallbackGet - GET /callback
    async fn callback_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::CallbackGetQueryParams,
    ) -> Result<CallbackGetResponse, String>;

    /// Returns a greeting.
    ///
    /// HelloGet - GET /hello
    async fn hello_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<HelloGetResponse, String>;

    /// User logout (redirect to Auth0 logout).
    ///
    /// LogoutGet - GET /logout
    async fn logout_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::LogoutGetQueryParams,
    ) -> Result<LogoutGetResponse, String>;

    /// Get user information.
    ///
    /// UserinfoGet - GET /userinfo
    async fn userinfo_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<UserinfoGetResponse, String>;
}
