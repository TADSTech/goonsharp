use wasm_bindgen::prelude::*;
use goonsharp_parser::{lex, parse};
use goonsharp_codegen::transpile;
use serde::Serialize;

#[derive(Serialize)]
pub struct CompileResult {
    success: bool,
    output: Option<String>,
    errors: Vec<String>,
}

#[wasm_bindgen]
pub fn compile_to_json(source: String) -> String {
    let (tokens, lex_errors) = lex(&source);

    if !lex_errors.is_empty() {
        let error_msgs: Vec<String> = lex_errors
            .iter()
            .map(|e| format!("Lex error: {:?}", e))
            .collect();
        let res = CompileResult {
            success: false,
            output: None,
            errors: error_msgs,
        };
        return serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string());
    }

    let tokens = match tokens {
        Some(t) => t,
        None => {
            let res = CompileResult {
                success: false,
                output: None,
                errors: vec!["Failed to tokenize".to_string()],
            };
            return serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string());
        }
    };

    let (ast, parse_errors) = parse(tokens, source.len());

    if !parse_errors.is_empty() {
        let error_msgs: Vec<String> = parse_errors
            .iter()
            .map(|e| format!("Parse error: {:?}", e))
            .collect();
        let res = CompileResult {
            success: false,
            output: None,
            errors: error_msgs,
        };
        return serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string());
    }

    let res = match ast {
        Some(program) => CompileResult {
            success: true,
            output: Some(transpile(&program)),
            errors: vec![],
        },
        None => CompileResult {
            success: false,
            output: None,
            errors: vec!["Failed to parse program".to_string()],
        },
    };

    serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "GoonSharp Web Playground v69.0.0".to_string()
}

/*
How to Mock the File System for the Browser Environment:
WebAssembly in JS runtimes does not have native access to std::fs (it will panic or fail).
To support things like module imports across files:
1. Define a Vfs (Virtual File System) trait in your compiler core:
   trait Vfs { fn read_file(&self, path: &str) -> Option<String>; }
2. For the native build (#[cfg(not(target_arch = "wasm32"))]), implement Vfs via std::fs::read_to_string.
3. For WebAssembly builds, implement Vfs as an in-memory dictionary (HashMap<String, String>).
4. Expose a wasm_bindgen function allowing JavaScript to preload the dictionary before triggering the compile_to_json.
*/
