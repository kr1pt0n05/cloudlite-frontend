use std::sync::Mutex;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl, EndpointSet, EndpointNotSet, reqwest, PkceCodeVerifier, AccessToken, RefreshToken};
use oauth2::basic::BasicClient;
use tauri_plugin_opener::OpenerExt;

use crate::auth::error;
use crate::auth::error::AuthError;

pub struct AuthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

struct CodeChallenge {
    verifier: PkceCodeVerifier,
}

struct AuthToken{
    access_token: String,
    expires_in: std::time::Duration,
    refresh_token: String,
}

pub struct AuthService {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,

    code_challenge: Mutex<Option<CodeChallenge>>,
    token: Option<AuthToken>,
}
impl AuthService {
    pub fn new(config: AuthConfig) -> Result<Self, oauth2::url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.to_string()))
            .set_auth_uri(AuthUrl::new(config.auth_url.to_string())?)
            .set_token_uri(TokenUrl::new(config.token_url.to_string())?)
            .set_redirect_uri(RedirectUrl::new(config.redirect_url.to_string())?);
        Ok(AuthService{client, code_challenge: Mutex::new(None), token: None})
    }

    pub async fn start_login(&self, app: tauri::AppHandle) -> Result<(), AuthError> {
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
            *code_challenge = Some(CodeChallenge{verifier: pkce_verifier});
        }

        // Open Auth URL in users native browser
        app.opener()
            .open_url(auth_url, None::<&str>)
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
            let pkce_verifier = self.code_challenge
                .lock()
                .unwrap()
                .take()
                .ok_or(AuthError::MissingPkceVerifier)?;
            pkce_verifier
        };

        // ToDo: Check that code returned matches csrf_token

        let _token_result = self.client
            .exchange_code(AuthorizationCode::new("auth_code".to_string()))
            .set_pkce_verifier(pkce_verifier.verifier)
            .request_async(&client)
            .await;

        Ok(())
    }

}