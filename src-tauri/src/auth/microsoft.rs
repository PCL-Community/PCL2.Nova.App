#[allow(dead_code)]
const CLIENT_ID: &str = "391fbcc2-29ef-4c2f-82e1-2ed757b47f3c";

use serde::{Deserialize, Serialize};

use crate::core::NovaError;

#[derive(Deserialize, Serialize, Debug)]
pub struct CodePair {
    pub user_code: Option<String>,
    pub device_code: Option<String>,
    pub interval: Option<u32>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Token {
    pub token_type: String,
    pub scope: String,
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn device_auth() -> Result<CodePair, NovaError> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
    let request_body = format!(
        "client_id={}&scope=XboxLive.signin%20offline_access",
        CLIENT_ID
    );
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.insert(
        reqwest::header::USER_AGENT, 
        reqwest::header::HeaderValue::from_str("PCL2 Nova MCore/0.0.0")
            .map_err(|e| {
                NovaError::msg(&e.to_string())
            })?
        );
    let client = reqwest::ClientBuilder::new()
        .default_headers(default_headers)
        .build()
        .map_err(|e| {
            NovaError::msg(&e.to_string())
            })?;
    match client.post(request_uri).body(request_body).send().await {
        Ok(response) => match serde_json::from_str::<CodePair>(
            &response.text().await.map_err(|e| {
                NovaError::msg(&e.to_string())
            })?
        ) {
            Ok(data) => Ok(data),
            Err(err) => Err(NovaError::msg(&format!("Json 解析出错{}", err))),
        },
        Err(_) => Err(NovaError::msg("Failed to get CodePair.")),
    }
}

pub async fn user_auth(device_code: String, interval: Option<u64>) -> Result<Token, NovaError> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string();
    let request_body = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", CLIENT_ID),
        ("device_code", &device_code),
    ];
    let http_client = reqwest::Client::new();
    loop {
        let ref_uri = &request_uri;
        let request = http_client.post(ref_uri).form(&request_body).header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        );
        let response = request.send().await.unwrap();
        let response_text = response.text().await.unwrap();
        if response_text.contains("authorization_pending") {
            std::thread::sleep(std::time::Duration::from_secs(interval.unwrap_or(5)));
            continue;
        } // Polling
        if response_text.contains("access_token") {
            return serde_json::from_str(response_text.as_str())
                .map_err(|e| NovaError::msg(&e.to_string()));
        }
        break;
    }
    Err(NovaError::msg("Polling failed."))
}

pub async fn refresh() -> Result<String, String> {
    Ok("".to_string())
}
