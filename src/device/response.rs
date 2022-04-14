use crate::device::response::{report::MachineInfo, firmware::{version::FirmwareVersion, startup::FirmwareStartupResult, compile_option::{CompileOptions, is_extended_compile_options, parse_extended_compile_options}}, util::{message::Message, echo::EchoMessage}, state::gcode_state::GCodeState};

use self::setting::{DeviceSetting, group::DeviceSettingGroup, description::DeviceSettingDescription};

use super::DeviceInfo;
pub mod firmware;
pub mod report;
pub mod setting;
pub mod util;
pub mod state;


pub fn read_response(response: &str, device_info: &mut DeviceInfo) -> Result<(), String> {
    println!("Parse resonse: {}", response);
    if MachineInfo::is_response(response) {
        match MachineInfo::from(response) {
            Ok(info) => { 
                device_info.update_machine_info(info);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if GCodeState::is_response(response) {
        match GCodeState::from(response) {
            Ok(value) => { 
                device_info.set_gcode_state(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if Message::is_response(response) {
        match Message::from(response) {
            Ok(value) => { 
                device_info.set_last_message(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if EchoMessage::is_response(response) {
        match EchoMessage::from(response) {
            Ok(value) => { 
                device_info.set_last_echo_message(value);
                Ok(())
            }
            Err(err) => Err(err)
        }
    } else if is_setting_response(response) {
        match read_setting_response(response, device_info) {
            Ok(false) => panic!("Could not find setting parser: \"{}\"", response),
            Ok(true) => Ok(()),
            Err(err) => Err(err),            
        }
    } else if is_firmware_info_response(response) {
        match read_firmware_info_response(response, device_info) {
            Ok(false) => panic!("Could not find firmware info parser: \"{}\"", response),
            Ok(true) => Ok(()),
            Err(err) => Err(err),            
        }
    } else {
        Err(format!("Unknown response format: \"{}\"", response))
    }    
}

fn is_firmware_info_response(response: &str) -> bool {
    false
}

fn is_setting_response(response: &str) -> bool {
    DeviceSetting::is_response(response) || DeviceSettingGroup::is_response(response) || DeviceSettingDescription::is_response(response)
}

/// Checks if message is a setting message and parses and stores its content
/// 
/// # Returns
/// - True if parsing was successful
/// - False if response did not match any setting format
/// 
/// # Error
/// Returns an error when parsing fails
fn read_setting_response(response: &str, device_info: &mut DeviceInfo) -> Result<bool, String> {
    if DeviceSetting::is_response(response) {
        match DeviceSetting::from(response) {
            Ok(setting) => { 
                device_info.settings_mut().put_setting(setting);
                return Ok(true)
            }
            Err(err) => return Err(err)
        }
    } else if DeviceSettingGroup::is_response(response) {
        match DeviceSettingGroup::from(response) {
            Ok(group) => {
                device_info.settings_mut().put_setting_group(group);
                return Ok(true)
            }
            Err(err) => return Err(err)
        }
    } else if DeviceSettingDescription::is_response(response) {
        match DeviceSettingDescription::from(response) {
            Ok(description) => {
                device_info.settings_mut().put_setting_description(description);
                return Ok(true)
            }
            Err(err) => return Err(err)
        }
    }

    return Ok(false);
}

/// Checks if message is a firmware info message and parses and stores its content
/// 
/// # Returns
/// - True if parsing was successful
/// - False if response did not match any firmware info format
/// 
/// # Error
/// Returns an error when parsing fails
fn read_firmware_info_response(response: &str, device_info: &mut DeviceInfo) -> Result<bool, String> {
    if FirmwareVersion::is_response(response) {
        match FirmwareVersion::from(response) {
            Ok(value) => { 
                device_info.firmware_info_mut().set_version(Some(value));
                Ok(true)
            }
            Err(err) => Err(err)
        }
    } else if FirmwareStartupResult::is_response(response) {
        match FirmwareStartupResult::from(response) {
            Ok(value) => { 
                device_info.firmware_info().set_startup_result(Some(value));
                Ok(true)
            }
            Err(err) => Err(err)
        }
    } else if CompileOptions::is_response(response) {
        match CompileOptions::from(response) {
            Ok(value) => { 
                device_info.firmware_info_mut().set_compile_options(Some(value));
                Ok(true)
            }
            Err(err) => Err(err)
        }
    } else if is_extended_compile_options(response) {
        match parse_extended_compile_options(response) {
            Ok(value) => { 
                device_info.firmware_info_mut().set_extended_compile_options(Some(value));
                Ok(true)
            }
            Err(err) => Err(err)
        }
    } else {
        Ok(false)
    }    
}