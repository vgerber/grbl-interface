use std::result::Result;

use crate::device::axis::Axis;

pub mod machine_status;


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

pub enum AccessoryState {
    SpindleClockwise,
    SpindleCounterClockwise,
    FloodCoolantEnabled,
    MistCoolantEnabled,
    ToolChangePending,
}

pub enum PendantControl {
    Released,
    Taken,
}

pub enum ArcMode {
    Radius,
    Diameter,
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

pub fn get_signal(signal: &str) -> Result<MachineSignal, String> {
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

pub fn get_pendant_control(state: i8) -> Result<PendantControl, String> {
    match state {
        0 => Ok(PendantControl::Released),
        1 => Ok(PendantControl::Taken),
        _ => Err(format!("Unknon pendant control state {}", state))
    }
}

pub fn get_arc_mode(mode: &str) -> Result<ArcMode, String> {
    match mode {
        "0" => Ok(ArcMode::Radius),
        "1" => Ok(ArcMode::Diameter),
        _ => Err(format!("Unknown arc mode \"{}\"", mode))
    }
}



pub struct MachinePosition(f32, f32, f32);

pub struct BufferState {
    block_buffers_free: i32,
    rx_buffers_free: i32,
}

pub struct HomingState {
    completed: bool,
    axis_bitmask: i32,
}

pub struct MachineSpeed {
    feed_rate: i32,
    spindel_programmed_rpm: i32,
    spindel_actual_rpm: Option<i32>
}

pub struct OverrideValues {
    feed_rate_percentage: i32,
    rapids_percentage: i32,
    spindle_speed_percentage: i32
}

/// Reponse for report message
/// 
/// # TODO
/// * Add "In:<result>" messages
pub struct ReportResponse {
    /// <Status>[:Sub Status]
    machine_status: MachineStatus,

    /// MPos: Absolute position in machine workspace
    global_position: Option<MachinePosition>,

    /// WPos: Relative position to machine offset
    local_position: Option<MachinePosition>,

    /// WCO: Relative position offset
    local_offset: Option<MachinePosition>,

    buffer_state: Option<BufferState>,
    line_number: Option<i32>,
    machine_speed: Option<MachineSpeed>,
    machine_signals: Option<Vec<MachineSignal>>,
    machine_coordinate_system: Option<String>,
    override_values: Option<OverrideValues>, 
    accessory_state: Option<AccessoryState>,
    pendant_control: Option<PendantControl>,
    homing_state: Option<HomingState>,
    scaled_axis: Option<Vec<Axis>>,
    tool_length_reference_offset_set: Option<bool>,
    firmware: Option<String>,

}

impl ReportResponse {

    /// Reads startup line reponse and status
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores all status messages from this message
    /// let response = StartupResponse::from("<Status:0|State2|...|StateN>");
    /// ```
    pub fn from(message: &str) -> Result<StartupResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // ">line:status:code"
        if trimmed_message.starts_with("<") && trimmed_message.ends_with(">") {
            let message_payload = &trimmed_message[1..];
            let segments: Vec<&str> = message_payload.split(":").collect();
            
            // expect <line>:<status>[:code]
            if segments.len() < 2 {
                return Err(format!("Invalid count of startup segments \"{}\"", message_payload));    
            }
            
            // reead <:code> value
            let mut result_code = -1;
            if segments.len() >= 3 {
                result_code = match segments[2].parse() {
                    Ok(value) => value,
                    Err(_) => -1
                };
            }
            
            // read status message
            let executed_line = String::from(segments[0]);
            let result = match segments[1] {
                "ok" => StartupResult::Ok,
                "error" => StartupResult::Error(result_code),
                _ => return Err(format!("Invalid result \"{}\"", segments[1])),
            };

            return Ok(StartupResponse {
                executed_line,
                result,
            })
        }
        Err(format!("Could not read startup \"{}\"", message))        
    }
    
    pub fn executed_line(&self) -> &String {
        &self.executed_line
    }

    pub fn result(&self) -> &StartupResult {
        &self.result
    }
}