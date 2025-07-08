use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const CLIENT_ID: &str = "TQpTFeqQLgsQbuzJBpb4eEAMtfRfdvDz";
const CLIENT_SECRET: &str = "B0j62qdEEIGQmW35bFoIeTOpMmrfDwlD";
const REDIRECT_URI: &str = "http://localhost:8765/oauth/callback";
const AUTHORIZATION_ENDPOINT: &str = "https://connect.linux.do/oauth2/authorize";
const TOKEN_ENDPOINT: &str = "https://connect.linux.do/oauth2/token";
const USER_ENDPOINT: &str = "https://connect.linux.do/api/user";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthState {
    pub state: String,
    pub creation_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenApiResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumUser {
    pub id: u64,
    pub username: String,
    pub name: Option<String>,
    pub avatar_template: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub access_token: String,
    pub user_info: ForumUser,
}



/// Base64 URL encode without padding
fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Create SHA256 hash
fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generate random bytes
fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

/// Create OAuth state for forum login
pub fn create_oauth_state() -> OAuthState {
    let state_bytes = generate_random_bytes(16);
    let state = base64_url_encode(&state_bytes);

    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    OAuthState {
        state,
        creation_time,
    }
}

/// Generate OAuth authorization URL for forum
pub fn generate_authorize_url(oauth_state: &OAuthState) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(AUTHORIZATION_ENDPOINT)?;

    url.query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", REDIRECT_URI)
        .append_pair("state", &oauth_state.state);

    Ok(url.to_string())
}

/// Get access token using authorization code
pub async fn get_access_token(
    code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut data = HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("code", code);
    data.insert("redirect_uri", REDIRECT_URI);

    let response = client
        .post(TOKEN_ENDPOINT)
        .form(&data)
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("Token request failed: {}", error_text).into());
    }

    let token_response: TokenApiResponse = response.json().await?;
    Ok(token_response.access_token)
}

/// Get user information using access token
pub async fn get_user_info(
    access_token: &str,
) -> Result<ForumUser, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(USER_ENDPOINT)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("User info request failed: {}", error_text).into());
    }

    let user_info: ForumUser = response.json().await?;
    Ok(user_info)
}

/// Complete OAuth flow and return login result
pub async fn complete_oauth_flow(
    oauth_state: &OAuthState,
    code: &str,
    state: &str,
) -> Result<LoginResult, Box<dyn std::error::Error>> {
    // Verify state parameter
    if state != oauth_state.state {
        return Err("State parameter mismatch".into());
    }

    // Get access token
    let access_token = get_access_token(code).await?;

    // Get user information
    let user_info = get_user_info(&access_token).await?;

    Ok(LoginResult {
        access_token,
        user_info,
    })
}
