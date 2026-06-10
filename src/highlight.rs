use std::ops::Range;

use iced::Color;
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings};

use crate::lang::lexer::TokenKind;
use crate::theme::{GREEN, MAUVE, OVERLAY0, PEACH, SAPPHIRE_DIM, SKY, SUBTEXT1, TEXT_COL, YELLOW};
use crate::types::HighlightedLine;

fn token_color(kind: TokenKind) -> Option<Color> {
	Some(match kind {
		TokenKind::Keyword => MAUVE,
		TokenKind::Atom => PEACH,
		TokenKind::Number | TokenKind::Offset => YELLOW,
		TokenKind::DocComment => GREEN,
		TokenKind::SectionSep => SAPPHIRE_DIM,
		TokenKind::Comment => OVERLAY0,
		TokenKind::Operator => SKY,
		TokenKind::Punct => SUBTEXT1,
		TokenKind::Ident => TEXT_COL,
		TokenKind::Whitespace | TokenKind::Unknown => return None,
	})
}

pub fn build_ksl_lines(source: &str) -> Vec<HighlightedLine> {
	source
		.lines()
		.map(|line| {
			let highlights = crate::lang::lexer::tokenize(line)
				.into_iter()
				.filter_map(|t| token_color(t.kind).map(|c| (t.range, c)))
				.collect();
			HighlightedLine {
				text: line.to_string(),
				highlights,
			}
		})
		.collect()
}

pub fn build_source_lines(code: &str) -> Vec<HighlightedLine> {
	let ss = SyntaxSet::load_defaults_newlines();
	let ts = ThemeSet::load_defaults();
	let theme = &ts.themes["base16-ocean.dark"];
	let syntax = ss
		.find_syntax_by_extension("cpp")
		.unwrap_or_else(|| ss.find_syntax_plain_text());
	let mut hl = HighlightLines::new(syntax, theme);

	LinesWithEndings::from(code)
		.map(|line| {
			let ranges = hl.highlight_line(line, &ss).unwrap_or_default();
			let mut text = String::new();
			let mut highlights: Vec<(Range<usize>, Color)> = Vec::new();
			let mut offset = 0usize;

			for (style, frag) in &ranges {
				let end = offset + frag.len();
				let fg = Color {
					r: style.foreground.r as f32 / 255.0,
					g: style.foreground.g as f32 / 255.0,
					b: style.foreground.b as f32 / 255.0,
					a: style.foreground.a as f32 / 255.0,
				};
				highlights.push((offset..end, fg));
				text.push_str(frag);
				offset = end;
			}

			// Strip the trailing newline syntect adds; clamp highlight ranges to match.
			if text.ends_with('\n') {
				text.pop();
				let len = text.len();
				for (r, _) in &mut highlights {
					r.end = r.end.min(len);
				}
				highlights.retain(|(r, _)| r.start < r.end);
			}

			HighlightedLine { text, highlights }
		})
		.collect()
}
