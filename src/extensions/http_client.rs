use anyhow::{Context, Result};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// HTTP client for extension registry interactions
pub struct HttpClient {
    client: Client,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
}

/// HTTP error types with detailed status code handling
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Extension not found: {0}")]
    NotFound(String),
    
    #[error("Rate limit exceeded. Try again later")]
    RateLimited,
    
    #[error("Registry server error: {status}")]
    ServerError { status: u16 },
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Request timeout")]
    Timeout,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(format!("pm-extension-client/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            client,
            base_url,
            timeout: Duration::from_secs(30),
            max_retries: 3,
        })
    }
    
    /// Create a new HTTP client with custom configuration
    pub fn with_config(base_url: String, timeout: Duration, max_retries: u32) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(format!("pm-extension-client/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            client,
            base_url,
            timeout,
            max_retries,
        })
    }
    
    /// Perform a GET request with retry logic
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, HttpError> {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'));
        
        for attempt in 0..=self.max_retries {
            match self.client.get(&url).send().await {
                Ok(response) => {
                    return self.handle_response(response).await;
                }
                Err(e) if attempt < self.max_retries => {
                    // Retry on network errors
                    if e.is_timeout() || e.is_connect() {
                        tokio::time::sleep(Duration::from_millis(1000 * (2_u64.pow(attempt)))).await;
                        continue;
                    }
                    return Err(HttpError::NetworkError(e.to_string()));
                }
                Err(e) => {
                    if e.is_timeout() {
                        return Err(HttpError::Timeout);
                    }
                    return Err(HttpError::NetworkError(e.to_string()));
                }
            }
        }
        
        Err(HttpError::NetworkError("Max retries exceeded".to_string()))
    }
    
    /// Download binary data from a URL
    pub async fn download_binary(&self, url: &str) -> Result<Vec<u8>, HttpError> {
        for attempt in 0..=self.max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    let status = response.status();
                    match status {
                        StatusCode::OK => {
                            return response.bytes().await
                                .map_err(|e| HttpError::NetworkError(e.to_string()))
                                .map(|bytes| bytes.to_vec());
                        }
                        StatusCode::NOT_FOUND => {
                            return Err(HttpError::NotFound(url.to_string()));
                        }
                        StatusCode::TOO_MANY_REQUESTS => {
                            return Err(HttpError::RateLimited);
                        }
                        _ if status.is_server_error() => {
                            return Err(HttpError::ServerError { status: status.as_u16() });
                        }
                        _ => {
                            return Err(HttpError::NetworkError(format!("Unexpected status: {}", status)));
                        }
                    }
                }
                Err(e) if attempt < self.max_retries => {
                    // Retry on network errors
                    if e.is_timeout() || e.is_connect() {
                        tokio::time::sleep(Duration::from_millis(1000 * (2_u64.pow(attempt)))).await;
                        continue;
                    }
                    return Err(HttpError::NetworkError(e.to_string()));
                }
                Err(e) => {
                    if e.is_timeout() {
                        return Err(HttpError::Timeout);
                    }
                    return Err(HttpError::NetworkError(e.to_string()));
                }
            }
        }
        
        Err(HttpError::NetworkError("Max retries exceeded".to_string()))
    }
    
    /// Handle HTTP response and convert to typed result
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, HttpError> {
        let status = response.status();
        
        match status {
            StatusCode::OK => {
                let text = response.text().await
                    .map_err(|e| HttpError::NetworkError(e.to_string()))?;
                
                serde_json::from_str::<T>(&text)
                    .map_err(|e| HttpError::InvalidResponse(format!("JSON parse error: {}", e)))
            }
            StatusCode::NOT_FOUND => {
                Err(HttpError::NotFound("Resource not found".to_string()))
            }
            StatusCode::UNAUTHORIZED => {
                Err(HttpError::AuthenticationFailed)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(HttpError::RateLimited)
            }
            _ if status.is_server_error() => {
                Err(HttpError::ServerError { status: status.as_u16() })
            }
            _ => {
                let text = response.text().await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(HttpError::NetworkError(format!("HTTP {}: {}", status, text)))
            }
        }
    }
    
    /// Check if a URL is reachable
    pub async fn health_check(&self) -> Result<bool, HttpError> {
        let url = format!("{}/health", self.base_url.trim_end_matches('/'));
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                if e.is_timeout() {
                    Err(HttpError::Timeout)
                } else {
                    Err(HttpError::NetworkError(e.to_string()))
                }
            }
        }
    }
}

/// Convert HttpError to anyhow::Error for integration with existing error handling
impl From<HttpError> for anyhow::Error {
    fn from(err: HttpError) -> Self {
        anyhow::Error::new(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
    
    #[tokio::test]
    async fn test_successful_get_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "success"
            })))
            .mount(&mock_server)
            .await;
        
        let client = HttpClient::new(mock_server.uri()).unwrap();
        let result: serde_json::Value = client.get("test").await.unwrap();
        
        assert_eq!(result["message"], "success");
    }
    
    #[tokio::test]
    async fn test_not_found_error() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/notfound"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;
        
        let client = HttpClient::new(mock_server.uri()).unwrap();
        let result: Result<serde_json::Value, HttpError> = client.get("notfound").await;
        
        assert!(matches!(result, Err(HttpError::NotFound(_))));
    }
    
    #[tokio::test]
    async fn test_rate_limit_error() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/rate-limited"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&mock_server)
            .await;
        
        let client = HttpClient::new(mock_server.uri()).unwrap();
        let result: Result<serde_json::Value, HttpError> = client.get("rate-limited").await;
        
        assert!(matches!(result, Err(HttpError::RateLimited)));
    }
    
    #[tokio::test]
    async fn test_server_error() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/server-error"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;
        
        let client = HttpClient::new(mock_server.uri()).unwrap();
        let result: Result<serde_json::Value, HttpError> = client.get("server-error").await;
        
        assert!(matches!(result, Err(HttpError::ServerError { status: 500 })));
    }
}