use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
	ToggleSpan(usize),
	/// `Some((name, ksl_line_idx))` — activate; `None` — clear.
	/// The line index is needed to resolve which declaration scope to use when
	/// the same name is bound multiple times (e.g. shadowing across if/else branches).
	SetActiveVariable(Option<(String, usize)>),
	/// Double-click on a function-call identifier: scroll the view to its `fn` definition.
	JumpToDefinition(String),
	/// User clicked a .kvl entry in the sidebar.
	SelectKvl(PathBuf),
	/// User clicked a directory header in the sidebar.
	ToggleDir(PathBuf),
	ToggleSidebar,
}
