use std::{collections::HashMap, sync::{Arc, Mutex, mpsc::{Sender, self, Receiver}}, thread::{JoinHandle, self}, time::Duration};

use log::error;

use crate::{device::{DeviceInfo, response::read_response}, endpoint::{serial::SerialEndpoint, Endpoint}};

type DeviceDescription = (String, DeviceEndpointType);

#[derive(Clone, Copy, Debug)]
pub enum DeviceEndpointType {
    Serial,
    Ethernet
}

struct DeviceHandle {
    device_id: String,
    device_info: Arc<Mutex<DeviceInfo>>,
    read_thread: Option<JoinHandle<()>>,
    tx_close: Option<Sender<bool>>,
    tx_write: Option<Sender<String>>,
}

pub struct DeviceService {
    device_handles: HashMap<String, DeviceHandle>,
}

impl DeviceHandle {
    
    pub fn open(device_description: &DeviceDescription) -> Result<DeviceHandle, String> {
        let device_info = match DeviceInfo::from(device_description.0.as_str()) {
            Ok(info) => info,
            Err(err) => return Err(format!("Could not create device \"{}\" info: {}", device_description.0, err).to_string())
        };

        let (tx_close, rx_close) = mpsc::channel();
        let (tx_write, rx_write): (Sender<String>, Receiver<String>) = mpsc::channel();


        let device_info = Arc::new(Mutex::new(device_info));
        let thread_device_info = Arc::clone(&device_info);
        let thread_device_desc = device_description.clone();
        let read_thread = Some(thread::spawn(move || {
            let device_info = thread_device_info;
            let device_description = thread_device_desc;
            
            // create and open endpoint
            let mut endpoint = DeviceHandle::get_endpoint(&device_description);
             
            if let Err(_) = endpoint.open() {
                error!("Failed to open endpoint {}", device_description.0);
                return;
            }

            // run until close signal received
            // check close -> read serial -> write serial -> check close -> ...
            loop {

                // check for close signal
                if let Ok(close) = rx_close.recv_timeout(Duration::from_millis(1)) {
                    if close {
                        return;
                    }
                } 

                // read all new messages and update the device info
                let new_messages = endpoint.read_new_messages(Duration::from_millis(1));
                if !new_messages.is_empty() {
                    let mut current_device_info = device_info.lock().unwrap();
                    for message in new_messages {
                        if let Err(err) = read_response(message, &mut current_device_info) {
                            error!("{}: {}", device_description.0, err)
                        }
                    }
                }
                
                // read the next command and write it to the device endpoint 
                if let Ok(msg) = rx_write.recv_timeout(Duration::from_millis(1)) {
                    println!("Write: {}", &msg);
                    if let Err(err) = endpoint.write(msg.as_str()) {
                        error!("{}: {}", device_description.0, err)
                    }
                }

                // let the serial endpoint breath
                thread::sleep(Duration::from_millis(10));
            }
        }));
        Ok(DeviceHandle {
            device_id: device_description.0.clone(),
            device_info: Arc::clone(&device_info),
            read_thread,
            tx_close: Some(tx_close),
            tx_write: Some(tx_write),
        })
    }

    fn get_endpoint(device: &DeviceDescription) -> Box<dyn Endpoint> {
        match device.1 {
            DeviceEndpointType::Serial => Box::new(SerialEndpoint::from(device.0.as_str(), 115200)),
            DeviceEndpointType::Ethernet => unimplemented!("Ethernet is not implemented")
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        match self.read_thread.take() {
            Some(thread) => {
                if let Err(_) = self.tx_close.take().unwrap().send(true) {
                    panic!("Cannot close connection for \"{}\"", self.device_id)
                }
                thread.join().unwrap();
            }
            None => panic!("Connection to \"{}\" already closed", self.device_id)
        }
        Ok(())
    }

    pub fn write(&self, command: &str) -> Result<(), String> {
        match &self.tx_write {
            Some(tx) => {
                match tx.send(command.to_string()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Cannot write command to \"{}\"", self.device_id).to_string())
                }
            }
            None => panic!("Write channel to \"{}\" closed", self.device_id)
        }
    }
    
    pub fn get_device_info(&self) -> DeviceInfo {
        self.device_info.lock().unwrap().clone()
    }
}

impl DeviceService { 
    
    pub fn new() -> DeviceService {
        DeviceService { device_handles: HashMap::new() }
    }

    pub fn get_available_devices() -> Vec<DeviceDescription> {
        SerialEndpoint::find_serial_ports().iter().map(|p| (p.port_name.clone(), DeviceEndpointType::Serial)).collect()
    }

    pub fn open_device(&mut self, device: &DeviceDescription) -> Result<(), String> {
        if self.device_handles.contains_key(&device.0) {
            return Err(format!("Device \"{}\" is already connected", device.0).to_string());
        }
        
        let device_id = device.0.clone();
        match DeviceHandle::open(device) {
            Ok(handle) => {
                self.device_handles.insert(device_id, handle);
                Ok(())
            }
            Err(err) => Err(format!("Could not create device \"{}\": {}", device_id, err).to_string()),
        }
    }

    pub fn close_device(&mut self, device_id: &String) -> Result<(), String> {
        match self.device_handles.get_mut(device_id) {
            Some(handle) => {                
                let close_result = handle.close();
                if let None = self.device_handles.remove(device_id) {
                    panic!("Could not remove device {}", device_id)
                }
                close_result
            }
            None => Err(format!("Device \"{}\" not found", device_id)),
        }
    }

    pub fn is_device_connected(&self, device_id: &String) -> bool {
        self.device_handles.contains_key(device_id)
    }

    pub fn get_device_info(&self, device_id : &String) -> Result<DeviceInfo, String> {
        match self.device_handles.get(device_id) {
            Some(handler) => Ok(handler.get_device_info()),
            None => Err("Device not found".to_string()),
        }     
    }

    pub fn write_device_command(&mut self, device_id : &String, command: &str) -> Result<(), String> {
        match self.device_handles.get(device_id) {
            Some(handle) => handle.write(command),
            None => Err("Device not found".to_string()),
        }        
    }
}

impl Drop for DeviceService {
    
    fn drop(&mut self) {
        let ids: Vec<String> =  self.device_handles.iter().map(|x| { x.0.clone() }).collect();
    
        for id in ids {
            if let Err(_) = self.close_device(&id) {
                panic!("Cannot close devicd \"{}\"", id)
            }
        }
    }
}