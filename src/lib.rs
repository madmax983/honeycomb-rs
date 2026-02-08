//! honeycomb-rs - A minimal, security-focused Honeycomb.io client for Rust
//!
//! Modern replacement for the unmaintained `libhoney-rust` with pure crates.io
//! dependencies and security-focused design.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
// Allow some overly pedantic clippy lints for practical code
#![allow(clippy::cast_possible_truncation)] // Date math uses intentional truncation
#![allow(clippy::cast_possible_wrap)] // Date algorithms use wrap-around behavior
#![allow(clippy::cast_sign_loss)] // Date conversions are mathematically sound
#![allow(clippy::cast_precision_loss)] // Duration math with f64 is acceptable
#![allow(clippy::must_use_candidate)] // Not all getters need must_use
#![allow(clippy::missing_const_for_fn)] // Const fn everywhere is overly restrictive
#![allow(clippy::struct_field_names)] // http_client in Client is fine
#![allow(clippy::significant_drop_tightening)] // Lock scoping is intentional
#![allow(clippy::items_after_statements)] // Constants after code is readable

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
