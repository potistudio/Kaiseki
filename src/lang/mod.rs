pub mod lexer;

use std::ops::Range;

/// One entry in a source map written by the LLM alongside the KSL it generates.
/// Maps a KSL trigger line to a range of lines in the original decompiled-C source.
pub struct SourceSpan {
	/// Short label shown on the accordion toggle button.
	pub label: &'static str,
	/// 0-based KSL display-line index that carries the expand/collapse button.
	/// Typically the section-header comment line (// ── name ──).
	pub ksl_trigger_line: usize,
	/// 0-based range into the decompiled-C Panel line vector.
	pub source_lines: Range<usize>,
}

/// Hand-authored source map for the sample KSL / C pair.
/// In production the LLM writes this alongside the KSL it generates.
pub const SAMPLE_SOURCE_MAP: &[SourceSpan] = &[
	SourceSpan {
		label: "acquire stream",
		ksl_trigger_line: 52,
		source_lines: 33..54,
	},
	SourceSpan {
		label: "resolve time",
		ksl_trigger_line: 62,
		source_lines: 54..83,
	},
	SourceSpan {
		label: "key-frame data",
		ksl_trigger_line: 75,
		source_lines: 83..185,
	},
];

/// KSL (Kaiseki Script Language) — lifted rendering of NIM_GetStreamFPV.
pub const SAMPLE_KSL: &str = "// KSL — Kaiseki Script Language
// A high-level pseudocode notation for decompiled binary analysis.
//
// Syntax overview:
//   expr!                   if (res != 0) { _CxxThrowException(...) }
//   a ?? b                  null-coalesce: use b when a is null
//   let x = e else return   guard: unwrap or bail with early return
//   T?                      nullable pointer / optional annotation
//   @0xNN  field: *T        struct field at fixed memory offset
//   @0xNN  fn m(self) -> T  virtual method at vtable offset
//   recv->method()          virtual / pointer-member dispatch
//   var x: T = default      mutable variable, zero-initialized
//
// Original: NIM_GetStreamFPV  addr: 0xe2b340
//   sym: ?NIM_GetStreamFPV@@YAXPEAVBEE_Layer@@AEBVTDB_StreamIDPath@@\\
//        HPEBUT_Time@@2PEAT_BEE_StreamFPV@@PEAUFEE_KfcInfo@@PEBVTDB_Stream@@@Z

// ── Type layout annotations ────────────────────────────────────────────────────────────────────

struct BEE_Layer {
\t@0x0290  item:  *BEE_Item,
}

virtual interface Stream {
\t@0x0170  fn is_parametric(self)                                         -> bool,
\t          fn has_keys(self)                                              -> bool,
\t          fn get_value(self, time: Time, raw: bool,
\t                       unused: *void, out: *KfcInfo, bag: *ParamBag?)   -> HResult,
\t          fn time_to_index(self, time: Time, out: *i32)                 -> HResult,
\t          fn get_key(self, index: i32, unused: *void,
\t                     prev_hold: *bool, next_hold: *bool)                -> HResult,
}

// ── Function ────────────────────────────────────────────────────────────────────────────

/// Get stream FPV and optionally populate key-frame interpolation info.
///
/// Time parameters:
///   - both null        →  use current playhead (comp-space → layer-space)
///   - comp_time only   →  derive layer_time via CompToLayerTime
///   - layer_time only  →  derive comp_time  via LayerToCompTime
fn get_stream_fpv(
\tlayer:       *BEE_Layer,
\tpath:        &StreamIDPath,
\tmode:        i32,
\tcomp_time:   *Time?,         // null = use current playhead
\tlayer_time:  *Time?,         // null = derived from comp_time
\tout_fpv:     *StreamFPV,
\tout_kfc:     *KfcInfo?,      // null = skip KFC computation
\tstream:      *Stream?,       // null = auto-lookup from layer + path
) throws {

\t// ── acquire stream ────────────────────────────────────────────────────────────────────
\tlet stream = stream ?? BEE_GetStream(layer, path)!

\tBEE_GetStreamFPVPlusWithStreamP(
\t\tlayer, path, stream, comp_time, layer_time, mode, out_fpv, null,
\t)!

\t// KFC output is optional — bail early when the caller does not need it
\tlet out_kfc = out_kfc else return

\t// ── resolve time coordinates ───────────────────────────────────────────────────────────────
\tlet (comp_t, layer_t): (Time, Time) =
\t\tif comp_time != null {
\t\t\tlet ct = *comp_time
\t\t\t(ct, BEE_CompToLayerTime(layer, ct)!)
\t\t} else if layer_time != null {
\t\t\tlet lt = *layer_time
\t\t\t(BEE_LayerToCompTime(layer, lt)!, lt)
\t\t} else {
\t\t\tlet ct = BEE_GetItemCurrentTime(layer->item, null)!
\t\t\t(ct, BEE_CompToLayerTime(layer, ct)!)
\t\t}

\t// ── key-frame data (parametric streams only) ──────────────────────────────────────────
\tvar kfc_value: KfcInfo = default

\tif stream->is_parametric() {
\t\tstream->get_value(layer_t, false, null, &kfc_value, null)!

\t\tif stream->has_keys() {
\t\t\t// Neighbor-key easing fields — null pointer means \"skip this component\"
\t\t\tvar ease_in:     *KfcInfo? = if *out_kfc          != null { out_kfc + 0x08 } else { null }
\t\t\tvar ease_out:    *KfcInfo? = if *(out_kfc + 0x10) != null { out_kfc + 0x10 } else { null }
\t\t\tvar tangent_in:  *KfcInfo? = if *(out_kfc + 0x18) != null { out_kfc + 0x18 } else { null }
\t\t\tvar tangent_out: *KfcInfo? = if *(out_kfc + 0x1c) != null { out_kfc + 0x1c } else { null }

\t\t\tif layer != null {
\t\t\t\tvar current_time: Time = default
\t\t\t\tBEE_GetItemCurrentTime(layer->item, &current_time)!
\t\t\t\t// FUN_180e2acf0 — compute tangent / easing data from adjacent keys
\t\t\t\tFUN_180e2acf0(
\t\t\t\t\t&current_time, layer, stream,
\t\t\t\t\tease_in, ease_out, tangent_in, tangent_out, ease_in,
\t\t\t\t)
\t\t\t}
\t\t}

\t\t// Resolve the key index for the layer-space time
\t\tif kfc_value == null {
\t\t\t// FUN_180e2afa0 — binary-search key index (no-keyframes fallback)
\t\t\tout_kfc->key_index = FUN_180e2afa0(layer, stream, &comp_t, 0, ...)
\t\t} else {
\t\t\tout_kfc->key_index = stream->time_to_index(layer_t)!
\t\t}

\t\t// Continuity flags: is this key on the boundary of a hold (step) segment?
\t\tif kfc_value == null {
\t\t\tout_kfc->is_hold = true
\t\t} else {
\t\t\tvar prev_hold: bool
\t\t\tvar next_hold: bool
\t\t\tstream->get_key(out_kfc->key_index, null, &prev_hold, &next_hold)!
\t\t\tout_kfc->is_prev_hold = prev_hold
\t\t\tout_kfc->is_next_hold = next_hold
\t\t}

\t} else {
\t\tout_kfc->is_hold = false
\t}

\tout_kfc->value = kfc_value
}
";
