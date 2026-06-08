mod lang;

use std::{collections::HashSet, ops::Range, sync::Arc};

use iced::{
	alignment, mouse,
	widget::{canvas, column, container, row, scrollable},
	Background, Border, Color, Element, Font, Length, Pixels, Point, Rectangle, Size, Task,
};
use syntect::{
	easy::HighlightLines,
	highlighting::ThemeSet,
	parsing::SyntaxSet,
	util::LinesWithEndings,
};

use lang::{SourceSpan, SAMPLE_KSL, SAMPLE_SOURCE_MAP};
use lang::lexer::TokenKind;

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

// ── Layout constants (match the original GPUI version) ────────────────────────

const ROW_H:          f32 = 22.0;
const GUTTER_W:       f32 = 56.0;
const CHAR_W:         f32 = 7.8; // approx. Consolas 13 px glyph advance
const FONT_SIZE:      f32 = 13.0;
const TOP_PAD:        f32 = 8.0;

// Connection gutter geometry
const DOT_FROM_RIGHT: f32 = 16.0;
const LEADER_GAP:     f32 = 6.0;
const DOT_R:          f32 = 3.0;
const RAIL_W:         f32 = 2.0;
const ARROW_W:        f32 = 5.0;
const ARROW_H:        f32 = 4.0;

// ── Color palette (Catppuccin Mocha) ─────────────────────────────────────────

const fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
}

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
	Color { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
}

const BASE:           Color = rgb(0x1e, 0x1e, 0x2e);
const MANTLE:         Color = rgb(0x18, 0x18, 0x25);
const SURFACE0:       Color = rgb(0x31, 0x32, 0x44);
const OVERLAY0:       Color = rgb(0x6c, 0x70, 0x86);
const SUBTEXT1:       Color = rgb(0xba, 0xc2, 0xde);
const SUBTEXT0:       Color = rgb(0xa6, 0xad, 0xc8);
const TEXT_COL:       Color = rgb(0xcd, 0xd6, 0xf4);
const LAVENDER:       Color = rgb(0xb4, 0xbe, 0xfe);
const SKY:            Color = rgb(0x89, 0xdc, 0xeb);
const TEAL:           Color = rgb(0x94, 0xe2, 0xd5);
const GREEN:          Color = rgb(0xa6, 0xe3, 0xa1);
const YELLOW:         Color = rgb(0xf9, 0xe2, 0xaf);
const PEACH:          Color = rgb(0xfa, 0xb3, 0x87);
const MAUVE:          Color = rgb(0xcb, 0xa6, 0xf7);

const GUTTER_FG:      Color = rgba(0x58, 0x5b, 0x70, 0xff);
const GUTTER_DIM:     Color = rgba(0x58, 0x5b, 0x70, 0x44);
const SNIPPET_BG:     Color = rgba(0x18, 0x18, 0x25, 0xdd);
const ACCORDION_FG:   Color = rgba(0xe6, 0x89, 0x45, 0xcc);
const ACCORDION_BG:   Color = rgba(0x31, 0x32, 0x44, 0x88);
const SNIPPET_BORDER: Color = rgba(0xe6, 0x89, 0x45, 0x66);
const RAIL_COL:       Color = rgba(0x58, 0x5b, 0x70, 0x88);
const ARROW_COL:      Color = rgba(0x58, 0x5b, 0x70, 0xcc);
const LEADER_COL:     Color = rgba(0x58, 0x5b, 0x70, 0x66);
const SAPPHIRE_DIM:   Color = rgba(0x74, 0xc7, 0xec, 0x99);

const fn var_bg(role: VarRole) -> Color {
	match role {
		VarRole::Definition => rgba(0xb4, 0xbe, 0xfe, 0x55),
		VarRole::Write      => rgba(0xf9, 0xe2, 0xaf, 0x55),
		VarRole::Read       => rgba(0x94, 0xe2, 0xd5, 0x55),
	}
}

const fn var_dot(role: VarRole) -> Color {
	match role {
		VarRole::Definition => LAVENDER,
		VarRole::Write      => YELLOW,
		VarRole::Read       => TEAL,
	}
}

// ── Domain types ──────────────────────────────────────────────────────────────

/// One syntax-highlighted line: the raw text plus a list of (byte-range, colour) spans.
#[derive(Clone, Debug)]
struct HighlightedLine {
	text:       String,
	/// (byte_range, foreground_color) — non-overlapping, source order.
	highlights: Vec<(Range<usize>, Color)>,
}

/// Semantic role of one identifier occurrence within the KSL source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VarRole {
	Definition,
	Write,
	Read,
}

/// One occurrence of the active variable that should be highlighted.
#[derive(Clone, Debug)]
struct VarOccurrence {
	ksl_line_idx: usize,
	byte_range:   Range<usize>,
	role:         VarRole,
}

/// Flat display-list entry, either a KSL line or an expanded C source line.
#[derive(Clone, Debug)]
enum DisplayRow {
	KslLine {
		ksl_idx:     usize,
		span_idx:    Option<usize>, // Some → this row carries the accordion toggle button
		is_expanded: bool,
		label:       &'static str,
	},
	SourceLine { source_idx: usize, is_last: bool },
}

// ── KSL syntax highlighting ───────────────────────────────────────────────────

fn token_color(kind: TokenKind) -> Option<Color> {
	Some(match kind {
		TokenKind::Keyword    => MAUVE,
		TokenKind::Atom       => PEACH,
		TokenKind::Number | TokenKind::Offset => YELLOW,
		TokenKind::DocComment => GREEN,
		TokenKind::SectionSep => SAPPHIRE_DIM,
		TokenKind::Comment    => OVERLAY0,
		TokenKind::Operator   => SKY,
		TokenKind::Punct      => SUBTEXT1,
		TokenKind::Ident      => TEXT_COL,
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
			HighlightedLine { text: line.to_string(), highlights }
		})
		.collect()
}

// ── Syntect code highlighting ─────────────────────────────────────────────────

fn build_source_lines(code: &str) -> Vec<HighlightedLine> {
	let ss     = SyntaxSet::load_defaults_newlines();
	let ts     = ThemeSet::load_defaults();
	let theme  = &ts.themes["base16-ocean.dark"];
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

/// Scan every KSL line for tokens whose text equals `name` and classify each
/// occurrence as Definition / Write / Read based on neighbouring tokens.
///
/// Role heuristics (same as GPUI version):
///   • Preceded by `let` or `var`     → Definition
///   • Followed by `=` (not `==`)     → Write
///   • Everything else                → Read
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

			let role = if matches!(&prev_kind, Some((TokenKind::Keyword, kw)) if *kw == "let" || *kw == "var") {
				VarRole::Definition
			} else if matches!(&next, Some((TokenKind::Operator, op)) if *op == "=") {
				VarRole::Write
			} else {
				VarRole::Read
			};

			result.push(VarOccurrence {
				ksl_line_idx: line_idx,
				byte_range:   token.range.clone(),
				role,
			});
		}
	}

	result
}

// ── Application state ─────────────────────────────────────────────────────────

struct KaisekiState {
	ksl_lines:       Arc<Vec<HighlightedLine>>,
	source_lines:    Arc<Vec<HighlightedLine>>,
	source_map:      Arc<Vec<SourceSpan>>,
	expanded_spans:  HashSet<usize>,
	active_variable: Option<String>,
	var_occurrences: Arc<Vec<VarOccurrence>>,
}

#[derive(Debug, Clone)]
enum Message {
	ToggleSpan(usize),
	SetActiveVariable(Option<String>),
}

impl KaisekiState {
	fn new() -> (Self, Task<Message>) {
		let state = Self {
			ksl_lines:    Arc::new(build_ksl_lines(SAMPLE_KSL)),
			source_lines: Arc::new(build_source_lines(SAMPLE_CODE)),
			source_map:   Arc::new(
				SAMPLE_SOURCE_MAP
					.iter()
					.map(|s| SourceSpan {
						label:            s.label,
						ksl_trigger_line: s.ksl_trigger_line,
						source_lines:     s.source_lines.clone(),
					})
					.collect(),
			),
			expanded_spans:  HashSet::new(),
			active_variable: None,
			var_occurrences: Arc::new(Vec::new()),
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
			Message::SetActiveVariable(name) => {
				self.var_occurrences = Arc::new(match &name {
					Some(n) => find_var_occurrences(n, &self.ksl_lines),
					None    => Vec::new(),
				});
				self.active_variable = name;
			}
		}
		Task::none()
	}

	fn view(&self) -> Element<'_, Message> {
		let display_rows  = Arc::new(self.build_display_rows());
		let canvas_height = display_rows.len() as f32 * ROW_H + TOP_PAD * 2.0;

		let code_canvas = CodeCanvas {
			display_rows:    Arc::clone(&display_rows),
			ksl_lines:       Arc::clone(&self.ksl_lines),
			source_lines:    Arc::clone(&self.source_lines),
			var_occurrences: Arc::clone(&self.var_occurrences),
			source_map:      Arc::clone(&self.source_map),
			active_variable: self.active_variable.clone(),
		};

		column![
			// ── App title bar ────────────────────────────────────────────────
			container(
				iced::widget::text("kaiseki")
					.font(Font::MONOSPACE)
					.size(13)
					.color(SUBTEXT0)
			)
			.width(Length::Fill)
			.height(40)
			.align_y(alignment::Vertical::Center)
			.padding(iced::Padding::default().left(16))
			.style(move |_| container::Style {
				background: Some(Background::Color(MANTLE)),
				..Default::default()
			}),

			// ── Panel header with language badge ─────────────────────────────
			panel_header(),

			// ── Scrollable code canvas ───────────────────────────────────────
			// The canvas is given a fixed height equal to the full content height.
			// iced's scrollable widget handles viewport clipping; the canvas renders
			// all rows and the dots scroll with the lines they annotate.
			scrollable(
				canvas(code_canvas)
					.width(Length::Fill)
					.height(Length::Fixed(canvas_height))
			)
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
		let mut rows  = Vec::with_capacity(ksl_count);

		for ksl_idx in 0..ksl_count {
			let span_idx = self
				.source_map
				.iter()
				.position(|s| s.ksl_trigger_line == ksl_idx);

			let (is_expanded, label) = match span_idx {
				Some(si) => (self.expanded_spans.contains(&si), self.source_map[si].label),
				None     => (false, ""),
			};

			rows.push(DisplayRow::KslLine { ksl_idx, span_idx, is_expanded, label });

			if let Some(si) = span_idx {
				if self.expanded_spans.contains(&si) {
					let range = self.source_map[si].source_lines.clone();
					let last  = range.end.saturating_sub(1);
					for source_idx in range {
						rows.push(DisplayRow::SourceLine {
							source_idx,
							is_last: source_idx == last,
						});
					}
				}
			}
		}

		rows
	}
}

// ── Panel header widget ───────────────────────────────────────────────────────

fn panel_header<'a>() -> Element<'a, Message> {
	column![
		container(
			row![
				iced::widget::text("lifted.ksl — get_stream_fpv")
					.font(Font::MONOSPACE)
					.size(11)
					.color(OVERLAY0),
				container(
					iced::widget::text("KSL")
						.font(Font::MONOSPACE)
						.size(9)
						.color(TEXT_COL)
				)
				.padding(iced::Padding { top: 1.0, bottom: 1.0, left: 5.0, right: 5.0 })
				.style(move |_| container::Style {
					background: Some(Background::Color(rgba(0x89, 0xb4, 0xfa, 0x55))),
					border: Border { radius: 3.0.into(), ..Default::default() },
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
		container(iced::widget::Space::new(Length::Fill, 1))
			.style(move |_| container::Style {
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
	display_rows:    Arc<Vec<DisplayRow>>,
	ksl_lines:       Arc<Vec<HighlightedLine>>,
	source_lines:    Arc<Vec<HighlightedLine>>,
	var_occurrences: Arc<Vec<VarOccurrence>>,
	source_map:      Arc<Vec<SourceSpan>>,
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
				if pos.x >= GUTTER_W {
					let char_col = ((pos.x - GUTTER_W) / CHAR_W) as usize;
					let line     = &self.ksl_lines[*ksl_idx];
					for token in lang::lexer::tokenize(&line.text) {
						if token.kind == TokenKind::Ident
							&& char_col >= token.range.start
							&& char_col < token.range.end
						{
							let name = line.text[token.range].to_string();
							let msg  = if Some(&name) == self.active_variable.as_ref() {
								Message::SetActiveVariable(None) // toggle off on second click
							} else {
								Message::SetActiveVariable(Some(name))
							};
							return (canvas::event::Status::Captured, Some(msg));
						}
					}
					// Click on whitespace / non-ident: clear selection
					if self.active_variable.is_some() {
						return (
							canvas::event::Status::Captured,
							Some(Message::SetActiveVariable(None)),
						);
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
				DisplayRow::KslLine { ksl_idx, span_idx, is_expanded, label } => {
					let line = &self.ksl_lines[*ksl_idx];

					// Variable occurrence background tints
					for occ in self.var_occurrences.iter().filter(|o| o.ksl_line_idx == *ksl_idx) {
						let x0 = GUTTER_W + occ.byte_range.start as f32 * CHAR_W;
						let x1 = GUTTER_W + occ.byte_range.end as f32 * CHAR_W;
						frame.fill_rectangle(
							Point::new(x0, y),
							Size::new(x1 - x0, ROW_H),
							var_bg(occ.role),
						);
					}

					draw_gutter(&mut frame, *ksl_idx + 1, y, GUTTER_FG);
					draw_line(&mut frame, line, GUTTER_W, y + 4.5);

					if span_idx.is_some() {
						draw_accordion_btn(&mut frame, bounds.width, y, *is_expanded, label);
					}
				}

				DisplayRow::SourceLine { source_idx, is_last } => {
					let line   = &self.source_lines[*source_idx];
					let row_h  = if *is_last { ROW_H + 4.0 } else { ROW_H };

					frame.fill_rectangle(
						Point::new(0.0, y),
						Size::new(bounds.width, row_h),
						SNIPPET_BG,
					);
					// Left orange border (2 px)
					frame.fill_rectangle(
						Point::new(0.0, y),
						Size::new(2.0, row_h),
						SNIPPET_BORDER,
					);

					draw_gutter(&mut frame, *source_idx + 1, y, GUTTER_DIM);
					draw_line(&mut frame, line, GUTTER_W, y + 4.5);
				}
			}
		}

		// Connection gutter: rail, arrows, leader lines, dots
		if !self.var_occurrences.is_empty() {
			draw_connection_gutter(
				&mut frame,
				bounds,
				&self.display_rows,
				&self.ksl_lines,
				&self.var_occurrences,
			);
		}

		vec![frame.into_geometry()]
	}

	fn mouse_interaction(
		&self,
		_state: &(),
		_bounds: Rectangle,
		_cursor: mouse::Cursor,
	) -> mouse::Interaction {
		mouse::Interaction::default()
	}
}

// ── Canvas rendering helpers ──────────────────────────────────────────────────

/// Approximate pixel width of the accordion button for a given label.
fn accordion_btn_width(label: &str) -> f32 {
	// "▶ C  " is 5 visible characters; add the label length + 16 px horizontal padding.
	(5 + label.len()) as f32 * CHAR_W + 16.0
}

/// Render a right-aligned line number into the gutter column.
fn draw_gutter(frame: &mut canvas::Frame, num: usize, y: f32, color: Color) {
	let s      = num.to_string();
	let text_w = s.len() as f32 * CHAR_W;
	let x      = GUTTER_W - 16.0 - text_w;
	draw_text(frame, &s, x, y + 4.5, color);
}

/// Render a syntax-highlighted line starting at `x_start`.
/// Fills gaps between highlight spans with the default text colour.
fn draw_line(frame: &mut canvas::Frame, line: &HighlightedLine, x_start: f32, y: f32) {
	let text   = &line.text;
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
fn draw_accordion_btn(
	frame:       &mut canvas::Frame,
	canvas_w:    f32,
	y:           f32,
	is_expanded: bool,
	label:       &str,
) {
	let chevron  = if is_expanded { "▼" } else { "▶" };
	let btn_text = format!("{chevron} C  {label}");
	let btn_w    = accordion_btn_width(label);
	let btn_x    = canvas_w - 8.0 - btn_w;
	let btn_y    = y + 3.0;

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
// Renders the variable-occurrence visualisation directly in the code canvas.
// Because the canvas scrolls with the content, dots sit exactly at the y
// position of the rows they annotate — no scroll-offset correction needed.
//
//   │←── gutter 56px ──→│←── code ──────────→│ DOT_FROM_RIGHT │
//   │                    │                    │                │
//   │                    │ let local_90 = …   ├────────────────● def (lavender)
//   │                    │                    │                │
//   │                    │ local_90 = val;    ├───────────────●  write (yellow)
//   │                    │                    │                │
//   │                    │ foo(local_90)       ├──────────────●─┘ read (teal)

fn draw_connection_gutter(
	frame:        &mut canvas::Frame,
	bounds:       Rectangle,
	display_rows: &[DisplayRow],
	lines:        &[HighlightedLine],
	occurrences:  &[VarOccurrence],
) {
	let dot_x = bounds.width - DOT_FROM_RIGHT;

	struct Entry { y: f32, role: VarRole, text_len: usize }

	let entries: Vec<Entry> = occurrences
		.iter()
		.filter_map(|occ| {
			// Find the display-list index of the KSL row that holds this occurrence.
			let display_idx = display_rows.iter().position(|r| {
				matches!(r, DisplayRow::KslLine { ksl_idx, .. } if *ksl_idx == occ.ksl_line_idx)
			})?;
			let y        = TOP_PAD + display_idx as f32 * ROW_H + ROW_H / 2.0;
			let text_len = lines.get(occ.ksl_line_idx).map(|l| l.text.len()).unwrap_or(0);
			Some(Entry { y, role: occ.role, text_len })
		})
		.collect();

	if entries.is_empty() {
		return;
	}

	let y_first = entries.first().unwrap().y;
	let y_last  = entries.last().unwrap().y;

	// ── Vertical spanning rail ────────────────────────────────────────────────
	if y_first < y_last {
		frame.fill_rectangle(
			Point::new(dot_x - RAIL_W / 2.0, y_first),
			Size::new(RAIL_W, y_last - y_first),
			RAIL_COL,
		);
	}

	// ── Direction arrows between consecutive occurrences ──────────────────────
	for pair in entries.windows(2) {
		let y_mid = (pair[0].y + pair[1].y) / 2.0;
		let mut path = canvas::path::Builder::new();
		path.move_to(Point::new(dot_x - ARROW_W / 2.0, y_mid - ARROW_H / 2.0));
		path.line_to(Point::new(dot_x + ARROW_W / 2.0, y_mid - ARROW_H / 2.0));
		path.line_to(Point::new(dot_x, y_mid + ARROW_H / 2.0));
		path.close();
		frame.fill(&path.build(), ARROW_COL);
	}

	// ── Leader lines + role-coloured dots ─────────────────────────────────────
	for entry in &entries {
		let text_end_x = GUTTER_W + entry.text_len as f32 * CHAR_W;
		let lx0        = text_end_x + LEADER_GAP;
		let lx1        = dot_x - DOT_R - LEADER_GAP;
		if lx1 > lx0 {
			frame.fill_rectangle(
				Point::new(lx0, entry.y - 0.5),
				Size::new(lx1 - lx0, 1.0),
				LEADER_COL,
			);
		}

		// Role-coloured circle dot
		let mut path = canvas::path::Builder::new();
		path.circle(Point::new(dot_x, entry.y), DOT_R);
		frame.fill(&path.build(), var_dot(entry.role));
	}
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> iced::Result {
	iced::application("kaiseki", KaisekiState::update, KaisekiState::view)
		.window(iced::window::Settings {
			size:     iced::Size::new(1400.0, 900.0),
			min_size: Some(iced::Size::new(800.0, 600.0)),
			..Default::default()
		})
		.run_with(KaisekiState::new)
}
