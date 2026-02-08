# honeycomb-rs

A minimal, security-focused Honeycomb.io client for Rust.

Modern replacement for the unmaintained `libhoney-rust` with:
- ✅ Pure crates.io dependencies (no git deps)
- ✅ Modern reqwest 0.11+ with async/sync support
- ✅ Exponential backoff retry logic
- ✅ Event batching for efficiency
- ✅ Type-safe configuration

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
- `http`: Enable HTTP transmission to Honeycomb API

## License

MIT

