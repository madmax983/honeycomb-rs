//! Basic usage example for honeycomb-rs
//!
//! This example demonstrates how to:
//! - Create a Honeycomb client with configuration
//! - Create and send events with various field types
//! - Flush the buffer to ensure delivery
//!
//! Run with: cargo run --example `basic_usage` --features http

use honeycomb_rs::{Client, Config, Event, Options, TransmissionOptions};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Honeycomb-rs Basic Usage Example ===\n");

    // Step 1: Create configuration
    // In production, you would load the API key from environment variables
    // or a secure configuration system, not hardcode it!
    println!("1. Creating Honeycomb configuration...");

    let options = Options::new("your-api-key-here", "my-dataset")
        .with_api_host("https://api.honeycomb.io")
        .with_sample_rate(1); // Send all events (no sampling)

    let transmission_options = TransmissionOptions::default()
        .with_max_batch_size(50) // Send batch when 50 events collected
        .with_batch_timeout(Duration::from_millis(100)) // Or after 100ms
        .with_pending_work_capacity(10000); // Queue capacity

    let config = Config::new("your-api-key-here", "my-dataset")
        .with_options(options)
        .with_transmission_options(transmission_options);

    println!("   ✓ Configuration created\n");

    // Step 2: Create the Honeycomb client
    println!("2. Initializing Honeycomb client...");
    let client = Client::new(config)?;
    println!("   ✓ Client initialized\n");

    // Step 3: Send some events simulating a web application
    println!("3. Sending telemetry events...\n");

    // Event 1: Successful HTTP request
    {
        let mut event = Event::new();
        event.add_field("event_type", "http_request");
        event.add_field("method", "GET");
        event.add_field("path", "/api/users");
        event.add_field("status_code", 200);
        event.add_field("duration_ms", 45);
        event.add_field("user_agent", "Mozilla/5.0");
        event.add_field("success", true);

        client.send(event)?;
        println!("   ✓ Sent: GET /api/users (200 OK, 45ms)");
    }

    // Event 2: Failed request with error details
    {
        let mut event = Event::new();
        event.add_field("event_type", "http_request");
        event.add_field("method", "POST");
        event.add_field("path", "/api/orders");
        event.add_field("status_code", 500);
        event.add_field("duration_ms", 1250);
        event.add_field("error", "Database connection timeout");
        event.add_field("success", false);

        client.send(event)?;
        println!("   ✓ Sent: POST /api/orders (500 Error, 1250ms)");
    }

    // Event 3: Slow query warning
    {
        let mut event = Event::new();
        event.add_field("event_type", "database_query");
        event.add_field("query_type", "SELECT");
        event.add_field("table", "orders");
        event.add_field("duration_ms", 3500);
        event.add_field("rows_returned", 15000);
        event.add_field("slow_query", true);

        client.send(event)?;
        println!("   ✓ Sent: Database query warning (3500ms)");
    }

    // Event 4: User authentication event
    {
        let mut event = Event::new();
        event.add_field("event_type", "user_authentication");
        event.add_field("user_id", "user_12345");
        event.add_field("auth_method", "oauth2");
        event.add_field("success", true);
        event.add_field("ip_address", "192.0.2.1");
        event.add_field("duration_ms", 125);

        client.send(event)?;
        println!("   ✓ Sent: User authentication success");
    }

    // Event 5: Cache hit/miss tracking
    {
        let mut event = Event::new();
        event.add_field("event_type", "cache_access");
        event.add_field("cache_key", "user_profile_12345");
        event.add_field("cache_hit", true);
        event.add_field("duration_ms", 2);
        event.add_field("cache_backend", "redis");

        client.send(event)?;
        println!("   ✓ Sent: Cache hit event");
    }

    println!("\n   Total events buffered: {}", client.buffered_events());

    // Step 4: Flush the buffer to ensure all events are sent
    println!("\n4. Flushing buffer to send all pending events...");
    client.flush()?;
    println!("   ✓ Buffer flushed\n");

    // Step 5: Check statistics
    let stats = client.stats().snapshot();
    println!("5. Statistics:");
    println!("   - Events added:     {}", stats.events_added);
    println!("   - Events sent:      {}", stats.events_sent);
    println!("   - Events dropped:   {}", stats.events_dropped);
    println!("   - Batches sent:     {}", stats.batches_sent);
    println!("   - Batches failed:   {}", stats.batches_failed);

    println!("\n=== Example Complete ===");
    println!("\nNote: Replace 'your-api-key-here' with your actual Honeycomb API key");
    println!("      and update the dataset name to match your Honeycomb configuration.");

    Ok(())
}
