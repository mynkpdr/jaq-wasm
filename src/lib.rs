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

type CompiledFilter = jaq_core::Filter<data::JustLut<Val>>;

/// Run a jq-like filter against a single JSON input and return CLI-style bytes.
///
/// JavaScript callers typically use the higher-level package wrapper built on top
/// of this lower-level wasm export.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = runJsonBytes))]
pub fn run_json_bytes(filter_source: &str, input_json: &str) -> Result<Vec<u8>, String> {
    let output_values = evaluate_filter(filter_source, input_json)?;
    format_stdout_output(&output_values)
}

/// Run a jq-like filter against a single JSON input and return a JSON array.
///
/// The returned string is valid JSON representing every produced output value.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = runJsonValuesJson))]
pub fn run_json_values_json(filter_source: &str, input_json: &str) -> Result<String, String> {
    let output_values = evaluate_filter(filter_source, input_json)?;
    let json_values = output_values
        .iter()
        .map(jaq_value_to_json)
        .collect::<Result<Vec<_>, _>>()?;

    serde_json::to_string(&json_values)
        .map_err(|error| format!("failed to serialize filter results: {error}"))
}

fn evaluate_filter(filter_source: &str, input_json: &str) -> Result<Vec<Val>, String> {
    let compiled_filter = compile_filter(filter_source)?;
    let input_value = parse_input_json(input_json)?;
    let execution_context =
        Ctx::<data::JustLut<Val>>::new(&compiled_filter.lut, Vars::new([]));

    compiled_filter
        .id
        .run((execution_context, input_value))
        .map(unwrap_valr)
        .map(|result| result.map_err(|error| error.to_string()))
        .collect()
}

fn format_stdout_output(output_values: &[Val]) -> Result<Vec<u8>, String> {
    let mut stdout_bytes = Vec::new();
    let pretty_printer = jaq_json::write::Pp {
        indent: Some("  ".to_owned()),
        sort_keys: false,
        styles: Default::default(),
        sep_space: true,
    };

    for value in output_values {
        jaq_json::write::write(&mut stdout_bytes, &pretty_printer, 0, value)
            .map_err(|error| format!("output write error: {error}"))?;
        stdout_bytes.push(b'\n');
    }

    Ok(stdout_bytes)
}

fn parse_input_json(input_json: &str) -> Result<Val, String> {
    serde_json::from_str(input_json).map_err(|error| format!("invalid JSON input: {error}"))
}

fn compile_filter(filter_source: &str) -> Result<CompiledFilter, String> {
    let arena = Arena::default();
    let loader = Loader::new(built_in_defs());
    let parsed_modules = loader
        .load(
            &arena,
            File {
                code: filter_source,
                path: (),
            },
        )
        .map_err(format_load_errors)?;

    load::import(&parsed_modules, |_| {
        Err("filesystem access is disabled in the WebAssembly build".into())
    })
        .map_err(format_load_errors)?;

    Compiler::default()
        .with_funs(built_in_funs())
        .compile(parsed_modules)
        .map_err(format_compile_errors)
}

fn built_in_defs() -> impl Iterator<Item = load::parse::Def<&'static str>> {
    jaq_core::defs().chain(jaq_std::defs()).chain(jaq_json::defs())
}

fn built_in_funs() -> impl Iterator<Item = jaq_core::native::Fun<data::JustLut<Val>>> {
    jaq_core::funs::<data::JustLut<Val>>()
        .chain(jaq_std::funs::<data::JustLut<Val>>().filter(|(name, _, _)| *name != "env"))
        .chain(jaq_json::funs())
        .chain(jaq_fmts::funs::<data::JustLut<Val>>())
        .chain([jaq_core::native::run(("env", jaq_core::native::v(0), |_context| {
            jaq_core::native::bome(Ok(empty_env_value()))
        }))])
}

fn empty_env_value() -> Val {
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

fn jaq_value_to_json(value: &Val) -> Result<Value, String> {
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
            .map(jaq_value_to_json)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Val::Obj(entries) => {
            let mut object = Map::new();

            for (key, value) in entries.iter() {
                let key = jaq_object_key_to_json(key)?;
                let value = jaq_value_to_json(value)?;
                if object.insert(key.clone(), value).is_some() {
                    return Err(format!("duplicate object key after JSON conversion: {key}"));
                }
            }

            Ok(Value::Object(object))
        }
    }
}

fn jaq_object_key_to_json(value: &Val) -> Result<String, String> {
    match jaq_value_to_json(value)? {
        Value::String(string) => Ok(string),
        other => serde_json::to_string(&other)
            .map_err(|error| format!("failed to stringify object key: {error}")),
    }
}
