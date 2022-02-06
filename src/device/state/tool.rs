const TOOL_LENGTH_REFERENCE_PREFIX: &str = "TLR:";


/// Returns tool length reference offset set e.g. "TLR:1" 
/// 
/// # Examples
/// ```
/// let tlr_offset_set = parse_tool_length_reference("TLR:1");
/// assert!(tlr_offset_set)
/// ```
pub fn parse_tool_length_reference(message: &str) -> Result<bool, String> {
    if is_tool_length_reference(message) {
        let tlr_message = &message[TOOL_LENGTH_REFERENCE_PREFIX.len()..];
        
        // only accept value 1 as offset set
        return match tlr_message.parse::<i8>() {
            Ok(value) => Ok(value == 1),
            Err(_) => Err(format!("Cannot interpret tool reference length offset set value \"{}\"", tlr_message))
        };
    }
    Err(format!("Cannot read tool reference length \"{}\"", message))
}

/// Indicates if message has tlr syntax
pub fn is_tool_length_reference(message: &str) -> bool {
    message.starts_with(TOOL_LENGTH_REFERENCE_PREFIX)
}