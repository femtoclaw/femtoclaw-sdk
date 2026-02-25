//! FemtoClaw Client.

use crate::{error::SdkError, types::*, Message, Response};
use reqwest::Client;
use std::sync::Arc;

pub struct Client {
    inner: Client,
    base_url: String,
    api_key: Option<String>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            inner: Client::new(),
            base_url: base_url.to_string(),
            api_key: None,
        }
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub async fn send_message(&self, message: &str) -> Result<Response, SdkError> {
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

        let response = req.send().await?;

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

        let response = self.inner.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(SdkError::Api(response.status().to_string()));
        }

        let body: serde_json::Value = response.json().await?;
        Ok(body["result"].as_str().unwrap_or("").to_string())
    }
}
