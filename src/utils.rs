use crate::types::{FileInfo, SearchResult};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Toon,  // Compact TOON-like format (default for AI)
    Json,  // Standard JSON (backward compatibility)
    Plain, // Ultra-simple space-separated
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "toon" => Some(OutputFormat::Toon),
            "json" => Some(OutputFormat::Json),
            "plain" => Some(OutputFormat::Plain),
            _ => None,
        }
    }
}

// TOON format: "path,size: src/main.rs,1524"
pub fn print_toon_files(files: &[FileInfo]) {
    if files.is_empty() {
        println!("path,size:");
        return;
    }
    println!("path,size:");
    for f in files {
        println!("\t{},{}", f.path, f.size);
    }
}

// TOON format for search results: "path,line,content: src/main.rs,42,fn main() {"
pub fn print_toon_search(results: &[SearchResult]) {
    if results.is_empty() {
        println!("path,line,content:");
        return;
    }
    println!("path,line,content:");
    for r in results {
        // Escape content if it contains special chars
        let content = if r.content.contains(',') || r.content.contains(':') {
            format!("\"{}\"", r.content)
        } else {
            r.content.clone()
        };
        println!("\t{},{},{}", r.path, r.line, content);
    }
}

// Plain format: "src/main.rs 1524"
pub fn print_plain_files(files: &[FileInfo]) {
    for f in files {
        println!("{} {}", f.path, f.size);
    }
}

// Plain format for search: "src/main.rs:42: fn main() {"
pub fn print_plain_search(results: &[SearchResult]) {
    for r in results {
        println!("{}:{}: {}", r.path, r.line, r.content);
    }
}

pub fn print_json<T: Serialize>(data: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string(data)?);
    Ok(())
}
