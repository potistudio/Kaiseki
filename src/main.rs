use gpui::*;
use std::{ops::Range, sync::Arc};
use syntect::{
	easy::HighlightLines,
	highlighting::{FontStyle as SyntectFontStyle, ThemeSet},
	parsing::SyntaxSet,
	util::LinesWithEndings,
};

const SAMPLE_CODE: &str = r#"// kaiseki — syntax-highlighted code viewer
use std::collections::HashMap;
use std::fmt;

/// A generic key-value store backed by a hash map.
#[derive(Debug, Default)]
struct Store<K, V> {
    data: HashMap<K, V>,
    name: String,
}

impl<K: std::hash::Hash + Eq, V> Store<K, V> {
    fn new(name: impl Into<String>) -> Self {
        Store {
            data: HashMap::new(),
            name: name.into(),
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<K: fmt::Display + Ord, V: fmt::Display> Store<K, V> {
    fn display_sorted(&self) {
        println!("Store '{}' ({} entries):", self.name, self.len());
        let mut pairs: Vec<_> = self.data.iter().collect();
        pairs.sort_by_key(|(k, _)| k.to_string());
        for (k, v) in pairs {
            println!("  {k} => {v}");
        }
    }
}

/// Compute the n-th Fibonacci number recursively.
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// A trait for objects that can describe themselves.
trait Describable {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Operator(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::Identifier(s) => write!(f, "{s}"),
            Token::Operator(op) => write!(f, "'{op}'"),
        }
    }
}

impl Describable for Token {
    fn describe(&self) -> String {
        match self {
            Token::Number(n) => format!("number literal {n}"),
            Token::Identifier(s) => format!("identifier `{s}`"),
            Token::Operator(op) => format!("operator `{op}`"),
        }
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = num.parse::<f64>() {
                    tokens.push(Token::Number(n));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            op => {
                tokens.push(Token::Operator(op));
                chars.next();
            }
        }
    }

    tokens
}

fn main() {
    let mut fib_store: Store<String, u64> = Store::new("fibonacci");
    for i in 0..10 {
        fib_store.insert(format!("fib_{i:02}"), fibonacci(i));
    }
    fib_store.display_sorted();

    let expr = "x + 3.14 * foo_bar - 2";
    println!("\nTokenizing: \"{expr}\"");
    for tok in tokenize(expr) {
        println!("  {tok:>20}  →  {}", tok.describe());
    }

    let values: Vec<i32> = (1..=20)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();

    println!("\nEven squares:");
    for chunk in values.chunks(4) {
        let row: Vec<_> = chunk.iter().map(|v| format!("{v:5}")).collect();
        println!("  {}", row.join(" "));
    }
}
"#;

// ── Data model ──────────────────────────────────────────────────────────────

struct HighlightedLine {
	text: SharedString,
	highlights: Vec<(Range<usize>, HighlightStyle)>,
}

struct CodeViewer {
	lines: Arc<Vec<HighlightedLine>>,
	scroll_handle: UniformListScrollHandle,
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
			.find_syntax_by_extension("rs")
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
		}
	}
}

// ── Rendering ────────────────────────────────────────────────────────────────

impl Render for CodeViewer {
	fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
				div().flex_1().overflow_hidden().child(
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
									.child(StyledText::new(line.text.clone()).with_highlights(line.highlights.clone()))
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
