const PENDANT_CONTROL_PREFIX: &str = "MPG:";

#[derive(Clone, Copy)]
pub enum PendantControl {
    Released,
    Taken,
}

/// Returns pendant control state for values 0 - 1
pub fn get_pendant_control(state: i8) -> Result<PendantControl, String> {
    match state {
        0 => Ok(PendantControl::Released),
        1 => Ok(PendantControl::Taken),
        _ => Err(format!("Unknown pendant control state {}", state))
    }
}

/// Returns pendant control state from "MPG:0"
/// 
/// # Examples
/// ```
/// let state = parse_pendant_control("MPG:1")
/// ```
pub fn parse_pendant_control(message: &str) -> Result<PendantControl, String> {
    if is_pendant_control(message) {
        let pendant_control_message = &message[PENDANT_CONTROL_PREFIX.len()..];
        let state: i8 = match pendant_control_message.parse() {
            Ok(state) => state,
            Err(_) => return Err(format!("Cannot interpret pendant control state \"{}\"", pendant_control_message))
        };
        return get_pendant_control(state);
    }
    Err(format!("Cannot read pendant control message \"{}\"", message))
}

/// Indicates if message has pendant control syntax
pub fn is_pendant_control(message: &str) -> bool {
    message.starts_with(PENDANT_CONTROL_PREFIX)
}