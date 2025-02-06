#[allow(dead_code)]
const CLIENT_ID: &str = "";

use serde::Deserialize;

use crate::core::utils::net::HttpClient;

#[derive(Deserialize, Debug)]
pub struct CodePair {
    pub user_code: Option<String>,
    pub device_code: Option<String>,
    pub interval: Option<u32>,
    pub error: Option<String>,
}

pub async fn device_auth() -> Result<CodePair, String> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
    let request_body = format!("client_id={}&scope=xbox.signin%20offlice_access", CLIENT_ID);
    let client = HttpClient::new();
    match client.post(&request_uri, &request_body).await {
        Ok(response) => {
            match serde_json::from_str::<CodePair>(&response.body) {
                Ok(data) => return Ok(data),
                Err(err) => {
                    return Err(format!("Json 解析出错{}", err));
                }
            };
        }
        Err(response) => {
            return Err("Failed to get CodePair".to_string());
        }
    }
}

pub async fn user_auth(device_code: &String, interval: Option<u64>) -> Result<String, String> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string();
    let request_body = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", CLIENT_ID),
        ("device_code", device_code),
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
        }
        if response_text.contains("access_token") {
            return Ok(response_text);
        }
        break;
    }
    return Err("Loop request failed.".to_string());
}

pub async fn refresh() -> Result<String, String> {
    return Ok("".to_string());
}
