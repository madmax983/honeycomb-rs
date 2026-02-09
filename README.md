# honeycomb-rs

[![CI](https://github.com/madmax983/honeycomb-rs/workflows/CI/badge.svg)](https://github.com/madmax983/honeycomb-rs/actions)
[![codecov](https://codecov.io/gh/madmax983/honeycomb-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/madmax983/honeycomb-rs)
[![Crates.io](https://img.shields.io/crates/v/honeycomb-rs.svg)](https://crates.io/crates/honeycomb-rs)
[![Documentation](https://docs.rs/honeycomb-rs/badge.svg)](https://docs.rs/honeycomb-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

A minimal, security-focused Honeycomb.io client for Rust.

Modern replacement for the unmaintained `libhoney-rust` with:
- Pure crates.io dependencies (no git deps)
- Modern reqwest 0.11+ blocking HTTP support (feature-gated)
- Exponential backoff retry logic
- Event batching for efficiency
- Type-safe configuration

## Installation

```toml
[dependencies]
honeycomb-rs = "0.1"
```

## Quick Start

```rust
use honeycomb_rs::{Client, Config, Event};

let config = Config::new("api-key", "dataset");
let client = Client::new(config)?;

let mut event = Event::new();
event.add_field("message", "Hello from Rust!");
event.add_field("duration_ms", 42);

client.send(event)?;
client.flush()?;
```

## Features

- `default`: Client without HTTP (for testing/mocking)
- `http`: Enable blocking HTTP transmission to Honeycomb API

## Delivery Semantics

`flush()` and `close()` are at-most-once for in-memory buffered events. If
transmission fails, callers should retry using their own upstream source data.
For convenience, `flush_with_retry(max_attempts)` retries transmission of the
already-drained batch with exponential backoff.
`close_with_retry(max_attempts)` provides the same behavior while consuming the
client.

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.83** or newer (latest stable).

## Quality Standards

This project maintains strict quality gates:

- 85%+ code coverage (enforced by CI)
- 80%+ mutation score (validates test effectiveness)
- Clippy pedantic + nursery lints
- Zero warnings policy

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
