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
