use crate::types::{FileInfo, SearchResult};
use crate::utils::{OutputFormat, print_json, print_toon_files, print_toon_search, print_plain_files, print_plain_search};
use anyhow::Result;
use std::path::PathBuf;
use glob_match::glob_match;
use walkdir::WalkDir;
use rayon::prelude::*;

/// Parallel walk for large directories - shows EVERYTHING
fn walk_files_parallel(root: &PathBuf, max_depth: usize) -> Vec<PathBuf> {
    WalkDir::new(root)
        .max_depth(max_depth)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

pub fn cmd_find(
    root: &PathBuf,
    pattern: &str,
    limit: usize,
    format: OutputFormat,
) -> Result<()> {
    let files = walk_files_parallel(root, 100); // 100 deep is plenty
    let mut results = Vec::new();

    for path in files.into_iter().take(limit * 10) { // Take more than limit since we filter
        if results.len() >= limit {
            break;
        }

        let relative = path.strip_prefix(root)?;
        let path_str = relative.to_string_lossy();

        if glob_match(pattern, &path_str) {
            let size = std::fs::metadata(&path).ok().map(|m| m.len()).unwrap_or(0);
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
    format: OutputFormat,
) -> Result<()> {
    let exts: Vec<String> = ext_filter
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let files = walk_files_parallel(root, 100);
    let mut results = Vec::new();

    for path in files.into_iter().take(limit * 100) { // Take more since we filter
        if results.len() >= limit {
            break;
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
        if let Ok(content) = std::fs::read_to_string(&path) {
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
    limit: usize,
    format: OutputFormat,
) -> Result<()> {
    let files = walk_files_parallel(root, 100);
    let mut results = Vec::new();

    for path in files.into_iter().take(limit) {
        let relative = path.strip_prefix(root)?;
        let size = std::fs::metadata(&path).ok().map(|m| m.len()).unwrap_or(0);

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
