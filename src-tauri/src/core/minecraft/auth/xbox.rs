pub async fn xbox_live_auth(access_token: &String) -> Result<String, String> {
    let request_url = "https://user.auth.xboxlive.com/user/authenticate".to_string();
    let request_body =  "{\"Properties\":{\"AuthMethod\": \"RPS\",\"SiteName\": \"user.auth.xboxlive.com\",\"RpsTicket\": \"d=".to_string()
              + access_token + "\"},\"RelyingParty\": \"http://auth.xboxlive.com\",\"TokenType\": \"JWT\"}";
    let http_client = reqwest::Client::new();
    let request = http_client
        .post(request_url)
        .body(request_body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json");
    let response = request.send().await.unwrap();
    if response.status() != reqwest::StatusCode::OK {
        return Err("Invalid request.".to_string());
    }
    return Ok("".to_string());
}
