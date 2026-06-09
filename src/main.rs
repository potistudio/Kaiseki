mod lang;

use std::{
	collections::HashSet,
	ops::Range,
	path::{Path, PathBuf},
	sync::Arc,
};

use iced::{
	Background, Border, Color, Element, Font, Length, Pixels, Point, Rectangle, Size, Task,
	alignment, mouse,
	widget::{button, canvas, column, container, row, scrollable},
};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings};

use lang::lexer::TokenKind;
use lang::{SAMPLE_KSL, SAMPLE_SOURCE_MAP, SourceSpan};

// ── Sample decompiled-C source (Ghidra output for NIM_GetStreamFPV) ───────────

const SAMPLE_CODE: &str = r#"
/* void __cdecl NIM_GetStreamFPV(class BEE_Layer * __ptr64,class TDB_StreamIDPath const &
   __ptr64,int,struct T_Time const * __ptr64,struct T_Time const * __ptr64,union BEE_StreamFPV *
   __ptr64,struct FEE_KfcInfo * __ptr64,class TDB_Stream const * __ptr64) */

void __cdecl
NIM_GetStreamFPV(BEE_Layer *param_1,TDB_StreamIDPath *param_2,int param_3,T_Time *param_4,
                T_Time *param_5,BEE_StreamFPV *param_6,FEE_KfcInfo *param_7,TDB_Stream *param_8)

{
  FEE_KfcInfo *pFVar1;
  BEE_Item *pBVar2;
  TDB_Stream *this;
  char cVar3;
  bool bVar4;
  int iVar5;
  FEE_KfcInfo *pFVar6;
  FEE_KfcInfo *pFVar7;
  FEE_KfcInfo local_a8 [8];
  FEE_KfcInfo *local_a0;
  undefined8 local_98;
  undefined8 local_90;
  TDB_Stream *local_88;
  undefined4 local_80;
  undefined4 local_7c;
  FEE_KfcInfo *local_78;
  FEE_KfcInfo *local_70;
  FEE_KfcInfo *local_68;
  BEE_StreamSpec local_60 [8];
  BEE_Layer *local_58;
  TDB_StreamIDPath local_50 [40];

                    /* 0xe2b340  3999
                       ?NIM_GetStreamFPV@@YAXPEAVBEE_Layer@@AEBVTDB_StreamIDPath@@HPEBUT_Time@@2PEAT BEE_StreamFPV@@PEAUFEE_KfcInfo@@PEBVTDB_Stream@@@Z
                        */
  local_a8[0] = (FEE_KfcInfo)0x0;
  if ((param_8 == (TDB_Stream *)0x0) &&
     (iVar5 = BEE_GetStream(param_1,param_2,&param_8), iVar5 != 0)) {
    local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
  }
  this = param_8;
  pFVar6 = (FEE_KfcInfo *)0x0;
  iVar5 = BEE_GetStreamFPVPlusWithStreamP
                    (param_1,param_2,param_8,param_4,param_5,param_3,param_6,(TDB_ParamBag *)0x0);
  if (iVar5 != 0) {
    local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
    _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
  }
  if (param_7 == (FEE_KfcInfo *)0x0) {
    return;
  }
  local_90 = 0x100000000;
  local_98 = 0x100000000;
  if (param_4 == (T_Time *)0x0) {
    if (param_5 == (T_Time *)0x0) {
      iVar5 = BEE_GetItemCurrentTime(*(BEE_Item **)(param_1 + 0x290),(T_Time *)&local_90,(int *)0x0)
      ;
      if (iVar5 != 0) {
        local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
        _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
      }
      iVar5 = BEE_CompToLayerTime(param_1,(T_Time *)&local_90,(T_Time *)&local_98);
      if (iVar5 != 0) {
        local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
        _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
      }
    }
    else {
      local_98 = *(undefined8 *)param_5;
      iVar5 = BEE_LayerToCompTime(param_1,(T_Time *)&local_98,(T_Time *)&local_90);
      if (iVar5 != 0) {
        local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
        _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
      }
    }
  }
  else {
    local_90 = *(undefined8 *)param_4;
    iVar5 = BEE_CompToLayerTime(param_1,(T_Time *)&local_90,(T_Time *)&local_98);
    if (iVar5 != 0) {
      local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
      _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
    }
  }
  cVar3 = (**(code **)(*(longlong *)this + 0x170))(this);
  if (cVar3 != '\0') {
    pFVar7 = (FEE_KfcInfo *)0x0;
    iVar5 = TDB_GetValue(this,(T_Time *)&local_98,'\0',(void *)0x0,(uchar *)local_a8,
                         (TDB_ParamBag *)0x0);
    if (iVar5 != 0) {
      local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
      _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
    }
    bVar4 = TDB_Stream::HasKeys(this);
    if (bVar4) {
      local_78 = param_7 + 8;
      local_a0 = param_7 + 0x10;
      if (*param_7 == (FEE_KfcInfo)0x0) {
        local_78 = pFVar6;
        local_a0 = pFVar6;
      }
      local_70 = param_7 + 0x1c;
      if (param_7[3] == (FEE_KfcInfo)0x0) {
        local_70 = pFVar6;
      }
      local_68 = param_7 + 0x18;
      if (param_7[2] == (FEE_KfcInfo)0x0) {
        local_68 = pFVar6;
      }
      local_88 = (TDB_Stream *)0x0;
      pFVar6 = param_7 + 5;
      pFVar1 = param_7 + 4;
      local_80 = 0;
      local_7c = 1;
      BEE_StreamSpec::BEE_StreamSpec(local_60);
      if (pFVar1 != (FEE_KfcInfo *)0x0) {
        *pFVar1 = (FEE_KfcInfo)0x0;
      }
      if (pFVar6 != (FEE_KfcInfo *)0x0) {
        *pFVar6 = (FEE_KfcInfo)0x0;
      }
      if (param_1 != (BEE_Layer *)0x0) {
        pBVar2 = *(BEE_Item **)(param_1 + 0x290);
        local_58 = param_1;
        TDB_StreamIDPath::operator=(local_50,param_2);
        if (local_88 == (TDB_Stream *)0x0) {
          iVar5 = BEE_GetStreamWithSpecNullOK(local_60,&local_88);
          if (iVar5 != 0) {
            local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
            _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
          }
          if (local_88 == (TDB_Stream *)0x0) goto LAB_180e2b5da;
        }
        bVar4 = TDB_Stream::HasKeys(local_88);
        if (bVar4) {
          iVar5 = BEE_GetItemCurrentTime(pBVar2,(T_Time *)&local_80,(int *)0x0);
          if (iVar5 != 0) {
            local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
            _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
          }
          pFVar7 = local_68;
          FUN_180e2acf0(&local_80,param_1,local_88,pFVar1,pFVar6,local_68,local_70,local_78,local_a0
                       );
        }
      }
LAB_180e2b5da:
      BEE_StreamSpec::~BEE_StreamSpec(local_60);
      if (local_a8[0] == (FEE_KfcInfo)0x0) {
        local_a8[0] = (FEE_KfcInfo)
                      FUN_180e2afa0(param_1,this,&local_90,param_7 + 0x24,0,
                                    (ulonglong)pFVar7 & 0xffffffffffffff00);
      }
      else {
        iVar5 = TDB_TimeToIndex(this,(T_Time *)&local_98,(int *)(param_7 + 0x24));
        if (iVar5 != 0) {
          local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
          _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
        }
      }
      if (local_a8[0] == (FEE_KfcInfo)0x0) {
        param_7[0x28] = (FEE_KfcInfo)0x1;
      }
      else {
        iVar5 = TDB_GetKey(this,*(int *)(param_7 + 0x24),(void *)0x0,(char *)(param_7 + 0x28),
                           (char *)(param_7 + 0x29));
        if (iVar5 != 0) {
          local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
                    /* WARNING: Subroutine does not return */
          _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
        }
      }
      goto LAB_180e2b65e;
    }
  }
  local_a8[0] = (FEE_KfcInfo)0x0;
  param_7[0x28] = (FEE_KfcInfo)0x0;
LAB_180e2b65e:
  param_7[0x20] = local_a8[0];
  return;
}


"#;

// ── Layout constants ──────────────────────────────────────────────────────────

const ROW_H: f32 = 22.0;
const CHAR_W: f32 = 7.8; // approx. Consolas 13 px glyph advance
const FONT_SIZE: f32 = 13.0;
const TOP_PAD: f32 = 8.0;

// Left-margin layout
//
//   ┌─ CONN_GUTTER_W ─┬─── GUTTER_W ───┬─── code ──────────
//   │  rail / dots    │  line numbers  │
//
const CONN_GUTTER_W: f32 = 28.0; // connection gutter (left of line numbers)
const GUTTER_W: f32 = 56.0; // line-number gutter
const CODE_X: f32 = CONN_GUTTER_W + GUTTER_W; // where code text starts

// Connection gutter geometry
const DOT_R: f32 = 3.5;
const RAIL_W: f32 = 2.0;

// Sidebar width
const SIDEBAR_W: f32 = 220.0;

// ── Color palette (Catppuccin Mocha) ─────────────────────────────────────────

const fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color {
		r: r as f32 / 255.0,
		g: g as f32 / 255.0,
		b: b as f32 / 255.0,
		a: 1.0,
	}
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
	Color {
		r: r as f32 / 255.0,
		g: g as f32 / 255.0,
		b: b as f32 / 255.0,
		a: a as f32 / 255.0,
	}
}

const BASE: Color = rgb(0x1e, 0x1e, 0x2e);
const MANTLE: Color = rgb(0x18, 0x18, 0x25);
const SURFACE0: Color = rgb(0x31, 0x32, 0x44);
const OVERLAY0: Color = rgb(0x6c, 0x70, 0x86);
const SUBTEXT1: Color = rgb(0xba, 0xc2, 0xde);
const SUBTEXT0: Color = rgb(0xa6, 0xad, 0xc8);
const TEXT_COL: Color = rgb(0xcd, 0xd6, 0xf4);
const BLUE: Color = rgb(0x89, 0xb4, 0xfa);
const SKY: Color = rgb(0x89, 0xdc, 0xeb);
const GREEN: Color = rgb(0xa6, 0xe3, 0xa1);
const YELLOW: Color = rgb(0xf9, 0xe2, 0xaf);
const PEACH: Color = rgb(0xfa, 0xb3, 0x87);
const MAUVE: Color = rgb(0xcb, 0xa6, 0xf7);

const GUTTER_FG: Color = rgba(0x58, 0x5b, 0x70, 0xff);
const GUTTER_DIM: Color = rgba(0x58, 0x5b, 0x70, 0x44);
const SNIPPET_BG: Color = rgba(0x18, 0x18, 0x25, 0xdd);
const ACCORDION_FG: Color = rgba(0xe6, 0x89, 0x45, 0xcc);
const ACCORDION_BG: Color = rgba(0x31, 0x32, 0x44, 0x88);
const SNIPPET_BORDER: Color = rgba(0xe6, 0x89, 0x45, 0x66);
const RAIL_COL: Color = rgba(0x58, 0x5b, 0x70, 0x88);
const SAPPHIRE_DIM: Color = rgba(0x74, 0xc7, 0xec, 0x99);

const fn var_bg(role: VarRole) -> Color {
	match role {
		VarRole::Definition => rgba(0xb4, 0xbe, 0xfe, 0x55),
		VarRole::Write => rgba(0xf9, 0xe2, 0xaf, 0x55),
		VarRole::Read => rgba(0x94, 0xe2, 0xd5, 0x55),
		VarRole::TypeRef => rgba(0x6c, 0x70, 0x86, 0x33),
	}
}

const fn var_dot(role: VarRole) -> Color {
	match role {
		VarRole::Definition => GREEN,
		VarRole::Read => BLUE,
		VarRole::Write => YELLOW,
		VarRole::TypeRef => OVERLAY0,
	}
}

// ── Domain types ──────────────────────────────────────────────────────────────

/// One syntax-highlighted line: the raw text plus a list of (byte-range, colour) spans.
#[derive(Clone, Debug)]
struct HighlightedLine {
	text: String,
	/// (byte_range, foreground_color) — non-overlapping, source order.
	highlights: Vec<(Range<usize>, Color)>,
}

/// Semantic role of one identifier occurrence within the KSL source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VarRole {
	Definition,
	Write,
	Read,
	/// Identifier appears as a type name, not as a value (e.g. after `:`, after `struct`).
	/// Background tint is drawn but excluded from the connection gutter.
	TypeRef,
}

/// One occurrence of the active variable that should be highlighted.
#[derive(Clone, Debug)]
struct VarOccurrence {
	ksl_line_idx: usize,
	byte_range: Range<usize>,
	role: VarRole,
}

/// Flat display-list entry, either a KSL line or an expanded C source line.
#[derive(Clone, Debug)]
enum DisplayRow {
	KslLine {
		ksl_idx: usize,
		span_idx: Option<usize>, // Some → this row carries the accordion toggle button
		is_expanded: bool,
		label: &'static str,
	},
	SourceLine {
		source_idx: usize,
		is_last: bool,
	},
}

/// Entry in the .kvp file tree shown in the sidebar.
enum KvlEntry {
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

// ── File loading ──────────────────────────────────────────────────────────────

/// Recursively scan a .kvp directory and return a KvlEntry tree.
fn scan_kvp(dir: &Path) -> Vec<KvlEntry> {
	let Ok(read_dir) = std::fs::read_dir(dir) else { return Vec::new() };
	let mut raw: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
	raw.sort_by_key(|e| e.file_name());
	raw.into_iter()
		.filter_map(|entry| {
			let path = entry.path();
			let file_name = entry.file_name().to_string_lossy().to_string();
			if path.is_dir() {
				let children = scan_kvp(&path);
				// Omit empty directories.
				(!children.is_empty()).then_some(KvlEntry::Dir { name: file_name, path: path.clone(), children })
			} else if path.extension().map_or(false, |ext| ext == "kvl") {
				let display_name = path
					.file_stem()
					.map(|s| s.to_string_lossy().to_string())
					.unwrap_or(file_name);
				Some(KvlEntry::File { display_name, path })
			} else {
				None
			}
		})
		.collect()
}

/// Extract the KSL text from the "--- view ---" section of a .kvl file.
///
/// .kvl format:
///   --- source ---
///   <decompiled C>
///   --- view ---
///   <KSL>
fn extract_kvl_view(content: &str) -> &str {
	const MARKER: &str = "--- view ---";
	content
		.find(MARKER)
		.map(|pos| content[pos + MARKER.len()..].trim_start_matches('\n'))
		.unwrap_or(content)
}

// ── Sidebar widget ────────────────────────────────────────────────────────────

/// Flatten a KvlEntry tree into a list of sidebar row widgets (recursive).
/// `expanded_dirs` controls which Dir entries are open.
fn render_kvl_tree<'a>(
	entries: &'a [KvlEntry],
	selected: &'a Option<PathBuf>,
	expanded_dirs: &'a HashSet<PathBuf>,
	indent: usize,
) -> Vec<Element<'a, Message>> {
	let mut elements: Vec<Element<'a, Message>> = Vec::new();
	let left_pad = 12.0 + indent as f32 * 14.0;

	for entry in entries {
		match entry {
			KvlEntry::File { display_name, path } => {
				let is_selected = selected.as_deref() == Some(path.as_path());
				let file_path = path.clone();
				elements.push(
					button(
						iced::widget::text(display_name.as_str())
							.font(Font::MONOSPACE)
							.size(11),
					)
					.style(move |_, _| button::Style {
						background: Some(Background::Color(
							if is_selected { SURFACE0 } else { Color::TRANSPARENT },
						)),
						text_color: if is_selected { TEXT_COL } else { SUBTEXT0 },
						border: Border::default(),
						..Default::default()
					})
					.on_press(Message::SelectKvl(file_path))
					.width(Length::Fill)
					.padding(iced::Padding {
						top: 4.0,
						bottom: 4.0,
						left: left_pad,
						right: 8.0,
					})
					.into(),
				);
			}
			KvlEntry::Dir { name, path, children } => {
				let is_expanded = expanded_dirs.contains(path);
				let dir_path = path.clone();
				let chevron = if is_expanded { "▾" } else { "▸" };
				elements.push(
					button(
						iced::widget::text(format!("{chevron} {name}"))
							.font(Font::MONOSPACE)
							.size(10),
					)
					.style(move |_, _| button::Style {
						background: Some(Background::Color(Color::TRANSPARENT)),
						text_color: OVERLAY0,
						border: Border::default(),
						..Default::default()
					})
					.on_press(Message::ToggleDir(dir_path))
					.width(Length::Fill)
					.padding(iced::Padding {
						top: 6.0,
						bottom: 2.0,
						left: left_pad,
						right: 8.0,
					})
					.into(),
				);
				if is_expanded {
					elements.extend(render_kvl_tree(children, selected, expanded_dirs, indent + 1));
				}
			}
		}
	}
	elements
}

/// Collect all Dir paths in a KvlEntry tree (for default-expanded initialization).
fn collect_dir_paths(entries: &[KvlEntry], out: &mut HashSet<PathBuf>) {
	for entry in entries {
		if let KvlEntry::Dir { path, children, .. } = entry {
			out.insert(path.clone());
			collect_dir_paths(children, out);
		}
	}
}

// ── KSL syntax highlighting ───────────────────────────────────────────────────

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

fn build_ksl_lines(source: &str) -> Vec<HighlightedLine> {
	source
		.lines()
		.map(|line| {
			let highlights = lang::lexer::tokenize(line)
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

// ── Syntect code highlighting ─────────────────────────────────────────────────

fn build_source_lines(code: &str) -> Vec<HighlightedLine> {
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

// ── Variable analysis ─────────────────────────────────────────────────────────

/// Returns true when the identifier at `tok_pos` names a TYPE rather than a VALUE.
///
/// Scans backward past whitespace and type-modifier tokens (`*`  `&`  `?`),
/// then inspects the first "interesting" predecessor:
///
///   `:`              → type annotation after a colon                     → yes
///   `->`  after `)`  → return-type position in a fn signature            → yes
///   `struct` / `interface` / `type` keyword                              → yes
///   anything else                                                        → no
fn is_type_position(tokens: &[lang::lexer::Token], tok_pos: usize, line: &str) -> bool {
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
fn find_var_occurrences(name: &str, lines: &[HighlightedLine]) -> Vec<VarOccurrence> {
	let mut result = Vec::new();

	for (line_idx, line) in lines.iter().enumerate() {
		let tokens = lang::lexer::tokenize(&line.text);

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

// ── Variable analysis ── helper: function-call / definition detection ─────────

/// Returns the KSL line index of the `fn <name>` definition, or `None`.
fn find_fn_definition(name: &str, lines: &[HighlightedLine]) -> Option<usize> {
	for (line_idx, line) in lines.iter().enumerate() {
		let tokens = lang::lexer::tokenize(&line.text);
		for (tok_pos, token) in tokens.iter().enumerate() {
			if token.kind != TokenKind::Ident || &line.text[token.range.clone()] != name {
				continue;
			}
			// Preceded by the `fn` keyword → this is the definition site.
			let prev = tokens[..tok_pos]
				.iter()
				.rev()
				.find(|t| t.kind != TokenKind::Whitespace);
			if matches!(prev, Some(t) if t.kind == TokenKind::Keyword && &line.text[t.range.clone()] == "fn") {
				return Some(line_idx);
			}
		}
	}
	None
}

/// Returns `true` when the identifier at `tok_pos` is immediately followed by `(`,
/// meaning it is the callee of a call expression.
fn is_fn_call(tokens: &[lang::lexer::Token], tok_pos: usize, line: &str) -> bool {
	tokens[tok_pos + 1..]
		.iter()
		.find(|t| t.kind != TokenKind::Whitespace)
		.map(|t| t.kind == TokenKind::Punct && &line[t.range.clone()] == "(")
		.unwrap_or(false)
}

/// Stable scrollable ID for the code panel.  Must match the `.id()` call in `view_code_panel`.
fn code_scrollable_id() -> scrollable::Id {
	scrollable::Id::new("kaiseki-code")
}

// ── Application state ─────────────────────────────────────────────────────────

struct KaisekiState {
	ksl_lines:       Arc<Vec<HighlightedLine>>,
	source_lines:    Arc<Vec<HighlightedLine>>,
	source_map:      Arc<Vec<SourceSpan>>,
	expanded_spans:  HashSet<usize>,
	active_variable: Option<String>,
	var_occurrences: Arc<Vec<VarOccurrence>>,
	// Pre-computed brace depths for scope-aware occurrence filtering.
	depth_map:       lang::scope::DepthMap,
	// ── File tree (populated when a .kvp directory is supplied) ──────────────
	kvl_tree:        Vec<KvlEntry>,
	selected_kvl:    Option<PathBuf>,
	/// Display name shown in the panel header (file stem, or "sample").
	selected_name:   String,
	/// Directories currently expanded in the sidebar tree.
	expanded_dirs:   HashSet<PathBuf>,
	sidebar_visible: bool,
}

#[derive(Debug, Clone)]
enum Message {
	ToggleSpan(usize),
	/// `Some((name, ksl_line_idx))` — activate; `None` — clear.
	/// The line index is needed to resolve which declaration scope to use when
	/// the same name is bound multiple times (e.g. shadowing across if/else branches).
	SetActiveVariable(Option<(String, usize)>),
	/// Double-click on a function-call identifier: scroll the view to its `fn` definition.
	JumpToDefinition(String),
	/// User clicked a .kvl entry in the sidebar.
	SelectKvl(PathBuf),
	/// User clicked a directory header in the sidebar.
	ToggleDir(PathBuf),
	ToggleSidebar,
}

impl KaisekiState {
	fn new(kvp_path: Option<PathBuf>) -> (Self, Task<Message>) {
		let kvl_tree = kvp_path.as_deref().map(scan_kvp).unwrap_or_default();

		let ksl_lines = build_ksl_lines(SAMPLE_KSL);
		let depth_map = {
			let raw: Vec<&str> = ksl_lines.iter().map(|l| l.text.as_str()).collect();
			lang::scope::DepthMap::build(&raw)
		};
		let state = Self {
			ksl_lines: Arc::new(ksl_lines),
			source_lines: Arc::new(build_source_lines(SAMPLE_CODE)),
			source_map: Arc::new(
				SAMPLE_SOURCE_MAP
					.iter()
					.map(|s| SourceSpan {
						label: s.label,
						ksl_trigger_line: s.ksl_trigger_line,
						source_lines: s.source_lines.clone(),
					})
					.collect(),
			),
			expanded_spans:  HashSet::new(),
			active_variable: None,
			var_occurrences: Arc::new(Vec::new()),
			depth_map,
			selected_kvl:  None,
			selected_name: "sample".to_string(),
			expanded_dirs: {
				let mut dirs = HashSet::new();
				collect_dir_paths(&kvl_tree, &mut dirs);
				dirs
			},
			kvl_tree,
			sidebar_visible: true,
		};
		(state, Task::none())
	}

	fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::ToggleSpan(idx) => {
				if !self.expanded_spans.remove(&idx) {
					self.expanded_spans.insert(idx);
				}
			}
			Message::JumpToDefinition(name) => {
				return self.jump_to_definition(&name);
			}
			Message::SetActiveVariable(val) => {
				let (new_occurrences, new_active) = match val {
					None => (Vec::new(), None),
					Some((name, clicked_line)) => {
						let all_occurrences = find_var_occurrences(&name, &self.ksl_lines);
						let raw: Vec<&str> = self.ksl_lines.iter()
							.map(|l| l.text.as_str()).collect();
						let scoped_occurrences = match lang::scope::visible_scope(
							&name, clicked_line, &raw, &self.depth_map,
						) {
							Some(range) => all_occurrences.into_iter()
								.filter(|occurrence| range.contains(&occurrence.ksl_line_idx))
								.collect(),
							None => all_occurrences,
						};
						(scoped_occurrences, Some(name))
					}
				};
				self.var_occurrences = Arc::new(new_occurrences);
				self.active_variable = new_active;
			}
			Message::ToggleDir(path) => {
				if !self.expanded_dirs.remove(&path) {
					self.expanded_dirs.insert(path);
				}
			}
			Message::ToggleSidebar => {
				self.sidebar_visible = !self.sidebar_visible;
			}
			Message::SelectKvl(path) => {
				let content = std::fs::read_to_string(&path).unwrap_or_default();
				let ksl_lines = build_ksl_lines(extract_kvl_view(&content));
				let depth_map = {
					let raw: Vec<&str> = ksl_lines.iter().map(|l| l.text.as_str()).collect();
					lang::scope::DepthMap::build(&raw)
				};
				let display_name = path
					.file_stem()
					.map(|s| s.to_string_lossy().to_string())
					.unwrap_or_else(|| "unknown".to_string());

				self.ksl_lines       = Arc::new(ksl_lines);
				// .kvl files do not carry a C source block yet — clear the accordion.
				self.source_lines    = Arc::new(Vec::new());
				self.source_map      = Arc::new(Vec::new());
				self.expanded_spans  .clear();
				self.active_variable = None;
				self.var_occurrences = Arc::new(Vec::new());
				self.depth_map       = depth_map;
				self.selected_kvl    = Some(path);
				self.selected_name   = display_name;
			}
		}
		Task::none()
	}

	fn jump_to_definition(&self, name: &str) -> Task<Message> {
		let Some(ksl_line_idx) = find_fn_definition(name, &self.ksl_lines) else {
			return Task::none();
		};
		let display_rows = self.build_display_rows();
		let Some(display_idx) = display_rows.iter().position(|r| {
			matches!(r, DisplayRow::KslLine { ksl_idx, .. } if *ksl_idx == ksl_line_idx)
		}) else {
			return Task::none();
		};
		// Place the target line a few rows from the top for comfortable reading.
		let y = TOP_PAD + display_idx as f32 * ROW_H;
		let scroll_y = (y - ROW_H * 3.0).max(0.0);
		scrollable::scroll_to(code_scrollable_id(), scrollable::AbsoluteOffset { x: 0.0, y: scroll_y })
	}

	fn view(&self) -> Element<'_, Message> {
		// 1 px vertical separator between sidebar (or collapsed strip) and code panel.
		let make_separator = || {
			container(iced::widget::Space::new(Length::Fixed(1.0), Length::Fill))
				.height(Length::Fill)
				.style(move |_| container::Style {
					background: Some(Background::Color(SURFACE0)),
					..Default::default()
				})
		};

		if self.sidebar_visible {
			row![self.view_sidebar(), make_separator(), self.view_code_panel()]
				.width(Length::Fill)
				.height(Length::Fill)
				.into()
		} else {
			// Collapsed sidebar: narrow MANTLE strip with a › button at the top.
			let expand_btn = button(
				iced::widget::text("›").font(Font::MONOSPACE).size(14),
			)
			.style(|_, _| button::Style {
				background: Some(Background::Color(Color::TRANSPARENT)),
				text_color: SUBTEXT0,
				border: Border::default(),
				..Default::default()
			})
			.on_press(Message::ToggleSidebar)
			.padding(iced::Padding { top: 6.0, bottom: 6.0, left: 8.0, right: 8.0 });

			let collapsed_strip = container(
				column![expand_btn].width(Length::Fill),
			)
			.width(Length::Fixed(28.0))
			.height(Length::Fill)
			.style(move |_| container::Style {
				background: Some(Background::Color(MANTLE)),
				..Default::default()
			});

			row![collapsed_strip, make_separator(), self.view_code_panel()]
				.width(Length::Fill)
				.height(Length::Fill)
				.into()
		}
	}

	fn view_sidebar(&self) -> Element<'_, Message> {
		let mut items: Vec<Element<'_, Message>> = Vec::new();

		// Sidebar section header: ‹ toggle on the left, "files" label next to it.
		items.push(
			container(
				row![
					button(
						iced::widget::text("‹").font(Font::MONOSPACE).size(12),
					)
					.style(|_, _| button::Style {
						background: Some(Background::Color(Color::TRANSPARENT)),
						text_color: SUBTEXT0,
						border: Border::default(),
						..Default::default()
					})
					.on_press(Message::ToggleSidebar)
					.padding(iced::Padding { top: 0.0, bottom: 0.0, left: 0.0, right: 8.0 }),
					iced::widget::text("files")
						.font(Font::MONOSPACE)
						.size(10)
						.color(OVERLAY0),
				]
				.align_y(alignment::Vertical::Center),
			)
			.width(Length::Fill)
			.height(28)
			.align_y(alignment::Vertical::Center)
			.padding(iced::Padding::default().left(12))
			.style(move |_| container::Style {
				background: Some(Background::Color(SURFACE0)),
				..Default::default()
			})
			.into(),
		);

		items.extend(render_kvl_tree(&self.kvl_tree, &self.selected_kvl, &self.expanded_dirs, 0));

		container(
			scrollable(column(items).width(Length::Fill)).height(Length::Fill),
		)
		.width(Length::Fixed(SIDEBAR_W))
		.height(Length::Fill)
		.style(move |_| container::Style {
			background: Some(Background::Color(MANTLE)),
			..Default::default()
		})
		.into()
	}

	fn view_code_panel(&self) -> Element<'_, Message> {
		let display_rows = Arc::new(self.build_display_rows());
		let canvas_height = display_rows.len() as f32 * ROW_H + TOP_PAD * 2.0;

		let code_canvas = CodeCanvas {
			display_rows: Arc::clone(&display_rows),
			ksl_lines: Arc::clone(&self.ksl_lines),
			source_lines: Arc::clone(&self.source_lines),
			var_occurrences: Arc::clone(&self.var_occurrences),
			source_map: Arc::clone(&self.source_map),
			active_variable: self.active_variable.clone(),
		};

		column![
			panel_header(self.selected_name.clone()),
			scrollable(
				canvas(code_canvas)
					.width(Length::Fill)
					.height(Length::Fixed(canvas_height)),
			)
			.id(code_scrollable_id())
			.width(Length::Fill)
			.height(Length::Fill),
		]
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
	}

	/// Flatten KSL lines and expanded accordion entries into a single display list.
	fn build_display_rows(&self) -> Vec<DisplayRow> {
		let ksl_count = self.ksl_lines.len();
		let mut rows = Vec::with_capacity(ksl_count);

		for ksl_idx in 0..ksl_count {
			let span_idx = self.source_map.iter().position(|s| s.ksl_trigger_line == ksl_idx);

			let (is_expanded, label) = match span_idx {
				Some(si) => (self.expanded_spans.contains(&si), self.source_map[si].label),
				None => (false, ""),
			};

			rows.push(DisplayRow::KslLine {
				ksl_idx,
				span_idx,
				is_expanded,
				label,
			});

			if let Some(si) = span_idx
				&& self.expanded_spans.contains(&si)
			{
				let range = self.source_map[si].source_lines.clone();
				let last = range.end.saturating_sub(1);
				for source_idx in range {
					rows.push(DisplayRow::SourceLine {
						source_idx,
						is_last: source_idx == last,
					});
				}
			}
		}

		rows
	}
}

// ── Panel header widget ───────────────────────────────────────────────────────

fn panel_header<'a>(file_name: impl Into<String>) -> Element<'a, Message> {
	let title = file_name.into();
	column![
		container(
			row![
				iced::widget::text(title)
					.font(Font::MONOSPACE)
					.size(11)
					.color(OVERLAY0),
				container(iced::widget::text("KSL").font(Font::MONOSPACE).size(9).color(TEXT_COL))
					.padding(iced::Padding {
						top: 1.0,
						bottom: 1.0,
						left: 5.0,
						right: 5.0
					})
					.style(move |_| container::Style {
						background: Some(Background::Color(rgba(0x89, 0xb4, 0xfa, 0x55))),
						border: Border {
							radius: 3.0.into(),
							..Default::default()
						},
						..Default::default()
					}),
			]
			.spacing(8)
			.align_y(alignment::Vertical::Center)
		)
		.width(Length::Fill)
		.height(27)
		.align_y(alignment::Vertical::Center)
		.padding(iced::Padding::default().left(16))
		.style(move |_| container::Style {
			background: Some(Background::Color(MANTLE)),
			..Default::default()
		}),
		// 1 px bottom separator line
		container(iced::widget::Space::new(Length::Fill, 1)).style(move |_| container::Style {
			background: Some(Background::Color(SURFACE0)),
			..Default::default()
		}),
	]
	.into()
}

// ── Canvas program ────────────────────────────────────────────────────────────

/// Full-panel canvas: renders all display rows and handles mouse events.
/// Wrapped in `scrollable` — canvas height equals total content height.
struct CodeCanvas {
	display_rows: Arc<Vec<DisplayRow>>,
	ksl_lines: Arc<Vec<HighlightedLine>>,
	source_lines: Arc<Vec<HighlightedLine>>,
	var_occurrences: Arc<Vec<VarOccurrence>>,
	source_map: Arc<Vec<SourceSpan>>,
	active_variable: Option<String>,
}

impl canvas::Program<Message> for CodeCanvas {
	type State = ();

	fn update(
		&self,
		_state: &mut (),
		event: canvas::Event,
		bounds: Rectangle,
		cursor: mouse::Cursor,
	) -> (canvas::event::Status, Option<Message>) {
		let canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event else {
			return (canvas::event::Status::Ignored, None);
		};
		let Some(pos) = cursor.position_in(bounds) else {
			return (canvas::event::Status::Ignored, None);
		};

		let row_y = pos.y - TOP_PAD;
		if row_y < 0.0 {
			return (canvas::event::Status::Ignored, None);
		}
		let row_idx = (row_y / ROW_H) as usize;

		let Some(display_row) = self.display_rows.get(row_idx) else {
			return (canvas::event::Status::Ignored, None);
		};

		match display_row {
			DisplayRow::KslLine { ksl_idx, span_idx, .. } => {
				// ── Accordion button click ────────────────────────────────────
				if let Some(si) = span_idx {
					let btn_w = accordion_btn_width(self.source_map[*si].label);
					let btn_x = bounds.width - 8.0 - btn_w;
					if pos.x >= btn_x {
						return (canvas::event::Status::Captured, Some(Message::ToggleSpan(*si)));
					}
				}

				// ── Variable identifier click ─────────────────────────────────
				if pos.x >= CODE_X {
					let char_col = ((pos.x - CODE_X) / CHAR_W) as usize;
					let line = &self.ksl_lines[*ksl_idx];
					for token in lang::lexer::tokenize(&line.text) {
						if token.kind == TokenKind::Ident && char_col >= token.range.start && char_col < token.range.end
						{
							let name = line.text[token.range].to_string();
							let msg = if Some(&name) == self.active_variable.as_ref() {
								Message::SetActiveVariable(None) // toggle off on second click
							} else {
								Message::SetActiveVariable(Some((name, *ksl_idx)))
							};
							return (canvas::event::Status::Captured, Some(msg));
						}
					}
					// Click on whitespace / non-ident: clear selection
					if self.active_variable.is_some() {
						return (canvas::event::Status::Captured, Some(Message::SetActiveVariable(None)));
					}
				}
			}
			DisplayRow::SourceLine { .. } => {}
		}

		(canvas::event::Status::Ignored, None)
	}

	fn draw(
		&self,
		_state: &(),
		renderer: &iced::Renderer,
		_theme: &iced::Theme,
		bounds: Rectangle,
		_cursor: mouse::Cursor,
	) -> Vec<canvas::Geometry> {
		let mut frame = canvas::Frame::new(renderer, bounds.size());

		// Canvas background
		frame.fill_rectangle(Point::ORIGIN, bounds.size(), BASE);

		for (row_idx, display_row) in self.display_rows.iter().enumerate() {
			let y = TOP_PAD + row_idx as f32 * ROW_H;

			match display_row {
				DisplayRow::KslLine {
					ksl_idx,
					span_idx,
					is_expanded,
					label,
				} => {
					let line = &self.ksl_lines[*ksl_idx];

					// Variable occurrence background tints
					for occ in self.var_occurrences.iter().filter(|o| o.ksl_line_idx == *ksl_idx) {
						let x0 = CODE_X + occ.byte_range.start as f32 * CHAR_W;
						let x1 = CODE_X + occ.byte_range.end as f32 * CHAR_W;
						frame.fill_rectangle(Point::new(x0, y), Size::new(x1 - x0, ROW_H), var_bg(occ.role));
					}

					draw_gutter(&mut frame, *ksl_idx + 1, y, GUTTER_FG);
					draw_line(&mut frame, line, CODE_X, y + 4.5);

					if span_idx.is_some() {
						draw_accordion_btn(&mut frame, bounds.width, y, *is_expanded, label);
					}
				}

				DisplayRow::SourceLine { source_idx, is_last } => {
					let line = &self.source_lines[*source_idx];
					let row_h = if *is_last { ROW_H + 4.0 } else { ROW_H };

					frame.fill_rectangle(Point::new(0.0, y), Size::new(bounds.width, row_h), SNIPPET_BG);
					// Left orange border (2 px)
					frame.fill_rectangle(Point::new(0.0, y), Size::new(2.0, row_h), SNIPPET_BORDER);

					draw_gutter(&mut frame, *source_idx + 1, y, GUTTER_DIM);
					draw_line(&mut frame, line, CODE_X, y + 4.5);
				}
			}
		}

		// Connection gutter: rail + dots in the left margin
		if !self.var_occurrences.is_empty() {
			draw_connection_gutter(&mut frame, &self.display_rows, &self.var_occurrences);
		}

		vec![frame.into_geometry()]
	}

	fn mouse_interaction(&self, _state: &(), _bounds: Rectangle, _cursor: mouse::Cursor) -> mouse::Interaction {
		mouse::Interaction::default()
	}
}

// ── Canvas rendering helpers ──────────────────────────────────────────────────

/// Approximate pixel width of the accordion button for a given label.
fn accordion_btn_width(label: &str) -> f32 {
	// "▶ C  " is 5 visible characters; add the label length + 16 px horizontal padding.
	(5 + label.len()) as f32 * CHAR_W + 16.0
}

/// Render a right-aligned line number into the gutter column (right of CONN_GUTTER_W).
fn draw_gutter(frame: &mut canvas::Frame, num: usize, y: f32, color: Color) {
	let s = num.to_string();
	let text_w = s.len() as f32 * CHAR_W;
	let x = CODE_X - 16.0 - text_w;
	draw_text(frame, &s, x, y + 4.5, color);
}

/// Render a syntax-highlighted line starting at `x_start`.
/// Fills gaps between highlight spans with the default text colour.
fn draw_line(frame: &mut canvas::Frame, line: &HighlightedLine, x_start: f32, y: f32) {
	let text = &line.text;
	let mut cursor = 0usize;

	for (range, color) in &line.highlights {
		if cursor < range.start {
			let seg = &text[cursor..range.start];
			if !seg.is_empty() {
				draw_text(frame, seg, x_start + cursor as f32 * CHAR_W, y, TEXT_COL);
			}
		}
		let seg = &text[range.clone()];
		if !seg.is_empty() {
			draw_text(frame, seg, x_start + range.start as f32 * CHAR_W, y, *color);
		}
		cursor = range.end;
	}

	if cursor < text.len() {
		let seg = &text[cursor..];
		if !seg.is_empty() {
			draw_text(frame, seg, x_start + cursor as f32 * CHAR_W, y, TEXT_COL);
		}
	}
}

/// Draw the accordion ▶/▼ toggle badge on the right side of a KSL row.
fn draw_accordion_btn(frame: &mut canvas::Frame, canvas_w: f32, y: f32, is_expanded: bool, label: &str) {
	let chevron = if is_expanded { "▼" } else { "▶" };
	let btn_text = format!("{chevron} C  {label}");
	let btn_w = accordion_btn_width(label);
	let btn_x = canvas_w - 8.0 - btn_w;
	let btn_y = y + 3.0;

	frame.fill_rectangle(Point::new(btn_x, btn_y), Size::new(btn_w, 16.0), ACCORDION_BG);
	draw_text(frame, &btn_text, btn_x + 8.0, btn_y + 2.0, ACCORDION_FG);
}

/// Low-level text draw: monospace, FONT_SIZE, top-left origin at (x, y).
fn draw_text(frame: &mut canvas::Frame, content: &str, x: f32, y: f32, color: Color) {
	frame.fill_text(canvas::Text {
		content: content.to_string(),
		position: Point::new(x, y),
		color,
		size: Pixels(FONT_SIZE),
		font: Font::MONOSPACE,
		horizontal_alignment: alignment::Horizontal::Left,
		vertical_alignment: alignment::Vertical::Top,
		..canvas::Text::default()
	});
}

// ── Connection gutter ─────────────────────────────────────────────────────────
//
// Drawn in the CONN_GUTTER_W strip to the LEFT of the line-number gutter.
// The canvas scrolls with the content so dots sit at the exact row y.
//
//   ┌─ CONN_GUTTER_W (20 px) ─┬─ GUTTER_W ─┬─ code ────────
//   │  ●  def  (lavender)     │   line #   │ let local_90 …
//   │  │                      │            │ foo(x)
//   │  ●  read (teal)         │            │ local_90 = …
//   │  │                      │            │ …
//   │  ●  write (yellow)      │            │ bar(local_90)

fn draw_connection_gutter(frame: &mut canvas::Frame, display_rows: &[DisplayRow], occurrences: &[VarOccurrence]) {
	// Dot is centered horizontally inside CONN_GUTTER_W.
	let dot_x = CONN_GUTTER_W / 2.0;

	struct Entry {
		y: f32,
		role: VarRole,
	}

	let entries: Vec<Entry> = occurrences
		.iter()
		.filter(|occ| occ.role != VarRole::TypeRef) // type names are not data flow
		.filter_map(|occ| {
			let display_idx = display_rows
				.iter()
				.position(|r| matches!(r, DisplayRow::KslLine { ksl_idx, .. } if *ksl_idx == occ.ksl_line_idx))?;
			let y = TOP_PAD + display_idx as f32 * ROW_H + ROW_H / 2.0;
			Some(Entry { y, role: occ.role })
		})
		.collect();

	if entries.is_empty() {
		return;
	}

	let y_first = entries.first().unwrap().y;
	let y_last = entries.last().unwrap().y;

	// ── Vertical spanning rail ────────────────────────────────────────────────
	if y_first < y_last {
		frame.fill_rectangle(
			Point::new(dot_x - RAIL_W / 2.0, y_first),
			Size::new(RAIL_W, y_last - y_first),
			RAIL_COL,
		);
	}

	// ── Per-role arrow markers ────────────────────────────────────────────────
	//
	//   Definition (green)   ↓  downward triangle on the rail — source of flow
	//   Read       (blue)    ──▶  tick + rightward triangle  — value flows out to code
	//   Write      (yellow)  ──◀  tick + leftward triangle   — value flows in from code
	//
	// tick_x1 is placed just left of the line-number column so the arrow tip
	// sits flush against the numbers with a small gap, regardless of digit count.
	let tick_x0 = dot_x + DOT_R + 2.0;
	let depth = 6.0; // arrowhead depth  (along the dominant axis)
	let half = 4.0; // arrowhead half-width (perpendicular axis)

	// Compute tick_x1 so the arrow tip lands just left of the line numbers.
	let max_line_num = display_rows
		.iter()
		.filter_map(|r| {
			if let DisplayRow::KslLine { ksl_idx, .. } = r {
				Some(ksl_idx + 1)
			} else {
				None
			}
		})
		.max()
		.unwrap_or(1);
	let num_digits = max_line_num.to_string().len();
	let tick_x1 = CODE_X - 16.0 - num_digits as f32 * CHAR_W - 4.0;

	for entry in &entries {
		let color = var_dot(entry.role);

		match entry.role {
			VarRole::Definition => {
				// ↓ downward triangle centred on (dot_x, entry.y)
				let mut path = canvas::path::Builder::new();
				path.move_to(Point::new(dot_x, entry.y + depth * 0.5)); // tip
				path.line_to(Point::new(dot_x - half, entry.y - depth * 0.5)); // top-left
				path.line_to(Point::new(dot_x + half, entry.y - depth * 0.5)); // top-right
				path.close();
				frame.fill(&path.build(), color);
			}
			VarRole::Read => {
				// ──▶  tick + right-pointing triangle at tick_x1
				let line_end = tick_x1 - depth;
				if line_end > tick_x0 {
					frame.fill_rectangle(
						Point::new(tick_x0, entry.y - 0.5),
						Size::new(line_end - tick_x0, 1.0),
						color,
					);
				}
				let mut path = canvas::path::Builder::new();
				path.move_to(Point::new(tick_x1, entry.y)); // tip (right)
				path.line_to(Point::new(tick_x1 - depth, entry.y - half)); // base top
				path.line_to(Point::new(tick_x1 - depth, entry.y + half)); // base bottom
				path.close();
				frame.fill(&path.build(), color);
			}
			VarRole::Write => {
				// ──◀  tick + left-pointing triangle at tick_x1
				// The open base faces the code (right); tip points back toward the variable.
				let line_end = tick_x1 - depth;
				if line_end > tick_x0 {
					frame.fill_rectangle(
						Point::new(tick_x0, entry.y - 0.5),
						Size::new(line_end - tick_x0, 1.0),
						color,
					);
				}
				let mut path = canvas::path::Builder::new();
				path.move_to(Point::new(tick_x1 - depth, entry.y)); // tip (left)
				path.line_to(Point::new(tick_x1, entry.y - half)); // base top-right
				path.line_to(Point::new(tick_x1, entry.y + half)); // base bottom-right
				path.close();
				frame.fill(&path.build(), color);
			}
			VarRole::TypeRef => {}
		}
	}
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> iced::Result {
	// .kvp path: CLI arg > dev default > None (sample mode).
	let kvp_path: Option<PathBuf> = std::env::args()
		.nth(1)
		.map(Into::into)
		.or_else(|| {
			// During development, fall back to the bundled example project.
			let default = PathBuf::from("./examples/after-effects.kvp");
			default.exists().then_some(default)
		});

	iced::application("kaiseki", KaisekiState::update, KaisekiState::view)
		.window(iced::window::Settings {
			size: iced::Size::new(1400.0, 900.0),
			min_size: Some(iced::Size::new(800.0, 600.0)),
			..Default::default()
		})
		.run_with(move || KaisekiState::new(kvp_path.clone()))
}
