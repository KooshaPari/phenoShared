//! HTTP client adapter implementing the HttpClient port.

use async_trait::async_trait;
use reqwest::{Client, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use std::time::Duration;
use phenotype_port_interfaces::outbound::http::{HttpClient, HttpRequest, HttpResponse, HttpClientExt};
use crate::error::{AdapterError, Result};

/// HTTP client configuration.
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub max_retries: u32,
    pub user_agent: Option<String>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            max_retries: 3,
            user_agent: Some("phenotype-http-adapter/0.1.0".into()),
        }
    }
}

/// HTTP client adapter using reqwest.
#[derive(Clone)]
pub struct ReqwestHttpClient {
    client: Client,
    config: HttpClientConfig,
}

impl ReqwestHttpClient {
    /// Creates a new HTTP client with the given config.
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .user_agent(config.user_agent.clone().unwrap_or_else(|| "phenotype-http-adapter".into()))
            .build()
            .map_err(|e| AdapterError::Http(e.to_string()))?;

        Ok(Self { client, config })
    }

    /// Creates a new HTTP client with default config.
    pub fn new_default() -> Result<Self> {
        Self::new(HttpClientConfig::default())
    }

    /// Builds a request from an HttpRequest.
    async fn build_request(&self, request: HttpRequest) -> Result<reqwest::Request> {
        let method = match request.method.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            other => return Err(AdapterError::Http(format!("Unknown HTTP method: {}", other))),
        };

        let mut builder = self.client.request(method, &request.url);

        // Add headers
        for (key, value) in request.headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| AdapterError::Http(format!("Invalid header name: {}", e)))?;
            let header_value = HeaderValue::from_str(&value)
                .map_err(|e| AdapterError::Http(format!("Invalid header value: {}", e)))?;
            builder = builder.header(header_name, header_value);
        }

        // Add body
        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        builder.build().map_err(|e| AdapterError::Request(e))
    }

    /// Converts a reqwest Response to HttpResponse.
    async fn convert_response(&self, response: reqwest::Response) -> Result<HttpResponse> {
        let status = response.status().as_u16();
        let headers: std::collections::HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(|e| AdapterError::Request(e))?
            .to_vec();

        Ok(HttpResponse {
            status,
            headers,
            body: Some(body),
        })
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn request(&self, request: HttpRequest) -> Result<HttpResponse> {
        let reqwest_request = self.build_request(request).await?;
        
        let response = self.client
            .execute(reqwest_request)
            .await
            .map_err(|e| AdapterError::Request(e))?;

        self.convert_response(response).await
    }
}

impl HttpClientExt for ReqwestHttpClient {
    fn get(url: &str) -> Result<Self> {
        Self::new_default()
    }
}

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self::new_default().expect("Failed to create default HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HttpClientConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.user_agent.is_some());
    }
}
