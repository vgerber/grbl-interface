pub mod machine_status;
pub mod signal;
pub mod accessory;

use std::result::Result;
use crate::device::axis::Axis;
use self::{machine_status::MachineStatus, signal::MachineSignal, accessory::AccessoryState};

pub enum PendantControl {
    Released,
    Taken,
}

pub enum ArcMode {
    Radius,
    Diameter,
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
    /// let response = ReportResponse::from("<Status:0|State2|...|StateN>");
    /// ```
    pub fn from(message: &str) -> Result<ReportResponse, String> {
        Err(format!("Could not read startup \"{}\"", message))        
    }

    pub fn is_report_response(message: &str) -> bool {
        message.starts_with("<") && message.ends_with(">")
    }
}