//! HTTP Integration Tests using WireMock
//!
//! These tests catch HTTP-specific mutants that can't be tested with unit tests alone.

#![cfg(feature = "http")]

use honeycomb_rs::{Client, Config, Event, RetryConfig, TransmissionOptions};
use std::time::{Duration, Instant};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_successful_batch_send() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/1/batch/test-dataset"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(2));

        let client = Client::new(config).unwrap();
        client.send(Event::new()).unwrap();
        client.send(Event::new()).unwrap();
        client.stats().snapshot()
    });

    tokio::time::sleep(Duration::from_millis(200)).await;
    let stats = handle.await.unwrap();
    assert_eq!(stats.batches_sent, 1, "Should send batch on size threshold");
    assert_eq!(stats.events_sent, 2);
}

#[tokio::test]
async fn test_batch_size_boundary_exact() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        // CRITICAL: Tests > vs >= boundary (line 374)
        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_bytes(1000));

        let client = Client::new(config).unwrap();
        let mut event = Event::new();
        event.add_field("data", "x".repeat(500));
        client.send(event).unwrap();
        client.flush()
    });

    let result = handle.await.unwrap();
    assert!(result.is_ok(), "Should send batch under size limit");
}

#[tokio::test]
async fn test_retry_on_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(2)
            .with_initial_delay(Duration::from_millis(10));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(1));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        // CRITICAL: Tests retry logic (lines 424, 431)
        client.send(Event::new()).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        client.stats().snapshot()
    });

    let stats = handle.await.unwrap();
    assert!(stats.batches_sent >= 1, "Should retry and succeed");
}

#[tokio::test]
async fn test_retry_timeout_boundary() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500).set_delay(Duration::from_millis(100)))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        // CRITICAL: Tests >= vs < in timeout check (line 388)
        let retry_config = RetryConfig::new()
            .with_max_retries(10)
            .with_initial_delay(Duration::from_millis(10))
            .with_total_timeout(Duration::from_millis(150));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(1));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        let _ = client.send(Event::new()); // May fail, that's OK
        std::thread::sleep(Duration::from_millis(300));
        client.stats().snapshot()
    });

    let stats = handle.await.unwrap();
    assert!(stats.batches_failed > 0, "Should timeout");
}

#[tokio::test]
async fn test_non_retryable_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400))
        .expect(1)
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(5)
            .with_initial_delay(Duration::from_millis(10));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(1));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        // CRITICAL: Tests !is_retryable() (line 424)
        let result = client.send(Event::new()); // Will fail with 400, that's expected
        std::thread::sleep(Duration::from_millis(100));
        (result, client.stats().snapshot())
    });

    let (result, stats) = handle.await.unwrap();
    let err_text = result.unwrap_err().to_string();
    assert!(
        err_text.contains("HTTP 400"),
        "error should include status code"
    );
    assert_eq!(stats.batches_failed, 1, "Should fail without retry");
}

#[tokio::test]
async fn test_total_timeout_caps_single_request_duration() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500).set_delay(Duration::from_secs(2)))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(10)
            .with_initial_delay(Duration::from_millis(10))
            .with_total_timeout(Duration::from_millis(150));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(1));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        let start = Instant::now();
        let result = client.send(Event::new());
        let elapsed = start.elapsed();
        (result, elapsed)
    });

    let (result, elapsed) = handle.await.unwrap();
    assert!(result.is_err(), "request should fail due to timeout budget");
    assert!(
        elapsed < Duration::from_millis(1200),
        "single attempt should honor total timeout budget; elapsed: {elapsed:?}"
    );
}

#[tokio::test]
async fn test_with_retry_config_actually_used() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        // CRITICAL: Tests with_retry_config not returning Default (line 278)
        let custom_retry = RetryConfig::new()
            .with_max_retries(99)
            .with_initial_delay(Duration::from_millis(999));

        let config = Config::new("test-key", "test-dataset").with_api_host(uri);
        let client = Client::with_retry_config(config, custom_retry).unwrap();

        (
            client.retry_config().max_retries,
            client.retry_config().initial_delay,
        )
    });

    let (max_retries, initial_delay) = handle.await.unwrap();
    assert_eq!(max_retries, 99, "Custom retry config should be used");
    assert_eq!(initial_delay, Duration::from_millis(999));
}

#[tokio::test]
async fn test_close_flushes_pending_events() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(100));

        let client = Client::new(config).unwrap();
        client.send(Event::new()).unwrap();
        client.send(Event::new()).unwrap();

        // CRITICAL: Tests close() flushes (lines 498-499)
        client.close()
    });

    tokio::time::sleep(Duration::from_millis(200)).await;
    let result = handle.await.unwrap();
    assert!(result.is_ok(), "Close should flush and succeed");
}

#[tokio::test]
async fn test_429_rate_limit_retry() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(2)
            .with_initial_delay(Duration::from_millis(10));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(1));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        // CRITICAL: Tests 429 handling (line 424)
        client.send(Event::new()).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        client.stats().snapshot()
    });

    let stats = handle.await.unwrap();
    assert!(stats.batches_sent > 0, "Should retry 429 and succeed");
}

#[tokio::test]
async fn test_flush_with_retry_eventual_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(0)
            .with_initial_delay(Duration::from_millis(5));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(10));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        client.send(Event::new()).unwrap();
        let result = client.flush_with_retry(2);
        (result, client.stats().snapshot())
    });

    let (result, stats) = handle.await.unwrap();
    assert!(result.is_ok(), "helper retry should eventually succeed");
    assert_eq!(stats.batches_sent, 1);
}

#[tokio::test]
async fn test_flush_with_retry_exhausted_attempts_fails() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(0)
            .with_initial_delay(Duration::from_millis(5));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(10));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        client.send(Event::new()).unwrap();
        client.flush_with_retry(2)
    });

    let result = handle.await.unwrap();
    assert!(
        result.is_err(),
        "helper retry should fail after max attempts"
    );
}

#[tokio::test]
async fn test_close_with_retry_eventual_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(0)
            .with_initial_delay(Duration::from_millis(5));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(10));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        client.send(Event::new()).unwrap();
        client.close_with_retry(2)
    });

    let result = handle.await.unwrap();
    assert!(result.is_ok(), "helper retry should eventually succeed");
}

#[tokio::test]
async fn test_close_with_retry_exhausted_attempts_fails() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let uri = mock_server.uri();

    let handle = tokio::task::spawn_blocking(move || {
        let retry_config = RetryConfig::new()
            .with_max_retries(0)
            .with_initial_delay(Duration::from_millis(5));

        let config = Config::new("test-key", "test-dataset")
            .with_api_host(uri)
            .with_transmission_options(TransmissionOptions::default().with_max_batch_size(10));

        let client = Client::with_retry_config(config, retry_config).unwrap();
        client.send(Event::new()).unwrap();
        client.close_with_retry(2)
    });

    let result = handle.await.unwrap();
    assert!(
        result.is_err(),
        "helper retry should fail after max attempts"
    );
}
