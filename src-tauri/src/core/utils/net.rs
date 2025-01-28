use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

pub struct HttpClient {
    client: Client,
}

pub struct HttpResponse {
    pub body: String,
    pub header: HeaderMap,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let header = response.headers().clone();
        let body = response.text().await?;
        Ok(HttpResponse { body, header })
    }

    pub async fn post(&self, url: &str, data: &str) -> Result<HttpResponse, Box<dyn Error>> {
        let response = self.client.post(url).body(data.to_string()).send().await?;
        let header = response.headers().clone();
        let body = response.text().await?;
        Ok(HttpResponse { body, header })
    }
}