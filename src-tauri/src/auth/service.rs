use oauth2::{AuthorizationCode, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl, EndpointSet, EndpointNotSet, reqwest};
use oauth2::basic::BasicClient;
use tauri_plugin_opener::OpenerExt;

pub struct AuthConfig {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

pub struct AuthService {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>
}
impl AuthService {
    pub fn new(config: AuthConfig) -> Result<Self, oauth2::url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.to_string()))
            .set_auth_uri(AuthUrl::new(config.auth_url.to_string())?)
            .set_token_uri(TokenUrl::new(config.token_url.to_string())?)
            .set_redirect_uri(RedirectUrl::new(config.redirect_url.to_string())?);
        Ok(AuthService{client})
    }

    pub async fn login(&self, app: tauri::AppHandle) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL.
        let (auth_url, _csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();
            auth_url.as_str();

        // Open Auth URL in users native browser
        app.opener().open_url(auth_url, None::<&str>).expect("Should open in browser");

        // ToDo: Use connection-pooling with  a ClientBuilder
        // ToDo: Set redirect-policy to none

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // ToDo: Check that code returned matches csrf_token

        let _token_result = self.client
        .exchange_code(AuthorizationCode::new("auth_code".to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&client)
            .await;
    }

}