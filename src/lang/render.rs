// Renders a KSL Document to a formatted, human-readable string.
// Functions here are infrastructure for future programmatic KSL generation.
#![allow(dead_code)]

use std::fmt::Write as _;
use super::ast::*;

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn render_document(doc: &Document) -> String {
	let mut out = String::new();
	for item in &doc.items {
		render_item(&mut out, item, 0);
	}
	out
}

// ── Items ─────────────────────────────────────────────────────────────────────

fn render_item(out: &mut String, item: &Item, indent: usize) {
	match item {
		Item::Comment(c)   => wln(out, indent, &format!("// {}", c)),
		Item::BlankLine    => out.push('\n'),
		Item::TypeAlias(t) => render_type_alias(out, t, indent),
		Item::Struct(s)    => render_struct(out, s, indent),
		Item::Interface(i) => render_interface(out, i, indent),
		Item::Function(f)  => render_fn(out, f, indent),
	}
}

fn render_type_alias(out: &mut String, ta: &TypeAlias, indent: usize) {
	render_docs(out, &ta.docs, indent);
	wln(out, indent, &format!("type {} = {};", ta.name, ty(&ta.ty)));
}

fn render_struct(out: &mut String, s: &StructDef, indent: usize) {
	render_docs(out, &s.docs, indent);
	wln(out, indent, &format!("struct {} {{", s.name));

	// Column widths: "@0x0000  " = 9 chars; name padded to max field name length.
	let name_col = s.fields.iter().map(|f| f.name.len()).max().unwrap_or(0);
	for f in &s.fields {
		let prefix = match f.offset {
			Some(o) => format!("@{:#06x}  ", o),
			None    => "          ".to_string(), // 10 spaces to match "@0x0000  "
		};
		let name_pad = format!("{:<width$}", f.name, width = name_col);
		let tail = f.comment.as_deref()
			.map(|c| format!("  // {}", c))
			.unwrap_or_default();
		wln(out, indent + 1, &format!("{}{}:  {},{}", prefix, name_pad, ty(&f.ty), tail));
	}
	wln(out, indent, "}");
}

fn render_interface(out: &mut String, iface: &InterfaceDef, indent: usize) {
	wln(out, indent, &format!("virtual interface {} {{", iface.name));

	// Compute the widest function signature to right-align return types.
	let sigs: Vec<String> = iface.methods.iter()
		.map(|m| format!("fn {}({})", m.name, params_inline(&m.params)))
		.collect();
	let sig_col = sigs.iter().map(|s| s.len()).max().unwrap_or(0);

	for (method, sig) in iface.methods.iter().zip(sigs.iter()) {
		let prefix = match method.vtable_offset {
			Some(o) => format!("@{:#06x}  ", o),
			None    => "          ".to_string(),
		};
		let ret = match &method.return_ty {
			Type::Unit => String::new(),
			t          => format!(" -> {}", ty(t)),
		};
		wln(
			out,
			indent + 1,
			&format!("{}{:<width$}{},", prefix, sig, ret, width = sig_col),
		);
	}
	wln(out, indent, "}");
}

fn render_fn(out: &mut String, f: &FnDef, indent: usize) {
	render_docs(out, &f.docs, indent);
	wln(out, indent, &format!("fn {}(", f.name));

	// Align parameter names.
	let name_col = f.params.iter().map(|p| p.name.len()).max().unwrap_or(0);
	for p in &f.params {
		let name_pad = format!("{:<width$}", format!("{}:", p.name), width = name_col + 1);
		let tail = p.comment.as_deref()
			.map(|c| format!("  // {}", c))
			.unwrap_or_default();
		wln(out, indent + 1, &format!("{} {},{}", name_pad, ty(&p.ty), tail));
	}

	let ret  = f.return_ty.as_ref().map(|t| format!(" -> {}", ty(t))).unwrap_or_default();
	let thr  = if f.throws { " throws" } else { "" };
	wln(out, indent, &format!("){}{} {{", ret, thr));

	render_block_body(out, &f.body, indent + 1);
	wln(out, indent, "}");
}

// ── Block / statements ────────────────────────────────────────────────────────

fn render_block_body(out: &mut String, block: &Block, indent: usize) {
	for stmt in &block.stmts {
		render_stmt(out, stmt, indent);
	}
}

fn render_stmt(out: &mut String, stmt: &Stmt, indent: usize) {
	match stmt {
		Stmt::Let { name, ty: t, value } => {
			let ann = t.as_ref().map(|t| format!(": {}", ty(t))).unwrap_or_default();
			wln(out, indent, &format!("let {}{} = {}", name, ann, expr(value, false)));
		}
		Stmt::Var { name, ty: t, value } => {
			let ann = t.as_ref().map(|t| format!(": {}", ty(t))).unwrap_or_default();
			match value {
				Some(v) => wln(out, indent, &format!("var {}{} = {}", name, ann, expr(v, false))),
				None    => wln(out, indent, &format!("var {}{}", name, ann)),
			}
		}
		Stmt::Assign { target, value } => {
			wln(out, indent, &format!("{} = {}", expr(target, false), expr(value, false)));
		}
		Stmt::If { cond, then_block, else_branch } => {
			wln(out, indent, &format!("if {} {{", expr(cond, false)));
			render_block_body(out, then_block, indent + 1);
			render_else(out, else_branch.as_ref(), indent);
		}
		Stmt::Return(e) => match e {
			Some(v) => wln(out, indent, &format!("return {}", expr(v, false))),
			None    => wln(out, indent, "return"),
		},
		Stmt::Expr(e)        => wln(out, indent, &expr(e, false)),
		Stmt::Comment(c)     => wln(out, indent, &format!("// {}", c)),
		Stmt::BlankLine      => out.push('\n'),
		Stmt::Section(title) => {
			// `// ── Title ─────────────────────────────────────────────────────`
			let header = format!("// ── {} ", title);
			let fill   = 80_usize.saturating_sub(indent + header.len());
			wln(out, indent, &format!("{}{}", header, "─".repeat(fill)));
		}
	}
}

fn render_else(out: &mut String, branch: Option<&ElseBranch>, indent: usize) {
	match branch {
		None => wln(out, indent, "}"),
		Some(ElseBranch::Block(b)) => {
			wln(out, indent, "} else {");
			render_block_body(out, b, indent + 1);
			wln(out, indent, "}");
		}
		Some(ElseBranch::If(s)) => {
			// Write `} else ` then the if stmt without a leading newline.
			write!(out, "{}}} else ", tabs(indent)).unwrap();
			render_stmt_continuation(out, s, indent);
		}
	}
}

/// Like `render_stmt` but without the leading indent (used after `} else `).
fn render_stmt_continuation(out: &mut String, stmt: &Stmt, indent: usize) {
	match stmt {
		Stmt::If { cond, then_block, else_branch } => {
			writeln!(out, "if {} {{", expr(cond, false)).unwrap();
			render_block_body(out, then_block, indent + 1);
			render_else(out, else_branch.as_ref(), indent);
		}
		_ => {
			// Fallback: indent + newline
			out.push('\n');
			render_stmt(out, stmt, indent);
		}
	}
}

// ── Expressions ───────────────────────────────────────────────────────────────

/// Render an expression. `needs_parens`: wrap in `(…)` when the expression
/// contains spaces (i.e. is not a simple atom) so it reads correctly at a
/// call site or inside a compound expression.
fn expr(e: &Expr, needs_parens: bool) -> String {
	let s = match e {
		Expr::Null    => "null".into(),
		Expr::Default => "default".into(),
		Expr::Bool(b) => b.to_string(),
		Expr::Int(n)  => n.to_string(),
		Expr::Hex(n)  => format!("{:#x}", n),
		Expr::Ident(n) => n.clone(),

		Expr::Binary(op, lhs, rhs) => {
			format!("{} {} {}", expr(lhs, true), binop(*op), expr(rhs, true))
		}
		Expr::Unary(op, e) => match op {
			UnOp::Not => format!("!{}", expr(e, true)),
			UnOp::Neg => format!("-{}", expr(e, true)),
		},

		Expr::Call { func, args } => {
			let args_str = args.iter().map(|a| match a {
				CallArg::Expr(e) => expr(e, false),
				CallArg::Varargs => "...".into(),
			}).collect::<Vec<_>>().join(", ");
			format!("{}({})", expr(func, true), args_str)
		}

		Expr::Field(e, f) => format!("{}.{}", expr(e, true), f),
		Expr::Arrow(e, f) => format!("{}->{}", expr(e, true), f),

		Expr::Deref(e)     => format!("*{}", expr(e, true)),
		Expr::AddrOf(e)    => format!("&{}", expr(e, true)),
		Expr::AddrOfMut(e) => format!("&mut {}", expr(e, true)),

		Expr::Throw(e)             => format!("{}!", expr(e, true)),
		Expr::NullCoalesce(lhs, rhs) => format!("{} ?? {}", expr(lhs, false), expr(rhs, false)),

		Expr::LetElse { name, value } => {
			format!("let {} = {} else return", name, expr(value, false))
		}

		Expr::Tuple(items) => {
			format!("({})", items.iter().map(|e| expr(e, false)).collect::<Vec<_>>().join(", "))
		}

		Expr::IfExpr { cond, then_block, else_block } => {
			let mut s = format!("if {} {{\n", expr(cond, false));
			let mut body = String::new();
			render_block_body(&mut body, then_block, 1);
			s.push_str(&body);
			match else_block {
				None    => s.push('}'),
				Some(b) => {
					s.push_str("} else {\n");
					let mut body2 = String::new();
					render_block_body(&mut body2, b, 1);
					s.push_str(&body2);
					s.push('}');
				}
			}
			s
		}

		Expr::As(e, t) => format!("{} as {}", expr(e, true), ty(t)),
	};

	// Wrap in parentheses when the caller needs an atom but we produced a
	// compound expression (has spaces and doesn't already start with `(`).
	if needs_parens && s.contains(' ') && !s.starts_with('(') {
		format!("({})", s)
	} else {
		s
	}
}

// ── Types ─────────────────────────────────────────────────────────────────────

pub fn ty(t: &Type) -> String {
	match t {
		Type::Named(n)    => n.clone(),
		Type::Pointer(t)  => format!("*{}", ty(t)),
		Type::Ref(t)      => format!("&{}", ty(t)),
		Type::RefMut(t)   => format!("&mut {}", ty(t)),
		Type::Nullable(t) => format!("{}?", ty(t)),
		Type::Tuple(ts)   => format!("({})", ts.iter().map(ty).collect::<Vec<_>>().join(", ")),
		Type::Unit        => "()".into(),
	}
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn render_docs(out: &mut String, docs: &[String], indent: usize) {
	for line in docs {
		wln(out, indent, &format!("/// {}", line));
	}
}

fn params_inline(params: &[Param]) -> String {
	params.iter()
		.map(|p| format!("{}: {}", p.name, ty(&p.ty)))
		.collect::<Vec<_>>()
		.join(", ")
}

fn binop(op: BinOp) -> &'static str {
	match op {
		BinOp::Add => "+",  BinOp::Sub => "-",  BinOp::Mul => "*",
		BinOp::Div => "/",  BinOp::Rem => "%",
		BinOp::Eq  => "==", BinOp::Ne  => "!=",
		BinOp::Lt  => "<",  BinOp::Le  => "<=",
		BinOp::Gt  => ">",  BinOp::Ge  => ">=",
		BinOp::And => "&&", BinOp::Or  => "||",
		BinOp::BitAnd => "&", BinOp::BitOr => "|", BinOp::BitXor => "^",
		BinOp::Shl => "<<", BinOp::Shr => ">>",
	}
}

fn tabs(n: usize) -> String { "\t".repeat(n) }

fn wln(out: &mut String, indent: usize, line: &str) {
	writeln!(out, "{}{}", tabs(indent), line).unwrap();
}
