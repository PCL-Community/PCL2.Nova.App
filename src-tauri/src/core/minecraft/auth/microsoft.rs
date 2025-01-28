const CLIENT_ID: &str = "";

pub async fn device_auth() -> Result<String, String> {
    let request_uri= "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode".to_string();
    let request_body= "clientid=".to_string() + CLIENT_ID + "&scope=XboxLive.signin%20offline_access";
    let http_client = reqwest::Client::new();
    let request = http_client.post(request_uri)
                                    .body(request_body)
                                    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    let response = request.send().await.unwrap();
    if response.status() != reqwest::StatusCode::OK {
        return Err("Invalid request.".to_string());
    }
    return Ok(response.text().await.unwrap());
}

pub async fn user_auth(device_code: &String, interval: Option<i32>) -> Result<String, String> {
    let request_uri = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string();
    let request_body = 
        "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id=".to_string() + CLIENT_ID
        + "&device_code=" + device_code;
    let http_client = reqwest::Client::new();
    let request = http_client.post(request_uri)
                                    .body(request_body)
                                    .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    
    let response = request.send().await.unwrap();
    return Ok("".to_string());
}

pub async fn refresh() -> Result<(), ()> {
    return Ok(());
}
