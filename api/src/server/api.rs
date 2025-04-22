use std::collections::HashMap;
use std::sync::OnceLock;
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};

#[derive(Clone)]
pub struct MyApi;

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

#[derive(Parser, Debug)]
#[command(name = "Auth0 CLI", about = "")]
pub struct ArgsAuth0 {
    #[arg(long)]
    pub auth0_domain: String,

    #[arg(long)]
    pub auth0_client_id: String,

    #[arg(long)]
    pub auth0_client_secret: String,

    #[arg(long, default_value = "http://localhost:3000/callback")]
    pub auth0_redirect_uri: String,

    #[arg(long)]
    pub auth0_response_type: String,

    #[arg(long)]
    pub auth0_scope: String,
}

pub static AUTH0: OnceLock<ArgsAuth0> = OnceLock::new();

impl MyApi {
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
        let client = Client::new();
        let auth0_config = AUTH0.get().ok_or("Auth0 configuration not initialized")?;

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("client_id", &auth0_config.auth0_client_id);
        params.insert("client_secret", &auth0_config.auth0_client_secret);
        params.insert("redirect_uri", &auth0_config.auth0_redirect_uri);

        let token_url = format!("https://{}/oauth/token", auth0_config.auth0_domain);
        
        let res = client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;

        if !res.status().is_success() {
            let error_text = res.text().await?;
            return Err(format!("Token exchange failed with status {}: {}", res.status(), error_text).into());
        }

        let token_response = res.json::<TokenResponse>().await?;
        println!("Token exchange successful: {:#?}", token_response);
        Ok(token_response)
    }

    pub fn decode_id_token(&self, id_token: &str) -> Result<IdTokenClaims, jsonwebtoken::errors::Error> {
        // Jeśli nie chcesz weryfikować podpisu, możesz ustawić dowolny klucz (Tylko do testów!)
        let dummy_secret = DecodingKey::from_secret("any-secret".as_ref());

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = false; // Możesz ustawić true, jeśli chcesz wymuszać ważność

        let token_data: TokenData<IdTokenClaims> = decode::<IdTokenClaims>(
            id_token,
            &dummy_secret,     // Klucz nie będzie użyty, jeśli signature validation jest wyłączona
            &validation,
        )?;

        Ok(token_data.claims)
    }
} 