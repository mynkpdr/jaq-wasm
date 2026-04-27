#![cfg(target_arch = "wasm32")]

use serde_json::json;
use jaq_wasm::{run_json_bytes, run_json_values_json};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_run_json_bytes() {
    let result = run_json_bytes(".a", r#"{"a": 42}"#).unwrap();
    let stdout = String::from_utf8(result).unwrap();
    assert_eq!(stdout, "42\n");
}

#[wasm_bindgen_test]
fn test_run_json_values_json() {
    let result = run_json_values_json(".a", r#"{"a": 42}"#).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed, json!([42]));
}

#[wasm_bindgen_test]
fn test_invalid_filter() {
    let error = run_json_values_json("![[", "null").unwrap_err();
    assert!(error.contains("expected"));
}
