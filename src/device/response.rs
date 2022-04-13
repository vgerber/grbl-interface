use crate::device::response::{report::MachineInfo, firmware::{version::FirmwareVersion, startup::FirmwareStartupResult, compile_option::{CompileOptions, is_extended_compile_options, parse_extended_compile_options}}, util::{message::Message, echo::EchoMessage}, state::gcode_state::GCodeState};

use super::DeviceInfo;
pub mod firmware;
pub mod report;
pub mod setting;
pub mod util;
pub mod state;


pub fn read_response(response: String, device_info: &mut DeviceInfo) -> Result<(), String> {
    println!("Parse resonse: {}", &response);
    if MachineInfo::is_response(&response) {
        match MachineInfo::from(&response) {
            Ok(info) => { 
                device_info.update_machine_info(info);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if FirmwareVersion::is_response(&response) {
        match FirmwareVersion::from(&response) {
            Ok(value) => { 
                device_info.set_version(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if FirmwareStartupResult::is_response(&response) {
        match FirmwareStartupResult::from(&response) {
            Ok(value) => { 
                device_info.set_startup_result(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if CompileOptions::is_response(&response) {
        match CompileOptions::from(&response) {
            Ok(value) => { 
                device_info.set_compile_options(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if is_extended_compile_options(&response) {
        match parse_extended_compile_options(&response) {
            Ok(value) => { 
                device_info.set_extended_compile_options(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if GCodeState::is_response(&response) {
        match GCodeState::from(&response) {
            Ok(value) => { 
                device_info.set_gcode_state(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if Message::is_response(&response) {
        match Message::from(&response) {
            Ok(value) => { 
                device_info.set_last_message(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if EchoMessage::is_response(&response) {
        match EchoMessage::from(&response) {
            Ok(value) => { 
                device_info.set_last_echo_message(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else {
        Err(format!("Unknown response format: {}", response))
    }    
}