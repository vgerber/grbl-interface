use std::{thread, time::Duration};

use grbli::{
    device::command::{
        general::SYNC,
        settings,
        state::{self},
    },
    service::device_service::{DeviceEndpointType, DeviceService},
};

#[test]
fn run_device_list() {
    let devices = DeviceService::get_available_devices();

    for device in devices {
        println!("{:?}", device)
    }
}

#[test]
fn run_device_init() {
    let mut service = DeviceService::new();
    let device_desc = ("/dev/ttyACM0".to_string(), DeviceEndpointType::Serial);

    service.open_device(&device_desc).unwrap();
    service
        .write_device_command(&device_desc.0, state::GET_INFO_EXTENDED)
        .unwrap();
    service
        .write_device_command(&device_desc.0, settings::GET_ALL)
        .unwrap();
    service
        .write_device_command(&device_desc.0, settings::GET_DETAILS)
        .unwrap();
    service
        .write_device_command(&device_desc.0, settings::GET_GROUPS)
        .unwrap();
    service.write_device_command(&device_desc.0, SYNC).unwrap();

    thread::sleep(Duration::from_millis(2500));

    let info = service.get_device_info(&device_desc.0).unwrap();
    println!("{:#?}", info);
}
