
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