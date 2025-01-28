use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct HttpClient {
    client: Client,
}

// 封装点常用方法，不够再加.jpg
impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn post(&self, url: &str, data: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.post(url).data(data).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}