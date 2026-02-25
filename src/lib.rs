//! FemtoClaw Client SDK.
//!
//! Programmatic API bindings for external integration with FemtoClaw runtime.

pub mod client;
pub mod error;
pub mod types;

pub use client::Client;
pub use error::SdkError;
pub use types::*;
