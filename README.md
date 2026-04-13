# 🛠️ FemtoClaw Client SDK

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

The **FemtoClaw SDK** provides the official programmatic bindings for integrating external applications with a FemtoClaw runtime. It handles all protocol serialization, asynchronous networking, and cluster coordination logic.

---

## 🚀 Key Features

- **Programmatic Interaction**: Send prompts to the autonomous loop and receive validated results.
- **Remote Capability Execution**: Directly trigger authorized capabilities on remote nodes.
- **Cluster Awareness**: Automatically handle node discovery and state synchronization.
- **Async-First Design**: Built on `tokio` and `reqwest` for high-performance integration.

---

## 💻 Usage Example

```rust
use femtoclaw_sdk::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize the client targeting a remote node
    let client = Client::new("http://node-01.internal:8080");

    // 2. Submit an instruction to the autonomous agent
    let response = client.send_message("Audit the security posture of this node.").await?;

    if let Some(msg) = response.message {
        println!("Agent Response: {}", msg);
    }

    Ok(())
}
```

---

## 📦 Installation

```toml
[dependencies]
femtoclaw-sdk = "1.0.3"
```

---

## 📄 Related Specifications
- **[FC-ABI-0001: Application Binary Interface](../femtoclaw-spec/FC-ABI-0001-Application_Binary_Interface_ABI_Specification.md)**
- **[FC-SDK-0001: Capability SDK Specification](../femtoclaw-spec/FC-SDK-0001-Capability_SDK_Specification.md)**

Copyright © 2026 FemtoClaw Project.
