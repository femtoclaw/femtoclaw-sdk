//! FemtoClaw Client.

use crate::error::SdkError;
use crate::types::{ChatRequest, Message, Response, ToolRequest};
use reqwest::Client as HttpClient;

pub struct FemtoClient {
    inner: HttpClient,
    base_url: String,
    api_key: Option<String>,
    timeout: std::time::Duration,
}

impl FemtoClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            inner: HttpClient::new(),
            base_url: base_url.to_string(),
            api_key: None,
            timeout: std::time::Duration::from_secs(30),
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<Response, SdkError> {
        let url = format!("{}/v1/chat", self.base_url);
        
        let mut req = self.inner.post(&url)
            .timeout(self.timeout)
            .json(&request);
        
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response: reqwest::Response = req.send().await?;

        if !response.status().is_success() {
            return Err(SdkError::Api(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let body: serde_json::Value = response.json().await?;
        Ok(serde_json::from_value(body)?)
    }

    pub async fn send_message(&self, message: &str) -> Result<Response, SdkError> {
        let request = ChatRequest::user_message(message);
        self.chat(request).await
    }

    pub async fn send_messages(&self, messages: Vec<Message>) -> Result<Response, SdkError> {
        let request = ChatRequest::new(messages);
        self.chat(request).await
    }

    pub async fn execute_tool(&self, request: ToolRequest) -> Result<String, SdkError> {
        let url = format!("{}/v1/tools/execute", self.base_url);
        
        let mut req = self.inner.post(&url)
            .timeout(self.timeout)
            .json(&request);

        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response: reqwest::Response = req.send().await?;

        if !response.status().is_success() {
            return Err(SdkError::Api(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let body: serde_json::Value = response.json().await?;
        Ok(body["result"].as_str().unwrap_or("").to_string())
    }

    pub async fn health(&self) -> Result<bool, SdkError> {
        let url = format!("{}/health", self.base_url);
        let response = self.inner.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}
