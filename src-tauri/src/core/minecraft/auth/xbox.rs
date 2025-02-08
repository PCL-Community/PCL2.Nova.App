const XBOX_LIVE_AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";

pub async fn xbox_live_auth(access_token: &String) -> anyhow::Result<()> {
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
    Ok(())
}