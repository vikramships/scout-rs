pub fn build_exclude_patterns(exclude: &Option<String>) -> Vec<String> {
    exclude
        .as_ref()
        .map(|e| e.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default()
}

pub fn should_exclude(path: &std::path::Path, excludes: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    excludes.iter().any(|p| path_str.contains(p))
}
