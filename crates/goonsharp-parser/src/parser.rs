/// GoonSharp Parser — chumsky-powered recursive descent parser.
///
/// Transforms a token stream from the lexer into a full AST.
/// Handles operator precedence, nesting, and all goon constructs.
use chumsky::prelude::*;

use crate::ast::*;
use crate::token::{Span, Token};

type PErr = Simple<Token>;

// ─── Helper: just a specific token ───────────────────────────────────────────

fn tok(t: Token) -> impl Parser<Token, Token, Error = PErr> + Clone {
    filter(move |t2: &Token| *t2 == t)
        .labelled("token")
}

fn ident() -> impl Parser<Token, String, Error = PErr> + Clone {
    filter_map(|span, tok: Token| match tok {
        Token::Ident(s) => Ok(s),
        _ => Err(Simple::expected_input_found(
            span,
            vec![],
            Some(tok),
        )),
    })
    .labelled("identifier")
}

fn spanned<T: Clone + 'static>(
    p: impl Parser<Token, T, Error = PErr> + Clone,
) -> impl Parser<Token, (T, Span), Error = PErr> + Clone {
    p.map_with_span(|val, span| (val, span))
}

// ─── Visibility ──────────────────────────────────────────────────────────────

fn visibility() -> impl Parser<Token, Visibility, Error = PErr> + Clone {
    tok(Token::GoonPub)
        .ignore_then(
            tok(Token::LParen)
                .ignore_then(
                    tok(Token::GoonCrate)
                        .to(Visibility::PubCrate)
                        .or(tok(Token::GoonSuper).to(Visibility::PubSuper)),
                )
                .then_ignore(tok(Token::RParen))
                .or_not(),
        )
        .map(|restriction| restriction.unwrap_or(Visibility::Public))
        .or(empty().to(Visibility::Private))
}

// ─── Type Parser ─────────────────────────────────────────────────────────────

fn type_parser() -> impl Parser<Token, Spanned<Type>, Error = PErr> + Clone {
    recursive(|ty| {
        // Unit type: ()
        let unit = tok(Token::LParen)
            .ignore_then(tok(Token::RParen))
            .map(|_| Type::Unit);

        // Never type: !
        let never = tok(Token::Not).map(|_| Type::Never);

        // Infer: _
        let infer = tok(Token::Underscore).map(|_| Type::Infer);

        // Tuple type: (A, B, C)
        let tuple_ty = tok(Token::LParen)
            .ignore_then(
                ty.clone()
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::RParen))
            .map(Type::Tuple);

        // Array type: [T; N] or Slice: [T]
        // NOTE: array size uses a simplified expression (literal or path) to avoid
        // infinite mutual recursion between type_parser ↔ expr_parser_inner.
        let array_size = filter_map(|span, tok: Token| match tok {
            Token::Int(n) => Ok(Expr::Literal(Literal::Int(n))),
            Token::Ident(s) => Ok(Expr::Path(vec![s])),
            _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
        });

        let array_or_slice = tok(Token::LBracket)
            .ignore_then(ty.clone())
            .then(
                tok(Token::Semi)
                    .ignore_then(spanned(array_size))
                    .or_not(),
            )
            .then_ignore(tok(Token::RBracket))
            .map(|(elem, size)| match size {
                Some(s) => Type::Array(Box::new(elem), Box::new(s)),
                None => Type::Slice(Box::new(elem)),
            });

        // Reference type: &T, &mut T, &'a T, &'a mut T
        let ref_ty = tok(Token::Ampersand)
            .ignore_then(
                filter_map(|span, tok: Token| match tok {
                    Token::Lifetime(l) => Ok(Some(l)),
                    _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
                })
                .or(empty().to(None)),
            )
            .then(tok(Token::GoonMut).or_not())
            .then(ty.clone())
            .map(|((lifetime, is_mut), inner)| Type::Reference {
                lifetime,
                is_mut: is_mut.is_some(),
                inner: Box::new(inner),
            });

        // dyn Trait
        let dyn_ty = tok(Token::GoonDyn)
            .ignore_then(ty.clone())
            .map(|t| Type::Dyn(Box::new(t)));

        // Path type: std::io::Result<T>
        let generic_args = tok(Token::Lt)
            .ignore_then(
                ty.clone()
                    .map(|t| GenericArg::Type(t))
                    .or(filter_map(|span, tok: Token| match tok {
                        Token::Lifetime(l) => Ok(GenericArg::Lifetime(l)),
                        _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
                    }))
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::Gt));

        let type_path_segment = ident()
            .or(filter_map(|span, tok: Token| match tok {
                Token::GoonSelfType => Ok("Self".to_string()),
                _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
            }))
            .then(generic_args.or_not())
            .map(|(name, generics)| TypePathSegment {
                name,
                generics: generics.unwrap_or_default(),
            });

        let path_ty = type_path_segment
            .separated_by(tok(Token::ColonColon))
            .at_least(1)
            .map(|segments| Type::Path(TypePath { segments }));

        let base = choice((
            unit, never, infer, tuple_ty, array_or_slice, ref_ty, dyn_ty, path_ty,
        ));

        spanned(base)
    }).boxed()
}

// ─── Pattern Parser ──────────────────────────────────────────────────────────

fn pattern_parser() -> impl Parser<Token, Spanned<Pattern>, Error = PErr> + Clone {
    recursive(|pat| {
        let wildcard = tok(Token::Underscore).to(Pattern::Wildcard);

        let rest = tok(Token::DotDot).to(Pattern::Rest);

        let literal = filter_map(|span, tok: Token| match tok {
            Token::Int(n) => Ok(Pattern::Literal(Literal::Int(n))),
            Token::Float(n) => Ok(Pattern::Literal(Literal::Float(n))),
            Token::Str(s) => Ok(Pattern::Literal(Literal::String(s))),
            Token::Char(c) => Ok(Pattern::Literal(Literal::Char(c))),
            Token::Gooning | Token::Edge => Ok(Pattern::Literal(Literal::Bool(true))),
            Token::NoGoon | Token::NoEdge => Ok(Pattern::Literal(Literal::Bool(false))),
            _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
        });

        let ref_pat = tok(Token::Ampersand)
            .ignore_then(tok(Token::GoonMut).or_not())
            .then(pat.clone())
            .map(|(m, p)| Pattern::Ref(m.is_some(), Box::new(p)));

        let mut_ident = tok(Token::GoonMut)
            .ignore_then(ident())
            .map(Pattern::MutIdent);

        // Tuple pattern: (a, b, c)
        let tuple_pat = tok(Token::LParen)
            .ignore_then(
                pat.clone()
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::RParen))
            .map(Pattern::Tuple);

        // Ident or path pattern — also handles TupleStruct
        let ident_or_path = ident()
            .separated_by(tok(Token::ColonColon))
            .at_least(1)
            .then(
                tok(Token::LParen)
                    .ignore_then(
                        pat.clone()
                            .separated_by(tok(Token::Comma))
                            .allow_trailing(),
                    )
                    .then_ignore(tok(Token::RParen))
                    .or_not(),
            )
            .map(|(segments, args)| {
                if let Some(args) = args {
                    let name = segments.join("::");
                    Pattern::TupleStruct(name, args)
                } else if segments.len() == 1 {
                    Pattern::Ident(segments.into_iter().next().unwrap())
                } else {
                    Pattern::Path(segments)
                }
            });

        let base = choice((
            wildcard, rest, literal, ref_pat, mut_ident, tuple_pat, ident_or_path,
        ));

        // Or patterns: a | b | c
        spanned(base)
            .separated_by(tok(Token::Pipe))
            .at_least(1)
            .map_with_span(|pats, span| {
                if pats.len() == 1 {
                    pats.into_iter().next().unwrap()
                } else {
                    (Pattern::Or(pats), span)
                }
            })
    }).boxed()
}

// ─── Expression Parser ───────────────────────────────────────────────────────

/// Inner expression parser (used recursively).
fn expr_parser_inner() -> impl Parser<Token, Expr, Error = PErr> + Clone {
    recursive(|expr: Recursive<Token, Expr, PErr>| {
        let spanned_expr = spanned(expr.clone());

        // ── Atoms ────────────────────────────────────────────
        let literal = filter_map(|span, tok: Token| match tok {
            Token::Int(n) => Ok(Expr::Literal(Literal::Int(n))),
            Token::Float(n) => Ok(Expr::Literal(Literal::Float(n))),
            Token::Str(s) => Ok(Expr::Literal(Literal::String(s))),
            Token::Char(c) => Ok(Expr::Literal(Literal::Char(c))),
            Token::Gooning | Token::Edge => Ok(Expr::Literal(Literal::Bool(true))),
            Token::NoGoon | Token::NoEdge => Ok(Expr::Literal(Literal::Bool(false))),
            _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
        });

        let self_val = tok(Token::GoonSelf).to(Expr::SelfValue);

        // Path/ident: foo, std::io::Result, GoonSelf
        let path_expr = filter_map(|span, tok: Token| match tok {
            Token::Ident(s) => Ok(s),
            Token::GoonSelfType => Ok("Self".to_string()),
            Token::GoonSome => Ok("Some".to_string()),
            Token::GoonNone => Ok("None".to_string()),
            Token::GoonOk => Ok("Ok".to_string()),
            Token::GoonErr => Ok("Err".to_string()),
            Token::GoonBox => Ok("Box".to_string()),
            _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
        })
        .separated_by(tok(Token::ColonColon))
        .at_least(1)
        .map(Expr::Path);

        // Parenthesized expression or tuple: (expr) or (a, b, c)
        let paren_or_tuple = tok(Token::LParen)
            .ignore_then(
                spanned_expr
                    .clone()
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::RParen))
            .map(|mut exprs| {
                if exprs.len() == 1 {
                    Expr::Paren(Box::new(exprs.remove(0)))
                } else {
                    Expr::Tuple(exprs)
                }
            });

        // Array: [1, 2, 3] or [0; 10]
        let array = tok(Token::LBracket)
            .ignore_then(
                spanned_expr
                    .clone()
                    .then(
                        tok(Token::Semi)
                            .ignore_then(spanned_expr.clone())
                            .map(Some)
                            .or(empty().to(None)),
                    )
                    .then(
                        tok(Token::Comma)
                            .ignore_then(spanned_expr.clone())
                            .repeated(),
                    ),
            )
            .then_ignore(tok(Token::RBracket))
            .map(|((first, repeat_count), rest)| {
                if let Some(count) = repeat_count {
                    Expr::ArrayRepeat {
                        value: Box::new(first),
                        count: Box::new(count),
                    }
                } else {
                    let mut elements = vec![first];
                    elements.extend(rest);
                    Expr::Array(elements)
                }
            })
            .or(tok(Token::LBracket)
                .ignore_then(tok(Token::RBracket))
                .to(Expr::Array(vec![])));

        // Block expression
        let block = block_parser(expr.clone());
        let block_expr = spanned(block.clone()).map(|(b, _)| Expr::Block(b));

        // If expression
        let if_expr = recursive(|if_e| {
            tok(Token::GoonIf)
                .ignore_then(spanned_expr.clone())
                .then(spanned(block.clone()))
                .then(
                    tok(Token::GoonElse)
                        .ignore_then(
                            spanned(if_e).map(|(e, span)| (Expr::If {
                                condition: Box::new(
                                    match &e {
                                        Expr::If { condition, .. } => *condition.clone(),
                                        _ => unreachable!(),
                                    }
                                ),
                                then_block: match &e {
                                    Expr::If { then_block, .. } => then_block.clone(),
                                    _ => unreachable!(),
                                },
                                else_block: match e {
                                    Expr::If { else_block, .. } => else_block,
                                    _ => unreachable!(),
                                },
                            }, span))
                            .or(spanned(block.clone()).map(|(b, span)| {
                                (Expr::Block(b), span)
                            })),
                        )
                        .or_not(),
                )
                .map(|((cond, then_block), else_block)| Expr::If {
                    condition: Box::new(cond),
                    then_block,
                    else_block: else_block.map(Box::new),
                })
        });

        // While loop
        let while_expr = tok(Token::GoonLoop)
            .ignore_then(spanned_expr.clone())
            .then(spanned(block.clone()))
            .map(|(cond, body)| Expr::While {
                condition: Box::new(cond),
                body,
            });

        // For loop
        let for_expr = tok(Token::GoonFor)
            .ignore_then(pattern_parser())
            .then_ignore(tok(Token::GoonIn))
            .then(spanned_expr.clone())
            .then(spanned(block.clone()))
            .map(|((pat, iter), body)| Expr::For {
                pattern: pat,
                iter: Box::new(iter),
                body,
            });

        // Infinite loop
        let loop_expr = tok(Token::GoonForever)
            .or(tok(Token::Edging))
            .ignore_then(spanned(block.clone()))
            .map(|body| Expr::Loop { body });

        // Match expression
        let match_arm = pattern_parser()
            .then(
                tok(Token::GoonIf)
                    .ignore_then(spanned_expr.clone())
                    .or_not(),
            )
            .then_ignore(tok(Token::FatArrow))
            .then(spanned_expr.clone())
            .then_ignore(tok(Token::Comma).or_not())
            .map(|((pat, guard), body)| MatchArm {
                pattern: pat,
                guard,
                body,
            });

        let match_expr = tok(Token::GoonMatch)
            .ignore_then(spanned_expr.clone())
            .then(
                tok(Token::LBrace)
                    .ignore_then(match_arm.repeated())
                    .then_ignore(tok(Token::RBrace)),
            )
            .map(|(expr, arms)| Expr::Match {
                expr: Box::new(expr),
                arms,
            });

        // Return
        let return_expr = tok(Token::GoonReturn)
            .ignore_then(spanned_expr.clone().or_not())
            .map(|e| Expr::Return(e.map(Box::new)));

        // Break
        let break_expr = tok(Token::Coom)
            .ignore_then(spanned_expr.clone().or_not())
            .map(|e| Expr::Break(e.map(Box::new)));

        // Continue
        let continue_expr = tok(Token::Nutting).to(Expr::Continue);

        // Print macro: goonprint!("format", args)
        let print_expr = tok(Token::GoonPrint)
            .ignore_then(tok(Token::LParen))
            .ignore_then(filter_map(|span, tok: Token| match tok {
                Token::Str(s) => Ok(s),
                _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
            }))
            .then(
                tok(Token::Comma)
                    .ignore_then(spanned_expr.clone())
                    .repeated(),
            )
            .then_ignore(tok(Token::RParen))
            .map(|(fmt, args)| Expr::Print {
                format_str: fmt,
                args,
            });

        // Eprint macro
        let eprint_expr = tok(Token::GoonEprint)
            .ignore_then(tok(Token::LParen))
            .ignore_then(filter_map(|span, tok: Token| match tok {
                Token::Str(s) => Ok(s),
                _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
            }))
            .then(
                tok(Token::Comma)
                    .ignore_then(spanned_expr.clone())
                    .repeated(),
            )
            .then_ignore(tok(Token::RParen))
            .map(|(fmt, args)| Expr::Eprint {
                format_str: fmt,
                args,
            });

        // Ruin (panic!)
        let ruin_expr = tok(Token::Ruin)
            .ignore_then(tok(Token::LParen))
            .ignore_then(
                filter_map(|span, tok: Token| match tok {
                    Token::Str(s) => Ok(Some(s)),
                    _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
                })
                .or(empty().to(None)),
            )
            .then_ignore(tok(Token::RParen))
            .map(Expr::Ruin);

        // PostNutClarity (dbg!)
        let dbg_expr = tok(Token::PostNutClarity)
            .ignore_then(tok(Token::LParen))
            .ignore_then(spanned_expr.clone())
            .then_ignore(tok(Token::RParen))
            .map(|e| Expr::PostNutClarity(Box::new(e)));

        // Vec macro: goonvec![1, 2, 3]
        let vec_expr = tok(Token::GoonVec)
            .ignore_then(tok(Token::LBracket))
            .ignore_then(
                spanned_expr
                    .clone()
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::RBracket))
            .map(Expr::VecMacro);

        // Closure: |a, b| expr or move |a| { ... }
        let closure_param = pattern_parser()
            .then(tok(Token::Colon).ignore_then(type_parser()).or_not())
            .map(|(pattern, ty)| ClosureParam { pattern, ty });

        let closure = tok(Token::GoonMove)
            .or_not()
            .then_ignore(tok(Token::Pipe))
            .then(
                closure_param
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::Pipe))
            .then(
                tok(Token::Arrow)
                    .ignore_then(type_parser())
                    .or_not(),
            )
            .then(spanned_expr.clone())
            .map(|(((is_move, params), ret_type), body)| Expr::Closure {
                is_move: is_move.is_some(),
                params,
                ret_type,
                body: Box::new(body),
            });

        // Reference: &expr, &mut expr
        let ref_expr = tok(Token::Ampersand)
            .ignore_then(tok(Token::GoonMut).or_not())
            .then(spanned_expr.clone())
            .map(|(is_mut, e)| Expr::Reference {
                is_mut: is_mut.is_some(),
                expr: Box::new(e),
            });

        // ── Atom: the smallest expression unit ──────────────

        let atom = choice((
            print_expr.boxed(),
            eprint_expr.boxed(),
            ruin_expr.boxed(),
            dbg_expr.boxed(),
            vec_expr.boxed(),
            if_expr.boxed(),
            while_expr.boxed(),
            for_expr.boxed(),
            loop_expr.boxed(),
            match_expr.boxed(),
            return_expr.boxed(),
            break_expr.boxed(),
            continue_expr.boxed(),
            literal.boxed(),
            self_val.boxed(),
            closure.boxed(),
            ref_expr.boxed(),
            paren_or_tuple.boxed(),
            array.boxed(),
            block_expr.boxed(),
            path_expr.boxed(),
        )).boxed();

        // ── Postfix operations ──────────────────────────────

        let call_args = tok(Token::LParen)
            .ignore_then(
                spanned_expr
                    .clone()
                    .separated_by(tok(Token::Comma))
                    .allow_trailing(),
            )
            .then_ignore(tok(Token::RParen));

        let index = tok(Token::LBracket)
            .ignore_then(spanned_expr.clone())
            .then_ignore(tok(Token::RBracket));

        #[derive(Clone)]
        enum Postfix {
            Call(Vec<Spanned<Expr>>),
            Method(String, Vec<Spanned<Expr>>),
            Field(String),
            TupleIdx(usize),
            Index(Spanned<Expr>),
            Try,
            Await,
        }

        let postfix_op = choice((
            // Method call: .foo(args)
            tok(Token::Dot)
                .ignore_then(ident())
                .then(call_args.clone())
                .map(|(name, args)| Postfix::Method(name, args)),
            // Await: .goonawait
            tok(Token::Dot)
                .ignore_then(tok(Token::GoonAwait))
                .to(Postfix::Await),
            // Tuple index: .0, .1
            tok(Token::Dot)
                .ignore_then(filter_map(|span, tok: Token| match tok {
                    Token::Int(n) => n
                        .parse::<usize>()
                        .map_err(|_| Simple::expected_input_found(span, vec![], Some(Token::Int(n)))),
                    _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
                }))
                .map(Postfix::TupleIdx),
            // Field access: .field
            tok(Token::Dot)
                .ignore_then(ident())
                .map(Postfix::Field),
            // Function call: (args)
            call_args.map(Postfix::Call),
            // Index: [expr]
            index.map(Postfix::Index),
            // Try: ?
            tok(Token::Question).to(Postfix::Try),
        )).boxed();

        let postfix = spanned(atom)
            .then(postfix_op.repeated())
            .foldl(|expr, op| {
                let span = expr.1.start..expr.1.end + 1; // approximate
                let new_expr = match op {
                    Postfix::Call(args) => Expr::Call {
                        func: Box::new(expr),
                        args,
                    },
                    Postfix::Method(name, args) => Expr::MethodCall {
                        receiver: Box::new(expr),
                        method: name,
                        generics: vec![],
                        args,
                    },
                    Postfix::Field(name) => Expr::Field {
                        expr: Box::new(expr),
                        field: name,
                    },
                    Postfix::TupleIdx(idx) => Expr::TupleIndex {
                        expr: Box::new(expr),
                        index: idx,
                    },
                    Postfix::Index(idx) => Expr::Index {
                        expr: Box::new(expr),
                        index: Box::new(idx),
                    },
                    Postfix::Try => Expr::Try(Box::new(expr)),
                    Postfix::Await => Expr::Await(Box::new(expr)),
                };
                (new_expr, span)
            })
            .map(|(e, _)| e).boxed();

        // ── Prefix unary ────────────────────────────────────

        let unary = tok(Token::Not)
            .to(UnaryOp::Not)
            .or(tok(Token::Minus).to(UnaryOp::Neg))
            .or(tok(Token::Star).to(UnaryOp::Deref))
            .map_with_span(|op, span| (op, span))
            .repeated()
            .then(spanned(postfix))
            .foldr(|(op, _op_span), expr| {
                let span = expr.1.clone();
                (
                    Expr::Unary {
                        op,
                        expr: Box::new(expr),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // ── Cast ────────────────────────────────────────────
        let cast = spanned(unary.clone())
            .then(tok(Token::GoonAs).ignore_then(type_parser()).repeated())
            .foldl(|expr, ty| {
                let span = expr.1.start..ty.1.end;
                (
                    Expr::Cast {
                        expr: Box::new(expr),
                        ty,
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // ── Binary operators by precedence (lowest to highest) ─

        // Multiplicative: * / %
        let product = spanned(cast.clone())
            .then(
                tok(Token::Star)
                    .to(BinOp::Mul)
                    .or(tok(Token::Slash).to(BinOp::Div))
                    .or(tok(Token::Percent).to(BinOp::Rem))
                    .then(spanned(cast))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Additive: + -
        let sum = spanned(product.clone())
            .then(
                tok(Token::Plus)
                    .to(BinOp::Add)
                    .or(tok(Token::Minus).to(BinOp::Sub))
                    .then(spanned(product))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Shift: << >>
        let shift = spanned(sum.clone())
            .then(
                tok(Token::Shl)
                    .to(BinOp::Shl)
                    .or(tok(Token::Shr).to(BinOp::Shr))
                    .then(spanned(sum))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Bitwise AND: &
        let bit_and = spanned(shift.clone())
            .then(
                tok(Token::Ampersand)
                    .to(BinOp::BitAnd)
                    .then(spanned(shift))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Bitwise XOR: ^
        let bit_xor = spanned(bit_and.clone())
            .then(
                tok(Token::Caret)
                    .to(BinOp::BitXor)
                    .then(spanned(bit_and))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Bitwise OR: |
        let bit_or = spanned(bit_xor.clone())
            .then(
                tok(Token::Pipe)
                    .to(BinOp::BitOr)
                    .then(spanned(bit_xor))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Comparison: == != < > <= >=
        let comparison = spanned(bit_or.clone())
            .then(
                choice((
                    tok(Token::EqEq).to(BinOp::Eq),
                    tok(Token::NotEq).to(BinOp::NotEq),
                    tok(Token::LtEq).to(BinOp::LtEq),
                    tok(Token::GtEq).to(BinOp::GtEq),
                    tok(Token::Lt).to(BinOp::Lt),
                    tok(Token::Gt).to(BinOp::Gt),
                ))
                .then(spanned(bit_or))
                .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Logical AND: &&
        let logical_and = spanned(comparison.clone())
            .then(
                tok(Token::And)
                    .to(BinOp::And)
                    .then(spanned(comparison))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Logical OR: ||
        let logical_or = spanned(logical_and.clone())
            .then(
                tok(Token::Or)
                    .to(BinOp::Or)
                    .then(spanned(logical_and))
                    .repeated(),
            )
            .foldl(|left, (op, right)| {
                let span = left.1.start..right.1.end;
                (
                    Expr::Binary {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    },
                    span,
                )
            })
            .map(|(e, _)| e).boxed();

        // Range: a..b, a..=b, ..b, a..
        let range = spanned(logical_or.clone())
            .then(
                tok(Token::DotDotEq)
                    .to(true)
                    .or(tok(Token::DotDot).to(false))
                    .then(spanned(logical_or.clone()).or_not())
                    .or_not(),
            )
            .map(|(left, range_part)| match range_part {
                Some((inclusive, end)) => Expr::Range {
                    start: Some(Box::new(left)),
                    end: end.map(Box::new),
                    inclusive,
                },
                None => left.0,
            }).boxed();

        // Assignment and compound assignment (right-associative)
        let assign_op = choice((
            tok(Token::PlusEq).to(Some(BinOp::Add)),
            tok(Token::MinusEq).to(Some(BinOp::Sub)),
            tok(Token::StarEq).to(Some(BinOp::Mul)),
            tok(Token::SlashEq).to(Some(BinOp::Div)),
            tok(Token::PercentEq).to(Some(BinOp::Rem)),
            tok(Token::AmpEq).to(Some(BinOp::BitAnd)),
            tok(Token::PipeEq).to(Some(BinOp::BitOr)),
            tok(Token::CaretEq).to(Some(BinOp::BitXor)),
            tok(Token::Eq).to(None),
        )).boxed();

        spanned(range.clone())
            .then(assign_op.then(spanned(range)).or_not())
            .map(|(left, assign)| match assign {
                Some((Some(op), right)) => Expr::CompoundAssign {
                    target: Box::new(left),
                    op,
                    value: Box::new(right),
                },
                Some((None, right)) => Expr::Assign {
                    target: Box::new(left),
                    value: Box::new(right),
                },
                None => left.0,
            })
    }).boxed()
}

/// Public expression parser producing spanned expressions.
pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = PErr> + Clone {
    spanned(expr_parser_inner()).boxed()
}

// ─── Block Parser ────────────────────────────────────────────────────────────

fn block_parser(
    expr: impl Parser<Token, Expr, Error = PErr> + Clone + 'static,
) -> impl Parser<Token, IndentBlock, Error = PErr> + Clone {
    let spanned_expr = spanned(expr.clone());

    // Let statement
    let let_stmt = tok(Token::Goon)
        .to(true) // mutable
        .or(tok(Token::GoonConst).to(false)) // immutable
        .then(pattern_parser())
        .then(tok(Token::Colon).ignore_then(type_parser()).or_not())
        .then(tok(Token::Eq).ignore_then(spanned_expr.clone()).or_not())
        .then_ignore(tok(Token::Semi))
        .map(|(((is_mut, pattern), ty), value)| {
            Stmt::Let {
                is_mut,
                pattern,
                ty,
                value,
            }
        });

    let expr_stmt = spanned_expr.clone().then(tok(Token::Semi).or_not()).map(
        |(expr, semi)| {
            if semi.is_some() {
                Stmt::Semi(expr)
            } else {
                Stmt::ExprStmt(expr)
            }
        },
    );

    let stmt = let_stmt.boxed().or(expr_stmt.boxed()).boxed();

    tok(Token::LBrace)
        .ignore_then(spanned(stmt).repeated())
        .then_ignore(tok(Token::RBrace))
        .map(|stmts| IndentBlock { stmts })
        .boxed()
}

// ─── Item Parsers ────────────────────────────────────────────────────────────

fn param_parser() -> impl Parser<Token, Param, Error = PErr> + Clone {
    // self/&self/&mut self parameters
    let self_param = tok(Token::Ampersand)
        .or_not()
        .then(tok(Token::GoonMut).or_not())
        .then_ignore(tok(Token::GoonSelf))
        .map(|(has_ref, has_mut)| {
            let ty_str = if has_ref.is_some() {
                if has_mut.is_some() {
                    "&mut Self"
                } else {
                    "&Self"
                }
            } else {
                "Self"
            };
            Param {
                pattern: (Pattern::Ident("self".to_string()), 0..0),
                ty: (
                    Type::Path(TypePath {
                        segments: vec![TypePathSegment {
                            name: ty_str.to_string(),
                            generics: vec![],
                        }],
                    }),
                    0..0,
                ),
            }
        });

    // Regular parameter: pattern: Type
    let regular = pattern_parser()
        .then_ignore(tok(Token::Colon))
        .then(type_parser())
        .map(|(pattern, ty)| Param { pattern, ty });

    self_param.boxed().or(regular.boxed()).boxed()
}

fn generic_params_parser() -> impl Parser<Token, Vec<GenericParam>, Error = PErr> + Clone {
    tok(Token::Lt)
        .ignore_then(
            // Lifetime parameter
            filter_map(|span, tok: Token| match tok {
                Token::Lifetime(l) => Ok(GenericParam::Lifetime(l)),
                _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
            })
            // Type parameter with optional bounds
            .or(ident()
                .then(
                    tok(Token::Colon)
                        .ignore_then(
                            spanned(type_parser().map(|(t, _s)| t))
                                .separated_by(tok(Token::Plus))
                                .at_least(1),
                        )
                        .or_not(),
                )
                .map(|(name, bounds)| GenericParam::Type {
                    name,
                    bounds: bounds.unwrap_or_default(),
                }))
            .separated_by(tok(Token::Comma))
            .allow_trailing(),
        )
        .then_ignore(tok(Token::Gt))
}

fn function_parser(
    vis: Visibility,
) -> impl Parser<Token, Function, Error = PErr> + Clone {
    tok(Token::GoonAsync)
        .or_not()
        .then_ignore(tok(Token::Goonsesh))
        .then(ident())
        .then(generic_params_parser().or_not())
        .then(
            tok(Token::LParen)
                .ignore_then(
                    param_parser()
                        .separated_by(tok(Token::Comma))
                        .allow_trailing(),
                )
                .then_ignore(tok(Token::RParen)),
        )
        .then(tok(Token::Arrow).ignore_then(type_parser()).or_not())
        .then(spanned(block_parser(expr_parser_inner())))
        .map(
            move |(((((is_async, name), generics), params), ret), body)| Function {
                visibility: vis.clone(),
                is_async: is_async.is_some(),
                name,
                generics: generics.unwrap_or_default(),
                params,
                return_type: ret,
                where_clause: vec![],
                body,
            },
        )
}

fn struct_parser(
    vis: Visibility,
) -> impl Parser<Token, StructDef, Error = PErr> + Clone {
    tok(Token::Goonstruct)
        .ignore_then(ident())
        .then(generic_params_parser().or_not())
        .then(
            // Named fields
            tok(Token::LBrace)
                .ignore_then(
                    visibility()
                        .then(ident())
                        .then_ignore(tok(Token::Colon))
                        .then(type_parser())
                        .map(|((field_vis, name), ty)| StructField {
                            visibility: field_vis,
                            name,
                            ty,
                        })
                        .separated_by(tok(Token::Comma))
                        .allow_trailing(),
                )
                .then_ignore(tok(Token::RBrace))
                .map(StructFields::Named)
                // Tuple struct
                .or(tok(Token::LParen)
                    .ignore_then(
                        type_parser()
                            .separated_by(tok(Token::Comma))
                            .allow_trailing(),
                    )
                    .then_ignore(tok(Token::RParen))
                    .then_ignore(tok(Token::Semi))
                    .map(StructFields::Tuple))
                // Unit struct
                .or(tok(Token::Semi).to(StructFields::Unit)),
        )
        .map(move |((name, generics), fields)| StructDef {
            visibility: vis.clone(),
            name,
            generics: generics.unwrap_or_default(),
            fields,
            where_clause: vec![],
        })
}

fn enum_parser(
    vis: Visibility,
) -> impl Parser<Token, EnumDef, Error = PErr> + Clone {
    let variant = ident()
        .then(
            // Tuple variant
            tok(Token::LParen)
                .ignore_then(
                    type_parser()
                        .separated_by(tok(Token::Comma))
                        .allow_trailing(),
                )
                .then_ignore(tok(Token::RParen))
                .map(StructFields::Tuple)
                // Named fields variant
                .or(tok(Token::LBrace)
                    .ignore_then(
                        ident()
                            .then_ignore(tok(Token::Colon))
                            .then(type_parser())
                            .map(|(name, ty)| StructField {
                                visibility: Visibility::Private,
                                name,
                                ty,
                            })
                            .separated_by(tok(Token::Comma))
                            .allow_trailing(),
                    )
                    .then_ignore(tok(Token::RBrace))
                    .map(StructFields::Named))
                .or(empty().to(StructFields::Unit)),
        )
        .then(
            tok(Token::Eq)
                .ignore_then(expr_parser())
                .or_not(),
        )
        .map(|((name, fields), discriminant)| EnumVariant {
            name,
            fields,
            discriminant,
        });

    tok(Token::Goonenum)
        .ignore_then(ident())
        .then(generic_params_parser().or_not())
        .then(
            tok(Token::LBrace)
                .ignore_then(
                    variant
                        .separated_by(tok(Token::Comma))
                        .allow_trailing(),
                )
                .then_ignore(tok(Token::RBrace)),
        )
        .map(move |((name, generics), variants)| EnumDef {
            visibility: vis.clone(),
            name,
            generics: generics.unwrap_or_default(),
            variants,
            where_clause: vec![],
        })
}

fn impl_parser() -> impl Parser<Token, ImplBlock, Error = PErr> + Clone {
    tok(Token::GoonImpl)
        .ignore_then(generic_params_parser().or_not())
        .then(type_parser())
        .then(
            // trait impl: goonimpl Trait goonfor Type
            tok(Token::GoonFor)
                .ignore_then(type_parser())
                .or_not(),
        )
        .then(
            tok(Token::LBrace)
                .ignore_then(
                    spanned(
                        visibility().then(function_parser(Visibility::Private)).map(
                            |(vis, mut f)| {
                                f.visibility = vis;
                                ImplItem::Function(f)
                            },
                        ),
                    )
                    .repeated(),
                )
                .then_ignore(tok(Token::RBrace)),
        )
        .map(
            |(((generics, first_type), for_type), items)| {
                let (trait_path, self_type) = if let Some(self_ty) = for_type {
                    (Some(first_type), self_ty)
                } else {
                    (None, first_type)
                };
                ImplBlock {
                    generics: generics.unwrap_or_default(),
                    trait_path,
                    self_type,
                    where_clause: vec![],
                    items,
                }
            },
        )
}

fn trait_parser(
    vis: Visibility,
) -> impl Parser<Token, TraitDef, Error = PErr> + Clone {
    tok(Token::GoonTrait)
        .ignore_then(ident())
        .then(generic_params_parser().or_not())
        .then(
            tok(Token::LBrace)
                .ignore_then(
                    spanned(
                        visibility()
                            .then(function_parser(Visibility::Private))
                            .map(|(vis, mut f)| {
                                f.visibility = vis;
                                TraitItem::Function(f)
                            }),
                    )
                    .repeated(),
                )
                .then_ignore(tok(Token::RBrace)),
        )
        .map(move |((name, generics), items)| TraitDef {
            visibility: vis.clone(),
            name,
            generics: generics.unwrap_or_default(),
            super_traits: vec![],
            where_clause: vec![],
            items,
        })
}

fn use_parser(
    vis: Visibility,
) -> impl Parser<Token, UsePath, Error = PErr> + Clone {
    // Simplified use tree: goonuse path::to::thing;
    let use_tree = recursive(|tree| {
        ident()
            .or(filter_map(|span, tok: Token| match tok {
                Token::GoonCrate => Ok("crate".to_string()),
                Token::GoonSuper => Ok("super".to_string()),
                Token::GoonSelf => Ok("self".to_string()),
                _ => Err(Simple::expected_input_found(span, vec![], Some(tok))),
            }))
            .then(
                tok(Token::ColonColon)
                    .ignore_then(
                        // Glob: *
                        tok(Token::Star).to(UseTree::Glob)
                        // Group: {a, b, c}
                        .or(tok(Token::LBrace)
                            .ignore_then(
                                tree.clone().separated_by(tok(Token::Comma)).allow_trailing(),
                            )
                            .then_ignore(tok(Token::RBrace))
                            .map(UseTree::Group))
                        // Nested path
                        .or(tree.clone()),
                    )
                    .or_not(),
            )
            .map(|(name, child)| match child {
                Some(child) => UseTree::Path(name, Box::new(child)),
                None => UseTree::Name(name),
            })
    });

    tok(Token::GoonUse)
        .ignore_then(use_tree)
        .then_ignore(tok(Token::Semi))
        .map(move |tree| UsePath {
            visibility: vis.clone(),
            tree,
        })
}

fn mod_parser(
    vis: Visibility,
) -> impl Parser<Token, ModDef, Error = PErr> + Clone {
    tok(Token::GoonMod)
        .ignore_then(ident())
        .then(tok(Token::Semi).to(None))
        .map(move |(name, items)| ModDef {
            visibility: vis.clone(),
            name,
            items,
        })
}

// ─── Attribute Parser ────────────────────────────────────────────────────────

fn attribute_parser() -> impl Parser<Token, Attribute, Error = PErr> + Clone {
    tok(Token::Hash)
        .ignore_then(tok(Token::LBracket))
        .ignore_then(
            ident()
                .separated_by(tok(Token::ColonColon))
                .at_least(1)
                .map(|parts| parts.join("::"))
        )
        .then(
            tok(Token::LParen)
                .ignore_then(
                    filter(|t: &Token| *t != Token::RParen)
                        .repeated()
                        .map(|tokens: Vec<Token>| {
                            tokens
                                .iter()
                                .map(|t| format!("{}", t))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }),
                )
                .then_ignore(tok(Token::RParen))
                .or_not(),
        )
        .then_ignore(tok(Token::RBracket))
        .map_with_span(|(path, args), span| Attribute { path, args, span })
}

// ─── Top-Level Program Parser ────────────────────────────────────────────────

pub fn item_parser() -> impl Parser<Token, Spanned<Item>, Error = PErr> + Clone {
    let inner = recursive(|_item| {
        attribute_parser()
            .repeated()
            .then(visibility())
            .then(choice((
                function_parser(Visibility::Private).map(Item::Function).boxed(),
                struct_parser(Visibility::Private).map(Item::Struct).boxed(),
                enum_parser(Visibility::Private).map(Item::Enum).boxed(),
                impl_parser().map(Item::Impl).boxed(),
                trait_parser(Visibility::Private).map(Item::Trait).boxed(),
                use_parser(Visibility::Private).map(Item::Use).boxed(),
                mod_parser(Visibility::Private).map(Item::Mod).boxed(),
            )))
            .map(|((attrs, vis), mut item)| {
                // Apply visibility
                match &mut item {
                    Item::Function(f) => f.visibility = vis.clone(),
                    Item::Struct(s) => s.visibility = vis.clone(),
                    Item::Enum(e) => e.visibility = vis.clone(),
                    Item::Trait(t) => t.visibility = vis.clone(),
                    Item::Use(u) => u.visibility = vis.clone(),
                    Item::Mod(m) => m.visibility = vis.clone(),
                    _ => {}
                }
                if attrs.is_empty() {
                    item
                } else {
                    Item::Attributed(attrs, Box::new((item, 0..0)))
                }
            })
    });

    spanned(inner).boxed()
}

pub fn program_parser() -> impl Parser<Token, Program, Error = PErr> {
    item_parser()
        .repeated()
        .then_ignore(end())
        .map(|items| Program { items })
        .boxed()
}
