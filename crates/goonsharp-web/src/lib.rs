use wasm_bindgen::prelude::*;
use goonsharp_parser::{lex, parse};
use goonsharp_codegen::transpile;

#[wasm_bindgen]
pub struct CompileResult {
    success: bool,
    rust_code: String,
    errors: String,
}

#[wasm_bindgen]
impl CompileResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn rust_code(&self) -> String {
        self.rust_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> String {
        self.errors.clone()
    }
}

#[wasm_bindgen]
pub fn compile_goonsharp(source: &str) -> CompileResult {
    let (tokens, lex_errors) = lex(source);

    if !lex_errors.is_empty() {
        let error_msgs: Vec<String> = lex_errors
            .iter()
            .map(|e| format!("Lex error: {:?}", e))
            .collect();
        return CompileResult {
            success: false,
            rust_code: String::new(),
            errors: error_msgs.join("\n"),
        };
    }

    let tokens = match tokens {
        Some(t) => t,
        None => {
            return CompileResult {
                success: false,
                rust_code: String::new(),
                errors: "Failed to tokenize".to_string(),
            };
        }
    };

    let (ast, parse_errors) = parse(tokens, source.len());

    if !parse_errors.is_empty() {
        let error_msgs: Vec<String> = parse_errors
            .iter()
            .map(|e| format!("Parse error: {:?}", e))
            .collect();
        return CompileResult {
            success: false,
            rust_code: String::new(),
            errors: error_msgs.join("\n"),
        };
    }

    match ast {
        Some(program) => CompileResult {
            success: true,
            rust_code: transpile(&program),
            errors: String::new(),
        },
        None => CompileResult {
            success: false,
            rust_code: String::new(),
            errors: "Failed to parse program".to_string(),
        },
    }
}

#[wasm_bindgen]
pub fn get_version() -> String {
    "GoonSharp Web Playground v69.0.0".to_string()
}
