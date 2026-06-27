mod provider_tests {
    use core_ai_pet::infrastructure::llm::provider::anthropic::{AnthropicProvider, DeltaEvent};
    use core_ai_pet::infrastructure::llm::config::{LLMConfig, LLMParams};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    fn create_test_config(base_url: &str) -> LLMConfig {
        LLMConfig {
            provider: "anthropic".to_string(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            base_url: Some(base_url.to_string()),
            secret_ref: "test-secret".to_string(),
            role: "test".to_string(),
            params: LLMParams {
                temperature: 0.7,
                max_tokens: 4096,
            },
        }
    }

    #[tokio::test]
    async fn test_ping_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "msg_test",
                "type": "message",
                "role": "assistant",
                "content": [{"type": "text", "text": "Hi"}],
                "model": "claude-3-5-sonnet-20241022",
                "stop_reason": "end_turn",
                "usage": {"input_tokens": 10, "output_tokens": 5}
            })))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let result = provider.ping(&config, "test-api-key").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ping_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "type": "error",
                "error": {
                    "type": "authentication_error",
                    "message": "Invalid API key"
                }
            })))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let result = provider.ping(&config, "invalid-key").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unauthorized");
    }

    #[tokio::test]
    async fn test_ping_network_error() {
        let provider = AnthropicProvider::new();
        let config = create_test_config("http://localhost:1"); // Invalid port

        let result = provider.ping(&config, "test-key").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("network_error"));
    }

    #[tokio::test]
    async fn test_invoke_stream_text_delta() {
        let mock_server = MockServer::start().await;

        let sse_response = r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}

data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" world"}}

data: {"type":"message_stop"}

"#;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(sse_response)
                .insert_header("content-type", "text/event-stream"))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let messages = vec![json!({"role": "user", "content": "Hi"})];

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        let result = provider.invoke_stream(
            &config,
            "test-key",
            &messages,
            None,
            move |event| {
                events_clone.lock().unwrap().push(event);
            }
        ).await;

        assert!(result.is_ok());
        let events = events.lock().unwrap();
        assert_eq!(events.len(), 3);

        match &events[0] {
            DeltaEvent::Text { delta } => assert_eq!(delta, "Hello"),
            _ => panic!("Expected Text event"),
        }

        match &events[1] {
            DeltaEvent::Text { delta } => assert_eq!(delta, " world"),
            _ => panic!("Expected Text event"),
        }

        match &events[2] {
            DeltaEvent::Stop { reason } => assert_eq!(reason, "end_turn"),
            _ => panic!("Expected Stop event"),
        }
    }

    #[tokio::test]
    async fn test_invoke_stream_thinking_delta() {
        let mock_server = MockServer::start().await;

        let sse_response = r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"thinking_delta","thinking":"Let me analyze this..."}}

data: {"type":"message_stop"}

"#;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(sse_response)
                .insert_header("content-type", "text/event-stream"))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let messages = vec![json!({"role": "user", "content": "Think about this"})];

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        provider.invoke_stream(
            &config,
            "test-key",
            &messages,
            None,
            move |event| {
                events_clone.lock().unwrap().push(event);
            }
        ).await.unwrap();

        let events = events.lock().unwrap();
        assert_eq!(events.len(), 2);

        match &events[0] {
            DeltaEvent::Thinking { delta } => assert_eq!(delta, "Let me analyze this..."),
            _ => panic!("Expected Thinking event"),
        }
    }

    #[tokio::test]
    async fn test_invoke_stream_usage_events() {
        let mock_server = MockServer::start().await;

        let sse_response = r#"data: {"type":"message_start","message":{"id":"msg_test","usage":{"input_tokens":10,"cache_read_input_tokens":5}}}

data: {"type":"message_delta","usage":{"output_tokens":20}}

data: {"type":"message_stop"}

"#;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(sse_response)
                .insert_header("content-type", "text/event-stream"))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let messages = vec![json!({"role": "user", "content": "Test"})];

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        provider.invoke_stream(
            &config,
            "test-key",
            &messages,
            None,
            move |event| {
                events_clone.lock().unwrap().push(event);
            }
        ).await.unwrap();

        let events = events.lock().unwrap();
        assert_eq!(events.len(), 3);

        match &events[0] {
            DeltaEvent::Usage { input_tokens, cached, .. } => {
                assert_eq!(*input_tokens, Some(10));
                assert_eq!(*cached, Some(5));
            }
            _ => panic!("Expected Usage event"),
        }

        match &events[1] {
            DeltaEvent::Usage { output_tokens, .. } => {
                assert_eq!(*output_tokens, Some(20));
            }
            _ => panic!("Expected Usage event"),
        }
    }

    #[tokio::test]
    async fn test_invoke_stream_error_event() {
        let mock_server = MockServer::start().await;

        let sse_response = r#"data: {"type":"error","error":{"type":"overloaded_error","message":"Service overloaded"}}

"#;

        Mock::given(method("POST"))
            .and(path("/v1/messages"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(sse_response)
                .insert_header("content-type", "text/event-stream"))
            .mount(&mock_server)
            .await;

        let provider = AnthropicProvider::new();
        let config = create_test_config(&mock_server.uri());
        let messages = vec![json!({"role": "user", "content": "Test"})];

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        provider.invoke_stream(
            &config,
            "test-key",
            &messages,
            None,
            move |event| {
                events_clone.lock().unwrap().push(event);
            }
        ).await.unwrap();

        let events = events.lock().unwrap();
        assert_eq!(events.len(), 1);

        match &events[0] {
            DeltaEvent::Error { recoverable, code, message } => {
                assert!(recoverable);
                assert_eq!(code, "overloaded_error");
                assert_eq!(message, "Service overloaded");
            }
            _ => panic!("Expected Error event"),
        }
    }

    #[tokio::test]
    async fn test_estimate_cost_known_models() {
        let provider = AnthropicProvider::new();

        let mut config = create_test_config("http://localhost");
        config.model = "claude-3-5-sonnet-20241022".to_string();
        config.params.max_tokens = 1000;
        let cost = provider.estimate_cost(&config, &[]);
        assert!(cost > 0.0);

        config.model = "claude-3-opus-20240229".to_string();
        let cost = provider.estimate_cost(&config, &[]);
        assert!(cost > 0.0);

        config.model = "claude-3-haiku-20240307".to_string();
        let cost = provider.estimate_cost(&config, &[]);
        assert!(cost > 0.0);
    }

    #[tokio::test]
    async fn test_estimate_cost_unknown_model() {
        let provider = AnthropicProvider::new();
        let mut config = create_test_config("http://localhost");
        config.model = "unknown-model".to_string();

        let cost = provider.estimate_cost(&config, &[]);
        assert_eq!(cost, 0.0);
    }
}
