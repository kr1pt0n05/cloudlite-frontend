use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use axum::extract::{Query, State};
use axum::Router;
use axum::routing::get;
use oauth2::{AuthorizationCode, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl, EndpointSet, EndpointNotSet, reqwest, PkceCodeVerifier, AccessToken, RefreshToken, TokenResponse};
use oauth2::basic::BasicClient;
use tauri_plugin_opener::OpenerExt;
use crate::auth::error::AuthError;
use tokio::sync::oneshot;

struct AuthResponse{
    code: String,
    csrf_token: String,
}


pub struct AuthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

enum AuthState {
    LoggedOut,
    PendingLogin(PendingLogin),
    Authenticated(AuthToken),
    Idle,
}

struct PendingLogin {
    verifier: Option<PkceCodeVerifier>,
    csrf: Option<CsrfToken>,
    redirect_url: Option<String>,
}

struct AuthToken{
    access_token: String,
    valid_until: std::time::Instant,
    refresh_token: String,
}

pub struct AuthService {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    state: Mutex<AuthState>,
}


impl AuthService {
    pub fn new(config: AuthConfig) -> Result<Self, oauth2::url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.to_string()))
            .set_auth_uri(AuthUrl::new(config.auth_url.to_string())?)
            .set_token_uri(TokenUrl::new(config.token_url.to_string())?)
            .set_redirect_uri(RedirectUrl::new(config.redirect_url.to_string())?);
        Ok(AuthService{client, state: Mutex::new(AuthState::LoggedOut)})
    }

    pub async fn begin_login(&self) -> Result<String, AuthError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();
            auth_url.as_str();

        // Acquire mutex and write code_challenge
        {
            let mut state = self
                .state
                .lock()
                .map_err(|_| AuthError::MutexPoisoned)?;

            match &*state {
                AuthState::LoggedOut => {
                    *state = AuthState::PendingLogin(PendingLogin {
                        verifier: Some(pkce_verifier),
                        csrf: Some(csrf_token),
                        redirect_url: Some(auth_url.to_string()),
                    });
                    Ok(auth_url.to_string())
                }
                _ => {
                    return Err(AuthError::UnexpectedState);
                }
            }
        }
    }

    pub async fn confirm_login(&self, app: tauri::AppHandle) -> Result<(), AuthError> {
        let login = {
            let mut state = self.state.lock().map_err(|_| AuthError::MutexPoisoned)?;

            match std::mem::replace(&mut *state, AuthState::Idle) {
                AuthState::PendingLogin(login) => login,
                old_state => {
                    *state = old_state;
                    return Err(AuthError::UnexpectedState);
                }
            }
        };

        let redirect_url = login
            .redirect_url
            .ok_or(AuthError::MissingAuthURL)?;

        let pkce_verifier = login
            .verifier
            .ok_or(AuthError::MissingPkceVerifier)?;

        let csrf_token = login
            .csrf
            .ok_or(AuthError::MissingCsrfToken)?;

        let (tx_code, rx_code) = oneshot::channel::<AuthResponse>();
        let (tx_shutdown, rx_shutdown) = oneshot::channel::<String>();

        tokio::spawn(async move {
            let _ = Self::start_webserver(tx_code, rx_shutdown).await;
        });

        app.opener()
            .open_url(redirect_url, None::<&str>)
            .map_err(|_| AuthError::UnexpectedState)?;

        let auth_response = rx_code
            .await
            .map_err(|_| AuthError::UnexpectedState)?;

        let _ = tx_shutdown.send(String::new());

        if csrf_token.into_secret() != auth_response.csrf_token {
            return Err(AuthError::InvalidCSRFToken);
        }

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| AuthError::CodeExchangeFailed)?;

        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(auth_response.code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&client)
            .await
            .map_err(|_| AuthError::CodeExchangeFailed)?;

        let mut state = self.state.lock().map_err(|_| AuthError::MutexPoisoned)?;
        *state = AuthState::Authenticated(AuthToken {
            access_token: token_result.access_token().secret().to_string(),
            valid_until: Instant::now() + token_result.expires_in().unwrap_or_default(),
            refresh_token: token_result.refresh_token().map(|t| t.secret().to_string()).unwrap_or_default(),
        });

        Ok(())
    }

    // Returns true, if JWT is present and at least 5 minutes valid. Returns false if no token is present or token is expired. Returns error if mutex is poisoned.
    // ToDo: Take JWT as argument to avoid double-locking
    // ToDo: Before throwing error, change to AuthState::LoggedOut
    pub fn is_authenticated(&self) -> Result<bool, AuthError> {
        let mut state = self.state.lock().map_err(|_| AuthError::MutexPoisoned)?;
        match &mut *state {
            AuthState::Authenticated(token) => Ok(token.valid_until > Instant::now() + Duration::from_secs(300)),
            _ => Ok(false),
        }
    }

    pub async fn get_access_token(&self) -> Result<String, AuthError> {
        if self.is_authenticated()? {
            let state = self.state.lock().map_err(|_| AuthError::MutexPoisoned)?;
            if let AuthState::Authenticated(token) = &*state {
                return Ok(token.access_token.clone());
            }
        }
        // ToDo: Request access_token if user is not authenticated anymore
        Err(AuthError::UnexpectedState)
    }


    async fn start_webserver(tx_code: tokio::sync::oneshot::Sender<AuthResponse>, rx_shutdown: tokio::sync::oneshot::Receiver<String>) -> Result<(), AuthError> {
        let shared_state = Arc::new(Mutex::new(Some((tx_code))));
        let app = Router::new()
            .route("/", get(Self::auth_code))
            .with_state(shared_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:4200")
            .await
            .map_err(|e| AuthError::ServerStartFailed)?;

        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                rx_shutdown.await.ok();
            })
            .await
            .map_err(|e| AuthError::ServerStartFailed)?;
        Ok(())
    }

    // ToDo: Rewrite this more cleanly
    // ToDo: Proper error handling
    async fn auth_code(Query(params): Query<HashMap<String, String>>, State(state): State<Arc<Mutex<Option<(tokio::sync::oneshot::Sender<AuthResponse>)>>>>) -> &'static str {
        let code = params.get("code");
        let csrf_token = params.get("state"); // CSRF token

        if code.is_none() || csrf_token.is_none(){
            panic!("Missing code or state in query parameters");
        }

        let auth_response = AuthResponse{
            code: code.unwrap().to_string(),
            csrf_token: csrf_token.unwrap().to_string(),
        };
        let channel = state.lock().unwrap().take();
        if channel.is_none(){panic!("Channel was already taken")};
        channel.unwrap().send(auth_response).ok();
        "Login ADD_STATE_HERE. You can close this page."
    }

}