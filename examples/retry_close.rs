//! Retry helper example for honeycomb-rs.
//!
//! Run with: cargo run --example retry_close --features http

use honeycomb_rs::{Client, Config, Event, RetryConfig, TransmissionOptions};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("your-api-key-here", "my-dataset").with_transmission_options(
        TransmissionOptions::default()
            .with_max_batch_size(10)
            .with_batch_timeout(Duration::from_millis(100)),
    );

    let retry_config = RetryConfig::new()
        .with_max_retries(2)
        .with_initial_delay(Duration::from_millis(100))
        .with_total_timeout(Duration::from_secs(5));

    let client = Client::with_retry_config(config, retry_config)?;

    let mut event = Event::new();
    event.add_field("message", "retry helper demo");
    event.add_field("component", "examples/retry_close");
    client.send(event)?;

    // Helper-layer retry: retries sending the drained batch up to 3 attempts.
    client.flush_with_retry(3)?;

    let mut event2 = Event::new();
    event2.add_field("message", "final event before close");
    client.send(event2)?;

    // Same helper-layer retry behavior while consuming the client.
    client.close_with_retry(3)?;

    Ok(())
}
