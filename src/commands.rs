use crate::filter::should_exclude;
use crate::types::{FileInfo, SearchResult};
use crate::utils::{glob_to_regex, print_json};
use anyhow::Result;
use std::path::PathBuf;

pub fn cmd_find(
    root: &PathBuf,
    pattern: &str,
    limit: usize,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
) -> Result<()> {
    let mut results = Vec::new();
    let regex_pattern = glob_to_regex(pattern);

    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden);

    for entry in builder.build() {
        if results.len() >= limit {
            break;
        }

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() || should_exclude(path, excludes) {
            continue;
        }

        let relative = path.strip_prefix(root)?;
        let path_str = relative.to_string_lossy();

        if regex_pattern.is_match(&path_str) {
            let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            results.push(FileInfo {
                path: path_str.to_string(),
                size,
            });
        }
    }

    print_json(&results)
}

pub fn cmd_search(
    root: &PathBuf,
    query: &str,
    ext_filter: Option<&str>,
    limit: usize,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
) -> Result<()> {
    let mut results = Vec::new();
    let exts: Vec<&str> = ext_filter
        .map(|e| e.split(',').map(|s| s.trim()).collect())
        .unwrap_or_default();

    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden);

    for entry in builder.build() {
        if results.len() >= limit {
            break;
        }

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() || should_exclude(path, excludes) {
            continue;
        }

        // Extension filter
        if !exts.is_empty() {
            let has_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| exts.contains(&e))
                .unwrap_or(false);
            if !has_ext {
                continue;
            }
        }

        // Search content
        if let Ok(content) = std::fs::read_to_string(path) {
            let relative = path.strip_prefix(root)?;
            for (line_num, line) in content.lines().enumerate() {
                if line.contains(query) {
                    results.push(SearchResult {
                        path: relative.to_string_lossy().to_string(),
                        line: line_num + 1,
                        content: line.trim().to_string(),
                    });
                    if results.len() >= limit {
                        break;
                    }
                }
            }
        }
    }

    print_json(&results)
}

pub fn cmd_list(
    root: &PathBuf,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
    limit: usize,
) -> Result<()> {
    let mut results = Vec::new();

    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden);

    for entry in builder.build() {
        if results.len() >= limit {
            break;
        }

        let entry = entry?;
        let path = entry.path();

        if path.is_dir() || should_exclude(path, excludes) {
            continue;
        }

        let relative = path.strip_prefix(root)?;
        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

        results.push(FileInfo {
            path: relative.to_string_lossy().to_string(),
            size,
        });
    }

    print_json(&results)
}
