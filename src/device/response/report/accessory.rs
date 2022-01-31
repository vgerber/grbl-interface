pub enum AccessoryState {
    SpindleClockwise,
    SpindleCounterClockwise,
    FloodCoolantEnabled,
    MistCoolantEnabled,
    ToolChangePending,
}

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