use std::result::Result;
use crate::device::{axis::Axis, state::{machine::{state::MachineState, position::MachinePosition, speed::MachineSpeed,}, buffer::BufferState, signal::MachineSignal, accessory::AccessoryState, pendant::PendantControl, homing::HomingState, overrides::Overrides}};

/// Reponse for report message
/// 
/// # TODO
/// * Add "In:<result>" messages
pub struct ReportResponse {
    /// <Status>[:Sub Status]
    machine_status: MachineState,

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
    override_values: Option<Overrides>, 
    accessory_state: Option<AccessoryState>,
    pendant_control: Option<PendantControl>,
    homing_state: Option<HomingState>,
    scaled_axes: Option<Vec<Axis>>,
    tool_length_reference_offset_set: Option<bool>,
    firmware: Option<String>,

}

impl ReportResponse {

    /// Reads startup line reponse and status
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

    /// Get a reference to the report response's machine status.
    pub fn machine_status(&self) -> &MachineState {
        &self.machine_status
    }

    /// Get a reference to the report response's global position.
    pub fn global_position(&self) -> Option<&MachinePosition> {
        self.global_position.as_ref()
    }

    /// Get a reference to the report response's local position.
    pub fn local_position(&self) -> Option<&MachinePosition> {
        self.local_position.as_ref()
    }

    /// Get a reference to the report response's local offset.
    pub fn local_offset(&self) -> Option<&MachinePosition> {
        self.local_offset.as_ref()
    }

    /// Get a reference to the report response's buffer state.
    pub fn buffer_state(&self) -> Option<&BufferState> {
        self.buffer_state.as_ref()
    }

    /// Get a reference to the report response's line number.
    pub fn line_number(&self) -> Option<i32> {
        self.line_number
    }

    /// Get a reference to the report response's machine speed.
    pub fn machine_speed(&self) -> Option<&MachineSpeed> {
        self.machine_speed.as_ref()
    }

    /// Get a reference to the report response's machine signals.
    pub fn machine_signals(&self) -> Option<&Vec<MachineSignal>> {
        self.machine_signals.as_ref()
    }

    /// Get a reference to the report response's machine coordinate system.
    pub fn machine_coordinate_system(&self) -> Option<&String> {
        self.machine_coordinate_system.as_ref()
    }

    /// Get a reference to the report response's override values.
    pub fn override_values(&self) -> Option<&Overrides> {
        self.override_values.as_ref()
    }

    /// Get a reference to the report response's accessory state.
    pub fn accessory_state(&self) -> Option<&AccessoryState> {
        self.accessory_state.as_ref()
    }

    /// Get a reference to the report response's pendant control.
    pub fn pendant_control(&self) -> Option<&PendantControl> {
        self.pendant_control.as_ref()
    }

    /// Get a reference to the report response's homing state.
    pub fn homing_state(&self) -> Option<&HomingState> {
        self.homing_state.as_ref()
    }

    /// Get a reference to the report response's scaled axis.
    pub fn scaled_axes(&self) -> Option<&Vec<Axis>> {
        self.scaled_axes.as_ref()
    }

    /// Get a reference to the report response's tool length reference offset set.
    pub fn tool_length_reference_offset_set(&self) -> Option<bool> {
        self.tool_length_reference_offset_set
    }

    /// Get a reference to the report response's firmware.
    pub fn firmware(&self) -> Option<&String> {
        self.firmware.as_ref()
    }
}