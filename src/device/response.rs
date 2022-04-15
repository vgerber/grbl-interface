use log::debug;

use crate::device::response::{
    firmware::{
        compile_option::{
            is_extended_compile_options, parse_extended_compile_options, CompileOptions,
        },
        startup::FirmwareStartupResult,
        version::FirmwareVersion,
    },
    report::MachineInfo,
    state::gcode_state::GCodeState,
    util::{echo::EchoMessage, message::Message},
};

use self::{
    setting::{description::DeviceSettingDescription, group::DeviceSettingGroup, DeviceSetting}, firmware::{driver::{name::{is_driver_name_response, parse_driver_name}, option::{is_driver_options_response, parse_driver_options}, version::{is_driver_version_response, parse_driver_version}}, board::{name::{is_board_name_response, parse_board_name}, aux::AuxPorts, storage::Storage}},
};

use super::DeviceInfo;
pub mod firmware;
pub mod report;
pub mod setting;
pub mod state;
pub mod util;
pub mod parser;


/// Reads any response and updates the device info accordingly
pub fn read_response(response: &str, device_info: &mut DeviceInfo) -> Result<(), String> {
    debug!("Parse response: {}", response);
    if MachineInfo::is_response(response) {
        match MachineInfo::from(response) {
            Ok(info) => {
                device_info.update_machine_info(info);
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if GCodeState::is_response(response) {
        match GCodeState::from(response) {
            Ok(value) => {
                device_info.set_gcode_state(value);
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if Message::is_response(response) {
        match Message::from(response) {
            Ok(value) => {
                device_info.set_last_message(value);
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if EchoMessage::is_response(response) {
        match EchoMessage::from(response) {
            Ok(value) => {
                device_info.set_last_echo_message(value);
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if is_setting_response(response) {
        read_setting_response(response, device_info)
    } else if is_firmware_info_response(response) {
        read_firmware_info_response(response, device_info)
    } else {
        Err(format!("Unknown response format: \"{}\"", response))
    }
}

/// Indicates if message can be parsed by firmware info parsers
fn is_firmware_info_response(response: &str) -> bool {
    FirmwareVersion::is_response(response)
        || FirmwareStartupResult::is_response(response)
        || CompileOptions::is_response(response)
        || is_extended_compile_options(response)
        || is_firmware_board_response(response)
        || is_firmware_driver_response(response)
}

// Indicates if message can be arsed by settings parsers
fn is_setting_response(response: &str) -> bool {
    DeviceSetting::is_response(response)
        || DeviceSettingGroup::is_response(response)
        || DeviceSettingDescription::is_response(response)
}

/// Checks if message is a setting message and parses and stores its content
///
/// # Error
/// Returns an error when parsing fails
fn read_setting_response(response: &str, device_info: &mut DeviceInfo) -> Result<(), String> {
    if DeviceSetting::is_response(response) {
        match DeviceSetting::from(response) {
            Ok(setting) => {
                device_info.settings_mut().put_setting(setting);
                Ok(())
            }
            Err(err) => return Err(err),
        }
    } else if DeviceSettingGroup::is_response(response) {
        match DeviceSettingGroup::from(response) {
            Ok(group) => {
                device_info.settings_mut().put_setting_group(group);
                Ok(())
            }
            Err(err) => return Err(err),
        }
    } else if DeviceSettingDescription::is_response(response) {
        match DeviceSettingDescription::from(response) {
            Ok(description) => {
                device_info
                    .settings_mut()
                    .put_setting_description(description);
                Ok(())
            }
            Err(err) => return Err(err),
        }
    } else {
        Err(format!("Cannot find parsers for settings message: \"{}\"", response))
    }
}

/// Parses and stores settings updates
/// 
/// # Error
/// Returns an error when parsing fails
fn read_firmware_info_response(
    response: &str,
    device_info: &mut DeviceInfo,
) -> Result<(), String> {
    if FirmwareVersion::is_response(response) {
        match FirmwareVersion::from(response) {
            Ok(value) => {
                device_info.firmware_info_mut().set_version(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if FirmwareStartupResult::is_response(response) {
        match FirmwareStartupResult::from(response) {
            Ok(value) => {
                device_info
                    .firmware_info_mut()
                    .set_startup_result(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if CompileOptions::is_response(response) {
        match CompileOptions::from(response) {
            Ok(value) => {
                device_info
                    .firmware_info_mut()
                    .set_compile_options(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if is_extended_compile_options(response) {
        match parse_extended_compile_options(response) {
            Ok(value) => {
                device_info
                    .firmware_info_mut()
                    .set_extended_compile_options(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if is_firmware_driver_response(response) {
        read_firmware_driver_response(response, device_info)
    } else if is_firmware_board_response(response) {
        read_firmware_board_response(response, device_info)
    } else {
        Err(format!("Cannot find parsers for firmware message: \"{}\"", response))
    }
}

fn is_firmware_driver_response(message: &str) -> bool {
    is_driver_name_response(message) || is_driver_options_response(message) || is_driver_version_response(message)
}

fn is_firmware_board_response(message: &str) -> bool {
    is_board_name_response(message) || AuxPorts::is_response(message) || Storage::is_response(message)
}

fn read_firmware_driver_response(response: &str, device_info: &mut DeviceInfo) -> Result<(), String> {
    if is_driver_name_response(response) {
        match parse_driver_name(response) {
            Ok(value) => {
                device_info.firmware_info_mut().driver_info_mut().set_name(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if is_driver_version_response(response) {
        match parse_driver_version(response) {
            Ok(value) => {
                device_info.firmware_info_mut().driver_info_mut().set_version(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if is_driver_options_response(response) {
        match parse_driver_options(response) {
            Ok(value) => {
                device_info.firmware_info_mut().driver_info_mut().set_options(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Err(format!("Cannot find parsers for firmware driver message: \"{}\"", response))
    }
}

fn read_firmware_board_response(response: &str, device_info: &mut DeviceInfo) -> Result<(), String> {
    if is_board_name_response(response) {
        match parse_board_name(response) {
            Ok(value) => {
                device_info.firmware_info_mut().board_info_mut().set_name(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if AuxPorts::is_response(response) {
        match AuxPorts::from(response) {
            Ok(value) => {
                device_info.firmware_info_mut().board_info_mut().set_aux(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else if Storage::is_response(response) {
        match Storage::from(response) {
            Ok(value) => {
                device_info.firmware_info_mut().board_info_mut().set_storage(Some(value));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Err(format!("Cannot find parsers for firmware board message: \"{}\"", response))
    }
}
