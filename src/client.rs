//! FemtoClaw Client.

use crate::error::SdkError;
use reqwest::Client as HttpClient;

pub struct FemtoClient {
    inner: HttpClient,
    base_url: String,
    api_key: Option<String>,
}

impl FemtoClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            inner: HttpClient::new(),
            base_url: base_url.to_string(),
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub async fn send_message(&self, message: &str) -> Result<crate::types::Response, SdkError> {
        let url = format!("{}/v1/chat", self.base_url);
        
        let request = serde_json::json!({
            "messages": [{
                "role": "user",
                "content": message
            }]
        });

        let mut req = self.inner.post(&url).json(&request);
        
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response: reqwest::Response = req.send().await?;

        if !response.status().is_success() {
            return Err(SdkError::Api(response.status().to_string()));
        }

        let body: serde_json::Value = response.json().await?;
        Ok(serde_json::from_value(body)?)
    }

    pub async fn execute_tool(&self, tool: &str, args: serde_json::Value) -> Result<String, SdkError> {
        let url = format!("{}/v1/tools/execute", self.base_url);
        
        let request = serde_json::json!({
            "tool": tool,
            "args": args
        });

        let response: reqwest::Response = self.inner.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(SdkError::Api(response.status().to_string()));
        }

        let body: serde_json::Value = response.json().await?;
        Ok(body["result"].as_str().unwrap_or("").to_string())
    }
}
