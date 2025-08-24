use std::time::Duration;

use grbli::{
    device::command::{realtime, settings, state},
    endpoint::{serial::SerialEndpoint, Endpoint},
};
use serialport::SerialPortType;

fn get_port_name(port_type: SerialPortType) -> String {
    match port_type {
        SerialPortType::UsbPort(info) => {
            String::from(format!("USB {}", info.manufacturer.unwrap_or_default()))
        }
        SerialPortType::PciPort => String::from("PCI"),
        SerialPortType::BluetoothPort => String::from("Bluetooth"),
        SerialPortType::Unknown => String::from("Unknown"),
    }
}

#[test]
fn search_for_serial_devices() {
    let ports = SerialEndpoint::find_serial_ports();
    if ports.is_empty() {
        println!("No usb ports found");
    }

    for port in ports {
        println!("{} {}", port.port_name, get_port_name(port.port_type));
    }
}

#[test]
fn open_test_device() {
    let mut port = SerialEndpoint::from("/dev/ttyACM0", 115_200);

    /*
    port.open().unwrap();
    println!("Opend!");

    port.write(format!("?").as_str()).unwrap();
    port.write(format!("?").as_str()).unwrap();
    port.write(format!("?").as_str()).unwrap();
    port.write(format!("?").as_str()).unwrap();
    port.write(format!("?").as_str()).unwrap();

    thread::sleep(Duration::from_millis(100));

    for message in port.read_new_messages(Duration::from_millis(100)) {
        println!("Received {}", message);
    }

    port.close().unwrap();
    */
    port.open().unwrap();

    println!("Opend!");
    println!("######################");
    println!("######################");

    port.write(state::GET_INFO).unwrap();
    port.write(state::GET_INFO_EXTENDED).unwrap();
    port.write(settings::GET_DETAILS).unwrap();
    port.write(settings::GET_GROUPS).unwrap();
    port.write(realtime::STATUS_REPORT).unwrap();
    port.write(format!("$\r").as_str()).unwrap();

    for message in port.read_new_messages(Duration::from_millis(2500)) {
        println!("Received {}", message);
    }
}
