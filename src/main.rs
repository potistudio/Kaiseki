use gpui::*;
use std::{ops::Range, sync::Arc};
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

// ── Data model ──────────────────────────────────────────────────────────────

struct HighlightedLine {
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
}

struct SmoothScroll {
	current_y: f32,
	target_y: f32,
	animating: bool,
}

struct CodeViewer {
	lines: Arc<Vec<HighlightedLine>>,
	scroll_handle: UniformListScrollHandle,
	smooth: SmoothScroll,
}

// ── Color helpers ────────────────────────────────────────────────────────────

fn syntect_to_hsla(c: syntect::highlighting::Color) -> Hsla {
	Rgba {
		r: c.r as f32 / 255.0,
		g: c.g as f32 / 255.0,
		b: c.b as f32 / 255.0,
		a: c.a as f32 / 255.0,
	}
	.into()
}

// ── Business logic ───────────────────────────────────────────────────────────

impl CodeViewer {
	fn new(_cx: &mut Context<Self>) -> Self {
		let ss = SyntaxSet::load_defaults_newlines();
		let ts = ThemeSet::load_defaults();
		let theme = &ts.themes["base16-ocean.dark"];
		let syntax = ss
			.find_syntax_by_extension("cpp")
			.unwrap_or_else(|| ss.find_syntax_plain_text());
		let mut hl = HighlightLines::new(syntax, theme);

		let lines: Vec<HighlightedLine> = LinesWithEndings::from(SAMPLE_CODE)
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

				HighlightedLine {
					text: text.into(),
					highlights,
				}
			})
			.collect();

		Self {
			lines: Arc::new(lines),
			scroll_handle: UniformListScrollHandle::new(),
			smooth: SmoothScroll {
				current_y: 0.0,
				target_y: 0.0,
				animating: false,
			},
		}
	}
}

// ── Rendering ────────────────────────────────────────────────────────────────

impl Render for CodeViewer {
	fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		// ── Advance smooth-scroll animation ───────────────────────────────
		if self.smooth.animating {
			let diff = self.smooth.target_y - self.smooth.current_y;
			if diff.abs() < 0.5 {
				self.smooth.current_y = self.smooth.target_y;
				self.smooth.animating = false;
			} else {
				self.smooth.current_y += diff * 0.18;
				window.request_animation_frame();
			}
			let base = self.scroll_handle.0.borrow().base_handle.clone();
			base.set_offset(point(px(0.0), px(self.smooth.current_y)));
		}

		let lines = self.lines.clone(); // Arc clone, O(1)
		let count = lines.len();
		let gutter_width = count.to_string().len();

		div()
			.size_full()
			.flex()
			.flex_col()
			.bg(rgb(0x1e1e2e)) // Catppuccin Base
			// ── Title bar ─────────────────────────────────────────────────
			.child(
				div()
					.h(px(40.))
					.flex()
					.items_center()
					.px(px(16.))
					.bg(rgb(0x181825)) // Catppuccin Mantle
					.text_size(px(13.))
					.text_color(rgba(0xa6adc8ff))
					.font_family("Consolas")
					.child("kaiseki — main.rs"),
			)
			// ── Code area ─────────────────────────────────────────────────
			.child(
				div()
					.flex_1()
					.overflow_hidden()
					.on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, window, cx| {
						let actual_y = this.scroll_handle.0.borrow().base_handle.offset().y.as_f32();

						if event.delta.precise() {
							// Touchpad: stay in sync, let GPUI handle natively
							this.smooth.current_y = actual_y;
							this.smooth.target_y = actual_y;
							return;
						}

						// Mouse wheel: undo instant scroll, drive smooth animation
						let line_height = window.line_height();
						let delta_y = event.delta.pixel_delta(line_height).y.as_f32();

						// Undo what uniform_list's scroll handler already applied
						{
							let state = this.scroll_handle.0.borrow();
							state.base_handle.set_offset(point(px(0.0), px(this.smooth.current_y)));
						}

						// Accumulate toward target and clamp
						this.smooth.target_y += delta_y;
						if let Some(size) = this.scroll_handle.0.borrow().last_item_size {
							let max_neg = -(size.contents.height.as_f32() - size.item.height.as_f32()).max(0.0);
							this.smooth.target_y = this.smooth.target_y.max(max_neg).min(0.0);
						}

						this.smooth.animating = true;
						cx.notify();
					}))
					.child(
						uniform_list("code-lines", count, move |range, _window, _cx| {
							range
								.map(|i| {
									let line = &lines[i];
									let num = format!("{:>width$}", i + 1, width = gutter_width);

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
										// Code text with highlights
										.child(
											StyledText::new(line.text.clone()).with_highlights(line.highlights.clone()),
										)
								})
								.collect::<Vec<_>>()
						})
						.size_full()
						.py(px(8.))
						.track_scroll(&self.scroll_handle),
					),
			)
	}
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
	gpui_platform::application().run(|cx: &mut App| {
		let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
		cx.open_window(
			WindowOptions {
				window_bounds: Some(WindowBounds::Windowed(bounds)),
				titlebar: Some(TitlebarOptions {
					title: Some("Kaiseki — Code Viewer".into()),
					..Default::default()
				}),
				..Default::default()
			},
			|_window, cx| cx.new(|cx| CodeViewer::new(cx)),
		)
		.unwrap();
		cx.activate(true);
	});
}
