/// GoonSharp Error Reporting — ariadne-powered, meme-flavored error messages.
///
/// When you mess up your goon code, you deserve to know in *style*.
use ariadne::{Color, Config, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

use crate::token::Token;

/// Goon-themed error flavor text for different error types.
const GOON_ERRORS: &[&str] = &[
    "bro you literally cannot do that",
    "skill issue detected",
    "this ain't it chief",
    "your goon game is weak",
    "the goon council has rejected your code",
    "post-nut clarity: this code was a mistake",
    "you've been gooning too long, take a break",
    "the sesh has been ruined",
    "unrecoverable goon failure",
    "this would make even the most hardened gooner cry",
];

/// Get a random goon error flavor based on a hash of the span.
fn goon_flavor(span_start: usize) -> &'static str {
    GOON_ERRORS[span_start % GOON_ERRORS.len()]
}

/// Format lexer errors using ariadne with goon flavor.
pub fn report_lex_errors(src: &str, filename: &str, errors: Vec<Simple<char>>) {
    for err in errors {
        let span = err.span();
        let msg = match err.reason() {
            chumsky::error::SimpleReason::Unexpected => {
                format!(
                    "{} — unexpected character{}",
                    goon_flavor(span.start),
                    if let Some(c) = err.found() {
                        format!(" '{}'", c)
                    } else {
                        " (end of file)".to_string()
                    }
                )
            }
            chumsky::error::SimpleReason::Unclosed { span: _, delimiter } => {
                format!(
                    "skill issue: unclosed delimiter '{}'",
                    delimiter
                )
            }
            chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
        };

        let report = Report::build(ReportKind::Error, filename, span.start)
            .with_config(Config::default().with_compact(false))
            .with_message(&msg)
            .with_label(
                Label::new((filename, span))
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            )
            .with_note(format!(
                "{}",
                "hint: check your goon syntax. every goonsesh needs matching braces."
                    .fg(Color::Cyan)
            ))
            .finish();

        report.eprint((filename, Source::from(src))).unwrap();
    }
}

/// Format parser errors using ariadne with goon flavor.
pub fn report_parse_errors(src: &str, filename: &str, errors: Vec<Simple<Token>>) {
    for err in errors {
        let span = err.span();
        let msg = match err.reason() {
            chumsky::error::SimpleReason::Unexpected => {
                let found = err
                    .found()
                    .map(|t| format!("{}", t))
                    .unwrap_or_else(|| "end of file".to_string());
                let expected: Vec<_> = err
                    .expected()
                    .filter_map(|e| e.as_ref().map(|t| format!("{}", t)))
                    .collect();

                if expected.is_empty() {
                    format!(
                        "{} — unexpected '{}'",
                        goon_flavor(span.start),
                        found
                    )
                } else {
                    format!(
                        "{} — found '{}', expected one of: {}",
                        goon_flavor(span.start),
                        found,
                        expected.join(", ")
                    )
                }
            }
            chumsky::error::SimpleReason::Unclosed { span: _, delimiter } => {
                format!(
                    "bro you forgot to close '{}' — the sesh is incomplete!",
                    delimiter
                )
            }
            chumsky::error::SimpleReason::Custom(msg) => {
                format!("{} — {}", goon_flavor(span.start), msg)
            }
        };

        let note = match err.found() {
            Some(Token::RBrace) => "hint: you might have an extra '}'. check your goonsesh blocks.",
            Some(Token::Semi) => "hint: unexpected semicolon. are you gooning too hard with the ';'?",
            Some(Token::Goonsesh) => "hint: 'goonsesh' starts a function. did you forget to close the previous one?",
            _ => match span.start % 5 {
                0 => "hint: check your syntax, king. the goon compiler is strict.",
                1 => "hint: did you mean to use a goon keyword here?",
                2 => "hint: make sure all your goonsesh blocks have matching braces {{ }}.",
                3 => "hint: goon types go after the colon, like `x: i32`.",
                _ => "hint: when in doubt, goon it out. but correctly.",
            },
        };

        let report = Report::build(ReportKind::Error, filename, span.start)
            .with_config(Config::default().with_compact(false))
            .with_message(&msg)
            .with_label(
                Label::new((filename, span.clone()))
                    .with_message(format!("{}", msg.fg(Color::Red)))
                    .with_color(Color::Red),
            )
            .with_note(format!("{}", note.fg(Color::Cyan)))
            .finish();

        report.eprint((filename, Source::from(src))).unwrap();
    }
}

/// Report a semantic / codegen error with goon flavor.
pub fn report_error(src: &str, filename: &str, span: std::ops::Range<usize>, message: &str) {
    let report = Report::build(ReportKind::Error, filename, span.start)
        .with_message(format!("goon error: {}", message))
        .with_label(
            Label::new((filename, span))
                .with_message(format!("{}", message.fg(Color::Red)))
                .with_color(Color::Red),
        )
        .finish();

    report.eprint((filename, Source::from(src))).unwrap();
}

/// Report a warning with goon flavor.
pub fn report_warning(src: &str, filename: &str, span: std::ops::Range<usize>, message: &str) {
    let report = Report::build(ReportKind::Warning, filename, span.start)
        .with_message(format!("goon warning: {}", message))
        .with_label(
            Label::new((filename, span))
                .with_message(format!("{}", message.fg(Color::Yellow)))
                .with_color(Color::Yellow),
        )
        .with_note("this won't stop the sesh, but you should probably fix it")
        .finish();

    report.eprint((filename, Source::from(src))).unwrap();
}
