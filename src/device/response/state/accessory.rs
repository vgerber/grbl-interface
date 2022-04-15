

const ACESSORY_STATE_PREFIX: &str = "A:";

#[derive(Clone, Copy, Debug)]
pub enum AccessoryState {
    SpindleClockwise,
    SpindleCounterClockwise,
    FloodCoolantEnabled,
    MistCoolantEnabled,
    ToolChangePending,
}

/// Returns state by single character value e.g. "S" 
pub fn get_accessory_state(state: &str) -> Result<AccessoryState, String> {
    match state {
        "S" => Ok(AccessoryState::SpindleClockwise),
        "C" => Ok(AccessoryState::SpindleCounterClockwise),
        "F" => Ok(AccessoryState::FloodCoolantEnabled),
        "M" => Ok(AccessoryState::MistCoolantEnabled),
        "T" => Ok(AccessoryState::ToolChangePending),
        _ => Err(format!("Unknown accessory state \"{}\"", state))
    }
}

/// Parses accessory states message and returns als states from "A:SCFMT"
/// 
/// # Examples
/// ```
/// let states = AccessoryState::from("A:C")
/// ```
pub fn parse_accessory_state(message: &str) -> Result<Vec<AccessoryState>, String> {
    if is_accessory_state(message) {
        let accessory_message = &message[ACESSORY_STATE_PREFIX.len()..];
        return parse_accessory_state_values(accessory_message);
    }
    Err(format!("Cannot read accessory state \"{}\"", message))
}

/// Parses options string and returns interpreted compile options and unkown compile options
fn parse_accessory_state_values(accessory_states_str: &str) -> Result<Vec<AccessoryState>, String> {
    let mut accessory_states: Vec<AccessoryState> = Vec::new();

    // parses accessory states as ascii chars
    // returns an error if a state is unknown
    for acs_byte in accessory_states_str.bytes() {
        let acs = match String::from_utf8(vec![acs_byte]) {
            Ok(string) => string,
            Err(_) => String::from("Invalid Symbol")
        }; 
        match get_accessory_state(&acs[..]) {
            Ok(acs) => accessory_states.push(acs),
            Err(_) => return Err(format!("Unknown accessory state \"{}\"", acs)),
        }
    }
    Ok(accessory_states)
}

pub fn is_accessory_state(message: &str) -> bool {
    message.starts_with(ACESSORY_STATE_PREFIX)
}