use std::{
	collections::HashSet,
	path::PathBuf,
	sync::Arc,
};

use iced::{
	Background, Border, Color, Element, Font, Length, Task,
	alignment, widget::{button, canvas, column, container, row, scrollable},
};

use crate::analysis::{find_fn_definition, find_var_occurrences};
use crate::canvas::CodeCanvas;
use crate::file_tree::{collect_dir_paths, extract_kvl_view, scan_kvp};
use crate::highlight::{build_ksl_lines, build_source_lines};
use crate::lang::{SAMPLE_KSL, SAMPLE_SOURCE_MAP, SourceSpan};
use crate::message::Message;
use crate::theme::{
	MANTLE, OVERLAY0, SIDEBAR_W, SUBTEXT0, SURFACE0, TEXT_COL, TOP_PAD, ROW_H,
	rgba,
};
use crate::types::{DisplayRow, HighlightedLine, KvlEntry, VarOccurrence};

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

// ── Application state ─────────────────────────────────────────────────────────

pub struct KaisekiState {
	ksl_lines:       Arc<Vec<HighlightedLine>>,
	source_lines:    Arc<Vec<HighlightedLine>>,
	source_map:      Arc<Vec<SourceSpan>>,
	expanded_spans:  HashSet<usize>,
	active_variable: Option<String>,
	var_occurrences: Arc<Vec<VarOccurrence>>,
	// Pre-computed brace depths for scope-aware occurrence filtering.
	depth_map:       crate::lang::scope::DepthMap,
	// ── File tree (populated when a .kvp directory is supplied) ──────────────
	kvl_tree:        Vec<KvlEntry>,
	selected_kvl:    Option<PathBuf>,
	/// Display name shown in the panel header (file stem, or "sample").
	selected_name:   String,
	/// Directories currently expanded in the sidebar tree.
	expanded_dirs:   HashSet<PathBuf>,
	sidebar_visible: bool,
}

impl KaisekiState {
	pub fn new(kvp_path: Option<PathBuf>) -> (Self, Task<Message>) {
		let kvl_tree = kvp_path.as_deref().map(scan_kvp).unwrap_or_default();

		let ksl_lines = build_ksl_lines(SAMPLE_KSL);
		let depth_map = {
			let raw: Vec<&str> = ksl_lines.iter().map(|l| l.text.as_str()).collect();
			crate::lang::scope::DepthMap::build(&raw)
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

	pub fn update(&mut self, message: Message) -> Task<Message> {
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
						let scoped_occurrences = match crate::lang::scope::visible_scope(
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
					crate::lang::scope::DepthMap::build(&raw)
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

	pub fn view(&self) -> Element<'_, Message> {
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
	pub fn build_display_rows(&self) -> Vec<DisplayRow> {
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

// ── Stable scrollable ID ──────────────────────────────────────────────────────

fn code_scrollable_id() -> scrollable::Id {
	scrollable::Id::new("kaiseki-code")
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

// ── Sidebar file-tree widget ──────────────────────────────────────────────────

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
