const ARC_MODE_PREFIX: &str = "D:";

#[derive(Clone, Copy)]
pub enum ArcMode {
    Radius,
    Diameter,
}

pub fn get_arc_mode(mode: &str) -> Result<ArcMode, String> {
    match mode {
        "0" => Ok(ArcMode::Radius),
        "1" => Ok(ArcMode::Diameter),
        _ => Err(format!("Unknown arc mode \"{}\"", mode))
    }
}

/// Returns arc mode from arc mode message e.g. "D:0"
/// 
/// # Examples
/// ```
/// let arc_mode = parse_arc_mode("D:0")
/// ```
pub fn parse_arc_mode(message: &str) -> Result<ArcMode, String> {
    if is_arc_mode(message) {
        let arc_mode_message = &message[ARC_MODE_PREFIX.len()..];
        return get_arc_mode(arc_mode_message);
    }
    Err(format!("Cannot read arc mode message \"{}\"", message))
}

/// Indicates if message has arc mode syntax
pub fn is_arc_mode(message: &str) -> bool {
    message.starts_with(ARC_MODE_PREFIX)
}
