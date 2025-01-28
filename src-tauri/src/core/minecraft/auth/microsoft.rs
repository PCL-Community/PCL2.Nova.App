#[allow(dead_code)]
const CLIENT_ID: &str = "";

pub async fn device_auth() -> Result<String, String> {
    let request_uri= "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode".to_string();
    let request_body= [("clientid", CLIENT_ID), ("scope","XboxLive.signin%20offline_access")];
    let http_client = reqwest::Client::new();
    let request = http_client.post(request_uri)
                                    .form(&request_body)
                                    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    let response = request.send().await.unwrap();
    if response.status() != reqwest::StatusCode::OK {
        return Err("Invalid request.".to_string());
    }
    return Ok(response.text().await.unwrap());
}

pub async fn user_auth(device_code: &String, interval: Option<u64>) -> Result<String, String> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string();
    let request_body = [("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                                           ("client_id", CLIENT_ID),
                                           ("device_code", device_code)];
    let http_client = reqwest::Client::new();
    loop {
        let ref_uri = &request_uri;
        let request = http_client.post(ref_uri)
                                    .form(&request_body)
                                    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
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
