//! SDK Types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
    pub args: serde_json::Value,
}

impl ToolCall {
    pub fn new(tool: impl Into<String>, args: serde_json::Value) -> Self {
        Self {
            tool: tool.into(),
            args,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub message: Option<String>,
    pub tool_call: Option<ToolCall>,
}

impl Response {
    pub fn message(content: impl Into<String>) -> Self {
        Self {
            message: Some(content.into()),
            tool_call: None,
        }
    }

    pub fn tool_call(tool: &str, args: serde_json::Value) -> Self {
        Self {
            message: None,
            tool_call: Some(ToolCall::new(tool, args)),
        }
    }

    pub fn is_message(&self) -> bool {
        self.message.is_some()
    }

    pub fn is_tool_call(&self) -> bool {
        self.tool_call.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
}

impl ChatRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn user_message(content: impl Into<String>) -> Self {
        Self {
            messages: vec![Message::user(content)],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub tool: String,
    pub args: serde_json::Value,
}

impl ToolRequest {
    pub fn new(tool: impl Into<String>, args: serde_json::Value) -> Self {
        Self {
            tool: tool.into(),
            args,
        }
    }
}
