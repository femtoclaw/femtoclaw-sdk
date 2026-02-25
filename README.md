# FemtoClaw SDK

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

FemtoClaw Client SDK — programmatic API bindings for external integration.

## Overview

`femtoclaw-sdk` provides client libraries for programmatic access to FemtoClaw runtime. It enables external applications and services to integrate with FemtoClaw agents.

This SDK is designed for building custom clients, web interfaces, and automation tools.

## Features

- **HTTP Client**: Send messages toFemtoClaw runtime
- **Tool Execution**: Execute capabilities remotely
- **Async Runtime**: Full async/await support with Tokio
- **Error Handling**: Comprehensive error types
- **Type Safety**: Strongly typed message and response types

## Installation

```toml
[dependencies]
femtoclaw-sdk = "1.0"
```

## Usage

```rust
use femtoclaw_sdk::{Client, Message, Role};

let client = Client::new("http://localhost:8080")
    .with_api_key("your-api-key");

// Send a message
let response = client.send_message("What is the weather?").await?;
println!("Response: {:?}", response);

// Execute a tool
let result = client.execute_tool(
    "web_get",
    serde_json::json!({"url": "https://example.com"})
).await?;
println!("Result: {}", result);
```

## API Reference

### Client

```rust
Client::new(base_url: &str) -> Self
Client::with_api_key(self, api_key: &str) -> Self
Client::send_message(&self, message: &str) -> Result<Response, SdkError>
Client::execute_tool(&self, tool: &str, args: Value) -> Result<String, SdkError>
```

### Types

```rust
// Message types
Message { role: Role, content: String }
Role: System, User, Assistant, Tool

// Tool call types  
ToolCall { tool: String, args: Value }

// Response types
Response { message: Option<String>, tool_call: Option<ToolCall> }
```

## Architecture

```
┌─────────────────────────────────────────────┐
│         External Application               │
│  ┌─────────────────────────────────────┐   │
│  │      femtoclaw-sdk                  │   │
│  │  ┌───────────┐ ┌────────────────┐   │   │
│  │  │  Client   │ │    Types      │   │   │
│  │  └───────────┘ └────────────────┘   │   │
│  └─────────────────────────────────────┘   │
└─────────────────┬───────────────────────────┘
                  │ HTTP/WebSocket
                  ▼
┌─────────────────────────────────────────────┐
│         femtoclaw-remote                   │
│              (API Server)                    │
└─────────────────────────────────────────────┘
```

## Requirements

- Rust 1.75 or later
- serde 1.x
- serde_json 1.x
- reqwest 0.12 (with json, rustls-tls)
- tokio 1.x
- thiserror 1.x

## Related Specifications

- [FC-SDK-0001: Capability SDK Specification](../femtoclaw-spec/FC-SDK-0001-Capability_SDK_Specification.md)
- [FC-DEPLOY-0001: Deployment Specification](../femtoclaw-spec/FC-DEPLOY-0001-FemtoClaw_Deployment_and_Operational_Environment_Specification.md)

## Related Crates

| Crate | Purpose |
|-------|---------|
| `femtoclaw-remote` | HTTP/WebSocket API server |
| `femtoclaw-cli` | Interactive CLI |

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
