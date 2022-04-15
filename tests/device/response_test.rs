use grbli::device::{response::{read_response}, DeviceInfo};



#[test]
pub fn read_stores_driver_name() {
    let mut device_info = DeviceInfo::from("test").unwrap();
    read_response("[DRIVER:driver_test]", &mut device_info).unwrap();
    assert_eq!(device_info.firmware_info().driver_info().name().unwrap().as_str(), "driver_test")
}

#[test]
pub fn read_stores_board_name() {
    let mut device_info = DeviceInfo::from("test").unwrap();
    read_response("[BOARD:board_test]", &mut device_info).unwrap();
    assert_eq!(device_info.firmware_info().board_info().name().unwrap().as_str(), "board_test")
}

#[test]
pub fn read_stores_setting() {
    let mut device_info = DeviceInfo::from("test").unwrap();
    read_response("$13=abc", &mut device_info).unwrap();
    assert_eq!(device_info.settings().get_setting(&13).unwrap().value(), "abc")
}