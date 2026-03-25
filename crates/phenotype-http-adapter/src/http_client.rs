//! # HTTP Client
//!
//! Simple HTTP client implementation using `reqwest`.

use reqwest::Client;

/// Simple HTTP response type.
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Simple HTTP client using reqwest.
#[derive(Clone)]
pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    /// Create a new HTTP client.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Send a GET request.
    pub async fn get(&self, url: &str) -> Result<HttpResponse, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?.to_vec();
        Ok(HttpResponse { status, body })
    }

    /// Send a POST request with body.
    pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, reqwest::Error> {
        let response = self.client.post(url).body(body).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?.to_vec();
        Ok(HttpResponse { status, body })
    }

    /// Send a PUT request with body.
    pub async fn put(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, reqwest::Error> {
        let response = self.client.put(url).body(body).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?.to_vec();
        Ok(HttpResponse { status, body })
    }

    /// Send a DELETE request.
    pub async fn delete(&self, url: &str) -> Result<HttpResponse, reqwest::Error> {
        let response = self.client.delete(url).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?.to_vec();
        Ok(HttpResponse { status, body })
    }
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}
