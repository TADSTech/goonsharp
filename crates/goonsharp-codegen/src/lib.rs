/// GoonSharp Codegen — AST to Rust source code transpiler.
///
/// Walks the GoonSharp AST and emits valid Rust source code.
/// This is where goon keywords become real Rust — the transpilation layer.

use goonsharp_parser::ast::*;

/// Transpile a GoonSharp AST to Rust source code.
pub fn transpile(program: &Program) -> String {
    let mut out = String::new();
    for (item, _span) in &program.items {
        emit_item(&mut out, item, 0);
        out.push('\n');
    }
    out
}

fn indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("    ");
    }
}

// ─── Items ───────────────────────────────────────────────────────────────────

fn emit_item(out: &mut String, item: &Item, level: usize) {
    match item {
        Item::Function(f) => emit_function(out, f, level),
        Item::Struct(s) => emit_struct(out, s, level),
        Item::Enum(e) => emit_enum(out, e, level),
        Item::Impl(i) => emit_impl(out, i, level),
        Item::Trait(t) => emit_trait(out, t, level),
        Item::Use(u) => emit_use(out, u, level),
        Item::Mod(m) => emit_mod(out, m, level),
        Item::TypeAlias(t) => emit_type_alias(out, t, level),
        Item::Const(c) => emit_const(out, c, level),
        Item::Static(s) => emit_static(out, s, level),
        Item::Attributed(attrs, inner) => {
            for attr in attrs {
                indent(out, level);
                out.push_str("#[");
                out.push_str(&attr.path);
                if let Some(args) = &attr.args {
                    out.push('(');
                    out.push_str(args);
                    out.push(')');
                }
                out.push_str("]\n");
            }
            emit_item(out, &inner.0, level);
        }
    }
}

fn emit_visibility(out: &mut String, vis: &Visibility) {
    match vis {
        Visibility::Private => {}
        Visibility::Public => out.push_str("pub "),
        Visibility::PubCrate => out.push_str("pub(crate) "),
        Visibility::PubSuper => out.push_str("pub(super) "),
    }
}

fn emit_function(out: &mut String, f: &Function, level: usize) {
    indent(out, level);
    emit_visibility(out, &f.visibility);
    if f.is_async {
        out.push_str("async ");
    }
    out.push_str("fn ");
    out.push_str(&f.name);
    emit_generics(out, &f.generics);
    out.push('(');
    for (i, param) in f.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        emit_param(out, param);
    }
    out.push(')');
    if let Some((ret_ty, _)) = &f.return_type {
        out.push_str(" -> ");
        emit_type(out, ret_ty);
    }
    out.push(' ');
    emit_block(out, &f.body.0, level);
    out.push('\n');
}

fn emit_param(out: &mut String, param: &Param) {
    // Special handling for self parameters
    let pat = &param.pattern.0;
    if let Pattern::Ident(name) = pat {
        if name == "self" {
            let ty_str = if let Type::Path(tp) = &param.ty.0 {
                tp.segments.first().map(|s| s.name.as_str()).unwrap_or("")
            } else {
                ""
            };
            match ty_str {
                "&Self" => {
                    out.push_str("&self");
                    return;
                }
                "&mut Self" => {
                    out.push_str("&mut self");
                    return;
                }
                "Self" => {
                    out.push_str("self");
                    return;
                }
                _ => {}
            }
        }
    }
    emit_pattern(out, pat);
    out.push_str(": ");
    emit_type(out, &param.ty.0);
}

fn emit_generics(out: &mut String, generics: &[GenericParam]) {
    if generics.is_empty() {
        return;
    }
    out.push('<');
    for (i, g) in generics.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        match g {
            GenericParam::Type { name, bounds } => {
                out.push_str(name);
                if !bounds.is_empty() {
                    out.push_str(": ");
                    for (j, (bound, _)) in bounds.iter().enumerate() {
                        if j > 0 {
                            out.push_str(" + ");
                        }
                        emit_type(out, bound);
                    }
                }
            }
            GenericParam::Lifetime(l) => {
                out.push('\'');
                out.push_str(l);
            }
            GenericParam::Const { name, ty } => {
                out.push_str("const ");
                out.push_str(name);
                out.push_str(": ");
                emit_type(out, &ty.0);
            }
        }
    }
    out.push('>');
}

fn emit_struct(out: &mut String, s: &StructDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &s.visibility);
    out.push_str("struct ");
    out.push_str(&s.name);
    emit_generics(out, &s.generics);
    match &s.fields {
        StructFields::Named(fields) => {
            out.push_str(" {\n");
            for (_i, field) in fields.iter().enumerate() {
                indent(out, level + 1);
                emit_visibility(out, &field.visibility);
                out.push_str(&field.name);
                out.push_str(": ");
                emit_type(out, &field.ty.0);
                out.push(',');
                out.push('\n');
            }
            indent(out, level);
            out.push('}');
        }
        StructFields::Tuple(types) => {
            out.push('(');
            for (i, (ty, _)) in types.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_type(out, ty);
            }
            out.push_str(");");
        }
        StructFields::Unit => {
            out.push(';');
        }
    }
    out.push('\n');
}

fn emit_enum(out: &mut String, e: &EnumDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &e.visibility);
    out.push_str("enum ");
    out.push_str(&e.name);
    emit_generics(out, &e.generics);
    out.push_str(" {\n");
    for variant in &e.variants {
        indent(out, level + 1);
        out.push_str(&variant.name);
        match &variant.fields {
            StructFields::Named(fields) => {
                out.push_str(" {\n");
                for field in fields {
                    indent(out, level + 2);
                    out.push_str(&field.name);
                    out.push_str(": ");
                    emit_type(out, &field.ty.0);
                    out.push_str(",\n");
                }
                indent(out, level + 1);
                out.push('}');
            }
            StructFields::Tuple(types) => {
                out.push('(');
                for (i, (ty, _)) in types.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_type(out, ty);
                }
                out.push(')');
            }
            StructFields::Unit => {}
        }
        if let Some((disc, _)) = &variant.discriminant {
            out.push_str(" = ");
            emit_expr(out, disc, level + 1);
        }
        out.push_str(",\n");
    }
    indent(out, level);
    out.push_str("}\n");
}

fn emit_impl(out: &mut String, imp: &ImplBlock, level: usize) {
    indent(out, level);
    out.push_str("impl");
    emit_generics(out, &imp.generics);
    out.push(' ');
    if let Some((trait_path, _)) = &imp.trait_path {
        emit_type(out, trait_path);
        out.push_str(" for ");
    }
    emit_type(out, &imp.self_type.0);
    out.push_str(" {\n");
    for (item, _) in &imp.items {
        match item {
            ImplItem::Function(f) => emit_function(out, f, level + 1),
            ImplItem::Type(t) => emit_type_alias(out, t, level + 1),
            ImplItem::Const(c) => emit_const(out, c, level + 1),
        }
    }
    indent(out, level);
    out.push_str("}\n");
}

fn emit_trait(out: &mut String, t: &TraitDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &t.visibility);
    out.push_str("trait ");
    out.push_str(&t.name);
    emit_generics(out, &t.generics);
    out.push_str(" {\n");
    for (item, _) in &t.items {
        match item {
            TraitItem::Function(f) => emit_function(out, f, level + 1),
            TraitItem::FunctionSig(_sig) => {
                // TODO: emit function signature without body
            }
            TraitItem::Type(t) => emit_type_alias(out, t, level + 1),
            TraitItem::Const(c) => emit_const(out, c, level + 1),
        }
    }
    indent(out, level);
    out.push_str("}\n");
}

fn emit_use(out: &mut String, u: &UsePath, level: usize) {
    indent(out, level);
    emit_visibility(out, &u.visibility);
    out.push_str("use ");
    emit_use_tree(out, &u.tree);
    out.push_str(";\n");
}

fn emit_use_tree(out: &mut String, tree: &UseTree) {
    match tree {
        UseTree::Path(name, child) => {
            out.push_str(name);
            out.push_str("::");
            emit_use_tree(out, child);
        }
        UseTree::Name(name) => out.push_str(name),
        UseTree::Rename(name, alias) => {
            out.push_str(name);
            out.push_str(" as ");
            out.push_str(alias);
        }
        UseTree::Glob => out.push('*'),
        UseTree::Group(items) => {
            out.push('{');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_use_tree(out, item);
            }
            out.push('}');
        }
    }
}

fn emit_mod(out: &mut String, m: &ModDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &m.visibility);
    out.push_str("mod ");
    out.push_str(&m.name);
    match &m.items {
        Some(items) => {
            out.push_str(" {\n");
            for (item, _) in items {
                emit_item(out, item, level + 1);
            }
            indent(out, level);
            out.push_str("}\n");
        }
        None => {
            out.push_str(";\n");
        }
    }
}

fn emit_type_alias(out: &mut String, t: &TypeAliasDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &t.visibility);
    out.push_str("type ");
    out.push_str(&t.name);
    emit_generics(out, &t.generics);
    if let Some((ty, _)) = &t.ty {
        out.push_str(" = ");
        emit_type(out, ty);
    }
    out.push_str(";\n");
}

fn emit_const(out: &mut String, c: &ConstDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &c.visibility);
    out.push_str("const ");
    out.push_str(&c.name);
    out.push_str(": ");
    emit_type(out, &c.ty.0);
    out.push_str(" = ");
    emit_expr(out, &c.value.0, level);
    out.push_str(";\n");
}

fn emit_static(out: &mut String, s: &StaticDef, level: usize) {
    indent(out, level);
    emit_visibility(out, &s.visibility);
    out.push_str("static ");
    if s.is_mut {
        out.push_str("mut ");
    }
    out.push_str(&s.name);
    out.push_str(": ");
    emit_type(out, &s.ty.0);
    out.push_str(" = ");
    emit_expr(out, &s.value.0, level);
    out.push_str(";\n");
}

// ─── Types ───────────────────────────────────────────────────────────────────

fn emit_type(out: &mut String, ty: &Type) {
    match ty {
        Type::Path(path) => {
            for (i, seg) in path.segments.iter().enumerate() {
                if i > 0 {
                    out.push_str("::");
                }
                out.push_str(&seg.name);
                if !seg.generics.is_empty() {
                    out.push('<');
                    for (j, arg) in seg.generics.iter().enumerate() {
                        if j > 0 {
                            out.push_str(", ");
                        }
                        match arg {
                            GenericArg::Type((ty, _)) => emit_type(out, ty),
                            GenericArg::Lifetime(l) => {
                                out.push('\'');
                                out.push_str(l);
                            }
                            GenericArg::Const((expr, _)) => emit_expr(out, expr, 0),
                        }
                    }
                    out.push('>');
                }
            }
        }
        Type::Reference {
            lifetime,
            is_mut,
            inner,
        } => {
            out.push('&');
            if let Some(l) = lifetime {
                out.push('\'');
                out.push_str(l);
                out.push(' ');
            }
            if *is_mut {
                out.push_str("mut ");
            }
            emit_type(out, &inner.0);
        }
        Type::Tuple(types) => {
            out.push('(');
            for (i, (ty, _)) in types.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_type(out, ty);
            }
            out.push(')');
        }
        Type::Array(elem, size) => {
            out.push('[');
            emit_type(out, &elem.0);
            out.push_str("; ");
            emit_expr(out, &size.0, 0);
            out.push(']');
        }
        Type::Slice(elem) => {
            out.push('[');
            emit_type(out, &elem.0);
            out.push(']');
        }
        Type::FnPointer { params, ret } => {
            out.push_str("fn(");
            for (i, (ty, _)) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_type(out, ty);
            }
            out.push(')');
            if let Some((ret, _)) = ret.as_deref() {
                out.push_str(" -> ");
                emit_type(out, ret);
            }
        }
        Type::Dyn(inner) => {
            out.push_str("dyn ");
            emit_type(out, &inner.0);
        }
        Type::ImplTrait(inner) => {
            out.push_str("impl ");
            emit_type(out, &inner.0);
        }
        Type::Infer => out.push('_'),
        Type::Never => out.push('!'),
        Type::Unit => out.push_str("()"),
    }
}

// ─── Patterns ────────────────────────────────────────────────────────────────

fn emit_pattern(out: &mut String, pat: &Pattern) {
    match pat {
        Pattern::Wildcard => out.push('_'),
        Pattern::Ident(name) => out.push_str(name),
        Pattern::MutIdent(name) => {
            out.push_str("mut ");
            out.push_str(name);
        }
        Pattern::RefIdent(is_mut, name) => {
            out.push_str("ref ");
            if *is_mut {
                out.push_str("mut ");
            }
            out.push_str(name);
        }
        Pattern::Tuple(pats) => {
            out.push('(');
            for (i, (p, _)) in pats.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_pattern(out, p);
            }
            out.push(')');
        }
        Pattern::Struct(name, fields) => {
            out.push_str(name);
            out.push_str(" { ");
            for (i, field) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&field.name);
                if let Some((pat, _)) = &field.pattern {
                    out.push_str(": ");
                    emit_pattern(out, pat);
                }
            }
            out.push_str(" }");
        }
        Pattern::TupleStruct(name, pats) => {
            out.push_str(name);
            out.push('(');
            for (i, (p, _)) in pats.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_pattern(out, p);
            }
            out.push(')');
        }
        Pattern::Literal(lit) => emit_literal(out, lit),
        Pattern::Or(pats) => {
            for (i, (p, _)) in pats.iter().enumerate() {
                if i > 0 {
                    out.push_str(" | ");
                }
                emit_pattern(out, p);
            }
        }
        Pattern::Binding(name, inner) => {
            out.push_str(name);
            out.push_str(" @ ");
            emit_pattern(out, &inner.0);
        }
        Pattern::Rest => out.push_str(".."),
        Pattern::Ref(is_mut, inner) => {
            out.push('&');
            if *is_mut {
                out.push_str("mut ");
            }
            emit_pattern(out, &inner.0);
        }
        Pattern::Range(start, end) => {
            if let Some(inner) = start {
                emit_expr(out, &inner.0, 0);
            }
            out.push_str("..=");
            if let Some(inner) = end {
                emit_expr(out, &inner.0, 0);
            }
        }
        Pattern::Path(segments) => {
            for (i, seg) in segments.iter().enumerate() {
                if i > 0 {
                    out.push_str("::");
                }
                out.push_str(seg);
            }
        }
    }
}

// ─── Expressions ─────────────────────────────────────────────────────────────

fn emit_expr(out: &mut String, expr: &Expr, level: usize) {
    match expr {
        Expr::Literal(lit) => emit_literal(out, lit),
        Expr::Path(segments) => {
            for (i, seg) in segments.iter().enumerate() {
                if i > 0 {
                    out.push_str("::");
                }
                out.push_str(seg);
            }
        }
        Expr::SelfValue => out.push_str("self"),
        Expr::Binary { left, op, right } => {
            emit_expr(out, &left.0, level);
            out.push(' ');
            out.push_str(op.as_rust_str());
            out.push(' ');
            emit_expr(out, &right.0, level);
        }
        Expr::Unary { op, expr: inner } => {
            out.push_str(op.as_rust_str());
            emit_expr(out, &inner.0, level);
        }
        Expr::Call { func, args } => {
            emit_expr(out, &func.0, level);
            out.push('(');
            for (i, (arg, _)) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, arg, level);
            }
            out.push(')');
        }
        Expr::Pipeline { left, right } => {
            emit_expr(out, &right.0, level);
            out.push('(');
            emit_expr(out, &left.0, level);
            out.push(')');
        }
        Expr::MethodCall {
            receiver,
            method,
            generics: _,
            args,
        } => {
            emit_expr(out, &receiver.0, level);
            out.push('.');
            out.push_str(method);
            out.push('(');
            for (i, (arg, _)) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, arg, level);
            }
            out.push(')');
        }
        Expr::Field { expr: inner, field } => {
            emit_expr(out, &inner.0, level);
            out.push('.');
            out.push_str(field);
        }
        Expr::TupleIndex { expr: inner, index } => {
            emit_expr(out, &inner.0, level);
            out.push('.');
            out.push_str(&index.to_string());
        }
        Expr::Index {
            expr: inner,
            index,
        } => {
            emit_expr(out, &inner.0, level);
            out.push('[');
            emit_expr(out, &index.0, level);
            out.push(']');
        }
        Expr::If {
            condition,
            then_block,
            else_block,
        } => {
            out.push_str("if ");
            emit_expr(out, &condition.0, level);
            out.push(' ');
            emit_block(out, &then_block.0, level);
            if let Some(else_expr) = else_block {
                out.push_str(" else ");
                match &else_expr.0 {
                    Expr::Block(b) => emit_block(out, b, level),
                    Expr::If { .. } => emit_expr(out, &else_expr.0, level),
                    _ => emit_expr(out, &else_expr.0, level),
                }
            }
        }
        Expr::While { condition, body } => {
            out.push_str("while ");
            emit_expr(out, &condition.0, level);
            out.push(' ');
            emit_block(out, &body.0, level);
        }
        Expr::For {
            pattern,
            iter,
            body,
        } => {
            out.push_str("for ");
            emit_pattern(out, &pattern.0);
            out.push_str(" in ");
            emit_expr(out, &iter.0, level);
            out.push(' ');
            emit_block(out, &body.0, level);
        }
        Expr::Loop { body } => {
            out.push_str("loop ");
            emit_block(out, &body.0, level);
        }
        Expr::Match { expr: inner, arms } => {
            out.push_str("match ");
            emit_expr(out, &inner.0, level);
            out.push_str(" {\n");
            for arm in arms {
                indent(out, level + 1);
                emit_pattern(out, &arm.pattern.0);
                if let Some((guard, _)) = &arm.guard {
                    out.push_str(" if ");
                    emit_expr(out, guard, level + 1);
                }
                out.push_str(" => ");
                emit_expr(out, &arm.body.0, level + 1);
                out.push_str(",\n");
            }
            indent(out, level);
            out.push('}');
        }
        Expr::Block(block) => emit_block(out, block, level),
        Expr::Return(inner) => {
            out.push_str("return");
            if let Some(boxed) = inner {
                out.push(' ');
                emit_expr(out, &boxed.0, level);
            }
        }
        Expr::Break(inner) => {
            out.push_str("break");
            if let Some(boxed) = inner {
                out.push(' ');
                emit_expr(out, &boxed.0, level);
            }
        }
        Expr::Continue => out.push_str("continue"),
        Expr::Closure {
            is_move,
            params,
            ret_type,
            body,
        } => {
            if *is_move {
                out.push_str("move ");
            }
            out.push('|');
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_pattern(out, &param.pattern.0);
                if let Some((ty, _)) = &param.ty {
                    out.push_str(": ");
                    emit_type(out, ty);
                }
            }
            out.push('|');
            if let Some((ty, _)) = ret_type {
                out.push_str(" -> ");
                emit_type(out, ty);
            }
            out.push(' ');
            emit_expr(out, &body.0, level);
        }
        Expr::Tuple(exprs) => {
            out.push('(');
            for (i, (e, _)) in exprs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, level);
            }
            if exprs.len() == 1 {
                out.push(',');
            }
            out.push(')');
        }
        Expr::Array(exprs) => {
            out.push('[');
            for (i, (e, _)) in exprs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, level);
            }
            out.push(']');
        }
        Expr::ArrayRepeat { value, count } => {
            out.push('[');
            emit_expr(out, &value.0, level);
            out.push_str("; ");
            emit_expr(out, &count.0, level);
            out.push(']');
        }
        Expr::StructLit {
            name,
            fields,
            rest,
        } => {
            for (i, seg) in name.iter().enumerate() {
                if i > 0 {
                    out.push_str("::");
                }
                out.push_str(seg);
            }
            out.push_str(" { ");
            for (i, field) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&field.name);
                if let Some((val, _)) = &field.value {
                    out.push_str(": ");
                    emit_expr(out, val, level);
                }
            }
            if let Some(boxed) = rest {
                if !fields.is_empty() {
                    out.push_str(", ");
                }
                out.push_str("..");
                emit_expr(out, &boxed.0, level);
            }
            out.push_str(" }");
        }
        Expr::Range {
            start,
            end,
            inclusive,
        } => {
            if let Some(boxed) = start {
                emit_expr(out, &boxed.0, level);
            }
            if *inclusive {
                out.push_str("..=");
            } else {
                out.push_str("..");
            }
            if let Some(boxed) = end {
                emit_expr(out, &boxed.0, level);
            }
        }
        Expr::Reference { is_mut, expr: inner } => {
            out.push('&');
            if *is_mut {
                out.push_str("mut ");
            }
            emit_expr(out, &inner.0, level);
        }
        Expr::Deref(inner) => {
            out.push('*');
            emit_expr(out, &inner.0, level);
        }
        Expr::Try(inner) => {
            emit_expr(out, &inner.0, level);
            out.push('?');
        }
        Expr::Await(inner) => {
            emit_expr(out, &inner.0, level);
            out.push_str(".await");
        }
        Expr::Assign { target, value } => {
            emit_expr(out, &target.0, level);
            out.push_str(" = ");
            emit_expr(out, &value.0, level);
        }
        Expr::CompoundAssign { target, op, value } => {
            emit_expr(out, &target.0, level);
            out.push(' ');
            out.push_str(op.as_rust_str());
            out.push_str("= ");
            emit_expr(out, &value.0, level);
        }
        Expr::Cast { expr: inner, ty } => {
            emit_expr(out, &inner.0, level);
            out.push_str(" as ");
            emit_type(out, &ty.0);
        }
        Expr::MacroCall { name, args } => {
            out.push_str(name);
            out.push('(');
            out.push_str(args);
            out.push(')');
        }
        Expr::Print { format_str, args } => {
            out.push_str("println!(\"");
            out.push_str(format_str);
            out.push('"');
            for (arg, _) in args {
                out.push_str(", ");
                emit_expr(out, arg, level);
            }
            out.push(')');
        }
        Expr::Eprint { format_str, args } => {
            out.push_str("eprintln!(\"");
            out.push_str(format_str);
            out.push('"');
            for (arg, _) in args {
                out.push_str(", ");
                emit_expr(out, arg, level);
            }
            out.push(')');
        }
        Expr::Ruin(msg) => {
            out.push_str("panic!(");
            if let Some(m) = msg {
                out.push('"');
                out.push_str(m);
                out.push('"');
            }
            out.push(')');
        }
        Expr::PostNutClarity(inner) => {
            out.push_str("dbg!(");
            emit_expr(out, &inner.0, level);
            out.push(')');
        }
        Expr::VecMacro(exprs) => {
            out.push_str("vec![");
            for (i, (e, _)) in exprs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, level);
            }
            out.push(']');
        }
        Expr::Paren(inner) => {
            out.push('(');
            emit_expr(out, &inner.0, level);
            out.push(')');
        }
        Expr::QualifiedPath {
            ty,
            as_trait,
            item,
        } => {
            out.push('<');
            emit_type(out, &ty.0);
            if let Some((trait_ty, _)) = as_trait {
                out.push_str(" as ");
                emit_type(out, trait_ty);
            }
            out.push_str(">::");
            out.push_str(item);
        }
        Expr::RawRust(code) => {
            out.push_str(code);
        }
    }
}

fn emit_literal(out: &mut String, lit: &Literal) {
    match lit {
        Literal::Int(n) => out.push_str(n),
        Literal::Float(n) => out.push_str(n),
        Literal::String(s) => {
            out.push('"');
            out.push_str(s);
            out.push('"');
        }
        Literal::Char(c) => {
            out.push('\'');
            // Escape special chars
            match c {
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\\' => out.push_str("\\\\"),
                '\'' => out.push_str("\\'"),
                c => out.push(*c),
            }
            out.push('\'');
        }
        Literal::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
    }
}

// ─── Blocks ──────────────────────────────────────────────────────────────────

fn emit_block(out: &mut String, body: &IndentBlock, level: usize) {
    out.push_str("{\n");
    for (stmt, _) in &body.stmts {
        emit_stmt(out, stmt, level + 1);
    }
    indent(out, level);
    out.push('}');
}

fn emit_stmt(out: &mut String, stmt: &Stmt, level: usize) {
    match stmt {
        Stmt::Let {
            is_mut,
            pattern,
            ty,
            value,
        } => {
            indent(out, level);
            out.push_str("let ");
            if *is_mut {
                out.push_str("mut ");
            }
            emit_pattern(out, &pattern.0);
            if let Some((t, _)) = ty {
                out.push_str(": ");
                emit_type(out, t);
            }
            if let Some((v, _)) = value {
                out.push_str(" = ");
                emit_expr(out, v, level);
            }
            out.push_str(";\n");
        }
        Stmt::Assignment { identifier, value } => {
            indent(out, level);
            out.push_str(identifier);
            out.push_str(" = ");
            emit_expr(out, &value.0, level);
            out.push_str(";\n");
        }
        Stmt::Semi((expr, _)) => {
            indent(out, level);
            emit_expr(out, expr, level);
            out.push_str(";\n");
        }
        Stmt::ExprStmt((expr, _)) => {
            indent(out, level);
            emit_expr(out, expr, level);
            out.push('\n');
        }
        Stmt::Suite(block) => {
            indent(out, level);
            emit_block(out, block, level);
        }
        Stmt::Item((item, _)) => {
            emit_item(out, item, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use goonsharp_parser::compile_to_ast;

    #[test]
    fn transpile_hello_goon() {
        let src = r#"goonsesh main() {
    goonprint!("hello goon");
}"#;
        let ast = compile_to_ast(src, "test.goons").expect("parse failed");
        let rust = transpile(&ast);
        assert!(rust.contains("fn main()"));
        assert!(rust.contains("println!(\"hello goon\")"));
    }

    #[test]
    fn transpile_struct() {
        let src = r#"goonstruct Sesh {
    intensity: i32,
    duration: f64,
}"#;
        let ast = compile_to_ast(src, "test.goons").expect("parse failed");
        let rust = transpile(&ast);
        assert!(rust.contains("struct Sesh"));
        assert!(rust.contains("intensity: i32"));
    }
}
