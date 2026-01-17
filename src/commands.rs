use crate::types::{FileInfo, SearchResult};
use crate::utils::OutputFormat;
use anyhow::Result;
use std::io::{self, Write};
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

/// Flush output for streaming
fn flush() {
    let _ = io::stdout().flush();
}

/// Stream a single file result
fn stream_file(file: &FileInfo, format: OutputFormat) {
    match format {
        OutputFormat::Toon => println!("\t{},{}", file.path, file.size),
        OutputFormat::Json => {
            let json = serde_json::to_string(file).unwrap();
            println!("{}", json);
        }
        OutputFormat::Plain => println!("{} {}", file.path, file.size),
    }
    flush();
}

/// Stream a single search result
fn stream_search(result: &SearchResult, format: OutputFormat) {
    let content = if result.content.contains(',') || result.content.contains(':') {
        format!("\"{}\"", result.content)
    } else {
        result.content.clone()
    };
    match format {
        OutputFormat::Toon => println!("\t{},{},{}", result.path, result.line, content),
        OutputFormat::Json => {
            let json = serde_json::to_string(result).unwrap();
            println!("{}", json);
        }
        OutputFormat::Plain => println!("{}:{}: {}", result.path, result.line, result.content),
    }
    flush();
}

pub fn cmd_find(
    root: &PathBuf,
    pattern: &str,
    limit: usize,
    format: OutputFormat,
    stream: bool,
) -> Result<()> {
    let files = walk_files_parallel(root, 100);
    let mut count = 0;

    if stream && format == OutputFormat::Toon {
        println!("path,size:");
        flush();
    }

    for path in files.into_iter() {
        if count >= limit {
            break;
        }

        let relative = path.strip_prefix(root)?;
        let path_str = relative.to_string_lossy();

        if glob_match(pattern, &path_str) {
            let size = std::fs::metadata(&path).ok().map(|m| m.len()).unwrap_or(0);
            let file = FileInfo {
                path: path_str.to_string(),
                size,
            };

            if stream {
                stream_file(&file, format);
            } else if format == OutputFormat::Toon {
                // Collect for header
                print!("\t{},{}", file.path, file.size);
            } else if format == OutputFormat::Json {
                println!("{}", serde_json::to_string(&file)?);
            } else {
                println!("{} {}", file.path, file.size);
            }
            count += 1;
        }
    }

    if format == OutputFormat::Toon && !stream {
        println!();
    }

    Ok(())
}

pub fn cmd_search(
    root: &PathBuf,
    query: &str,
    ext_filter: Option<&str>,
    limit: usize,
    format: OutputFormat,
    stream: bool,
) -> Result<()> {
    let exts: Vec<String> = ext_filter
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let files = walk_files_parallel(root, 100);
    let mut count = 0;

    if stream && format == OutputFormat::Toon {
        println!("path,line,content:");
        flush();
    }

    for path in files.into_iter() {
        if count >= limit {
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
                if count >= limit {
                    break;
                }
                if line.contains(query) {
                    let result = SearchResult {
                        path: relative.to_string_lossy().to_string(),
                        line: line_num + 1,
                        content: line.trim().to_string(),
                    };

                    if stream {
                        stream_search(&result, format);
                    } else if format == OutputFormat::Toon {
                        print!("\t{},{},", result.path, result.line);
                        if result.content.contains(',') || result.content.contains(':') {
                            print!("\"{}\"", result.content);
                        } else {
                            print!("{}", result.content);
                        }
                        println!();
                    } else if format == OutputFormat::Json {
                        println!("{}", serde_json::to_string(&result)?);
                    } else {
                        println!("{}:{}: {}", result.path, result.line, result.content);
                    }
                    count += 1;
                }
            }
        }
    }

    Ok(())
}

pub fn cmd_list(
    root: &PathBuf,
    limit: usize,
    format: OutputFormat,
    stream: bool,
) -> Result<()> {
    let files = walk_files_parallel(root, 100);
    let mut count = 0;

    if stream && format == OutputFormat::Toon {
        println!("path,size:");
        flush();
    }

    for path in files.into_iter() {
        if count >= limit {
            break;
        }

        let relative = path.strip_prefix(root)?;
        let size = std::fs::metadata(&path).ok().map(|m| m.len()).unwrap_or(0);
        let file = FileInfo {
            path: relative.to_string_lossy().to_string(),
            size,
        };

        if stream {
            stream_file(&file, format);
        } else if format == OutputFormat::Toon {
            print!("\t{},{}", file.path, file.size);
            println!();
        } else if format == OutputFormat::Json {
            println!("{}", serde_json::to_string(&file)?);
        } else {
            println!("{} {}", file.path, file.size);
        }
        count += 1;
    }

    Ok(())
}

/// Estimate: Show file count and total size for a directory
pub fn cmd_estimate(root: &PathBuf, sample_limit: usize) -> Result<()> {
    // Use sequential walk for consistent sampling (parallel gives random order)
    let files: Vec<PathBuf> = WalkDir::new(root)
        .max_depth(100)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Sample-based estimation for speed
    let sample_size = std::cmp::min(files.len(), sample_limit);
    let mut total_size: u64 = 0;
    let mut ext_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for path in files.iter().take(sample_size) {
        if let Ok(meta) = std::fs::metadata(path) {
            total_size += meta.len();
        }

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            *ext_counts.entry(ext.to_string()).or_insert(0) += 1;
        }
    }

    // Extrapolate
    let total_files = files.len();
    let estimated_total_size = if sample_size > 0 {
        (total_size as f64 / sample_size as f64) * total_files as f64
    } else {
        0.0
    };

    // Format size nicely
    fn format_size(bytes: f64) -> String {
        if bytes >= 1_000_000_000.0 {
            format!("{:.2} GB", bytes / 1_000_000_000.0)
        } else if bytes >= 1_000_000.0 {
            format!("{:.2} MB", bytes / 1_000_000.0)
        } else if bytes >= 1_000.0 {
            format!("{:.2} KB", bytes / 1_000.0)
        } else {
            format!("{} B", bytes as u64)
        }
    }

    println!("file_count: {}", total_files);
    println!("sampled: {}", sample_size);
    println!("total_size: {} (estimated)", format_size(estimated_total_size));
    println!("sample_size: {} (actual)", format_size(total_size as f64));

    // Top 5 extensions
    let mut ext_vec: Vec<_> = ext_counts.iter().collect();
    ext_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("top_extensions:");
    for (ext, count) in ext_vec.iter().take(5) {
        println!("  {}: {}", ext, count);
    }

    Ok(())
}
