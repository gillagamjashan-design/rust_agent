//! Integration tests for AgentBridge

use rusty_ide::{AgentBridge, AgentContext, AgentResponse};
use std::fs;
use std::time::Duration;

#[test]
fn test_bridge_initialization() {
    let bridge = AgentBridge::new();
    assert!(bridge.is_ok(), "Bridge should initialize successfully");

    let bridge = bridge.unwrap();
    assert!(
        bridge.agent_dir().exists(),
        "Agent directory should be created"
    );
}

#[test]
fn test_send_request() {
    let bridge = AgentBridge::new().unwrap();

    let context = AgentContext::new("Test query".to_string())
        .with_workspace("/tmp/test".to_string());

    let result = bridge.send_request(context);
    assert!(result.is_ok(), "Should send request successfully");

    assert!(
        bridge.request_path().exists(),
        "Request file should be created"
    );

    // Clean up
    let _ = bridge.clear();
}

#[test]
fn test_clear() {
    let bridge = AgentBridge::new().unwrap();

    let context = AgentContext::new("Test".to_string());
    bridge.send_request(context).unwrap();

    let result = bridge.clear();
    assert!(result.is_ok());
    assert!(!bridge.request_path().exists());
}
