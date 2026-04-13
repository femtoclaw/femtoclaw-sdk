# FemtoClaw SDK

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Stable-green.svg)]()

FemtoClaw Client SDK — programmatic API bindings for industrial agent integration.

## Overview

`femtoclaw-sdk` provides the official client libraries for programmatic access to the FemtoClaw runtime via the `femtoclaw-remote` API. It enables external applications to participate in autonomous agent loops and execute capabilities remotely.

## Features

- **Autonomous Interaction**: Support for multi-step agent conversations.
- **Remote Tool Execution**: Execute authorized capabilities on remote runtime nodes.
- **Type-safe Protocol**: Rust bindings for the FemtoClaw Protocol (Spec 03).
- **Async First**: Built on `tokio` and `reqwest` for high-performance integration.

## Usage

```rust
use femtoclaw_sdk::{Client, Message, Role};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://localhost:8080");

    // Send a message to the remote autonomous loop
    let response = client.send_message("Audit the security of the /tmp directory.").await?;
    println!("Response: {}", response.message.unwrap());
    
    Ok(())
}
```

## Installation

```toml
[dependencies]
femtoclaw-sdk = "1.0.3"
```

## License
Apache 2.0 — see [LICENSE](LICENSE).
