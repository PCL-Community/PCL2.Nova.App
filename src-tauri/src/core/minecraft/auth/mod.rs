use futures_util::TryFutureExt;
use microsoft::{CodePair, Token};
use tauri::{AppHandle, Url, WebviewUrl, WebviewWindowBuilder};
use xbox::XboxToken;

pub mod microsoft;
pub mod xbox;

#[tauri::command]
pub async fn device_auth() -> Result<CodePair, String> {
    microsoft::device_auth().await
        .map_err(
            |e| {
                e.to_string()
            }
        )
}

#[tauri::command]
pub async fn user_login(code_pair: CodePair, handle: AppHandle) -> Result<XboxToken, String> {
    WebviewWindowBuilder::new(
        &handle, 
        "auth", 
        WebviewUrl::External(
            Url::parse("https://www.microsoft.com/link")
            .expect("Failed create an auth window.")
        )
    ).build().map_err(|e| { e.to_string() })?;
    // Poll
    let _token = tokio::spawn(microsoft::user_auth(
        code_pair.device_code.clone().unwrap(), Some(code_pair.interval.unwrap() as u64)
    )).await
        .map_err(|e| { e.to_string() })?.map_err(|e| { e.to_string() })?;
    let _xbox_token = xbox::xbox_live_auth(&_token.access_token).await
        .map_err(|e| { e.to_string() })?;
    Ok(_xbox_token)
}