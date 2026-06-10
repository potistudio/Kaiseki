use crate::lang::lexer::{self, Token, TokenKind};
use crate::types::{HighlightedLine, VarOccurrence, VarRole};

/// Returns true when the identifier at `tok_pos` names a TYPE rather than a VALUE.
///
/// Scans backward past whitespace and type-modifier tokens (`*`  `&`  `?`),
/// then inspects the first "interesting" predecessor:
///
///   `:`              → type annotation after a colon                     → yes
///   `->`  after `)`  → return-type position in a fn signature            → yes
///   `struct` / `interface` / `type` keyword                              → yes
///   anything else                                                        → no
fn is_type_position(tokens: &[Token], tok_pos: usize, line: &str) -> bool {
	let mut idx = tok_pos;
	loop {
		let prev = tokens[..idx]
			.iter()
			.enumerate()
			.rev()
			.find(|(_, t)| t.kind != TokenKind::Whitespace);
		let Some((prev_idx, prev_tok)) = prev else {
			return false;
		};
		let s = &line[prev_tok.range.clone()];
		match prev_tok.kind {
			// Type modifiers — keep scanning further back
			TokenKind::Operator if s == "*" || s == "&" => {
				idx = prev_idx;
			}
			TokenKind::Punct if s == "?" => {
				idx = prev_idx;
			}
			// Colon: definitely a type annotation
			TokenKind::Punct if s == ":" => return true,
			// Arrow: return-type when preceded by `)`, member-access otherwise
			TokenKind::Operator if s == "->" => {
				return tokens[..prev_idx]
					.iter()
					.rev()
					.find(|t| t.kind != TokenKind::Whitespace)
					.map(|t| t.kind == TokenKind::Punct && &line[t.range.clone()] == ")")
					.unwrap_or(false);
			}
			// Type-defining keywords
			TokenKind::Keyword => {
				return matches!(s, "struct" | "interface" | "type");
			}
			_ => return false,
		}
	}
}

/// Scan every KSL line for tokens whose text equals `name` and classify each
/// occurrence as Definition / Write / Read / TypeRef based on neighbouring tokens.
///
/// Role heuristics:
///   • In type-annotation position (after `:`, `->`, or a type keyword) → TypeRef
///   • Preceded by `let` or `var`                                        → Definition
///   • Followed by `=` (not `==`)                                        → Write
///   • Everything else                                                   → Read
pub fn find_var_occurrences(name: &str, lines: &[HighlightedLine]) -> Vec<VarOccurrence> {
	let mut result = Vec::new();

	for (line_idx, line) in lines.iter().enumerate() {
		let tokens = lexer::tokenize(&line.text);

		for (tok_pos, token) in tokens.iter().enumerate() {
			if token.kind != TokenKind::Ident {
				continue;
			}
			if &line.text[token.range.clone()] != name {
				continue;
			}

			let prev_kind = tokens[..tok_pos]
				.iter()
				.rev()
				.find(|t| t.kind != TokenKind::Whitespace)
				.map(|t| (t.kind, &line.text[t.range.clone()]));

			let next = tokens[tok_pos + 1..]
				.iter()
				.find(|t| t.kind != TokenKind::Whitespace)
				.map(|t| (t.kind, &line.text[t.range.clone()]));

			let role = if is_type_position(&tokens, tok_pos, &line.text) {
				VarRole::TypeRef
			} else if matches!(&prev_kind, Some((TokenKind::Keyword, kw)) if matches!(*kw, "let" | "var" | "fn")) {
				// let x / var x / fn name(…)
				VarRole::Definition
			} else if matches!(&next, Some((TokenKind::Punct, p)) if *p == ":") {
				// name: Type  — function parameter or struct field binding
				VarRole::Definition
			} else if matches!(&next, Some((TokenKind::Operator, op)) if *op == "=") {
				VarRole::Write
			} else {
				VarRole::Read
			};

			result.push(VarOccurrence {
				ksl_line_idx: line_idx,
				byte_range: token.range.clone(),
				role,
			});
		}
	}

	result
}

/// Returns the KSL line index of the `fn <name>` definition, or `None`.
pub fn find_fn_definition(name: &str, lines: &[HighlightedLine]) -> Option<usize> {
	for (line_idx, line) in lines.iter().enumerate() {
		let tokens = lexer::tokenize(&line.text);
		for (tok_pos, token) in tokens.iter().enumerate() {
			if token.kind != TokenKind::Ident || &line.text[token.range.clone()] != name {
				continue;
			}
			// Preceded by the `fn` keyword → this is the definition site.
			let prev = tokens[..tok_pos].iter().rev().find(|t| t.kind != TokenKind::Whitespace);
			if matches!(prev, Some(t) if t.kind == TokenKind::Keyword && &line.text[t.range.clone()] == "fn") {
				return Some(line_idx);
			}
		}
	}
	None
}

/// Returns `true` when the identifier at `tok_pos` is immediately followed by `(`,
/// meaning it is the callee of a call expression.
pub fn is_fn_call(tokens: &[Token], tok_pos: usize, line: &str) -> bool {
	tokens[tok_pos + 1..]
		.iter()
		.find(|t| t.kind != TokenKind::Whitespace)
		.map(|t| t.kind == TokenKind::Punct && &line[t.range.clone()] == "(")
		.unwrap_or(false)
}
