/// GoonSharp Token types — every keyword, operator, and literal the language supports.
///
/// This is where the goon magic begins. Each goon-themed keyword maps to a Rust concept,
/// but with 1000% more vibes.
use std::fmt;

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    // ── Goon Keywords ──────────────────────────────────────────────
    Goonsesh,         // fn
    Goon,             // let mut
    GoonConst,        // let (immutable)
    GoonPrint,        // println! (macro-call style)
    GoonEprint,       // eprintln!
    GoonFormat,       // format!
    GoonLoop,         // while
    GoonFor,          // for
    GoonIf,           // if
    GoonElse,         // else
    GoonReturn,       // return
    Gooning,          // true
    NoGoon,           // false
    Edge,             // true  (alt)
    NoEdge,           // false (alt)
    Goonstruct,       // struct
    Goonenum,         // enum
    GoonMatch,        // match
    GoonImpl,         // impl
    GoonTrait,        // trait
    GoonAsync,        // async
    GoonAwait,        // .await
    GoonMacro,        // macro_rules!
    Coom,             // break
    Ruin,             // panic!
    PostNutClarity,   // dbg!
    GoonMod,          // mod
    GoonUse,          // use
    GoonPub,          // pub
    GoonIn,           // in
    GoonAs,           // as
    GoonSelf,         // self
    GoonSelfType,     // Self
    GoonType,         // type
    GoonWhere,        // where
    GoonForever,      // loop (infinite)
    Edging,           // loop (alt, infinite)
    Nutting,          // continue
    GoonMut,          // mut
    GoonRef,          // ref
    GoonMove,         // move
    GoonDyn,          // dyn
    GoonBox,          // Box
    GoonSome,         // Some
    GoonNone,         // None
    GoonOk,           // Ok
    GoonErr,          // Err
    GoonUnsafe,       // unsafe
    GoonExtern,       // extern
    GoonCrate,        // crate
    GoonSuper,        // super
    GoonStatic,       // static
    GoonConst2,       // const (top-level)
    GoonVec,          // vec!

    // ── Literals ───────────────────────────────────────────────────
    Int(String),
    Float(String),
    Str(String),
    Char(char),
    Bool(bool),

    // ── Identifiers & Paths ────────────────────────────────────────
    Ident(String),
    Lifetime(String),    // 'a

    // ── Operators ──────────────────────────────────────────────────
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Eq,          // =
    EqEq,        // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >
    LtEq,        // <=
    GtEq,        // >=
    And,         // &&
    Or,          // ||
    Not,         // !
    Ampersand,   // &
    Pipe,        // |
    Caret,       // ^
    Tilde,       // ~
    Shl,         // <<
    Shr,         // >>
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    AmpEq,       // &=
    PipeEq,      // |=
    CaretEq,     // ^=
    ShlEq,       // <<=
    ShrEq,       // >>=
    Dot,         // .
    DotDot,      // ..
    DotDotEq,    // ..=
    Arrow,       // ->
    FatArrow,    // =>
    Question,    // ?
    ColonColon,  // ::
    Hash,        // #
    Underscore,  // _ (as keyword)

    // ── Delimiters ─────────────────────────────────────────────────
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]

    // ── Punctuation ────────────────────────────────────────────────
    Comma,       // ,
    Colon,       // :
    Semi,        // ;
    At,          // @

    // ── Special ────────────────────────────────────────────────────
    /// A raw Rust identifier (e.g. `r#type`)
    RawIdent(String),
}

impl Token {
    /// Returns the Rust keyword equivalent for goon keywords, if applicable.
    pub fn rust_equivalent(&self) -> Option<&'static str> {
        match self {
            Token::Goonsesh => Some("fn"),
            Token::Goon => Some("let mut"),
            Token::GoonConst => Some("let"),
            Token::GoonPrint => Some("println!"),
            Token::GoonEprint => Some("eprintln!"),
            Token::GoonFormat => Some("format!"),
            Token::GoonLoop => Some("while"),
            Token::GoonFor => Some("for"),
            Token::GoonIf => Some("if"),
            Token::GoonElse => Some("else"),
            Token::GoonReturn => Some("return"),
            Token::Gooning | Token::Edge => Some("true"),
            Token::NoGoon | Token::NoEdge => Some("false"),
            Token::Goonstruct => Some("struct"),
            Token::Goonenum => Some("enum"),
            Token::GoonMatch => Some("match"),
            Token::GoonImpl => Some("impl"),
            Token::GoonTrait => Some("trait"),
            Token::GoonAsync => Some("async"),
            Token::GoonAwait => Some("await"),
            Token::GoonMacro => Some("macro_rules!"),
            Token::Coom => Some("break"),
            Token::Ruin => Some("panic!"),
            Token::PostNutClarity => Some("dbg!"),
            Token::GoonMod => Some("mod"),
            Token::GoonUse => Some("use"),
            Token::GoonPub => Some("pub"),
            Token::GoonIn => Some("in"),
            Token::GoonAs => Some("as"),
            Token::GoonSelf => Some("self"),
            Token::GoonSelfType => Some("Self"),
            Token::GoonType => Some("type"),
            Token::GoonWhere => Some("where"),
            Token::GoonForever | Token::Edging => Some("loop"),
            Token::Nutting => Some("continue"),
            Token::GoonMut => Some("mut"),
            Token::GoonRef => Some("ref"),
            Token::GoonMove => Some("move"),
            Token::GoonDyn => Some("dyn"),
            Token::GoonBox => Some("Box"),
            Token::GoonSome => Some("Some"),
            Token::GoonNone => Some("None"),
            Token::GoonOk => Some("Ok"),
            Token::GoonErr => Some("Err"),
            Token::GoonUnsafe => Some("unsafe"),
            Token::GoonExtern => Some("extern"),
            Token::GoonCrate => Some("crate"),
            Token::GoonSuper => Some("super"),
            Token::GoonStatic => Some("static"),
            Token::GoonConst2 => Some("const"),
            Token::GoonVec => Some("vec!"),
            _ => None,
        }
    }

    /// Is this token a keyword (goon or otherwise)?
    pub fn is_keyword(&self) -> bool {
        self.rust_equivalent().is_some()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Goonsesh => write!(f, "goonsesh"),
            Token::Goon => write!(f, "goon"),
            Token::GoonConst => write!(f, "goonconst"),
            Token::GoonPrint => write!(f, "goonprint!"),
            Token::GoonEprint => write!(f, "gooneprint!"),
            Token::GoonFormat => write!(f, "goonformat!"),
            Token::GoonLoop => write!(f, "goonloop"),
            Token::GoonFor => write!(f, "goonfor"),
            Token::GoonIf => write!(f, "goonif"),
            Token::GoonElse => write!(f, "goonelse"),
            Token::GoonReturn => write!(f, "goonreturn"),
            Token::Gooning => write!(f, "gooning"),
            Token::NoGoon => write!(f, "no_goon"),
            Token::Edge => write!(f, "edge"),
            Token::NoEdge => write!(f, "no_edge"),
            Token::Goonstruct => write!(f, "goonstruct"),
            Token::Goonenum => write!(f, "goonenum"),
            Token::GoonMatch => write!(f, "goonmatch"),
            Token::GoonImpl => write!(f, "goonimpl"),
            Token::GoonTrait => write!(f, "goontrait"),
            Token::GoonAsync => write!(f, "goonasync"),
            Token::GoonAwait => write!(f, "goonawait"),
            Token::GoonMacro => write!(f, "goonmacro!"),
            Token::Coom => write!(f, "coom"),
            Token::Ruin => write!(f, "ruin"),
            Token::PostNutClarity => write!(f, "post_nut_clarity"),
            Token::GoonMod => write!(f, "goonmod"),
            Token::GoonUse => write!(f, "goonuse"),
            Token::GoonPub => write!(f, "goonpub"),
            Token::GoonIn => write!(f, "goonin"),
            Token::GoonAs => write!(f, "goonas"),
            Token::GoonSelf => write!(f, "goonself"),
            Token::GoonSelfType => write!(f, "GoonSelf"),
            Token::GoonType => write!(f, "goontype"),
            Token::GoonWhere => write!(f, "goonwhere"),
            Token::GoonForever => write!(f, "goonforever"),
            Token::Edging => write!(f, "edging"),
            Token::Nutting => write!(f, "nutting"),
            Token::GoonMut => write!(f, "goonmut"),
            Token::GoonRef => write!(f, "goonref"),
            Token::GoonMove => write!(f, "goonmove"),
            Token::GoonDyn => write!(f, "goondyn"),
            Token::GoonBox => write!(f, "GoonBox"),
            Token::GoonSome => write!(f, "goonsome"),
            Token::GoonNone => write!(f, "goonnone"),
            Token::GoonOk => write!(f, "goonok"),
            Token::GoonErr => write!(f, "goonerr"),
            Token::GoonUnsafe => write!(f, "goonunsafe"),
            Token::GoonExtern => write!(f, "goonextern"),
            Token::GoonCrate => write!(f, "gooncrate"),
            Token::GoonSuper => write!(f, "goonsuper"),
            Token::GoonStatic => write!(f, "goonstatic"),
            Token::GoonConst2 => write!(f, "goonconst2"),
            Token::GoonVec => write!(f, "goonvec!"),
            Token::Int(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "\"{}\"", s),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::Bool(b) => write!(f, "{}", b),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Lifetime(s) => write!(f, "'{}", s),
            Token::RawIdent(s) => write!(f, "r#{}", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Eq => write!(f, "="),
            Token::EqEq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::LtEq => write!(f, "<="),
            Token::GtEq => write!(f, ">="),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::Ampersand => write!(f, "&"),
            Token::Pipe => write!(f, "|"),
            Token::Caret => write!(f, "^"),
            Token::Tilde => write!(f, "~"),
            Token::Shl => write!(f, "<<"),
            Token::Shr => write!(f, ">>"),
            Token::PlusEq => write!(f, "+="),
            Token::MinusEq => write!(f, "-="),
            Token::StarEq => write!(f, "*="),
            Token::SlashEq => write!(f, "/="),
            Token::PercentEq => write!(f, "%="),
            Token::AmpEq => write!(f, "&="),
            Token::PipeEq => write!(f, "|="),
            Token::CaretEq => write!(f, "^="),
            Token::ShlEq => write!(f, "<<="),
            Token::ShrEq => write!(f, ">>="),
            Token::Dot => write!(f, "."),
            Token::DotDot => write!(f, ".."),
            Token::DotDotEq => write!(f, "..="),
            Token::Arrow => write!(f, "->"),
            Token::FatArrow => write!(f, "=>"),
            Token::Question => write!(f, "?"),
            Token::ColonColon => write!(f, "::"),
            Token::Hash => write!(f, "#"),
            Token::Underscore => write!(f, "_"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Semi => write!(f, ";"),
            Token::At => write!(f, "@"),
        }
    }
}
