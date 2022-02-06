const FIRMWARE_PREFIX: &str = "FW:";

/// Returns firmware from message e.g. "FW:grblHAL" 
/// 
/// # Examples
/// ```
/// let fw = parse_firmware("FW:grblHAL");
/// assert_eq!("grblHAL", fw)
/// ```
pub fn parse_firmware(message: &str) -> Result<String, String> {
    if is_firmware(message) {
        return Ok((message[FIRMWARE_PREFIX.len()..]).to_string());
    }
    Err(format!("Cannot read firmware message \"{}\"", message))
}

/// Indicates if message has firmware syntax
pub fn is_firmware(message: &str) -> bool {
    message.starts_with(FIRMWARE_PREFIX)
}