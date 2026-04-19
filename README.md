# 🛠️ FemtoClaw Client SDK: Programmatic Agent Integration

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

The **FemtoClaw SDK** is the official programmatic interface for integrating external applications with the FemtoClaw industrial agent runtime. It handles the complexities of the FemtoClaw Protocol (Spec 03), asynchronous networking, and cluster-aware state synchronization, allowing developers to focus on building high-level automation and user experiences.

Whether you are building a custom dashboard, a mobile interface, or a background service that needs to participate in autonomous execution loops, this SDK provides the type-safe foundations required for industrial reliability.

---

## 🚀 Key Features

- **Autonomous Loop Support**: Submit instructions to the agent and receive validated responses from multi-step reasoning cycles.
- **Remote Capability Execution**: Directly trigger authorized "Claws" on remote runtime nodes.
- **Cluster Awareness**: Built-in support for node discovery and state reconciliation in distributed environments (Spec 41).
- **Strict Protocol Enforcement**: Leverages the shared `femtoclaw-protocol` to ensure all outbound and inbound messages are normative.
- **Async-First Architecture**: Fully integrated with `tokio` and `reqwest` for high-concurrency production environments.

---

## 💻 Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
femtoclaw-sdk = "1.0.3"
```

---

## 📖 Usage Guide

### 1. Connecting to a Remote Node
The `Client` is the primary entry point for all interactions.

```rust
use femtoclaw_sdk::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the client targeting a specific industrial node
    let client = Client::new("http://node-01.internal:8080")
        .with_timeout(std::time::Duration::from_secs(30));

    // Submit an instruction to the autonomous loop
    let response = client.send_message("Audit the security posture of this node.").await?;

    if let Some(msg) = response.message {
        println!("Agent Response: {}", msg);
    }

    Ok(())
}
```

### 2. Handling Multi-Step Responses
The SDK correctly parses both final messages and intermediate tool calls, allowing your application to provide rich feedback to users during long-running tasks.

```rust
match response.output {
    ProtocolOutput::Message(m) => println!("Final: {}", m.content),
    ProtocolOutput::ToolCall(tc) => println!("Executing: {} with args: {:?}", tc.tool, tc.args),
}
```

### 3. Remote Tool Triggering (Authorized Only)
You can directly invoke system capabilities without initiating a full reasoning loop, provided the target node's policy permits the action.

```rust
let result = client.execute_tool("net", json!({
    "url": "https://api.internal/health",
    "method": "GET"
})).await?;
```

---

## 📐 Advanced Configuration

### Authentication & Security
For production environments, the SDK supports secure header injection for API keys or mTLS-based authentication.

```rust
let client = Client::new("https://secure-node.enterprise.com")
    .with_api_key(std::env::var("FEMTO_API_KEY")?);
```

### Cluster Synchronization
The SDK can be configured to participate in state sync broadcasts, ensuring your client application maintains a synchronized view of the agent's history.

---

## 📄 Related Specifications
- **[FC-ABI-0001: Application Binary Interface](../femtoclaw-spec/FC-ABI-0001-Application_Binary_Interface_ABI_Specification.md)**
- **[FC-SDK-0001: Capability SDK Specification](../femtoclaw-spec/FC-SDK-0001-Capability_SDK_Specification.md)**
- **[FC-41: Distributed Runtime Model](../femtoclaw-spec/41-FemtoClaw_Distributed_Runtime_Model_Specification.md)**

Copyright © 2026 FemtoClaw Project.
