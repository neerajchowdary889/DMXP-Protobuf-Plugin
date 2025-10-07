pub mod parser;
pub mod parse;

// Re-export the main parsing function for easy access
pub use parse::parse_proto_file;


// Helper function to extract boolean values from options
fn extract_bool_value(line: &str, key: &str) -> Option<bool> {
    extract_string_value(line, key)
        .and_then(|s| s.parse().ok())
}

// Helper function to extract numeric values from options
fn extract_number_value<T: std::str::FromStr>( line: &str, key: &str) -> Option<T> {
    extract_string_value(line, key)
        .and_then(|s| s.parse().ok())
}

/// Extract string values from option declarations
/// 
/// Parses option lines to extract quoted string values for specific keys.
/// 
/// # Arguments
/// * `line` - The line containing the option
/// * `key` - The key to extract the value for
/// 
/// # Returns
/// * `Option<String>` - The extracted string value or None if not found
fn extract_string_value(line: &str, key: &str) -> Option<String> {
    // First try with parentheses: option (key) = "value";
    if let Some(start) = line.find(&format!("({})", key)) {
        let mut rest = &line[start + key.len() + 2..]; // skip (key)
        rest = rest.trim_start();
        if rest.starts_with(')') {
            rest = rest[1..].trim_start();
        }
        if let Some(eq_pos) = rest.find('=') {
            let mut value_part = &rest[eq_pos + 1..];
            if let Some(end) = value_part.find(';') {
                value_part = &value_part[..end];
            }
            return Some(value_part.trim().trim_matches('"').to_string());
        }
    }

    // Fallback without parentheses: option key = "value";
    if let Some(start) = line.find(&format!("{}", key)) {
        let mut rest = &line[start + key.len()..];
        rest = rest.trim_start();
        if let Some(eq_pos) = rest.find('=') {
            let mut value_part = &rest[eq_pos + 1..];
            if let Some(end) = value_part.find(';') {
                value_part = &value_part[..end];
            }
            return Some(value_part.trim().trim_matches('"').to_string());
        }
    }
    None
}

/// Check if a line represents a field declaration
/// 
/// Uses heuristics to determine if a line contains a field definition
/// by checking for type, name, and number pattern.
/// 
/// # Arguments
/// * `line` - The line to check
/// 
/// # Returns
/// * `bool` - True if the line appears to be a field declaration
fn is_field_line(line: &str) -> bool {
    // Simple heuristic: field lines contain a type, name, and number
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts.len() >= 3 && 
    !line.starts_with("message") && 
    !line.starts_with("service") && 
    !line.starts_with("enum") &&
    !line.starts_with("option") &&
    !line.starts_with("rpc") &&
    (parts[2].contains('=') || (parts.len() > 3 && parts[3].contains('=')))
}
