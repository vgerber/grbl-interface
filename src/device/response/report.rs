use std::result::Result;

use crate::device::util::axis::Axis;

use super::state::{machine::{state::MachineState, position::{MachinePosition, is_local_position_offset, parse_local_position_offset, is_coordinate_system, parse_coordinate_system, is_scaled_axes, parse_scaled_axes, is_local_position, is_global_position, parse_local_position, parse_global_position}, speed::MachineSpeed}, buffer::BufferState, signal::{MachineSignal, is_machine_signal, parse_machine_signal}, overrides::Overrides, accessory::{AccessoryState, is_accessory_state, parse_accessory_state}, pendant::{PendantControl, is_pendant_control, parse_pendant_control}, homing::HomingState, modes::{ArcMode, is_arc_mode, parse_arc_mode}, gcode::{is_line_number, parse_line_number}, tool::{is_tool_length_reference, parse_tool_length_reference}, firmware::{is_firmware, parse_firmware}, input::{is_input_wait_result, parse_input_wait_result}};

const REPORT_PREFIX: &str = "<";
const REPORT_SUFFIX: &str = ">";

/// Reponse for report message
#[derive(Clone, Debug, PartialEq)]
pub struct MachineInfo {
    /// <Status>[:Sub Status]
    machine_state: MachineState,

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
    accessory_state: Option<Vec<AccessoryState>>,
    pendant_control: Option<PendantControl>,
    homing_state: Option<HomingState>,
    scaled_axes: Option<Vec<Axis>>,
    tool_length_reference_offset_set: Option<bool>,
    firmware: Option<String>,
    input_wait_result_succeeded: Option<bool>,
    arc_mode: Option<ArcMode>,
}

impl MachineInfo {
    /// Reads startup line reponse and status
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores all status messages from this message
    /// let response = MachineInfo::from("<Status:0|State2|...|StateN>");
    /// ```
    pub fn from(message: &str) -> Result<MachineInfo, String> {
        if MachineInfo::is_response(message) {
            let report_message = message
                .strip_prefix(REPORT_PREFIX)
                .unwrap()
                .strip_suffix(REPORT_SUFFIX)
                .unwrap();
            let report_states: Vec<&str> = report_message.split("|").collect();
            return MachineInfo::parse_report_states(report_states);
        }
        Err(format!("Cannot read report response \"{}\"", message))
    }

    pub fn is_response(message: &str) -> bool {
        message.starts_with(REPORT_PREFIX) && message.ends_with(REPORT_SUFFIX)
    }

    fn parse_report_states(states: Vec<&str>) -> Result<MachineInfo, String> {
        // create initial report template with mandatory fields
        let mut report = match MachineInfo::parse_mandatory_report_state(&states) {
            Ok(report) => report,
            Err(error) => return Err(error),
        };

        // fill empty slots in report with remaining state options
        for state in states {
            if BufferState::is_buffer_state(state) {
                report.buffer_state = match BufferState::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                };
            } else if is_line_number(state) {
                report.line_number = match parse_line_number(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if MachineSpeed::is_machine_speed(state) {
                report.machine_speed = match MachineSpeed::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_machine_signal(state) {
                report.machine_signals = match parse_machine_signal(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_local_position_offset(state) {
                report.local_offset = match parse_local_position_offset(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_coordinate_system(state) {
                report.machine_coordinate_system = match parse_coordinate_system(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if Overrides::is_overrides_values(state) {
                report.override_values = match Overrides::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_accessory_state(state) {
                report.accessory_state = match parse_accessory_state(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_pendant_control(state) {
                report.pendant_control = match parse_pendant_control(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if HomingState::is_homing_state(state) {
                report.homing_state = match HomingState::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_arc_mode(state) {
                report.arc_mode = match parse_arc_mode(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_scaled_axes(state) {
                report.scaled_axes = match parse_scaled_axes(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_tool_length_reference(state) {
                report.tool_length_reference_offset_set = match parse_tool_length_reference(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_firmware(state) {
                report.firmware = match parse_firmware(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            } else if is_input_wait_result(state) {
                report.input_wait_result_succeeded = match parse_input_wait_result(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(MachineInfo::format_state_error(error)),
                }
            }
        }

        Ok(report)
    }

    fn parse_mandatory_report_state(states: &Vec<&str>) -> Result<MachineInfo, String> {
        // parsing mandatory states
        if states.len() < 2 {
            return Err(String::from(
                "Report states requires machine state and position in every report",
            ));
        }

        let machine_state = match MachineState::from(states[0]) {
            Ok(state) => state,
            Err(err) => return Err(MachineInfo::format_state_error(err)),
        };

        let mut local_position: Option<MachinePosition> = None;
        let mut global_position: Option<MachinePosition> = None;
        let machine_position_string = states[1];

        if !is_local_position(machine_position_string)
            && !is_global_position(machine_position_string)
        {
            return Err(MachineInfo::format_state_error(String::from(
                "Report should contain a local or global/machine position",
            )));
        }

        if is_local_position(machine_position_string) {
            local_position = match parse_local_position(machine_position_string) {
                Ok(position) => Some(position),
                Err(error) => return Err(MachineInfo::format_state_error(error)),
            };
        }

        if is_global_position(machine_position_string) {
            global_position = match parse_global_position(machine_position_string) {
                Ok(position) => Some(position),
                Err(error) => return Err(MachineInfo::format_state_error(error)),
            };
        }

        Ok(MachineInfo {
            machine_state,
            local_position,
            global_position,
            local_offset: None,
            buffer_state: None,
            line_number: None,
            machine_signals: None,
            machine_speed: None,
            machine_coordinate_system: None,
            override_values: None,
            accessory_state: None,
            pendant_control: None,
            homing_state: None,
            scaled_axes: None,
            tool_length_reference_offset_set: None,
            firmware: None,
            input_wait_result_succeeded: None,
            arc_mode: None,
        })
    }

    fn format_state_error(error: String) -> String {
        format!("Report parsing failed: {}", error)
    }

    /// Get a reference to the report response's machine state.
    pub fn machine_state(&self) -> &MachineState {
        &self.machine_state
    }

    pub fn set_machine_state(&mut self, machine_state: MachineState) {
        self.machine_state = machine_state;
    }

    /// Get a reference to the report response's global position.
    pub fn global_position(&self) -> Option<&MachinePosition> {
        self.global_position.as_ref()
    }

    pub fn set_global_position(&mut self, global_position: MachinePosition) {
        self.global_position = Some(global_position)
    }

    /// Get a reference to the report response's local position.
    pub fn local_position(&self) -> Option<&MachinePosition> {
        self.local_position.as_ref()
    }

    pub fn set_local_position(&mut self, local_position: MachinePosition) {
        self.local_position = Some(local_position)
    }

    /// Get a reference to the report response's local offset.
    pub fn local_offset(&self) -> Option<&MachinePosition> {
        self.local_offset.as_ref()
    }

    pub fn set_local_offset(&mut self, local_offset: MachinePosition) {
        self.local_offset = Some(local_offset)
    }

    /// Get a reference to the report response's buffer state.
    pub fn buffer_state(&self) -> Option<&BufferState> {
        self.buffer_state.as_ref()
    }

    pub fn set_buffer_state(&mut self, buffer_state: BufferState) {
        self.buffer_state = Some(buffer_state)
    }

    /// Get a reference to the report response's line number.
    pub fn line_number(&self) -> Option<i32> {
        self.line_number
    }

    pub fn set_line_number(&mut self, line_number: i32) {
        self.line_number = Some(line_number)
    }

    /// Get a reference to the report response's machine speed.
    pub fn machine_speed(&self) -> Option<&MachineSpeed> {
        self.machine_speed.as_ref()
    }

    pub fn set_machine_speed(&mut self, machine_speed: MachineSpeed) {
        self.machine_speed = Some(machine_speed)
    }

    /// Get a reference to the report response's machine signals.
    pub fn machine_signals(&self) -> Option<&Vec<MachineSignal>> {
        self.machine_signals.as_ref()
    }

    pub fn set_machine_signals(&mut self, machine_signals: Vec<MachineSignal>) {
        self.machine_signals = Some(machine_signals)
    }

    /// Get a reference to the report response's machine coordinate system.
    pub fn machine_coordinate_system(&self) -> Option<&String> {
        self.machine_coordinate_system.as_ref()
    }

    pub fn set_machine_coordinate_system(&mut self, machine_coordinate_system: String) {
        self.machine_coordinate_system = Some(machine_coordinate_system)
    }

    /// Get a reference to the report response's override values.
    pub fn override_values(&self) -> Option<&Overrides> {
        self.override_values.as_ref()
    }

    pub fn set_override_values(&mut self, override_values: Overrides) {
        self.override_values = Some(override_values)
    }

    /// Get a reference to the report response's accessory state.
    pub fn accessory_state(&self) -> Option<&Vec<AccessoryState>> {
        self.accessory_state.as_ref()
    }

    pub fn set_accessory_state(&mut self, accessory_state: Vec<AccessoryState>) {
        self.accessory_state = Some(accessory_state)
    }

    /// Get a reference to the report response's pendant control.
    pub fn pendant_control(&self) -> Option<&PendantControl> {
        self.pendant_control.as_ref()
    }

    pub fn set_pendant_control(&mut self, pendant_control: PendantControl) {
        self.pendant_control = Some(pendant_control)
    }

    /// Get a reference to the report response's homing state.
    pub fn homing_state(&self) -> Option<&HomingState> {
        self.homing_state.as_ref()
    }

    pub fn set_homing_state(&mut self, homing_state: HomingState) {
        self.homing_state = Some(homing_state)
    }

    /// Get a reference to the report response's scaled axis.
    pub fn scaled_axes(&self) -> Option<&Vec<Axis>> {
        self.scaled_axes.as_ref()
    }

    pub fn set_scaled_axes(&mut self, scaled_axes: Vec<Axis>) {
        self.scaled_axes = Some(scaled_axes)
    }

    /// Get a reference to the report response's tool length reference offset set.
    pub fn tool_length_reference_offset_set(&self) -> Option<bool> {
        self.tool_length_reference_offset_set
    }

    pub fn set_tool_length_reference_offset_set(&mut self, tlros: bool) {
        self.tool_length_reference_offset_set = Some(tlros)
    }

    /// Get a reference to the report response's firmware.
    pub fn firmware(&self) -> Option<&String> {
        self.firmware.as_ref()
    }

    pub fn set_firmware(&mut self, firmware: String) {
        self.firmware = Some(firmware)
    }

    /// Get a reference to the report response's input wait result succeeded.
    pub fn input_wait_result_succeeded(&self) -> Option<bool> {
        self.input_wait_result_succeeded
    }

    pub fn set_input_wait_result_succeeded(&mut self, iwrs: bool) {
        self.input_wait_result_succeeded = Some(iwrs)
    }

    /// Get a reference to the report response's arc mode.
    pub fn arc_mode(&self) -> Option<&ArcMode> {
        self.arc_mode.as_ref()
    }

    pub fn set_arc_mode(&mut self, arc_mode: ArcMode) {
        self.arc_mode = Some(arc_mode)
    }
}
