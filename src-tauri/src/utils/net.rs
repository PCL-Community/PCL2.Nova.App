use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use std::error::Error;

pub struct HttpClient {
    client: Client,
}

pub struct HttpResponse {
    pub status: StatusCode,
    pub body: Option<String>,
    pub header: HeaderMap,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }
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
        let status = response.status();
        let body = response.text().await?;
        Ok(HttpResponse {
            status,
            body: Some(body),
            header,
        })
    }

    pub async fn head(&self, url: &str) -> Result<HttpResponse, Box<dyn Error>> {
        let response = self.client.head(url).send().await?;
        let header = response.headers().clone();
        let status = response.status();
        Ok(HttpResponse {
            status,
            body: None,
            header,
        })
    }

    pub async fn post(&self, url: &str, data: &str) -> Result<HttpResponse, Box<dyn Error>> {
        let response = self.client.post(url).body(data.to_string()).send().await?;
        let header = response.headers().clone();
        let status = response.status();
        let body = response.text().await?;
        Ok(HttpResponse {
            status,
            body: Some(body),
            header,
        })
    }
}
