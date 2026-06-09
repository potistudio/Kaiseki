use std::{ops::Range, path::PathBuf};

use iced::Color;

/// One syntax-highlighted line: the raw text plus a list of (byte-range, colour) spans.
#[derive(Clone, Debug)]
pub struct HighlightedLine {
	pub text:       String,
	/// (byte_range, foreground_color) — non-overlapping, source order.
	pub highlights: Vec<(Range<usize>, Color)>,
}

/// Semantic role of one identifier occurrence within the KSL source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VarRole {
	Definition,
	Write,
	Read,
	/// Identifier appears as a type name, not as a value (e.g. after `:`, after `struct`).
	/// Background tint is drawn but excluded from the connection gutter.
	TypeRef,
}

/// One occurrence of the active variable that should be highlighted.
#[derive(Clone, Debug)]
pub struct VarOccurrence {
	pub ksl_line_idx: usize,
	pub byte_range:   Range<usize>,
	pub role:         VarRole,
}

/// Flat display-list entry, either a KSL line or an expanded C source line.
#[derive(Clone, Debug)]
pub enum DisplayRow {
	KslLine {
		ksl_idx:     usize,
		span_idx:    Option<usize>, // Some → this row carries the accordion toggle button
		is_expanded: bool,
		label:       &'static str,
	},
	SourceLine {
		source_idx: usize,
		is_last:    bool,
	},
}

/// Entry in the .kvp file tree shown in the sidebar.
pub enum KvlEntry {
	File {
		/// Stem of the .kvl filename (without extension), used as the label.
		display_name: String,
		path:         PathBuf,
	},
	/// Sub-directory inside the .kvp package.
	Dir {
		name:     String,
		path:     PathBuf,
		children: Vec<KvlEntry>,
	},
}
