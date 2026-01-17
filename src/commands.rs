use crate::filter::should_exclude;
use crate::types::{FileInfo, SearchResult};
use crate::utils::{OutputFormat, print_json, print_toon_files, print_toon_search, print_plain_files, print_plain_search};
use anyhow::Result;
use std::path::PathBuf;
use glob_match::glob_match;

pub fn cmd_find(
    root: &PathBuf,
    pattern: &str,
    limit: usize,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
    format: OutputFormat,
) -> Result<()> {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden).threads(num_cpus::get());

    let mut results = Vec::new();

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

        if glob_match(pattern, &path_str) {
            let size = std::fs::metadata(path).ok().map(|m| m.len()).unwrap_or(0);
            results.push(FileInfo {
                path: path_str.to_string(),
                size,
            });
        }
    }

    match format {
        OutputFormat::Toon => print_toon_files(&results),
        OutputFormat::Json => print_json(&results)?,
        OutputFormat::Plain => print_plain_files(&results),
    }
    Ok(())
}

pub fn cmd_search(
    root: &PathBuf,
    query: &str,
    ext_filter: Option<&str>,
    limit: usize,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
    format: OutputFormat,
) -> Result<()> {
    let exts: Vec<String> = ext_filter
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden).threads(num_cpus::get());

    let mut results = Vec::new();

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
                .map(|e| exts.iter().any(|ext| ext == e))
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

    match format {
        OutputFormat::Toon => print_toon_search(&results),
        OutputFormat::Json => print_json(&results)?,
        OutputFormat::Plain => print_plain_search(&results),
    }
    Ok(())
}

pub fn cmd_list(
    root: &PathBuf,
    use_gitignore: bool,
    show_hidden: bool,
    excludes: &[String],
    limit: usize,
    format: OutputFormat,
) -> Result<()> {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.git_ignore(use_gitignore).hidden(!show_hidden).threads(num_cpus::get());

    let mut results = Vec::new();

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
        let size = std::fs::metadata(path).ok().map(|m| m.len()).unwrap_or(0);

        results.push(FileInfo {
            path: relative.to_string_lossy().to_string(),
            size,
        });
    }

    match format {
        OutputFormat::Toon => print_toon_files(&results),
        OutputFormat::Json => print_json(&results)?,
        OutputFormat::Plain => print_plain_files(&results),
    }
    Ok(())
}
