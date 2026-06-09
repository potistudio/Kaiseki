use iced::Color;

use crate::types::VarRole;

// ── Layout constants ──────────────────────────────────────────────────────────

pub const ROW_H: f32 = 22.0;
pub const CHAR_W: f32 = 7.8; // approx. Consolas 13 px glyph advance
pub const FONT_SIZE: f32 = 13.0;
pub const TOP_PAD: f32 = 8.0;

// Left-margin layout
//
//   ┌─ CONN_GUTTER_W ─┬─── GUTTER_W ───┬─── code ──────────
//   │  rail / dots    │  line numbers  │
//
pub const CONN_GUTTER_W: f32 = 28.0; // connection gutter (left of line numbers)
pub const GUTTER_W: f32 = 56.0;      // line-number gutter
pub const CODE_X: f32 = CONN_GUTTER_W + GUTTER_W; // where code text starts

// Connection gutter geometry
pub const DOT_R: f32 = 3.5;
pub const RAIL_W: f32 = 2.0;

// Sidebar width
pub const SIDEBAR_W: f32 = 220.0;

// ── Color palette (Catppuccin Mocha) ─────────────────────────────────────────

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
	Color {
		r: r as f32 / 255.0,
		g: g as f32 / 255.0,
		b: b as f32 / 255.0,
		a: 1.0,
	}
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
	Color {
		r: r as f32 / 255.0,
		g: g as f32 / 255.0,
		b: b as f32 / 255.0,
		a: a as f32 / 255.0,
	}
}

pub const BASE: Color    = rgb(0x1e, 0x1e, 0x2e);
pub const MANTLE: Color  = rgb(0x18, 0x18, 0x25);
pub const SURFACE0: Color = rgb(0x31, 0x32, 0x44);
pub const OVERLAY0: Color = rgb(0x6c, 0x70, 0x86);
pub const SUBTEXT1: Color = rgb(0xba, 0xc2, 0xde);
pub const SUBTEXT0: Color = rgb(0xa6, 0xad, 0xc8);
pub const TEXT_COL: Color = rgb(0xcd, 0xd6, 0xf4);
pub const BLUE: Color    = rgb(0x89, 0xb4, 0xfa);
pub const SKY: Color     = rgb(0x89, 0xdc, 0xeb);
pub const GREEN: Color   = rgb(0xa6, 0xe3, 0xa1);
pub const YELLOW: Color  = rgb(0xf9, 0xe2, 0xaf);
pub const PEACH: Color   = rgb(0xfa, 0xb3, 0x87);
pub const MAUVE: Color   = rgb(0xcb, 0xa6, 0xf7);

pub const GUTTER_FG: Color     = rgba(0x58, 0x5b, 0x70, 0xff);
pub const GUTTER_DIM: Color    = rgba(0x58, 0x5b, 0x70, 0x44);
pub const SNIPPET_BG: Color    = rgba(0x18, 0x18, 0x25, 0xdd);
pub const ACCORDION_FG: Color  = rgba(0xe6, 0x89, 0x45, 0xcc);
pub const ACCORDION_BG: Color  = rgba(0x31, 0x32, 0x44, 0x88);
pub const SNIPPET_BORDER: Color = rgba(0xe6, 0x89, 0x45, 0x66);
pub const RAIL_COL: Color      = rgba(0x58, 0x5b, 0x70, 0x88);
pub const SAPPHIRE_DIM: Color  = rgba(0x74, 0xc7, 0xec, 0x99);

pub const fn var_bg(role: VarRole) -> Color {
	match role {
		VarRole::Definition => rgba(0xb4, 0xbe, 0xfe, 0x55),
		VarRole::Write      => rgba(0xf9, 0xe2, 0xaf, 0x55),
		VarRole::Read       => rgba(0x94, 0xe2, 0xd5, 0x55),
		VarRole::TypeRef    => rgba(0x6c, 0x70, 0x86, 0x33),
	}
}

pub const fn var_dot(role: VarRole) -> Color {
	match role {
		VarRole::Definition => GREEN,
		VarRole::Read       => BLUE,
		VarRole::Write      => YELLOW,
		VarRole::TypeRef    => OVERLAY0,
	}
}
