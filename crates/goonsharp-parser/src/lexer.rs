/// GoonSharp Lexer — built on chumsky for real parsing power.
///
/// Turns raw `.goons` source into a stream of `(Token, Span)` with proper
/// error recovery and span tracking for beautiful ariadne error messages.
use chumsky::prelude::*;

use crate::token::{Span, Token};

/// Build the GoonSharp lexer.
///
/// Returns a chumsky parser that turns `&str` into `Vec<(Token, Span)>`.
pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let int = text::int(10).map(Token::Int);

    let float = text::int(10)
        .then_ignore(just('.'))
        .then(text::digits(10))
        .map(|(whole, frac)| Token::Float(format!("{}.{}", whole, frac)));

    // String literals with escape sequences
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('0').to('\0')),
    );

    let string = just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str);

    let char_lit = just('\'')
        .ignore_then(filter(|c| *c != '\\' && *c != '\'').or(escape))
        .then_ignore(just('\''))
        .map(Token::Char);

    // Lifetime: 'a, 'static, etc. — must come before char literals
    // We handle this by trying lifetime first in appropriate contexts
    let lifetime = just('\'')
        .ignore_then(text::ident())
        .map(Token::Lifetime);

    // Operators (longest match first)
    let op = choice((
        just("<<=").to(Token::ShlEq),
        just(">>=").to(Token::ShrEq),
        just("..=").to(Token::DotDotEq),
        just("&&").to(Token::And),
        just("||").to(Token::Or),
        just("==").to(Token::EqEq),
        just("!=").to(Token::NotEq),
        just("<=").to(Token::LtEq),
        just(">=").to(Token::GtEq),
        just("<<").to(Token::Shl),
        just(">>").to(Token::Shr),
        just("+=").to(Token::PlusEq),
        just("-=").to(Token::MinusEq),
        just("*=").to(Token::StarEq),
        just("/=").to(Token::SlashEq),
    ))
    .or(choice((
        just("%=").to(Token::PercentEq),
        just("&=").to(Token::AmpEq),
        just("|=").to(Token::PipeEq),
        just("^=").to(Token::CaretEq),
        just("->").to(Token::Arrow),
        just("=>").to(Token::FatArrow),
        just("::").to(Token::ColonColon),
        just("..").to(Token::DotDot),
        just("+").to(Token::Plus),
        just("-").to(Token::Minus),
        just("*").to(Token::Star),
        just("/").to(Token::Slash),
        just("%").to(Token::Percent),
        just("=").to(Token::Eq),
        just("<").to(Token::Lt),
    )))
    .or(choice((
        just(">").to(Token::Gt),
        just("!").to(Token::Not),
        just("&").to(Token::Ampersand),
        just("|").to(Token::Pipe),
        just("^").to(Token::Caret),
        just("~").to(Token::Tilde),
        just(".").to(Token::Dot),
        just("?").to(Token::Question),
        just("#").to(Token::Hash),
        just("@").to(Token::At),
    )));

    // Delimiters
    let delim = choice((
        just('(').to(Token::LParen),
        just(')').to(Token::RParen),
        just('{').to(Token::LBrace),
        just('}').to(Token::RBrace),
        just('[').to(Token::LBracket),
        just(']').to(Token::RBracket),
    ));

    // Punctuation
    let punct = choice((
        just(',').to(Token::Comma),
        just(':').to(Token::Colon),
        just(';').to(Token::Semi),
    ));

    // Keywords and identifiers
    // We need to check for goon keywords first (longest match)
    let ident_or_keyword = text::ident().map(|s: String| match s.as_str() {
        // ── Goon macro-style keywords (with ! handled separately) ──
        "goonsesh" => Token::Goonsesh,
        "goon" => Token::Goon,
        "goonconst" => Token::GoonConst,
        "goonloop" => Token::GoonLoop,
        "goonfor" => Token::GoonFor,
        "goonif" => Token::GoonIf,
        "goonelse" => Token::GoonElse,
        "goonreturn" => Token::GoonReturn,
        "gooning" => Token::Gooning,
        "no_goon" => Token::NoGoon,
        "edge" => Token::Edge,
        "no_edge" => Token::NoEdge,
        "goonstruct" => Token::Goonstruct,
        "goonenum" => Token::Goonenum,
        "goonmatch" => Token::GoonMatch,
        "goonimpl" => Token::GoonImpl,
        "goontrait" => Token::GoonTrait,
        "goonasync" => Token::GoonAsync,
        "goonawait" => Token::GoonAwait,
        "coom" => Token::Coom,
        "goonmod" => Token::GoonMod,
        "goonuse" => Token::GoonUse,
        "goonpub" => Token::GoonPub,
        "goonin" => Token::GoonIn,
        "goonas" => Token::GoonAs,
        "goonself" => Token::GoonSelf,
        "GoonSelf" => Token::GoonSelfType,
        "goontype" => Token::GoonType,
        "goonwhere" => Token::GoonWhere,
        "goonforever" => Token::GoonForever,
        "edging" => Token::Edging,
        "nutting" => Token::Nutting,
        "goonmut" => Token::GoonMut,
        "goonref" => Token::GoonRef,
        "goonmove" => Token::GoonMove,
        "goondyn" => Token::GoonDyn,
        "GoonBox" => Token::GoonBox,
        "goonsome" => Token::GoonSome,
        "goonnone" => Token::GoonNone,
        "goonok" => Token::GoonOk,
        "goonerr" => Token::GoonErr,
        "goonunsafe" => Token::GoonUnsafe,
        "goonextern" => Token::GoonExtern,
        "gooncrate" => Token::GoonCrate,
        "goonsuper" => Token::GoonSuper,
        "goonstatic" => Token::GoonStatic,
        "post_nut_clarity" => Token::PostNutClarity,
        // Underscore as a pattern
        "_" => Token::Underscore,
        s => Token::Ident(s.to_string()),
    });

    // Macro-style keywords: goonprint!, ruin!, goonmacro!, etc.
    // These are identifiers followed by !
    let macro_keyword = text::ident()
        .then_ignore(just('!'))
        .map(|s: String| match s.as_str() {
            "goonprint" => Token::GoonPrint,
            "gooneprint" => Token::GoonEprint,
            "goonformat" => Token::GoonFormat,
            "goonmacro" => Token::GoonMacro,
            "ruin" => Token::Ruin,
            "post_nut_clarity" => Token::PostNutClarity,
            "goonvec" => Token::GoonVec,
            // For other macro calls, emit as Ident + Not
            other => Token::Ident(format!("{}!", other)),
        });

    // Comments — skip them
    let line_comment = just('/').then(just('/'))
        .then(take_until(just('\n')))
        .padded()
        .to(());

    let block_comment = just('/').then(just('*'))
        .then(take_until(just('*').then(just('/'))))
        .padded()
        .to(());

    let comment = line_comment.or(block_comment);

    // The full token — try longest matches first
    let token = choice((
        comment.to(()).repeated().at_least(1).ignore_then(
            choice((
                float.clone(),
                int.clone(),
                string.clone(),
                char_lit.clone(),
                macro_keyword.clone(),
                ident_or_keyword.clone(),
                lifetime.clone(),
                op.clone(),
                delim.clone(),
                punct.clone(),
            )),
        ),
        float,
        int,
        string,
        char_lit,
        macro_keyword,
        ident_or_keyword,
        lifetime,
        op,
        delim,
        punct,
    ))
    .map_with_span(|tok, span| (tok, span))
    .padded_by(comment.repeated())
    .padded()
    .recover_with(skip_then_retry_until([]));

    token.repeated().then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_hello_goon() {
        let src = r#"goonsesh main() { goonprint!("hello goon"); }"#;
        let (tokens, errs) = lexer().parse_recovery(src);
        assert!(errs.is_empty(), "Lex errors: {:?}", errs);
        let tokens = tokens.unwrap();
        let kinds: Vec<_> = tokens.iter().map(|(t, _)| t.clone()).collect();
        assert_eq!(kinds[0], Token::Goonsesh);
        assert_eq!(kinds[1], Token::Ident("main".to_string()));
        assert_eq!(kinds[2], Token::LParen);
        assert_eq!(kinds[3], Token::RParen);
        assert_eq!(kinds[4], Token::LBrace);
        assert_eq!(kinds[5], Token::GoonPrint);
        assert_eq!(kinds[9], Token::RBrace);
    }

    #[test]
    fn lex_operators() {
        let src = "a + b * c == d && e || f";
        let (tokens, errs) = lexer().parse_recovery(src);
        assert!(errs.is_empty());
        let tokens = tokens.unwrap();
        let kinds: Vec<_> = tokens.iter().map(|(t, _)| t.clone()).collect();
        assert!(kinds.contains(&Token::Plus));
        assert!(kinds.contains(&Token::Star));
        assert!(kinds.contains(&Token::EqEq));
        assert!(kinds.contains(&Token::And));
        assert!(kinds.contains(&Token::Or));
    }

    #[test]
    fn lex_goon_keywords() {
        let src = "goon x = 42; goonif (x > 0) { goonreturn gooning; }";
        let (tokens, errs) = lexer().parse_recovery(src);
        assert!(errs.is_empty(), "errors: {:?}", errs);
        let tokens = tokens.unwrap();
        let kinds: Vec<_> = tokens.iter().map(|(t, _)| t.clone()).collect();
        assert_eq!(kinds[0], Token::Goon);
        assert_eq!(kinds[4], Token::GoonIf);
        assert_eq!(kinds[10], Token::GoonReturn);
        assert_eq!(kinds[11], Token::Gooning);
    }

    #[test]
    fn lex_struct() {
        let src = "goonstruct Sesh { intensity: i32, duration: f64 }";
        let (tokens, errs) = lexer().parse_recovery(src);
        assert!(errs.is_empty());
        let tokens = tokens.unwrap();
        assert_eq!(tokens[0].0, Token::Goonstruct);
    }
}
