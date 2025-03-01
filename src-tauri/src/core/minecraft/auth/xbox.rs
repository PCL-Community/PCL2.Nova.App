use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const XBOX_LIVE_AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";

#[derive(Deserialize, Serialize, Clone)]
pub struct XboxToken {
    #[serde(rename = "IssueInstant")]
    issue_instant: String,
    #[serde(rename = "NotAfter")]
    not_after: String,
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: HashMap<String, Vec<Uhs>>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Uhs {
    uhs: String
}

pub async fn xbox_live_auth(access_token: &String) -> anyhow::Result<XboxToken> {
    let request_body = format!(
        r#"{{"Properties":{{"AuthMethod":"RPS","SiteName":"user.auth.xboxlive.com","RpsTicket":"d={}"}}, "RelyingParty": "http://auth.xboxlive.com", "TokenType": "JWT"}}"#,
        access_token
    );
    
    let http_client = reqwest::Client::new();
    let request = http_client
        .post(XBOX_LIVE_AUTH_URL)
        .body(request_body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json");
    let back = request.send().await?;
    // 检查返回的状态码
    if !back.status().is_success() {
        return Err(anyhow::anyhow!("Failed to authenticate with Xbox Live, status code: {}", back.status()));
    }
    Ok(back.json::<XboxToken>().await?)
}