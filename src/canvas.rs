use std::sync::Arc;

use iced::{Color, Font, Pixels, Point, Rectangle, Size, alignment, mouse, widget::canvas};

use crate::lang::SourceSpan;
use crate::lang::lexer::TokenKind;
use crate::message::Message;
use crate::theme::{
	ACCORDION_BG, ACCORDION_FG, BASE, CHAR_W, CODE_X, CONN_GUTTER_W, DOT_R, FONT_SIZE, GUTTER_DIM, GUTTER_FG, RAIL_COL,
	RAIL_W, ROW_H, SNIPPET_BG, SNIPPET_BORDER, TEXT_COL, TOP_PAD, var_bg, var_dot,
};
use crate::types::{DisplayRow, HighlightedLine, VarOccurrence, VarRole};

/// Full-panel canvas: renders all display rows and handles mouse events.
/// Wrapped in `scrollable` — canvas height equals total content height.
pub struct CodeCanvas {
	pub display_rows: Arc<Vec<DisplayRow>>,
	pub ksl_lines: Arc<Vec<HighlightedLine>>,
	pub source_lines: Arc<Vec<HighlightedLine>>,
	pub var_occurrences: Arc<Vec<VarOccurrence>>,
	pub source_map: Arc<Vec<SourceSpan>>,
	pub active_variable: Option<String>,
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
					for token in crate::lang::lexer::tokenize(&line.text) {
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
pub fn accordion_btn_width(label: &str) -> f32 {
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
