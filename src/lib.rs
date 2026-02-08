//! honeycomb-rs - A minimal, security-focused Honeycomb.io client for Rust
//!
//! Modern replacement for the unmaintained `libhoney-rust` with pure crates.io
//! dependencies and security-focused design.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod batch;
mod client;
mod config;
mod error;
mod event;

pub use batch::{BatchBuffer, BatchStats, BatchStatsSnapshot};
pub use client::{Client, Response, RetryConfig};
pub use config::{Config, Options, TransmissionOptions};
pub use error::{Error, Result};
pub use event::{Event, FieldHolder};

/// API compatibility with libhoney-rust
pub mod client_types {
    pub use super::config::Options;
}

/// API compatibility with libhoney-rust
pub mod transmission {
    pub use super::config::TransmissionOptions as Options;
}
