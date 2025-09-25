use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const CLIENT_ID: &str = "v";
const AUTH_BASE_URL: &str = "https://auth.augmentcode.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentOAuthState {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
    pub creation_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedCode {
    pub code: String,
    pub state: String,
    pub tenant_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AugmentTokenResponse {
    pub access_token: String,
    pub tenant_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountStatus {
    pub is_banned: bool,
    pub status: String,
    pub error_message: Option<String>,
    pub response_code: Option<u16>,
    pub response_body: Option<String>,
    // 调试信息
    pub debug_info: DebugInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    pub request_url: String,
    pub request_headers: std::collections::HashMap<String, String>,
    pub request_body: String,
    pub response_headers: std::collections::HashMap<String, String>,
    pub response_body: String,
    pub response_status_text: String,
}

// 批量检测相关结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub tenant_url: String,
    pub id: Option<String>, // 用于前端识别是哪个token
    pub portal_url: Option<String>, // Portal URL用于获取使用次数信息
}

// Portal信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalInfo {
    pub credits_balance: i32,
    pub expiry_date: Option<String>,
    pub is_active: bool,
    pub has_unlimited_usage: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStatusResult {
    pub token_id: Option<String>, // 对应输入的id
    pub access_token: String, // 保留token用于前端更新
    pub tenant_url: String,
    pub status_result: AccountStatus,
    pub portal_info: Option<PortalInfo>, // Portal信息（如果有）
    pub portal_error: Option<String>, // Portal获取错误（如果有）
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub response_text: String,
    pub request_message: String,
}


#[derive(Debug, Deserialize)]
struct TokenApiResponse {
    access_token: String,
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
    (0..length).map(|_| rng.r#gen()).collect()
}

/// Create OAuth state with code verifier, challenge and state
pub fn create_augment_oauth_state() -> AugmentOAuthState {
    let code_verifier_bytes = generate_random_bytes(32);
    let code_verifier = base64_url_encode(&code_verifier_bytes);
    
    let code_challenge_bytes = sha256_hash(code_verifier.as_bytes());
    let code_challenge = base64_url_encode(&code_challenge_bytes);
    
    let state_bytes = generate_random_bytes(8);
    let state = base64_url_encode(&state_bytes);
    
    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    AugmentOAuthState {
        code_verifier,
        code_challenge,
        state,
        creation_time,
    }
}

/// Generate OAuth authorization URL
pub fn generate_augment_authorize_url(oauth_state: &AugmentOAuthState) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(&format!("{}/authorize", AUTH_BASE_URL))?;
    
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("code_challenge", &oauth_state.code_challenge)
        .append_pair("client_id", CLIENT_ID)
        .append_pair("state", &oauth_state.state)
        .append_pair("prompt", "login");
    
    Ok(url.to_string())
}

/// Parse the authorization code response
pub fn parse_code(code: &str) -> Result<ParsedCode, Box<dyn std::error::Error>> {
    let parsed: ParsedCode = serde_json::from_str(code)?;
    Ok(parsed)
}

/// Get access token using authorization code
pub async fn get_augment_access_token(
    tenant_url: &str,
    code_verifier: &str,
    code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let mut data = HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("client_id", CLIENT_ID);
    data.insert("code_verifier", code_verifier);
    data.insert("redirect_uri", "");
    data.insert("code", code);
    
    let token_url = format!("{}token", tenant_url);
    let response = client
        .post(&token_url)
        .json(&data)
        .send()
        .await?;

    let token_response: TokenApiResponse = response.json().await?;
    Ok(token_response.access_token)
}

/// Complete OAuth flow and return token with tenant URL
pub async fn complete_augment_oauth_flow(
    oauth_state: &AugmentOAuthState,
    code_input: &str,
) -> Result<AugmentTokenResponse, Box<dyn std::error::Error>> {
    let parsed_code = parse_code(code_input)?;

    let token = get_augment_access_token(
        &parsed_code.tenant_url,
        &oauth_state.code_verifier,
        &parsed_code.code,
    ).await?;

    Ok(AugmentTokenResponse {
        access_token: token,
        tenant_url: parsed_code.tenant_url,
    })
}

/// Check account ban status by testing chat-stream API
pub async fn check_account_ban_status(
    token: &str,
    tenant_url: &str,
) -> Result<AccountStatus, String> {
    let client = reqwest::Client::new();



    // Ensure tenant_url ends with a slash
    let base_url = if tenant_url.ends_with('/') {
        tenant_url.to_string()
    } else {
        format!("{}/", tenant_url)
    };

    let api_url = format!("{}find-missing", base_url);

    // Empty request body for find-missing endpoint
    let request_body = serde_json::json!({});

    // Prepare request headers for debugging
    let mut request_headers = HashMap::new();
    request_headers.insert("Content-Type".to_string(), "application/json".to_string());
    request_headers.insert("Authorization".to_string(), format!("Bearer {}", token));

    // Print debug info to console
    println!("=== API Request Debug Info ===");
    println!("URL: {}", api_url);
    println!("Method: POST");
    println!("Headers: {:#?}", request_headers);
    println!("Request Body: {}", request_body.to_string());
    println!("==============================");

    // Send request to find-missing API
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status_code = response.status().as_u16();
    let status_text = response.status().to_string();

    // Collect response headers
    let mut response_headers = HashMap::new();
    for (name, value) in response.headers() {
        response_headers.insert(
            name.to_string(),
            value.to_str().unwrap_or("<invalid utf8>").to_string(),
        );
    }

    // Print response debug info
    println!("=== API Response Debug Info ===");
    println!("Status Code: {} ({})", status_code, status_text);
    println!("Response Headers: {:#?}", response_headers);

    // Read response body
    let response_body = response.text().await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    println!("Response Body: {}", response_body);
    println!("===============================");

    // Create debug info
    let debug_info = DebugInfo {
        request_url: api_url,
        request_headers,
        request_body: request_body.to_string(),
        response_headers,
        response_body: response_body.clone(),
        response_status_text: status_text,
    };

    // Analyze response to determine ban status
    // First check for "suspended" keyword in response body regardless of status code
    if response_body.to_lowercase().contains("suspended") {
        Ok(AccountStatus {
            is_banned: true,
            status: "SUSPENDED".to_string(),
            error_message: Some("Account is suspended based on response content".to_string()),
            response_code: Some(status_code),
            response_body: Some(response_body),
            debug_info,
        })
    } else if response_body.to_lowercase().contains("invalid token") {
        // Special case for invalid token - not banned, just invalid
        Ok(AccountStatus {
            is_banned: false,
            status: "INVALID_TOKEN".to_string(),
            error_message: Some("Token is invalid".to_string()),
            response_code: Some(status_code),
            response_body: Some(response_body),
            debug_info,
        })
    } else if (200..300).contains(&status_code) {
        // Success status and no "suspended" keyword - account is active
        Ok(AccountStatus {
            is_banned: false,
            status: "ACTIVE".to_string(),
            error_message: None,
            response_code: Some(status_code),
            response_body: Some(response_body),
            debug_info,
        })
    } else {
        // Handle different error status codes (without "suspended" keyword)
        let (is_banned, status, error_message) = match status_code {
            401 => (true, "UNAUTHORIZED", "Token is invalid or account is banned"),
            403 => (true, "FORBIDDEN", "Access forbidden - account may be banned"),
            429 => (false, "RATE_LIMITED", "Rate limited - account is active but throttled"),
            500..=599 => (false, "SERVER_ERROR", "Server error - cannot determine ban status"),
            _ => (true, "UNKNOWN_ERROR", "Unknown error - possible ban"),
        };

        Ok(AccountStatus {
            is_banned,
            status: status.to_string(),
            error_message: Some(format!("{}: {}", error_message, response_body)),
            response_code: Some(status_code),
            response_body: Some(response_body),
            debug_info,
        })
    }
}

// 批量检测账号状态
pub async fn batch_check_account_status(
    tokens: Vec<TokenInfo>
) -> Result<Vec<TokenStatusResult>, String> {

    println!("=== Starting batch account status check for {} tokens ===", tokens.len());

    // 创建并发任务并立即spawn
    let mut handles = Vec::new();
    
    for token_info in tokens {
        let token = token_info.access_token.clone();
        let tenant_url = token_info.tenant_url.clone();
        let token_id = token_info.id.clone();
        let portal_url = token_info.portal_url.clone();
        
        let handle = tokio::spawn(async move {
            println!("Checking status for token: {:?}", token_id);
            
            // 并发执行账号状态检测和Portal信息获取
            let (status_result, portal_result) = if let Some(ref portal_url_ref) = portal_url {
                // 如果有portal_url，并行执行两个任务
                let (status_res, portal_res) = tokio::join!(
                    check_account_ban_status(&token, &tenant_url),
                    get_portal_info(portal_url_ref)
                );
                (status_res, Some(portal_res))
            } else {
                // 如果没有portal_url，只执行状态检测
                let status_res = check_account_ban_status(&token, &tenant_url).await;
                (status_res, None)
            };
            
            // 处理账号状态检测结果
            let status_result = match status_result {
                Ok(status) => status,
                Err(err) => {
                    // 如果出错，创建一个错误状态
                    AccountStatus {
                        is_banned: false,
                        status: "ERROR".to_string(),
                        error_message: Some(format!("Failed to check status: {}", err)),
                        response_code: None,
                        response_body: None,
                        debug_info: DebugInfo {
                            request_url: format!("{}find-missing", tenant_url),
                            request_headers: HashMap::new(),
                            request_body: "{}".to_string(),
                            response_headers: HashMap::new(),
                            response_body: format!("Error: {}", err),
                            response_status_text: "Error".to_string(),
                        },
                    }
                }
            };
            
            // 处理Portal信息获取结果
            let (portal_info, portal_error) = match portal_result {
                Some(Ok(info)) => (Some(info), None),
                Some(Err(err)) => (None, Some(err)),
                None => (None, None),
            };

            TokenStatusResult {
                token_id,
                access_token: token,
                tenant_url,
                status_result,
                portal_info,
                portal_error,
            }
        });
        
        handles.push(handle);
    }

    // 并发等待所有任务完成
    let mut results = Vec::new();
    for (index, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => results.push(result),
            Err(err) => {
                eprintln!("Task {} failed: {}", index, err);
                // 创建一个错误状态的结果
                results.push(TokenStatusResult {
                    token_id: Some(format!("task_{}", index)),
                    access_token: "".to_string(),
                    tenant_url: "".to_string(),
                    status_result: AccountStatus {
                        is_banned: false,
                        status: "ERROR".to_string(),
                        error_message: Some(format!("Task execution failed: {}", err)),
                        response_code: None,
                        response_body: None,
                        debug_info: DebugInfo {
                            request_url: "".to_string(),
                            request_headers: HashMap::new(),
                            request_body: "{}".to_string(),
                            response_headers: HashMap::new(),
                            response_body: format!("Task Error: {}", err),
                            response_status_text: "Error".to_string(),
                        },
                    },
                    portal_info: None,
                    portal_error: Some(format!("Task failed: {}", err)),
                });
            }
        }
    }
    
    println!("=== Batch check completed. Results: {} ===", results.len());
    
    Ok(results)
}

// 从Portal URL提取token
fn extract_token_from_portal_url(portal_url: &str) -> Option<String> {
    if let Ok(url) = url::Url::parse(portal_url) {
        url.query_pairs()
            .find(|(key, _)| key == "token")
            .map(|(_, value)| value.into_owned())
    } else {
        None
    }
}

// 获取Portal信息
async fn get_portal_info(portal_url: &str) -> Result<PortalInfo, String> {
    let token = extract_token_from_portal_url(portal_url)
        .ok_or("Failed to extract token from portal URL")?;

    // 获取customer信息
    let customer_url = format!("https://portal.withorb.com/api/v1/customer_from_link?token={}", token);
    
    let client = reqwest::Client::new();
    let customer_response = client
        .get(&customer_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("Failed to get customer info: {}", e))?;

    if !customer_response.status().is_success() {
        return Err(format!("Customer API request failed: {}", customer_response.status()));
    }

    let customer_text = customer_response.text().await
        .map_err(|e| format!("Failed to read customer response: {}", e))?;
    
    let customer_data: serde_json::Value = serde_json::from_str(&customer_text)
        .map_err(|e| format!("Failed to parse customer response: {}", e))?;

    // 提取customer_id和pricing_unit_id
    let customer_id = customer_data["customer"]["id"]
        .as_str()
        .ok_or("Customer ID not found")?;
    
    let pricing_unit_id = customer_data["customer"]["ledger_pricing_units"][0]["id"]
        .as_str()
        .ok_or("Pricing unit ID not found")?;
    

    // 获取ledger summary
    let ledger_url = format!(
        "https://portal.withorb.com/api/v1/customers/{}/ledger_summary?pricing_unit_id={}&token={}",
        customer_id, pricing_unit_id, token
    );

    let ledger_response = client
        .get(&ledger_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("Failed to get ledger info: {}", e))?;

    if !ledger_response.status().is_success() {
        return Err(format!("Ledger API request failed: {}", ledger_response.status()));
    }

    let ledger_text = ledger_response.text().await
        .map_err(|e| format!("Failed to read ledger response: {}", e))?;
    
    let ledger_data: serde_json::Value = serde_json::from_str(&ledger_text)
        .map_err(|e| format!("Failed to parse ledger response: {}", e))?;


    // 解析Portal信息（根据当前返回，credits_balance 为字符串，如 "9.00"）
    let credits_balance: i32 = ledger_data["credits_balance"].as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| v.floor() as i32)
        .unwrap_or(0);
    
    // println removed by request: parsed credits balance

    let mut expiry_date = None;
    let mut is_active = false;

    if let Some(credit_blocks) = ledger_data["credit_blocks"].as_array() {
        if let Some(first_block) = credit_blocks.first() {
            expiry_date = first_block["expiry_date"].as_str().map(|s| s.to_string());
            is_active = first_block["is_active"].as_bool().unwrap_or(false);
        }
    }

    Ok(PortalInfo {
        credits_balance,
        expiry_date,
        is_active,
    })
}