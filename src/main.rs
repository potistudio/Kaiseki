mod lang;

use gpui::{prelude::*, *};
use std::{collections::HashSet, ops::Range, sync::Arc};
use syntect::{
	easy::HighlightLines,
	highlighting::{FontStyle as SyntectFontStyle, ThemeSet},
	parsing::SyntaxSet,
	util::LinesWithEndings,
};

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

// -- Data model ---------------------------------------------------------------

struct HighlightedLine {
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
}

struct SmoothScroll {
	current_y: f32,
	target_y: f32,
	animating: bool,
}

struct Panel {
	lines: Arc<Vec<HighlightedLine>>,
	scroll_handle: UniformListScrollHandle,
	smooth: SmoothScroll,
}

impl Panel {
	fn from_code(code: &str, lang_ext: &str) -> Self {
		let ss = SyntaxSet::load_defaults_newlines();
		let ts = ThemeSet::load_defaults();
		let theme = &ts.themes["base16-ocean.dark"];
		let syntax = ss
			.find_syntax_by_extension(lang_ext)
			.unwrap_or_else(|| ss.find_syntax_plain_text());
		let mut hl = HighlightLines::new(syntax, theme);

		let lines: Vec<HighlightedLine> = LinesWithEndings::from(code)
			.map(|line| {
				let ranges = hl.highlight_line(line, &ss).unwrap_or_default();
				let mut text = String::new();
				let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();
				let mut offset = 0usize;

				for (style, frag) in &ranges {
					let end = offset + frag.len();
					highlights.push((
						offset..end,
						HighlightStyle {
							color: Some(syntect_to_hsla(style.foreground)),
							font_weight: style
								.font_style
								.contains(SyntectFontStyle::BOLD)
								.then_some(FontWeight::BOLD),
							font_style: style
								.font_style
								.contains(SyntectFontStyle::ITALIC)
								.then_some(FontStyle::Italic),
							..Default::default()
						},
					));
					text.push_str(frag);
					offset = end;
				}

				// Strip trailing newline and clamp highlight ranges to match.
				if text.ends_with('\n') {
					text.pop();
					let len = text.len();
					for (r, _) in &mut highlights {
						r.end = r.end.min(len);
					}
					highlights.retain(|(r, _)| r.start < r.end);
				}

				HighlightedLine { text: text.into(), highlights }
			})
			.collect();

		Self {
			lines: Arc::new(lines),
			scroll_handle: UniformListScrollHandle::new(),
			smooth: SmoothScroll { current_y: 0.0, target_y: 0.0, animating: false },
		}
	}
}

// -- Color helpers ------------------------------------------------------------

fn syntect_to_hsla(c: syntect::highlighting::Color) -> Hsla {
	Rgba {
		r: c.r as f32 / 255.0,
		g: c.g as f32 / 255.0,
		b: c.b as f32 / 255.0,
		a: c.a as f32 / 255.0,
	}
	.into()
}

// -- Smooth-scroll helpers ----------------------------------------------------

/// Advance one animation tick for a panel's smooth-scroll state.
/// Mutates `panel.smooth` and repositions the underlying scroll offset.
fn advance_smooth_scroll(panel: &mut Panel, window: &mut Window) {
	if !panel.smooth.animating {
		return;
	}
	let diff = panel.smooth.target_y - panel.smooth.current_y;
	if diff.abs() < 0.5 {
		panel.smooth.current_y = panel.smooth.target_y;
		panel.smooth.animating = false;
	} else {
		panel.smooth.current_y += diff * 0.18;
		window.request_animation_frame();
	}
	let base = panel.scroll_handle.0.borrow().base_handle.clone();
	base.set_offset(point(px(0.0), px(panel.smooth.current_y)));
}

/// Process a scroll-wheel event for one panel.
/// Returns `true` when a re-render should be requested via `cx.notify()`.
fn handle_panel_scroll(
	panel: &mut Panel,
	event: &ScrollWheelEvent,
	window: &mut Window,
) -> bool {
	let actual_y = panel.scroll_handle.0.borrow().base_handle.offset().y.to_f64() as f32;

	if event.delta.precise() {
		// Touchpad: stay in sync, let GPUI drive natively.
		panel.smooth.current_y = actual_y;
		panel.smooth.target_y = actual_y;
		return false;
	}

	// Mouse wheel: undo the instant scroll GPUI already applied, drive our animation.
	let line_height = window.line_height();
	let delta_y = event.delta.pixel_delta(line_height).y.to_f64() as f32;

	{
		let state = panel.scroll_handle.0.borrow();
		state.base_handle.set_offset(point(px(0.0), px(panel.smooth.current_y)));
	}

	panel.smooth.target_y += delta_y;
	if let Some(size) = panel.scroll_handle.0.borrow().last_item_size {
		let max_neg =
			-(size.contents.height.to_f64() - size.item.height.to_f64()).max(0.0) as f32;
		panel.smooth.target_y = panel.smooth.target_y.max(max_neg).min(0.0);
	}

	panel.smooth.animating = true;
	true
}

// -- Code row element ---------------------------------------------------------

fn render_code_row(
	num: String,
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
) -> Div {
	div()
		.h(px(22.))
		.flex()
		.flex_row()
		.items_center()
		.font_family("Consolas")
		.text_size(px(13.))
		// Gutter
		.child(
			div()
				.w(px(56.))
				.h_full()
				.flex()
				.items_center()
				.justify_end()
				.pr(px(16.))
				.text_size(px(12.))
				.text_color(rgba(0x585b70ff)) // Overlay0
				.flex_shrink_0()
				.child(num),
		)
		// Syntax-highlighted text
		.child(StyledText::new(text).with_highlights(highlights))
}

// -- Accordion display row ----------------------------------------------------

/// One entry in the flattened list rendered by the KSL panel.
/// Normal KSL lines and expanded C-source snippets share the same 22-px row
/// height, so a single uniform_list-style loop can handle both.
#[derive(Clone)]
enum DisplayRow {
	/// A normal KSL code line.  When `span_idx` is Some this line also shows
	/// the accordion toggle button (it is the section-header comment line).
	KslLine { ksl_idx: usize, span_idx: Option<usize>, is_expanded: bool, label: &'static str },
	/// A decompiled-C line revealed by an open accordion entry.
	SourceLine { source_idx: usize, is_last: bool },
}

// -- Top-level view -----------------------------------------------------------

struct KaisekiApp {
	main_panel: Panel,              // KSL lifted pseudocode (primary view)
	source_panel: Panel,            // decompiled C (accordion snippets)
	source_map: Vec<lang::SourceSpan>,
	expanded_spans: HashSet<usize>, // indices into source_map
	focus_handle: FocusHandle,
}

impl KaisekiApp {
	fn new(cx: &mut Context<Self>) -> Self {
		Self {
			main_panel: Panel::from_code(lang::SAMPLE_KSL, "rs"),
			source_panel: Panel::from_code(SAMPLE_CODE, "cpp"),
			source_map: lang::SAMPLE_SOURCE_MAP
				.iter()
				.map(|s| lang::SourceSpan {
					label: s.label,
					ksl_trigger_line: s.ksl_trigger_line,
					source_lines: s.source_lines.clone(),
				})
				.collect(),
			expanded_spans: HashSet::new(),
			focus_handle: cx.focus_handle(),
		}
	}

	/// Build the flat list of display rows from the current KSL lines and
	/// expansion state.  Expanded spans insert source-snippet rows directly
	/// after the trigger KSL line.
	fn build_display_rows(&self) -> Vec<DisplayRow> {
		let ksl_count = self.main_panel.lines.len();
		let mut rows = Vec::with_capacity(ksl_count);

		for ksl_idx in 0..ksl_count {
			let span_idx = self
				.source_map
				.iter()
				.position(|s| s.ksl_trigger_line == ksl_idx);

			let (is_expanded, label) = match span_idx {
				Some(si) => (self.expanded_spans.contains(&si), self.source_map[si].label),
				None => (false, ""),
			};

			rows.push(DisplayRow::KslLine { ksl_idx, span_idx, is_expanded, label });

			if let Some(si) = span_idx {
				if self.expanded_spans.contains(&si) {
					let range = self.source_map[si].source_lines.clone();
					let last = range.end.saturating_sub(1);
					for source_idx in range {
						rows.push(DisplayRow::SourceLine { source_idx, is_last: source_idx == last });
					}
				}
			}
		}

		rows
	}
}

// -- Rendering ----------------------------------------------------------------

impl Render for KaisekiApp {
	fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		advance_smooth_scroll(&mut self.main_panel, window);

		let display_rows = Arc::new(self.build_display_rows());
		let row_count = display_rows.len();

		let main_lines  = self.main_panel.lines.clone();
		let source_lines = self.source_panel.lines.clone();
		let main_scroll = self.main_panel.scroll_handle.clone();
		let main_gutter = self.main_panel.lines.len().to_string().len();
		let source_gutter = self.source_panel.lines.len().to_string().len();

		// One toggle listener per source-map span, built before the element tree.
		// Using MouseDownEvent instead of ClickEvent — on_mouse_down is on InteractiveElement
		// and works on plain Div without needing .id() / StatefulInteractiveElement.
		let mut toggle_fns: Vec<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>> =
			Vec::new();
		for sidx in 0..self.source_map.len() {
			let f = cx.listener(move |this, _: &MouseDownEvent, _window, cx| {
				if this.expanded_spans.contains(&sidx) {
					this.expanded_spans.remove(&sidx);
				} else {
					this.expanded_spans.insert(sidx);
				}
				cx.notify();
			});
			toggle_fns.push(Box::new(f));
		}
		let toggle_fns = Arc::new(toggle_fns);

		let scroll_listener = cx.listener(|this, event: &ScrollWheelEvent, window, cx| {
			if handle_panel_scroll(&mut this.main_panel, event, window) {
				cx.notify();
			}
		});

		div()
			.size_full()
			.flex()
			.flex_col()
			.bg(rgb(0x1e1e2e))
			.key_context("KaisekiApp")
			.track_focus(&self.focus_handle)
			// -- App title bar ------------------------------------------------
			.child(
				div()
					.h(px(40.))
					.flex()
					.items_center()
					.px(px(16.))
					.bg(rgb(0x181825))
					.text_size(px(13.))
					.text_color(rgba(0xa6adc8ff))
					.font_family("JetBrains Mono")
					.child("kaiseki"),
			)
			// -- KSL panel with inline accordion ------------------------------
			.child(
				div()
					.flex_1()
					.flex()
					.flex_col()
					.overflow_hidden()
					.child(panel_header("lifted.ksl — get_stream_fpv", "KSL", rgba(0x89b4fa55)))
					.child(
						div()
							.flex_1()
							.overflow_hidden()
							.on_scroll_wheel(scroll_listener)
							.child(
								uniform_list(
									"ksl-rows",
									row_count,
									move |range, _window, _cx| {
										range
											.map(|i| match &display_rows[i] {
												DisplayRow::KslLine {
													ksl_idx,
													span_idx,
													is_expanded,
													label,
												} => {
													let line = &main_lines[*ksl_idx];
													let num = format!(
														"{:>width$}",
														ksl_idx + 1,
														width = main_gutter
													);
													match span_idx {
														Some(sidx) => {
															let sidx = *sidx;
															let handler =
																Arc::clone(&toggle_fns);
																					render_accordion_row(
																num,
																line.text.clone(),
																line.highlights.clone(),
																label,
																*is_expanded,
																move |e, w, cx| {
																	(handler[sidx])(e, w, cx)
																},
															)
														}
														None => render_code_row(
															num,
															line.text.clone(),
															line.highlights.clone(),
														),
													}
												}
												DisplayRow::SourceLine {
													source_idx,
													is_last,
												} => {
													let line = &source_lines[*source_idx];
													let num = format!(
														"{:>width$}",
														source_idx + 1,
														width = source_gutter
													);
													render_source_snippet_row(
														num,
														line.text.clone(),
														line.highlights.clone(),
														*is_last,
													)
												}
											})
											.collect::<Vec<_>>()
									},
								)
								.size_full()
								.py(px(8.))
								.track_scroll(main_scroll),
							),
					),
			)
	}
}

/// Render a thin sub-title bar with a language badge.
fn panel_header(label: &str, badge: &str, badge_bg: impl Into<Hsla>) -> impl IntoElement {
	let badge_bg: Hsla = badge_bg.into();
	div()
		.h(px(28.))
		.flex()
		.items_center()
		.px(px(16.))
		.bg(rgb(0x181825))
		.border_b_1()
		.border_color(rgb(0x313244))
		.text_color(rgba(0x6c7086ff)) // Subtext0
		.font_family("JetBrains Mono")
		.text_size(px(11.))
		.child(label.to_string())
		.child(
			div()
				.ml(px(8.))
				.px(px(5.))
				.py(px(1.))
				.rounded(px(3.))
				.bg(badge_bg)
				.text_color(rgba(0xcdd6f4ff)) // Text
				.text_size(px(9.))
				.font_weight(FontWeight::BOLD)
				.child(badge.to_string()),
		)
}

/// A KSL code row that carries an accordion toggle button on the right.
/// `label` is the section name; `is_expanded` controls the chevron direction.
fn render_accordion_row(
	num: String,
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
	label: &str,
	is_expanded: bool,
	on_toggle: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> Div {
	let chevron = if is_expanded { "▼" } else { "▶" };
	div()
		.h(px(22.))
		.flex()
		.flex_row()
		.items_center()
		.font_family("Consolas")
		.text_size(px(13.))
		// Gutter
		.child(
			div()
				.w(px(56.))
				.h_full()
				.flex()
				.items_center()
				.justify_end()
				.pr(px(16.))
				.text_size(px(12.))
				.text_color(rgba(0x585b70ff))
				.flex_shrink_0()
				.child(num),
		)
		// Syntax-highlighted text
		.child(div().flex_1().child(StyledText::new(text).with_highlights(highlights)))
		// Toggle button — on_mouse_down works on plain Div (InteractiveElement), no .id() needed
		.child(
			div()
				.flex_shrink_0()
				.flex()
				.items_center()
				.gap(px(4.))
				.px(px(8.))
				.mr(px(8.))
				.h(px(16.))
				.rounded(px(3.))
				.bg(rgba(0x31324488))
				.text_color(rgba(0xe68945cc)) // C orange tint
				.text_size(px(10.))
				.font_family("JetBrains Mono")
				.cursor_pointer()
				.on_mouse_down(MouseButton::Left, on_toggle)
				.child(format!("{chevron} C  {label}")),
		)
}

/// A decompiled-C snippet row shown inside an accordion expansion.
/// Visually distinct via a left border and slightly darker background.
fn render_source_snippet_row(
	num: String,
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
	is_last: bool,
) -> Div {
	div()
		.h(px(22.))
		.flex()
		.flex_row()
		.items_center()
		.font_family("Consolas")
		.text_size(px(13.))
		.bg(rgba(0x181825dd))
		.border_l(px(2.))
		.border_color(rgba(0xe6894566)) // C orange – same family as toggle button
		.when(is_last, |d| d.pb(px(4.)))
		// Gutter
		.child(
			div()
				.w(px(56.))
				.h_full()
				.flex()
				.items_center()
				.justify_end()
				.pr(px(16.))
				.text_size(px(12.))
				.text_color(rgba(0x585b7044)) // dimmer than main gutter
				.flex_shrink_0()
				.child(num),
		)
		.child(StyledText::new(text).with_highlights(highlights))
}

// -- Entry point --------------------------------------------------------------

fn main() {
	Application::new().run(|app| {
		let options = WindowOptions {
			titlebar: Some(TitlebarOptions {
				title: Some("Kaiseki".into()),
				..Default::default()
			}),
			..Default::default()
		};

		app.open_window(options, |window, cx| {
				let entity = cx.new(KaisekiApp::new);
				// Give the root view keyboard focus so Tab events are captured.
				let focus_handle = entity.read(cx).focus_handle.clone();
				window.focus(&focus_handle);
				entity
			})
			.expect("Failed to open window");
		app.activate(true);
	});
}
