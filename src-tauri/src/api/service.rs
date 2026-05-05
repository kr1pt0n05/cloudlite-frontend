use std::sync::Arc;
use reqwest::Client;
use crate::auth::error::AuthError;
use crate::auth::service::AuthService;
use crate::db::service::Changelog;

// ToDo: Move this into .env
const API_KEY: &str = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICI2OTVwUHVSYzNpODFsbzJyZ2hyWEJMX2ExbUdzTGhHOFJaY1Q4TVdaeHhrIn0.eyJleHAiOjE3Nzc4NjIzODcsImlhdCI6MTc3Nzg0NDM4NywianRpIjoidHJydGNjOjMyMGUxMjg5LTY2MDktZTViOS02MjQ4LWJkNTQ3MTA5OGU0YSIsImlzcyI6Imh0dHA6Ly9sb2NhbGhvc3Q6ODA4MC9yZWFsbXMvZGV2ZWxvcG1lbnQiLCJhdWQiOlsiZnJvbnRlbmQtY2xpZW50IiwiYWNjb3VudCJdLCJzdWIiOiJmYmRlZjAxZC1iZDY1LTQ1MTgtYjI0NS1hZmJjN2ViZDVhNWMiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJzZXJ2aWNlLWNsaWVudC0wMSIsImFjciI6IjEiLCJhbGxvd2VkLW9yaWdpbnMiOlsiLyoiXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbIm9mZmxpbmVfYWNjZXNzIiwiZGVmYXVsdC1yb2xlcy1kZXZlbG9wbWVudCIsInVtYV9hdXRob3JpemF0aW9uIl19LCJyZXNvdXJjZV9hY2Nlc3MiOnsic2VydmljZS1jbGllbnQtMDEiOnsicm9sZXMiOlsidW1hX3Byb3RlY3Rpb24iXX0sImZyb250ZW5kLWNsaWVudCI6eyJyb2xlcyI6WyJzZXJ2aWNlIl19LCJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImNsaWVudEhvc3QiOiIxNzIuMTkuMC4xIiwicHJlZmVycmVkX3VzZXJuYW1lIjoic2VydmljZS1hY2NvdW50LXNlcnZpY2UtY2xpZW50LTAxIiwiY2xpZW50QWRkcmVzcyI6IjE3Mi4xOS4wLjEiLCJjbGllbnRfaWQiOiJzZXJ2aWNlLWNsaWVudC0wMSJ9.LfeX2fnJP0ZOA6S6tBCcObtfQOE50x9VmPFhSMCn12IgpgW9F-fi6y6rnuTHngcq8--5Q7D4idzJB-XLtzUnF2DkOl6P1azfCn6foFuurfyi_b_ncId1ojsG7vJnsdkbzc1jUjwJcBrV7Csspnj9EyPBcjlnDg3bJh9ZH0BD9BNJ-xdbCOoWpNqpOzmAeg0r-eil4EXvpvc_EUZGVE4luxkJydqv0yNewojaEMfXuD0eKKkWzYXNQLdiRcbQY6S2GY6WTut1qqsL2kl1ZhvQz6m2vPGNXwGjnx4ADaaZSOB89VueXZZ3WGv3I1ASA6eElQ9Jzdqajtucel3UeH1pUw";

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
            .map_err(|e| AuthError::APIRequestError(e.to_string()))?;

        let changelogs: Vec<Changelog> = response
            .json()
            .await
            .map_err(|e| AuthError::APIRequestError(e.to_string()))?;
        Ok(changelogs)
    }
}