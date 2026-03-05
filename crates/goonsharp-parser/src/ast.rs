/// GoonSharp Abstract Syntax Tree
///
/// The AST represents the structure of a GoonSharp program after parsing.
/// Each node carries span information for error reporting with ariadne.

use crate::token::Span;

pub type Spanned<T> = (T, Span);

// ─── Program ─────────────────────────────────────────────────────────────────

/// A complete GoonSharp source file.
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Spanned<Item>>,
}

// ─── Items (top-level declarations) ──────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Struct(StructDef),
    Enum(EnumDef),
    Impl(ImplBlock),
    Trait(TraitDef),
    Use(UsePath),
    Mod(ModDef),
    TypeAlias(TypeAliasDef),
    Const(ConstDef),
    Static(StaticDef),
    /// Attribute on an item: #[...]
    Attributed(Vec<Attribute>, Box<Spanned<Item>>),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub path: String,
    pub args: Option<String>,
    pub span: Span,
}

// ─── Visibility ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Private,
    Public,
    PubCrate,
    PubSuper,
}

// ─── Function ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Function {
    pub visibility: Visibility,
    pub is_async: bool,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Spanned<Type>>,
    pub where_clause: Vec<WherePredicate>,
    pub body: Spanned<Block>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub pattern: Spanned<Pattern>,
    pub ty: Spanned<Type>,
}

#[derive(Debug, Clone)]
pub struct SelfParam {
    pub is_ref: bool,
    pub is_mut: bool,
}

// ─── Generics ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum GenericParam {
    Type {
        name: String,
        bounds: Vec<Spanned<Type>>,
    },
    Lifetime(String),
    Const {
        name: String,
        ty: Spanned<Type>,
    },
}

#[derive(Debug, Clone)]
pub struct WherePredicate {
    pub ty: Spanned<Type>,
    pub bounds: Vec<Spanned<Type>>,
}

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Type {
    /// Simple named type: `i32`, `String`, `Sesh`
    Path(TypePath),
    /// Reference: `&T`, `&mut T`, `&'a T`
    Reference {
        lifetime: Option<String>,
        is_mut: bool,
        inner: Box<Spanned<Type>>,
    },
    /// Tuple: `(A, B, C)`
    Tuple(Vec<Spanned<Type>>),
    /// Array: `[T; N]`
    Array(Box<Spanned<Type>>, Box<Spanned<Expr>>),
    /// Slice: `[T]`
    Slice(Box<Spanned<Type>>),
    /// Function pointer: `fn(A, B) -> C`
    FnPointer {
        params: Vec<Spanned<Type>>,
        ret: Option<Box<Spanned<Type>>>,
    },
    /// Dynamic trait object: `dyn Trait`
    Dyn(Box<Spanned<Type>>),
    /// Impl trait: `impl Trait`
    ImplTrait(Box<Spanned<Type>>),
    /// Inferred: `_`
    Infer,
    /// Never type: `!`
    Never,
    /// Unit type: `()`
    Unit,
}

#[derive(Debug, Clone)]
pub struct TypePath {
    pub segments: Vec<TypePathSegment>,
}

#[derive(Debug, Clone)]
pub struct TypePathSegment {
    pub name: String,
    pub generics: Vec<GenericArg>,
}

#[derive(Debug, Clone)]
pub enum GenericArg {
    Type(Spanned<Type>),
    Lifetime(String),
    Const(Spanned<Expr>),
}

// ─── Struct ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StructDef {
    pub visibility: Visibility,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub fields: StructFields,
    pub where_clause: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub enum StructFields {
    Named(Vec<StructField>),
    Tuple(Vec<Spanned<Type>>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub visibility: Visibility,
    pub name: String,
    pub ty: Spanned<Type>,
}

// ─── Enum ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub visibility: Visibility,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub where_clause: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: StructFields,
    pub discriminant: Option<Spanned<Expr>>,
}

// ─── Impl ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub generics: Vec<GenericParam>,
    pub trait_path: Option<Spanned<Type>>,
    pub self_type: Spanned<Type>,
    pub where_clause: Vec<WherePredicate>,
    pub items: Vec<Spanned<ImplItem>>,
}

#[derive(Debug, Clone)]
pub enum ImplItem {
    Function(Function),
    Type(TypeAliasDef),
    Const(ConstDef),
}

// ─── Trait ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub visibility: Visibility,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub super_traits: Vec<Spanned<Type>>,
    pub where_clause: Vec<WherePredicate>,
    pub items: Vec<Spanned<TraitItem>>,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
    Function(Function),
    /// Required method (no body)
    FunctionSig(FunctionSig),
    Type(TypeAliasDef),
    Const(ConstDef),
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub visibility: Visibility,
    pub is_async: bool,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Spanned<Type>>,
    pub where_clause: Vec<WherePredicate>,
}

// ─── Use ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct UsePath {
    pub visibility: Visibility,
    pub tree: UseTree,
}

#[derive(Debug, Clone)]
pub enum UseTree {
    Path(String, Box<UseTree>),
    Name(String),
    Rename(String, String),
    Glob,
    Group(Vec<UseTree>),
}

// ─── Module ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ModDef {
    pub visibility: Visibility,
    pub name: String,
    /// None = external module (mod foo;), Some = inline module
    pub items: Option<Vec<Spanned<Item>>>,
}

// ─── Type Alias ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TypeAliasDef {
    pub visibility: Visibility,
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub ty: Option<Spanned<Type>>,
}

// ─── Const / Static ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ConstDef {
    pub visibility: Visibility,
    pub name: String,
    pub ty: Spanned<Type>,
    pub value: Spanned<Expr>,
}

#[derive(Debug, Clone)]
pub struct StaticDef {
    pub visibility: Visibility,
    pub is_mut: bool,
    pub name: String,
    pub ty: Spanned<Type>,
    pub value: Spanned<Expr>,
}

// ─── Statements ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Stmt {
    /// `goon x = 5;` or `goonconst y = 10;`
    Let {
        is_mut: bool,
        pattern: Spanned<Pattern>,
        ty: Option<Spanned<Type>>,
        value: Option<Spanned<Expr>>,
    },
    /// Expression with semicolon
    Semi(Spanned<Expr>),
    /// Expression without semicolon (tail expression)
    Expr(Spanned<Expr>),
    /// An item inside a block
    Item(Spanned<Item>),
}

// ─── Patterns ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Pattern {
    /// `_`
    Wildcard,
    /// `x`, `my_var`
    Ident(String),
    /// `mut x`
    MutIdent(String),
    /// `ref x`, `ref mut x`
    RefIdent(bool, String),
    /// `(a, b, c)`
    Tuple(Vec<Spanned<Pattern>>),
    /// `Foo { x, y: z }`
    Struct(String, Vec<FieldPattern>),
    /// `Foo(a, b)` or `Foo::Bar(x)`
    TupleStruct(String, Vec<Spanned<Pattern>>),
    /// `1`, `"hello"`, `gooning`, `no_goon`
    Literal(Literal),
    /// `a | b`
    Or(Vec<Spanned<Pattern>>),
    /// `a @ pattern`
    Binding(String, Box<Spanned<Pattern>>),
    /// `..`
    Rest,
    /// `&pat`, `&mut pat`
    Ref(bool, Box<Spanned<Pattern>>),
    /// `1..=5`
    Range(Option<Box<Spanned<Expr>>>, Option<Box<Spanned<Expr>>>),
    /// Path pattern like `Some(x)` — uses string for simplicity
    Path(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct FieldPattern {
    pub name: String,
    pub pattern: Option<Spanned<Pattern>>,
}

// ─── Expressions ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    /// Literal value
    Literal(Literal),
    /// Variable / path: `x`, `std::io::Result`
    Path(Vec<String>),
    /// Self
    SelfValue,
    /// Binary operation: `a + b`
    Binary {
        left: Box<Spanned<Expr>>,
        op: BinOp,
        right: Box<Spanned<Expr>>,
    },
    /// Unary operation: `!x`, `-x`
    Unary {
        op: UnaryOp,
        expr: Box<Spanned<Expr>>,
    },
    /// Function call: `foo(a, b)`
    Call {
        func: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },
    /// Method call: `x.foo(a, b)`
    MethodCall {
        receiver: Box<Spanned<Expr>>,
        method: String,
        generics: Vec<GenericArg>,
        args: Vec<Spanned<Expr>>,
    },
    /// Field access: `x.field`
    Field {
        expr: Box<Spanned<Expr>>,
        field: String,
    },
    /// Tuple index: `x.0`
    TupleIndex {
        expr: Box<Spanned<Expr>>,
        index: usize,
    },
    /// Index: `x[i]`
    Index {
        expr: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>,
    },
    /// If expression
    If {
        condition: Box<Spanned<Expr>>,
        then_block: Spanned<Block>,
        else_block: Option<Box<Spanned<Expr>>>,
    },
    /// While loop
    While {
        condition: Box<Spanned<Expr>>,
        body: Spanned<Block>,
    },
    /// For loop
    For {
        pattern: Spanned<Pattern>,
        iter: Box<Spanned<Expr>>,
        body: Spanned<Block>,
    },
    /// Infinite loop (`goonforever` / `edging`)
    Loop {
        body: Spanned<Block>,
    },
    /// Match expression
    Match {
        expr: Box<Spanned<Expr>>,
        arms: Vec<MatchArm>,
    },
    /// Block expression
    Block(Block),
    /// Return
    Return(Option<Box<Spanned<Expr>>>),
    /// Break
    Break(Option<Box<Spanned<Expr>>>),
    /// Continue
    Continue,
    /// Closure: `|a, b| a + b` or `goonmove |a| { ... }`
    Closure {
        is_move: bool,
        params: Vec<ClosureParam>,
        ret_type: Option<Spanned<Type>>,
        body: Box<Spanned<Expr>>,
    },
    /// Tuple: `(a, b, c)`
    Tuple(Vec<Spanned<Expr>>),
    /// Array literal: `[1, 2, 3]`
    Array(Vec<Spanned<Expr>>),
    /// Array repeat: `[0; 10]`
    ArrayRepeat {
        value: Box<Spanned<Expr>>,
        count: Box<Spanned<Expr>>,
    },
    /// Struct literal: `Sesh { x: 1, y: 2 }`
    StructLit {
        name: Vec<String>,
        fields: Vec<FieldInit>,
        rest: Option<Box<Spanned<Expr>>>,
    },
    /// Range: `a..b`, `a..=b`, `..b`, `a..`
    Range {
        start: Option<Box<Spanned<Expr>>>,
        end: Option<Box<Spanned<Expr>>>,
        inclusive: bool,
    },
    /// Reference: `&x`, `&mut x`
    Reference {
        is_mut: bool,
        expr: Box<Spanned<Expr>>,
    },
    /// Dereference: `*x`
    Deref(Box<Spanned<Expr>>),
    /// Try operator: `x?`
    Try(Box<Spanned<Expr>>),
    /// Await: `x.goonawait`
    Await(Box<Spanned<Expr>>),
    /// Assignment: `x = 5`
    Assign {
        target: Box<Spanned<Expr>>,
        value: Box<Spanned<Expr>>,
    },
    /// Compound assignment: `x += 5`
    CompoundAssign {
        target: Box<Spanned<Expr>>,
        op: BinOp,
        value: Box<Spanned<Expr>>,
    },
    /// Type cast: `x goonas i32`
    Cast {
        expr: Box<Spanned<Expr>>,
        ty: Spanned<Type>,
    },
    /// Macro call: `println!("hi")`, `vec![1, 2, 3]`
    MacroCall {
        name: String,
        args: String, // raw token string for macro args
    },
    /// GoonPrint: `goonprint!("format", args...)`
    Print {
        format_str: String,
        args: Vec<Spanned<Expr>>,
    },
    /// GoonEprint: `gooneprint!("format", args...)`
    Eprint {
        format_str: String,
        args: Vec<Spanned<Expr>>,
    },
    /// Ruin (panic!): `ruin!("message")`
    Ruin(Option<String>),
    /// PostNutClarity (dbg!): `post_nut_clarity!(expr)`
    PostNutClarity(Box<Spanned<Expr>>),
    /// Vec macro: `goonvec![1, 2, 3]`
    VecMacro(Vec<Spanned<Expr>>),
    /// Grouped expression (parenthesized)
    Paren(Box<Spanned<Expr>>),
    /// Type-qualified path: `<Type as Trait>::method`
    QualifiedPath {
        ty: Spanned<Type>,
        as_trait: Option<Spanned<Type>>,
        item: String,
    },
    /// Raw Rust escape hatch: `rust!{ ... }`
    RawRust(String),
}

#[derive(Debug, Clone)]
pub struct ClosureParam {
    pub pattern: Spanned<Pattern>,
    pub ty: Option<Spanned<Type>>,
}

#[derive(Debug, Clone)]
pub struct FieldInit {
    pub name: String,
    pub value: Option<Spanned<Expr>>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Spanned<Pattern>,
    pub guard: Option<Spanned<Expr>>,
    pub body: Spanned<Expr>,
}

// ─── Block ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Spanned<Stmt>>,
}

// ─── Literals ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Literal {
    Int(String),
    Float(String),
    String(String),
    Char(char),
    Bool(bool),
}

// ─── Operators ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

impl BinOp {
    pub fn as_rust_str(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::Eq => "==",
            BinOp::NotEq => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::LtEq => "<=",
            BinOp::GtEq => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::BitXor => "^",
            BinOp::Shl => "<<",
            BinOp::Shr => ">>",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
}

impl UnaryOp {
    pub fn as_rust_str(&self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::Deref => "*",
        }
    }
}
