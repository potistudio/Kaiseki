# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Development

### Build

```bash
cargo build
```

### Git

一つのコンテキスト（機能を追加した、不具合を修正した、仕様を変更した等）ごとに、細かくコミットしてください。
コミットメッセージは簡潔にし、変更理由があれば、コミット詳細に書いてください。
また、コミットと同時に、`./docs/changes.md`に、より詳細な変更履歴を追記していってください。

Commit Prefixes

- "feat: ": 新機能の追加
- "fix: ": 不具合修正
- "refactor: ": リファクタリング
- "docs: ": ドキュメントの変更
- "chore: ": コードベースに無関係な変更

### Reference

#### gpui

the source code is here: `./refs/zed/crates/gpui/`

#### Documents

Use `cargo doc` to refer crate api documents.

If you have new findings, document them into `./docs`.

### Coding Style

Use TAB to indent instead of SPACE.

Leave implementation intentions by writing as many comments as possible.
You can use any ASCII visual (such as table or chart).
Don't write obvious context that can be understand from the code in comments. (e.g. "This struct implements `~~~`")

Don't abbreviate variable or function names. Use specific names unless it becomes extremely verbose.

### Rust idiom

Use `inspect_err` instead of `map_err`.

## Architecture
