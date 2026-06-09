# Changes

## refactor: モジュール分割

1451行の `src/main.rs` を責務別に8モジュールへ分割した。

| ファイル | 内容 |
| --- | --- |
| `src/main.rs` | エントリーポイント (`main()`) のみ |
| `src/theme.rs` | カラーパレット定数・レイアウト定数・`var_bg`/`var_dot` |
| `src/types.rs` | `HighlightedLine`, `VarRole`, `VarOccurrence`, `DisplayRow`, `KvlEntry` |
| `src/message.rs` | `Message` enum |
| `src/file_tree.rs` | `scan_kvp`, `extract_kvl_view`, `collect_dir_paths` |
| `src/highlight.rs` | `build_ksl_lines`, `build_source_lines`, `token_color` |
| `src/analysis.rs` | `find_var_occurrences`, `find_fn_definition`, `is_type_position`, `is_fn_call` |
| `src/canvas.rs` | `CodeCanvas` + `canvas::Program` impl + `draw_*` ヘルパー群 |
| `src/app.rs` | `KaisekiState` + impl, `panel_header`, `render_kvl_tree`, `SAMPLE_CODE` |

## feat: サイドバー・kvpファイルツリー・kvlファイル読み込み

.kvp ディレクトリ（サンプルコードのパッケージ形式）をサイドバーのファイルツリーとして表示し、
.kvl ファイルをクリックするとコードパネルに読み込めるようにした。

### サイドバー

- `SIDEBAR_W = 220.0` 定数追加
- `KvlEntry` enum: `.kvp` 内のファイル/ディレクトリ構造を表す
- `scan_kvp(dir)`: ディレクトリを再帰スキャンして `KvlEntry` ツリーを構築
- `render_kvl_tree(...)`: ツリーをサイドバー行ウィジェット列に変換（インデント・選択状態対応）
- `collect_dir_paths(...)`: 初期展開状態のためディレクトリパスを収集

### kvlファイル形式

- `extract_kvl_view(content)`: `--- view ---` セクション以降のKSLテキストを抽出

### アプリ状態 (`KaisekiState`)

- `kvl_tree`, `selected_kvl`, `selected_name`, `expanded_dirs`, `sidebar_visible` フィールドを追加
- `Message::SelectKvl`, `ToggleDir`, `ToggleSidebar`, `JumpToDefinition` を追加
- `view()`: タイトルバー（サイドバートグルボタン付き）＋サイドバー＋コードパネルの3カラムレイアウトに変更
- `view_sidebar()`: サイドバーウィジェットの実装
- `view_code_panel()`: コードパネルをメソッドとして分離
- `jump_to_definition()`: `fn <name>` 定義行へスクロール
- CLIまたはデフォルトの `./examples/after-effects.kvp` から .kvp パスを読み込むよう `main()` を変更

### パネルヘッダー

- `panel_header(file_name)`: 表示中のファイル名を引数で受け取るよう変更

## feat: スコープ解析によるハイライト範囲の絞り込み

### 概要

変数名をクリックしたとき、ファイル全体でなく **クリック箇所が属するスコープ内の出現のみ** をハイライトするよう変更。

同一ブランチ内で同名変数が宣言される KSL パターン（例: if/else の各枝で `let ct = ...`）で、
別枝の `ct` が誤ってハイライトされていた問題を修正。

### 実装

**`src/lang/scope.rs` (新規)**

- `DepthMap` — KSL 全行の波括弧深度を事前計算。
  - `depth_start[i]`: 行 i の先頭時点の深度
  - `depth_min[i]`: 行 i 内で到達した最小深度（`} else {` のような行で途中で深度が落ちる場合に対応）
- `find_decl_sites(name)` — `let name` / `var name` / `fn name` / `name:` パターンで宣言行を列挙
- `visible_scope(name, at_line)` — クリック箇所を含む最も内側の宣言スコープ `[decl_line, scope_end)` を返す

**`src/main.rs`**

- `KaisekiState` に `depth_map: lang::scope::DepthMap` フィールドを追加
- `Message::SetActiveVariable` を `Option<(String, usize)>` に変更（名前 + クリックした KSL 行番号）
- `update()` — `visible_scope` で取得した行範囲で `find_var_occurrences` の結果をフィルタ

### スコープ境界の扱い

| 宣言パターン | depth | scope_end |
| --- | --- | --- |
| `let x` / `var x` (関数本体直下) | 1 | 関数閉じ `}` の行 |
| `let ct` (if ブランチ内) | 2 | そのブランチの `}` (または `} else {` の行) |
| 関数引数 `param:` | 0 | ファイル末尾 (depth が負になることはないため) |

宣言が複数ある場合は **最も深い（最も内側の）宣言**を採用し、シャドーイングを正確に処理。

## feat: 接続ガター左マージン移動・役割矢印・型参照除外

### 接続ガター — 右端 → 行番号左の専用カラムへ移動

レイアウトを 3 カラム構成に変更。

```
┌─ CONN_GUTTER_W (28px) ─┬─ GUTTER_W (56px) ─┬─ code ──
│  rail / arrows         │  line numbers     │
```

- `CONN_GUTTER_W = 28.0`、`CODE_X = CONN_GUTTER_W + GUTTER_W` 定数を追加。
- `draw_gutter` / `draw_line` / クリック判定 / 背景ハイライトのすべての x 座標を `CODE_X` 基準に統一。
- `draw_connection_gutter` から `bounds` / `lines` 引数を削除（左ガター固定なので不要）。
- リーダー線・右端クランプを廃止。ティック末端は行番号の桁数から動的に計算。

### 役割ごとの矢印マーカー

| ロール | 形状 | 色 |
| ------ | ---- | -- |
| Definition | ↓ 縦三角（レール上） | GREEN `#a6e3a1` |
| Read | `──▶` ティック＋右向き三角 | BLUE `#89b4fa` |
| Write | `──◀` ティック＋左向き三角 | YELLOW `#f9e2af` |

- `BLUE = #89b4fa`（Catppuccin Blue）を追加。不要になった `LAVENDER`・`TEAL`・`ARROW_COL`・`LEADER_COL` を削除。

### VarRole::TypeRef — 型参照の除外

- `VarRole::TypeRef` バリアント追加。型名位置の識別子（`: Type`、`struct Name`、`-> ReturnType` 等）を検出。
- 型参照は背景ハイライトを薄いグレーで表示するが、接続ガター（レール・矢印）には描画しない。
- `is_type_position()` 関数: トークン列を後ろ向きスキャンして型修飾子（`*` `&` `?`）を読み飛ばし、`:` / `->（戻り値位置のみ）` / 型定義キーワードで判定。

### Definition 検出の拡張

`let`/`var` 直後のみだった定義検出を以下に拡張：

| パターン | 例 | 検出方法 |
| -------- | -- | -------- |
| `let x` / `var x` | `let stream = …` | 直前が `let`/`var` |
| `fn name(…)` | `fn get_stream_fpv(…)` | 直前が `fn` |
| `name: Type` | `layer: *BEE_Layer` | 直後が `:` |

## refactor: GPUI → Iced 全面再実装

UIフレームワークを gpui 0.2.2 から **iced 0.13** に切り替えて全面書き直し。
機能・ビジュアルはすべて保持。

### アーキテクチャ変更点

| 項目 | GPUI版 | Iced版 |
|------|--------|--------|
| 状態管理 | `KaisekiApp` + `cx.notify()` | `KaisekiState` + Elm MEV |
| コードパネル | `uniform_list` + `StyledText` | 1枚の `canvas::Program` |
| スクロール | `UniformListScrollHandle` + スムーズスクロール独自実装 | `scrollable` ウィジェット |
| 接続ガター | 別建て absolute overlay canvas | コードcanvasに統合 |
| クリック検出 | `on_mouse_down` + pixel→char変換 | `canvas::Program::update` |
| アコーディオン | `on_mouse_down` per-span クロージャ | canvas内 hit-test で Message 発行 |

### 主要実装

- `KaisekiState::new / update / view` — Iced functional API エントリポイント
- `CodeCanvas` — `canvas::Program<Message>` 実装。全行描画 + マウスイベント処理を1クラスに集約
- `draw_line` — KSL / C ソース両対応のシンタックスハイライト描画（highlight span間のギャップをデフォルト色で補完）
- `draw_connection_gutter` — 接続ガター（縦レール・矢印・リーダー線・役割色ドット）を同一canvasに描画
- `panel_header` — iced ウィジェットで構成したパネルヘッダー（KSL バッジ付き）
- `build_ksl_lines` / `build_source_lines` — 起動時に全行をハイライト済みに変換して `Arc` で共有

### Cargo.toml

```
- gpui = "0.2.2"
+ iced = { version = "0.13", features = ["canvas"] }
```

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
