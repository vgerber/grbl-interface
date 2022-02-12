use std::result::Result;
use crate::device::{axis::Axis, state::{machine::{state::MachineState, position::{MachinePosition, is_local_position, is_global_position, parse_local_position, parse_global_position, is_local_position_offset, parse_local_position_offset, is_coordinate_system, parse_coordinate_system, is_scaled_axes, parse_scaled_axes}, speed::MachineSpeed,}, buffer::BufferState, signal::{MachineSignal, is_machine_signal, parse_machine_signal}, accessory::{AccessoryState, is_accessory_state, parse_accessory_state}, pendant::{PendantControl, is_pendant_control, parse_pendant_control}, homing::HomingState, overrides::Overrides, gcode::{is_line_number, parse_line_number}, modes::{is_arc_mode, ArcMode, parse_arc_mode}, tool::{is_tool_length_reference, parse_tool_length_reference}, firmware::{is_firmware, parse_firmware}, input::{is_input_wait_result, parse_input_wait_result}}};

const REPORT_PREFIX: &str = "<";
const REPORT_SUFFIX: &str = ">";

/// Reponse for report message
pub struct ReportResponse {
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
        if ReportResponse::is_report_response(message) {
            let report_message = message.strip_prefix(REPORT_PREFIX).unwrap().strip_suffix(REPORT_SUFFIX).unwrap();
            let report_states: Vec<&str> = report_message.split("|").collect();
            return ReportResponse::parse_report_states(report_states);
        }
        Err(format!("Cannot read report response \"{}\"", message))        
    }

    pub fn is_report_response(message: &str) -> bool {
        message.starts_with(REPORT_PREFIX) && message.ends_with(REPORT_SUFFIX)
    }

    fn parse_report_states(states: Vec<&str>) -> Result<ReportResponse, String> {
        // create initial report template with mandatory fields
        let mut report = match ReportResponse::parse_mandatory_report_state(&states) {
            Ok(report) => report,
            Err(error) => return Err(error),
        };

        // fill empty slots in report with remaining state options
        for state in states {
            if BufferState::is_buffer_state(state) {
                report.buffer_state = match BufferState::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                };
            } else if is_line_number(state) {
                 report.line_number = match parse_line_number(state) {
                     Ok(state) => Some(state),
                     Err(error) => return Err(ReportResponse::format_state_error(error)),
                 }
            } else if MachineSpeed::is_machine_speed(state) {
                report.machine_speed = match MachineSpeed::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_machine_signal(state) {
                report.machine_signals = match parse_machine_signal(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_local_position_offset(state) {
                report.local_offset = match parse_local_position_offset(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_coordinate_system(state) {
                report.machine_coordinate_system = match parse_coordinate_system(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if Overrides::is_overrides_values(state) {
                report.override_values = match Overrides::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_accessory_state(state) {
                report.accessory_state = match parse_accessory_state(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_pendant_control(state) {
                report.pendant_control = match parse_pendant_control(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if HomingState::is_homing_state(state) {
                report.homing_state = match HomingState::from(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_arc_mode(state) {
                report.arc_mode = match parse_arc_mode(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_scaled_axes(state) {
                report.scaled_axes = match parse_scaled_axes(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_tool_length_reference(state) {
                report.tool_length_reference_offset_set = match parse_tool_length_reference(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_firmware(state) {
                report.firmware = match parse_firmware(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            } else if is_input_wait_result(state) {
                report.input_wait_result_succeeded = match parse_input_wait_result(state) {
                    Ok(state) => Some(state),
                    Err(error) => return Err(ReportResponse::format_state_error(error)),
                }
            }
        }

        Ok(report)
    }

    fn parse_mandatory_report_state(states: &Vec<&str>) -> Result<ReportResponse, String> {
        // parsing mandatory states
        if states.len() < 2 {
            return Err(String::from("Report states requires machine state and position in every report"));
        }
        
        let machine_state = match MachineState::from(states[0]) {
            Ok(state) => state,
            Err(err) => return Err(ReportResponse::format_state_error(err))
        };

        let mut local_position: Option<MachinePosition> = None;
        let mut global_position: Option<MachinePosition> = None;
        let machine_position_string = states[1];

        
        if !is_local_position(machine_position_string) && !is_global_position(machine_position_string) {
            return Err(ReportResponse::format_state_error(String::from("Report should contain a local or global/machine position")));
        }

        if is_local_position(machine_position_string) {
            local_position = match parse_local_position(machine_position_string) {
                Ok(position) => Some(position),
                Err(error) => return Err(ReportResponse::format_state_error(error)),
            };
        }

        if is_global_position(machine_position_string) {
            global_position = match parse_global_position(machine_position_string) {
                Ok(position) => Some(position),
                Err(error) => return Err(ReportResponse::format_state_error(error)),
            };
        }

        Ok(ReportResponse {
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
    pub fn accessory_state(&self) -> Option<&Vec<AccessoryState>> {
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

    /// Get a reference to the report response's input wait result succeeded.
    pub fn input_wait_result_succeeded(&self) -> Option<bool> {
        self.input_wait_result_succeeded
    }

    /// Get a reference to the report response's arc mode.
    pub fn arc_mode(&self) -> Option<&ArcMode> {
        self.arc_mode.as_ref()
    }
}