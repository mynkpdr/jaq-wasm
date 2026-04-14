//! Wasm wrapper for jaq.

#![forbid(unsafe_code)]

use jaq_core::compile::{self, Undefined};
use jaq_core::data;
use jaq_core::load::{self, Arena, File, Loader};
use jaq_core::{unwrap_valr, Compiler, Ctx, Vars};
use jaq_json::Val;
use serde_json::{Map, Value};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

type Filter = jaq_core::Filter<data::JustLut<Val>>;

/// Run a jq-like filter against a single JSON value.
///
/// On success, the function returns the CLI stdout bytes for the produced values.
/// On failure, it returns an error string.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run_jaq(filter: &str, input: &str) -> Result<Vec<u8>, String> {
    run_jaq_stdout_impl(filter, input)
}

/// Run a jq-like filter and return a structured JSON envelope for JS callers.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run_jaq_values(filter: &str, input: &str) -> String {
    match run_jaq_values_impl(filter, input) {
        Ok(output) => serde_json::json!({"ok": output}).to_string(),
        Err(error) => serde_json::json!({"error": error}).to_string(),
    }
}

fn run_jaq_values_impl(filter: &str, input: &str) -> Result<Vec<Value>, String> {
    let filter = compile_filter(filter)?;
    let input = parse_input(input)?;
    let ctx = Ctx::<data::JustLut<Val>>::new(&filter.lut, Vars::new([]));

    let outputs = filter
        .id
        .run((ctx, input))
        .map(unwrap_valr)
        .map(|result| result.map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    outputs.iter().map(val_to_json_value).collect()
}

fn run_jaq_stdout_impl(filter: &str, input: &str) -> Result<Vec<u8>, String> {
    let filter = compile_filter(filter)?;
    let input = parse_input(input)?;
    let ctx = Ctx::<data::JustLut<Val>>::new(&filter.lut, Vars::new([]));
    let outputs = filter
        .id
        .run((ctx, input))
        .map(unwrap_valr)
        .map(|result| result.map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut output = Vec::new();
    let pp = jaq_json::write::Pp {
        indent: Some("  ".to_owned()),
        sort_keys: false,
        styles: Default::default(),
        sep_space: true,
    };

    for value in outputs {
        jaq_json::write::write(&mut output, &pp, 0, &value)
            .map_err(|error| format!("output write error: {error}"))?;
        output.push(b'\n');
    }

    Ok(output)
}

fn parse_input(input: &str) -> Result<Val, String> {
    serde_json::from_str(input).map_err(|error| format!("invalid JSON input: {error}"))
}

fn compile_filter(filter: &str) -> Result<Filter, String> {
    let arena = Arena::default();
    let loader = Loader::new(defs());
    let modules = loader
        .load(&arena, File { code: filter, path: () })
        .map_err(format_load_errors)?;

    load::import(&modules, |_| Err("filesystem access is disabled in wasm".into()))
        .map_err(format_load_errors)?;

    Compiler::default()
        .with_funs(funs())
        .compile(modules)
        .map_err(format_compile_errors)
}

fn defs() -> impl Iterator<Item = load::parse::Def<&'static str>> {
    jaq_core::defs().chain(jaq_std::defs()).chain(jaq_json::defs())
}

fn funs() -> impl Iterator<Item = jaq_core::native::Fun<data::JustLut<Val>>> {
    jaq_core::funs::<data::JustLut<Val>>()
        .chain(jaq_std::funs::<data::JustLut<Val>>().filter(|(name, _, _)| *name != "env"))
        .chain(jaq_json::funs())
        .chain(jaq_fmts::funs::<data::JustLut<Val>>())
        .chain([jaq_core::native::run(("env", jaq_core::native::v(0), |_cv| {
            jaq_core::native::bome(Ok(env_value()))
        }))])
}

fn env_value() -> Val {
    Val::obj(Default::default())
}

fn format_load_errors(errs: load::Errors<&str, ()>) -> String {
    let mut messages = Vec::new();

    for (_file, err) in errs {
        match err {
            load::Error::Io(errs) => {
                for (path, error) in errs {
                    messages.push(format!("could not load file {path}: {error}"));
                }
            }
            load::Error::Lex(errs) => {
                for (expected, found) in errs {
                    messages.push(format!(
                        "expected {}, found {found:?}",
                        expected.as_str(),
                    ));
                }
            }
            load::Error::Parse(errs) => {
                for (expected, found) in errs {
                    messages.push(format!(
                        "expected {}, found {found:?}",
                        expected.as_str(),
                    ));
                }
            }
        }
    }

    if messages.is_empty() {
        "load error".into()
    } else {
        messages.join("; ")
    }
}

fn format_compile_errors(errs: compile::Errors<&str, ()>) -> String {
    let mut messages = Vec::new();

    for (_file, errs) in errs {
        for (found, undefined) in errs {
            let message = match undefined {
                Undefined::Filter(arity) => {
                    format!("wrong number of arguments for `{found}` (arity {arity})")
                }
                other => format!("undefined {}", other.as_str()),
            };
            messages.push(message);
        }
    }

    if messages.is_empty() {
        "compile error".into()
    } else {
        messages.join("; ")
    }
}

fn val_to_json_value(value: &Val) -> Result<Value, String> {
    match value {
        Val::Null => Ok(Value::Null),
        Val::Bool(value) => Ok(Value::Bool(*value)),
        Val::Num(number) => {
            let number = number.to_string();
            serde_json::from_str(&number)
                .map_err(|error| format!("number {number} is not valid JSON: {error}"))
        }
        Val::BStr(bytes) | Val::TStr(bytes) => String::from_utf8(bytes.as_ref().to_vec())
            .map(Value::String)
            .map_err(|_| "output contains non-UTF-8 string data".to_string()),
        Val::Arr(values) => values
            .iter()
            .map(val_to_json_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Val::Obj(entries) => {
            let mut object = Map::new();

            for (key, value) in entries.iter() {
                let key = val_to_json_key(key)?;
                let value = val_to_json_value(value)?;
                if object.insert(key.clone(), value).is_some() {
                    return Err(format!("duplicate object key after JSON conversion: {key}"));
                }
            }

            Ok(Value::Object(object))
        }
    }
}

fn val_to_json_key(value: &Val) -> Result<String, String> {
    match val_to_json_value(value)? {
        Value::String(string) => Ok(string),
        other => serde_json::to_string(&other)
            .map_err(|error| format!("failed to stringify object key: {error}")),
    }
}