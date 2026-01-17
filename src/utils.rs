use serde::Serialize;

pub fn print_json<T: Serialize>(data: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string(data)?);
    Ok(())
}

pub fn glob_to_regex(pattern: &str) -> regex::Regex {
    let mut regex = pattern.to_string();

    // Escape regex special chars except *, ?, **
    regex = regex.replace('\\', "\\\\")
                 .replace('+', "\\+")
                 .replace('(', "\\(")
                 .replace(')', "\\)")
                 .replace('[', "\\[")
                 .replace(']', "\\]")
                 .replace('{', "\\{")
                 .replace('}', "\\}")
                 .replace('^', "\\^")
                 .replace('$', "\\$")
                 .replace('|', "\\|");

    // Convert glob patterns to regex
    regex = regex.replace("**", ".*");
    regex = regex.replace('*', "[^/]*");
    regex = regex.replace('?', ".");

    regex::Regex::new(&regex).unwrap_or_else(|_| regex::Regex::new(".*").unwrap())
}
