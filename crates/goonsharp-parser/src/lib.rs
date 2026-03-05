/// # GoonSharp Parser
///
/// The core parsing infrastructure for the GoonSharp programming language.
///
/// This crate provides:
/// - **Lexer** (`lexer`): Tokenizes `.goons` source files using chumsky.
/// - **AST** (`ast`): The abstract syntax tree representing GoonSharp programs.
/// - **Parser** (`parser`): Transforms token streams into the AST.
/// - **Error reporting** (`error`): Beautiful, goon-flavored error messages using ariadne.
///
/// ## Example
/// ```rust,no_run
/// use goonsharp_parser::{lex, parse, error};
///
/// let src = r#"goonsesh main() { goonprint!("gooning"); }"#;
/// let (tokens, lex_errors) = lex(src);
/// if !lex_errors.is_empty() {
///     error::report_lex_errors(src, "test.goons", lex_errors);
/// }
/// if let Some(tokens) = tokens {
///     let (ast, parse_errors) = parse(tokens, src.len());
///     if !parse_errors.is_empty() {
///         error::report_parse_errors(src, "test.goons", parse_errors);
///     }
/// }
/// ```

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

use chumsky::prelude::*;
use chumsky::Stream;

use token::{Span, Token};

/// Lex a GoonSharp source string into tokens.
///
/// Returns `(Option<Vec<(Token, Span)>>, Vec<LexError>)`.
pub fn lex(src: &str) -> (Option<Vec<(Token, Span)>>, Vec<Simple<char>>) {
    lexer::lexer().parse_recovery(src)
}

/// Parse a token stream into a GoonSharp AST.
///
/// `src_len` is the length of the original source string (for end-of-input span).
pub fn parse(
    tokens: Vec<(Token, Span)>,
    src_len: usize,
) -> (Option<ast::Program>, Vec<Simple<Token>>) {
    let stream = Stream::from_iter(src_len..src_len + 1, tokens.into_iter());
    parser::program_parser().parse_recovery(stream)
}

/// Convenience: lex + parse in one call.
pub fn compile_to_ast(
    src: &str,
    filename: &str,
) -> Result<ast::Program, ()> {
    let (tokens, lex_errors) = lex(src);

    if !lex_errors.is_empty() {
        error::report_lex_errors(src, filename, lex_errors);
        return Err(());
    }

    let tokens = match tokens {
        Some(t) => t,
        None => return Err(()),
    };

    let (ast, parse_errors) = parse(tokens, src.len());

    if !parse_errors.is_empty() {
        error::report_parse_errors(src, filename, parse_errors);
        return Err(());
    }

    ast.ok_or(())
}
