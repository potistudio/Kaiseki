use std::path::Path;

use crate::types::KvlEntry;

/// Recursively scan a .kvp directory and return a KvlEntry tree.
pub fn scan_kvp(dir: &Path) -> Vec<KvlEntry> {
	let Ok(read_dir) = std::fs::read_dir(dir) else { return Vec::new() };
	let mut raw: Vec<_> = read_dir.filter_map(|e| e.ok()).collect();
	raw.sort_by_key(|e| e.file_name());
	raw.into_iter()
		.filter_map(|entry| {
			let path = entry.path();
			let file_name = entry.file_name().to_string_lossy().to_string();
			if path.is_dir() {
				let children = scan_kvp(&path);
				// Omit empty directories.
				(!children.is_empty()).then_some(KvlEntry::Dir { name: file_name, path: path.clone(), children })
			} else if path.extension().map_or(false, |ext| ext == "kvl") {
				let display_name = path
					.file_stem()
					.map(|s| s.to_string_lossy().to_string())
					.unwrap_or(file_name);
				Some(KvlEntry::File { display_name, path })
			} else {
				None
			}
		})
		.collect()
}

/// Extract the KSL text from the "--- view ---" section of a .kvl file.
///
/// .kvl format:
///   --- source ---
///   <decompiled C>
///   --- view ---
///   <KSL>
pub fn extract_kvl_view(content: &str) -> &str {
	const MARKER: &str = "--- view ---";
	content
		.find(MARKER)
		.map(|pos| content[pos + MARKER.len()..].trim_start_matches('\n'))
		.unwrap_or(content)
}

/// Collect all Dir paths in a KvlEntry tree (for default-expanded initialization).
pub fn collect_dir_paths(entries: &[KvlEntry], out: &mut std::collections::HashSet<std::path::PathBuf>) {
	for entry in entries {
		if let KvlEntry::Dir { path, children, .. } = entry {
			out.insert(path.clone());
			collect_dir_paths(children, out);
		}
	}
}
