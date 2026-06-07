# Changes

## update: Variable highlighting — click-activate + text-end leader lines

前バージョンからの差分。

### インタラクション変更

- ホバー起動 → **クリック起動** に変更。
  - `on_mouse_move` / `on_hover` を削除し、`on_mouse_down(MouseButton::Left, ...)` に置き換え。
  - 同じ変数を再クリックするとアクティブ解除（トグル）。
  - `.id()` が不要になりシンプル化。
  - フィールド名: `hovered_variable` → `active_variable`

### 接続ガターキャンバス刷新

旧: パネル右端固定幅 40px の小さなキャンバス。引き出し線はキャンバス左端から開始。  
新: パネル全幅の透過オーバーレイキャンバス。引き出し線を**各行のテキスト末尾から**開始。

- `connection_gutter_canvas` に `lines: Arc<Vec<HighlightedLine>>` 引数を追加。
- `Entry` struct: `{ y, role, text_len }` — テキスト文字数から `text_end_x` を計算。
  - `text_end_x = GUTTER_W + text_len * CHAR_W`
  - `leader_x0  = text_end_x + LEADER_GAP (6px)`
  - `leader_x1  = dot_x - DOT_R - LEADER_GAP`
  - `leader_x1 > leader_x0` のときのみ描画（極端に長い行対策）
- ドット列: パネル右端から 16px 固定（`DOT_FROM_RIGHT = 16.0`）
- キャンバス配置: `.absolute().left(px(0.)).right(px(0.)).top(px(0.)).h_full()`

## feat: Variable highlighting on hover

`src/main.rs` に以下を追加。

### 新しい型

- `VarRole` enum: 変数出現の役割（`Definition` / `Write` / `Read`）
- `VarOccurrence` struct: 出現位置（行インデックス・バイト範囲・役割）

### 解析ロジック

- `find_var_occurrences(name, lines)`: 全行を再トークン化して同名 `Ident` トークンを収集し、
  隣接トークンから役割を推定する。
  - 直前が `let` / `var` キーワード → `Definition`
  - 直後が `=` 演算子（`==` 除外）→ `Write`
  - それ以外 → `Read`
- `hover_highlights_for_line(occurrences, line_idx)`: 出現箇所を
  `HighlightStyle::background_color` に変換する。色はカテゴリーごと：
  - Definition: Lavender `#b4befe55`
  - Write:      Yellow   `#f9e2af55`
  - Read:       Teal     `#94e2d555`

### ホバー検出

- `KaisekiApp` に `hovered_variable: Option<String>` と `var_occurrences: Vec<VarOccurrence>` を追加。
- `var_at_position(position)`: ウィンドウ座標 → 表示行インデックス → バイト列の変換で
  ホバー中の `Ident` トークン名を返す。レイアウト定数（タイトルバー高・ガター幅・行高・文字幅）に依存。
- `on_mouse_move` + `on_hover` を KSL コンテンツ div に付与。変数が変わったときのみ
  `cx.notify()` を呼び再レンダーコストを最小化。

### 接続ガターキャンバス

- `connection_gutter_canvas(occurrences, scroll_y, display_rows)`:
  パネル右端に 12 px の絶対配置キャンバスを描画する。
  - 最初〜最後の出現を繋ぐ縦線（`#585b7088`）
  - 隣接出現ペア間の下向き三角矢印（`PathBuilder` 使用）
  - 各出現位置の役割色付き 6×6 px 角丸ドット
