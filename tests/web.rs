#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use jaq_wasm::{run_jaq, run_jaq_values};
use serde_json::json;

#[wasm_bindgen_test]
fn test_run_jaq() {
    let result = run_jaq(".a", r#"{"a": 42}"#).unwrap();
    let stdout = String::from_utf8(result).unwrap();
    assert_eq!(stdout, "42\n");
}

#[wasm_bindgen_test]
fn test_run_jaq_values() {
    let result = run_jaq_values(".a", r#"{"a": 42}"#);
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, json!({"ok": [42]}));
}

#[wasm_bindgen_test]
fn test_invalid_filter() {
    let result = run_jaq_values("![[", "null");
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed.get("error").is_some());
}
