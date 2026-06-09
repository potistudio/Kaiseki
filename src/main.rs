mod analysis;
mod app;
mod canvas;
mod file_tree;
mod highlight;
mod lang;
mod message;
mod theme;
mod types;

use std::path::PathBuf;

use app::KaisekiState;

fn main() -> iced::Result {
	// .kvp path: CLI arg > dev default > None (sample mode).
	let kvp_path: Option<PathBuf> = std::env::args()
		.nth(1)
		.map(Into::into)
		.or_else(|| {
			// During development, fall back to the bundled example project.
			let default = PathBuf::from("./examples/after-effects.kvp");
			default.exists().then_some(default)
		});

	iced::application("kaiseki", KaisekiState::update, KaisekiState::view)
		.window(iced::window::Settings {
			size: iced::Size::new(1400.0, 900.0),
			min_size: Some(iced::Size::new(800.0, 600.0)),
			..Default::default()
		})
		.run_with(move || KaisekiState::new(kvp_path.clone()))
}
