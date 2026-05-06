use std::sync::Arc;
use reqwest::Client;
use crate::auth::error::AuthError;
use crate::auth::service::AuthService;
use crate::db::service::Changelog;

// ToDo: Move this into .env
const API_KEY: &str = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICI2OTVwUHVSYzNpODFsbzJyZ2hyWEJMX2ExbUdzTGhHOFJaY1Q4TVdaeHhrIn0.eyJleHAiOjE3NzgxMTgyOTgsImlhdCI6MTc3ODEwMDI5OCwianRpIjoidHJydGNjOjdiYjgxNjFlLWUyZjItNWQ0Zi1mN2Y4LWE5ZTUzOWU1NDk1MyIsImlzcyI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC9yZWFsbXMvZGV2ZWxvcG1lbnQiLCJhdWQiOlsiZnJvbnRlbmQtY2xpZW50IiwiYWNjb3VudCJdLCJzdWIiOiJmYmRlZjAxZC1iZDY1LTQ1MTgtYjI0NS1hZmJjN2ViZDVhNWMiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJzZXJ2aWNlLWNsaWVudC0wMSIsImFjciI6IjEiLCJhbGxvd2VkLW9yaWdpbnMiOlsiLyoiXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbIm9mZmxpbmVfYWNjZXNzIiwiZGVmYXVsdC1yb2xlcy1kZXZlbG9wbWVudCIsInVtYV9hdXRob3JpemF0aW9uIl19LCJyZXNvdXJjZV9hY2Nlc3MiOnsic2VydmljZS1jbGllbnQtMDEiOnsicm9sZXMiOlsidW1hX3Byb3RlY3Rpb24iXX0sImZyb250ZW5kLWNsaWVudCI6eyJyb2xlcyI6WyJzZXJ2aWNlIl19LCJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImNsaWVudEhvc3QiOiIxNzIuMTkuMC4xIiwicHJlZmVycmVkX3VzZXJuYW1lIjoic2VydmljZS1hY2NvdW50LXNlcnZpY2UtY2xpZW50LTAxIiwiY2xpZW50QWRkcmVzcyI6IjE3Mi4xOS4wLjEiLCJjbGllbnRfaWQiOiJzZXJ2aWNlLWNsaWVudC0wMSJ9.N-pLFo5WtoKd8sIZ54XA5P1_Tbc0c5rrxThquH8i3hD3ckVSmPt2Mgn01-bSXpsJfnZNW9Pd3RPuYDo9sZnhCNSiLcjNn7AOeegZdvTCuXKqkCRMcQlxg1siV5rh6gDiIg3vd08WTiT0arJnnbdof1As2zpa2WYGr9SP7_CSIR8K5KwoqUWpcpcXXl7jLHnviYQ0l_yMdf113za21hQZzGhYOUikFU2dHGo7k9t-OJ54YNWTGjvw-_bFI7JZXEmyergennaspm5S5KCknHN3a5PVUSgO096P8ys8cuZZAF1hsC_uPh3W7S_Km8rYRqasmHrOyH30aB4z-1RnqPikYQ";
pub struct ApiService{
    client: Client,
    auth: Arc<AuthService>,
    base_url: String,
}

impl ApiService{
    pub fn new(auth: Arc<AuthService>, base_url: String) -> Self{
        let client = Client::new();
        Self{client, auth, base_url}
    }

    // ToDo: Replace hard coded access_token
    pub async fn get_latest_changelogs(&self, latest_changelog_id: i64) -> Result<Vec<Changelog>, AuthError> {
        let access_token = self
            .auth
            .get_access_token()
            .await?;

        let response = self.client.get(format!("{}/changelogs?latestSyncedId={}", self.base_url, latest_changelog_id))
            .bearer_auth(API_KEY)
            .send()
            .await
            .map_err(|e| AuthError::APIRequestError(e.to_string()+" #RESPONSE"))?;

        let changelogs= response
            .json()
            .await
            .map_err(|e| AuthError::APIRequestError(e.to_string()+" #CHANGELOGS"))?;
        println!("Fetched changelogs: {:?}", changelogs);
        Ok(changelogs)
    }
}
