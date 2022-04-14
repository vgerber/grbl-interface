use std::{thread, time::Duration};

use grbli::{service::device_service::{DeviceService, DeviceEndpointType}, device::command::{state::*, general::SYNC}};



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
    service.write_device_command(&device_desc.0, format!("{}\n", GET_INFO_EXTENDED).as_str()).unwrap();
    service.write_device_command(&device_desc.0, format!("{}\n", SYNC).as_str()).unwrap();

    thread::sleep(Duration::from_millis(100));

    let info =  service.get_device_info(&device_desc.0).unwrap();
    println!("{:?}", info.firmware_info().compile_options().unwrap().options())
}