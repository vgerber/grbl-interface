
const MACHINE_SIGNALE_PREFIX: &str = "PN:";

#[derive(Clone, Copy)]
pub enum MachineSignal {
    ProbeTriggered,
    ProbeDisconnected,
    XLimitSwitchAsserted,
    YLimitSwitchAsserted,
    ZLimitSwitchAsserted,
    ALimitSwitchAsserted,
    BLimitSwitchAsserted,
    CLimitSwitchAsserted,
    DoorSwitchAsserted,
    ResetSwitchAsserted,
    FeedHoldSwitchAsserted,
    CycleStartSwitchAsserted,
    EStopSwitchAsserted,
    BlockDeleteSwitchAsserted,
    OptionalProgramStopSwitchAsserted,
    MotorWarning,
    MotorFault,
}

pub fn get_machine_signal(signal: &str) -> Result<MachineSignal, String> {
    match signal {
        "P" => Ok(MachineSignal::ProbeTriggered),
        "O" => Ok(MachineSignal::ProbeDisconnected),
        "X" => Ok(MachineSignal::XLimitSwitchAsserted),
        "Y" => Ok(MachineSignal::YLimitSwitchAsserted),
        "Z" => Ok(MachineSignal::ZLimitSwitchAsserted),
        "A" => Ok(MachineSignal::ALimitSwitchAsserted),
        "B" => Ok(MachineSignal::BLimitSwitchAsserted),
        "C" => Ok(MachineSignal::CLimitSwitchAsserted),
        "D" => Ok(MachineSignal::DoorSwitchAsserted),
        "R" => Ok(MachineSignal::ResetSwitchAsserted),
        "H" => Ok(MachineSignal::FeedHoldSwitchAsserted),
        "S" => Ok(MachineSignal::CycleStartSwitchAsserted),
        "E" => Ok(MachineSignal::EStopSwitchAsserted),
        "L" => Ok(MachineSignal::BlockDeleteSwitchAsserted),
        "T" => Ok(MachineSignal::OptionalProgramStopSwitchAsserted),
        "W" => Ok(MachineSignal::MotorWarning),
        "M" => Ok(MachineSignal::MotorFault),
        _ => Err(format!("Unknown signal \"{}\"", signal))
    }
}

/// Returns all signals from signal message e.g. "PN:WT"
/// 
/// # Examples
/// ```
/// let signals = parse_machine_signal("PN:POX")
/// ```
pub fn parse_machine_signal(message: &str) -> Result<Vec<MachineSignal>, String> {
    if is_machine_signal(message) {
        return parse_machine_signal_values(&message[MACHINE_SIGNALE_PREFIX.len()..]);
    }
    Err(format!("Cannot read machine signal \"{}\"", message))
}

/// Parses signals value string and returns all signals or an error if a signal is unknown
fn parse_machine_signal_values(signal_values: &str) -> Result<Vec<MachineSignal>, String> {
    let mut machine_signals: Vec<MachineSignal> = Vec::new();

    // parses accessory states as ascii chars
    // returns an error if a state is unknown
    for signal_byte in signal_values.bytes() {
        let signal = match String::from_utf8(vec![signal_byte]) {
            Ok(string) => string,
            Err(_) => String::from("Invalid Symbol")
        }; 
        match get_machine_signal(&signal[..]) {
            Ok(acs) => machine_signals.push(acs),
            Err(_) => return Err(format!("Unknown machine signal \"{}\"", signal)),
        }
    }
    Ok(machine_signals)
}

pub fn is_machine_signal(message: &str) -> bool {
    message.starts_with(MACHINE_SIGNALE_PREFIX)
}