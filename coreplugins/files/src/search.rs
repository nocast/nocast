use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};
use rayon::prelude::*;

/// Recursively get all files whose absolute path contains `query`.
/// Skips directories starting with '.'
/// Returns Vec of absolute path strings.
pub fn fast_search(root: &str, query: &str) -> Vec<String> {
    let root_path = PathBuf::from(root);

    // Filter function to skip hidden directories (starting with '.')
    let filter_hidden = |entry: &DirEntry| {
        let file_name = entry.file_name().to_string_lossy();
        // Skip entries (and their children) if directory starts with '.'
        if file_name.starts_with('.') || (entry.file_type().is_dir() && (file_name.contains("target"))) {
            false
        } else {
            true
        }
    };

    WalkDir::new(root_path)
        .into_iter()
        .filter_entry(filter_hidden)  // skip hidden dirs
        .par_bridge()                // parallelize iteration
        .filter_map(|entry| {
            match entry {
                Ok(dir_entry) => {
                    if dir_entry.file_type().is_file() {
                        match dir_entry.path().canonicalize() {
                            Ok(abs_path) => {
                                let abs_path_str = abs_path.to_string_lossy();
                                if abs_path_str.contains(query) {
                                    Some(abs_path_str.to_string())
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .collect()
}

