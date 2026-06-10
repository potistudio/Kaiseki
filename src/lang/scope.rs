// KSL lexical-scope analysis.
//
// Tracks `{` / `}` depth per line to scope variable occurrence highlighting.
//
// Depth model:
//
//   depth_start[i]  — number of open `{` at the BEGINNING of line i
//   depth_min[i]    — minimum depth reached DURING line i
//
// depth_min handles `} else {` lines: the `}` dips the depth below the
// opening value mid-line, which marks the old scope's boundary even though
// depth_start for the next line equals the opening depth again.

use super::lexer::{TokenKind, tokenize};
use std::ops::Range;

// ── Depth map ─────────────────────────────────────────────────────────────────

pub struct DepthMap {
	depth_start: Vec<i32>,
	depth_min: Vec<i32>,
	num_lines: usize,
}

impl DepthMap {
	/// Build from a slice of raw source lines (no trailing newlines).
	pub fn build(lines: &[&str]) -> Self {
		let mut depth_start = Vec::with_capacity(lines.len());
		let mut depth_min = Vec::with_capacity(lines.len());
		let mut current: i32 = 0;

		for line in lines {
			depth_start.push(current);
			let mut min = current;
			let bytes = line.as_bytes();
			let mut i = 0;

			while i < bytes.len() {
				// `{`/`}` inside line comments are not scope delimiters.
				if bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
					break;
				}
				if bytes[i] == b'{' {
					current += 1;
				} else if bytes[i] == b'}' {
					current -= 1;
					if current < min {
						min = current;
					}
				}
				i += 1;
			}

			depth_min.push(min);
		}

		DepthMap {
			depth_start,
			depth_min,
			num_lines: lines.len(),
		}
	}

	pub fn depth_at(&self, line_idx: usize) -> i32 {
		self.depth_start.get(line_idx).copied().unwrap_or(0)
	}

	/// First line index (exclusive) after `start_line` where brace depth dips
	/// below `target_depth` — either at the line's start or mid-line via `}`.
	/// Returns `num_lines` when the scope extends to end-of-file (depth 0).
	pub fn scope_end(&self, start_line: usize, target_depth: i32) -> usize {
		for i in (start_line + 1)..self.num_lines {
			if self.depth_start[i] < target_depth || self.depth_min[i] < target_depth {
				return i;
			}
		}
		self.num_lines
	}
}

// ── Declaration detection ─────────────────────────────────────────────────────

struct DeclSite {
	line: usize,
	depth: i32,
}

/// Find all lines on which `name` is introduced as a new binding.
///
/// Detected patterns (single-line):
///   `let name`  `var name`  `fn name`   — keyword-preceded
///   `name :`                            — parameter or struct-field binding
fn find_decl_sites(name: &str, lines: &[&str], depth_map: &DepthMap) -> Vec<DeclSite> {
	let mut result = Vec::new();

	for (line_idx, line) in lines.iter().enumerate() {
		let tokens = tokenize(line);

		for (tok_pos, token) in tokens.iter().enumerate() {
			if token.kind != TokenKind::Ident {
				continue;
			}
			if &line[token.range.clone()] != name {
				continue;
			}

			let prev = tokens[..tok_pos]
				.iter()
				.rev()
				.find(|t| t.kind != TokenKind::Whitespace)
				.map(|t| (t.kind, &line[t.range.clone()]));

			let next = tokens[tok_pos + 1..]
				.iter()
				.find(|t| t.kind != TokenKind::Whitespace)
				.map(|t| (t.kind, &line[t.range.clone()]));

			let is_decl = matches!(&prev, Some((TokenKind::Keyword, kw)) if matches!(*kw, "let" | "var" | "fn"))
				|| matches!(&next, Some((TokenKind::Punct, p)) if *p == ":");

			if is_decl {
				result.push(DeclSite {
					line: line_idx,
					depth: depth_map.depth_at(line_idx),
				});
			}
		}
	}

	result
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns the line range `[decl_line .. scope_end)` for the innermost
/// declaration of `name` that covers `at_line`.
///
/// "Innermost" = highest brace depth among declarations that:
///   1. appear at or before `at_line`, AND
///   2. whose scope (depth_end) extends past `at_line`.
///
/// Returns `None` when no covering declaration exists — caller should show
/// all occurrences (file-global or undeclared name).
pub fn visible_scope(name: &str, at_line: usize, lines: &[&str], depth_map: &DepthMap) -> Option<Range<usize>> {
	let decl_sites = find_decl_sites(name, lines, depth_map);

	let covering = decl_sites
		.iter()
		.filter(|d| {
			if d.line > at_line {
				return false;
			}
			let end = depth_map.scope_end(d.line, d.depth);
			at_line < end
		})
		.max_by_key(|d| d.depth);

	let decl = covering?;
	let scope_end = depth_map.scope_end(decl.line, decl.depth);
	Some(decl.line..scope_end)
}
