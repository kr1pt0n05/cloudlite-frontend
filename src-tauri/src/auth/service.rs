use std::sync::Mutex;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl, EndpointSet, EndpointNotSet, reqwest, PkceCodeVerifier, AccessToken, RefreshToken};
use oauth2::basic::BasicClient;
use tauri_plugin_opener::OpenerExt;

use crate::auth::error::AuthError;

pub struct AuthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

struct PendingLogin {
    verifier: Option<PkceCodeVerifier>,
    redirect_url: Option<String>,
}

struct AuthToken{
    access_token: String,
    expires_in: std::time::Duration,
    refresh_token: String,
}

pub struct AuthService {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,

    code_challenge: Mutex<PendingLogin>,
    token: Option<AuthToken>,
}
impl AuthService {
    pub fn new(config: AuthConfig) -> Result<Self, oauth2::url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.to_string()))
            .set_auth_uri(AuthUrl::new(config.auth_url.to_string())?)
            .set_token_uri(TokenUrl::new(config.token_url.to_string())?)
            .set_redirect_uri(RedirectUrl::new(config.redirect_url.to_string())?);
        Ok(AuthService{client, code_challenge: Mutex::new(PendingLogin{verifier: None, redirect_url: None}), token: None})
    }

    pub async fn get_redirect_url(&self) -> Result<String, AuthError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, _csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();
            auth_url.as_str();

        // Acquire mutex and write code_challenge
        {
            let mut code_challenge = self
                .code_challenge
                .lock()
                .map_err(|_| AuthError::MutexPoisoned)?;
            *code_challenge = PendingLogin {
                verifier: Some(pkce_verifier),
                redirect_url: Some(auth_url.to_string()),
            };
        }
        Ok(auth_url.to_string())
    }

    pub async fn redirect_auth(&self, app: tauri::AppHandle) -> Result<(), AuthError> {
        let redirect_url = {
            let pending_login = self
                .code_challenge
                .lock()
                .map_err(|_| AuthError::MutexPoisoned)?;

            pending_login
                .redirect_url
                .clone()
                .ok_or(AuthError::MissingAuthURL)?
        };

        // Open Auth URL in users native browser
        app.opener()
            .open_url(redirect_url, None::<&str>)
            .expect("Should open in browser");

        Ok(())
    }

    pub async fn end_login(&self) -> Result<(), AuthError> {
        // ToDo: Use connection-pooling with  a ClientBuilder
        // ToDo: Set redirect-policy to none

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let pkce_verifier = {
            let mut pending_login = self
                .code_challenge
                .lock()
                .map_err(|_| AuthError::MutexPoisoned)?;

            pending_login
                .verifier
                .take()
                .ok_or(AuthError::MissingPkceVerifier)?
        };

        // ToDo: Check that code returned matches csrf_token

        let _token_result = self.client
            .exchange_code(AuthorizationCode::new("auth_code".to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&client)
            .await;

        Ok(())
    }

}