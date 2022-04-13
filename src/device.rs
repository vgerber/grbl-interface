use self::response::{firmware::{FirmwareInfo, compile_option::CompileOptions, version::FirmwareVersion, startup::FirmwareStartupResult}, report::MachineInfo, util::{message::Message, echo::EchoMessage}, state::{gcode_state::GCodeState, compile::ExtendedCompileOption}};

pub mod response;
pub mod command;
pub mod util;


/// The device object stores all gathered information from a grbl controller
/// 
/// Each response will update this information
#[derive(Clone)]
pub struct DeviceInfo {
    id: String,
    firmware_info: Option<FirmwareInfo>,
    machine_info: Option<MachineInfo>,
    gcode_state: Option<GCodeState>,
    last_message: Option<Message>,
    last_echo_message: Option<EchoMessage>
}



impl DeviceInfo {

    pub fn from(id: &str) -> Result<DeviceInfo, String> {
        Ok(DeviceInfo {
            id: id.to_string(),
            firmware_info: None,
            machine_info: None,
            gcode_state: None,
            last_message: None,
            last_echo_message: None,
        })
    }

    /// Get a reference to the device's id.
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /// Get a reference to the device's last message.
    #[must_use]
    pub fn last_message(&self) -> Option<&Message> {
        self.last_message.as_ref()
    }

    /// Set the device's last message.
    pub fn set_last_message(&mut self, last_message: Message) {
        self.last_message = Some(last_message);
    }

    /// Get a reference to the device's last echo message.
    #[must_use]
    pub fn last_echo_message(&self) -> Option<&EchoMessage> {
        self.last_echo_message.as_ref()
    }

    /// Set the device's last echo message.
    pub fn set_last_echo_message(&mut self, last_echo_message: EchoMessage) {
        self.last_echo_message = Some(last_echo_message);
    }

    /// Get a reference to the device's firmware info.
    #[must_use]
    pub fn firmware_info(&self) -> Option<&FirmwareInfo> {
        self.firmware_info.as_ref()
    }

    /// Sets compile options active in the firmware
    pub fn set_extended_compile_options(&mut self, compile_options: Vec<ExtendedCompileOption>) {
        match self.firmware_info.as_mut() {
            Some(fw) => fw.set_extended_compile_options(Some(compile_options)),
            None => {
                match FirmwareInfo::from(None, None, None, Some(compile_options)) {
                    Ok(fw) => self.firmware_info = Some(fw),
                    Err(err) => panic!("Setting compile options failed: {}", err)
                }
            }
        }
    }

    /// Sets compile options active in the firmware
    pub fn set_compile_options(&mut self, compile_options: CompileOptions) {
        println!("Set compile options: {:#?}", compile_options);
        match self.firmware_info.as_mut() {
            Some(fw) => fw.set_compile_options(Some(compile_options)),
            None => {
                match FirmwareInfo::from(None, None, Some(compile_options), None) {
                    Ok(fw) => self.firmware_info = Some(fw),
                    Err(err) => panic!("Setting compile options failed: {}", err)
                }
            }
        }
    }

    /// Sets the version of the firmware
    pub fn set_version(&mut self, version: FirmwareVersion) {
        match self.firmware_info.as_mut() {
            Some(fw) => fw.set_version(Some(version)),
            None => {
                match FirmwareInfo::from(None, Some(version), None, None) {
                    Ok(fw) => self.firmware_info = Some(fw),
                    Err(err) => panic!("Setting firmware version failed: {}", err)
                }
            }
        }
    }

    /// Sets the result of the startup process
    /// 
    /// This might be never set if the user attaches after the controller started
    pub fn set_startup_result(&mut self, startup_result: FirmwareStartupResult) {
        match self.firmware_info.as_mut() {
            Some(fw) => fw.set_startup_result(Some(startup_result)),
            None => {
                match FirmwareInfo::from(Some(startup_result), None, None, None) {
                    Ok(fw) => self.firmware_info = Some(fw),
                    Err(err) => panic!("Setting firmware startup result failed: {}", err)
                }
            }
        }
    }

    /// Get a reference to the device's machine info.
    #[must_use]
    pub fn machine_info(&self) -> Option<&MachineInfo> {
        self.machine_info.as_ref()
    }

    /// Set the device's machine info.
    /// 
    /// Does only replace present values
    pub fn update_machine_info(&mut self, machine_info: MachineInfo) {
        // just set info as new info if no info was present
        // otherwhise check for each single value and replace it with the updated information
        if let None = self.machine_info {
            self.machine_info = Some(machine_info);
            return;
        }

        // update each property if present
        // do not replace any property to None
        let mut old_machine_info = self.machine_info.take().unwrap();

        old_machine_info.set_machine_state(machine_info.machine_state().clone());

        
        if let Some(gp) = machine_info.global_position() {
            old_machine_info.set_global_position(gp.clone());
        }

        if let Some(lp) = machine_info.local_position() {
            old_machine_info.set_local_position(lp.clone());
        }

        if let Some(lo) = machine_info.local_offset() {
            old_machine_info.set_local_offset(lo.clone());
        }

        if let Some(bs) = machine_info.buffer_state() {
            old_machine_info.set_buffer_state(*bs);
        }

        if let Some(ln) = machine_info.line_number() {
            old_machine_info.set_line_number(ln);
        }

        if let Some(ms) = machine_info.machine_speed() {
            old_machine_info.set_machine_speed(*ms);
        }

        if let Some(ms) = machine_info.machine_signals() {
            old_machine_info.set_machine_signals(ms.clone());
        }

        if let Some(mcs) = machine_info.machine_coordinate_system() {
            old_machine_info.set_machine_coordinate_system(mcs.clone());
        }

        if let Some(ms) = machine_info.machine_signals() {
            old_machine_info.set_machine_signals(ms.clone());
        }

        if let Some(ov) = machine_info.override_values() {
            old_machine_info.set_override_values(*ov);
        }

        if let Some(accessory_state) = machine_info.accessory_state() {
            old_machine_info.set_accessory_state(accessory_state.clone());
        }

        if let Some(pc) = machine_info.pendant_control() {
            old_machine_info.set_pendant_control(*pc);
        }

        if let Some(hs) = machine_info.homing_state() {
            old_machine_info.set_homing_state(hs.clone());
        }

        if let Some(sa) = machine_info.scaled_axes() {
            old_machine_info.set_scaled_axes(sa.clone());
        }

        if let Some(tlros) = machine_info.tool_length_reference_offset_set() {
            old_machine_info.set_tool_length_reference_offset_set(tlros);
        }
        
        if let Some(fw) = machine_info.firmware() {
            old_machine_info.set_firmware(fw.clone());
        }

        if let Some(iwrs) = machine_info.input_wait_result_succeeded() {
            old_machine_info.set_input_wait_result_succeeded(iwrs);
        }

        if let Some(ac) = machine_info.arc_mode() {
            old_machine_info.set_arc_mode(*ac);
        }

        // set updated old machine info to new info
        self.machine_info = Some(old_machine_info);
    }

    /// Get a reference to the device's gcode state.
    #[must_use]
    pub fn gcode_state(&self) -> Option<&GCodeState> {
        self.gcode_state.as_ref()
    }

    /// Set the device's gcode state.
    pub fn set_gcode_state(&mut self, gcode_state: GCodeState) {
        self.gcode_state = Some(gcode_state);
    }  
}

unsafe impl Send for DeviceInfo {}