

/// Parses a string property an removes its prefix and suffix
/// 
/// # Error
/// Returns an error when parsing fails
/// 
/// # Panics
/// Panics when string prefix/suffix strip operation fails
pub fn parse_string_property(message: &str, prefix: &str, suffix: &str, property_name: &str) -> Result<String, String> {
    if message.starts_with(prefix) && message.ends_with(suffix) {
        let property = message.strip_prefix(prefix).unwrap().strip_suffix(suffix).unwrap();
        match property.len() {
            0 => return Err(format!("{} is empty", property_name)),
            _ => return Ok(property.to_string()),
        }
    }
    Err(format!("Cannot read {}: \"{}\"", property_name, message))
}