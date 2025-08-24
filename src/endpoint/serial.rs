use std::{
    borrow::Borrow,
    panic,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use serialport::{SerialPort, SerialPortInfo, SerialPortType};

use crate::device::command::general;

use super::Endpoint;

/// Endpoint for serial connections
///
/// ```
/// // get all available ports
/// use grbli::endpoint::*;
/// use grbli::endpoint::serial::*;
/// use std::time::Duration;
///
/// let all_ports = SerialEndpoint::find_serial_ports();
///
/// // open connection
/// let mut endpoint = SerialEndpoint::from(&all_ports[0].port_name, 115200);
/// endpoint.open().unwrap();
///
/// // send first message
/// // and read responses
/// endpoint.write("$I\n").unwrap();
/// let response = endpoint.read_new_messages(Duration::from_millis(100));
///
/// // close connection after
/// endpoint.close();
/// ```
pub struct SerialEndpoint {
    // configuration of port
    port_name: String,
    baud_rate: u32,

    // handler for buffer read messages
    channel_read: (Sender<String>, Receiver<String>),
    channel_error: (Sender<String>, Receiver<String>),
    tx_write: Option<Sender<String>>,
    tx_close: Option<Sender<bool>>,

    // handle for active connection
    serial_thread: Option<JoinHandle<()>>,
}

impl SerialEndpoint {
    /// Returns list of all usb devices
    pub fn find_serial_ports() -> Vec<SerialPortInfo> {
        match serialport::available_ports() {
            Ok(serial_ports) => serial_ports
                .into_iter()
                .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_)))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Creates a new serial endpoint configuration
    pub fn from(port_name: &str, baud_rate: u32) -> SerialEndpoint {
        SerialEndpoint {
            port_name: port_name.to_string(),
            baud_rate: baud_rate,
            channel_read: mpsc::channel(),
            channel_error: mpsc::channel(),
            tx_write: None,
            tx_close: None,
            serial_thread: None,
        }
    }

    /// Starts a new thread with configured serial connection
    ///
    /// Connects all channels to thread and sets the join handle for this thread
    fn open_serial_port(&mut self) {
        // create channels for all data streams

        // channel for sending serial close signal
        let channel_close = mpsc::channel();
        self.tx_close = Some(channel_close.0);

        // channel for sending new commands
        let channel_write = mpsc::channel();
        self.tx_write = Some(channel_write.0);

        // channel for sending received commands
        let tx_read_ref = self.channel_read.0.clone();

        // channel for sending serial errors
        let tx_error_ref = self.channel_error.0.clone();

        // params of serial port
        // e.g /dev/ttyGRBL - 115200
        let port_name = self.port_name.clone();
        let baud_rate = self.baud_rate;

        self.serial_thread = Some(thread::spawn(move || {
            // open a new serial port
            // stop process if serial port could not be opened
            let mut serial_port = match serialport::new(port_name.clone(), baud_rate)
                .timeout(Duration::from_millis(1))
                .open()
            {
                Ok(serial_port) => serial_port,
                Err(_) => {
                    if let Err(_) = tx_error_ref.send(String::from(format!(
                        "Cannot open serial port {}",
                        port_name
                    ))) {
                        return;
                    }
                    return;
                }
            };

            if let Err(_) = serial_port.set_timeout(Duration::from_millis(10)) {
                let _ =
                    tx_error_ref.send(String::from(format!("Cannot set timeout {}", port_name)));
            }

            // set serial port config to 8N1
            if let Err(_) = serial_port.set_data_bits(serialport::DataBits::Eight) {
                let _ = tx_error_ref.send(String::from(format!(
                    "Cannot set data bits to 8 {}",
                    port_name
                )));
            }

            if let Err(_) = serial_port.set_stop_bits(serialport::StopBits::One) {
                let _ = tx_error_ref.send(String::from(format!(
                    "Cannot set stop bits to 1 {}",
                    port_name
                )));
                return;
            }

            if let Err(_) = serial_port.set_parity(serialport::Parity::None) {
                let _ = tx_error_ref.send(String::from(format!(
                    "Cannot set parity to None {}",
                    port_name
                )));
                return;
            }

            // start up sequence
            SerialEndpoint::send_grbl_startup(serial_port.try_clone().unwrap());

            // buffer for incomplete messages read from device
            let mut message_buffer = String::from("");

            loop {
                // read new serial data
                SerialEndpoint::read_buffer(
                    serial_port.try_clone().unwrap(),
                    &mut message_buffer,
                    tx_read_ref.borrow(),
                );

                // write pending serial data
                if let Err(err) = SerialEndpoint::write_buffer(
                    serial_port.try_clone().unwrap(),
                    channel_write.1.borrow(),
                ) {
                    if let Err(_) = tx_error_ref.send(err.clone()) {
                        return;
                    }
                }

                // check if serial port should be closed
                if let Ok(true) = channel_close.1.recv_timeout(Duration::from_millis(1)) {
                    break;
                }
            }
        }));
    }

    fn send_grbl_startup(mut serial_port: Box<dyn SerialPort>) {
        serial_port.write(general::SYNC.as_bytes()).unwrap();
        serial_port.write(general::SYNC.as_bytes()).unwrap();
        thread::sleep(Duration::from_secs(2));
        serial_port.flush().unwrap();
    }

    /// Reads from the serial device buffer and sends a new line to tx_read
    ///
    /// The count of new messages is send via tx_read_counter
    fn read_buffer(
        mut serial_port: Box<dyn SerialPort>,
        message_buffer: &mut String,
        tx_read: &Sender<String>,
    ) {
        // read new data from buffer and append to old message buffer
        // result is always an error because the message is encoded as ascii and not utf-8
        if let Ok(bytes) = serial_port.bytes_to_read() {
            let mut byte_buffer: Vec<u8> = vec![0; bytes as usize];
            let _ = serial_port.read(&mut byte_buffer);

            match String::from_utf8(byte_buffer) {
                Ok(s) => message_buffer.push_str(s.as_str()),
                Err(_) => {}
            }
        }

        // end of line marker
        let eol = "\n";

        // iterate over previous and newly received data
        // until no more new line characters are found
        // i.e. no more complete message found
        let mut data_find_index = message_buffer.find(eol);
        while data_find_index.is_some() {
            let new_line_index = data_find_index.unwrap();

            // push found line segment
            tx_read
                .send(message_buffer[..new_line_index - 1].to_string())
                .unwrap();

            // move cursor to new line
            message_buffer.replace_range(
                ..,
                &message_buffer[new_line_index + 1..].to_string().as_str(),
            );
            data_find_index = message_buffer.find(eol);
        }
    }

    /// Reads a message from rx_write and sends the content to the serial port
    ///
    /// Returns
    /// * True/False indicates if write was successful
    /// * An error if reading from rx failed
    fn write_buffer(
        mut serial_port: Box<dyn SerialPort>,
        rx_write: &Receiver<String>,
    ) -> Result<bool, String> {
        if let Ok(message) = rx_write.recv_timeout(Duration::from_millis(1)) {
            if let Err(_) = serial_port.write(message.as_bytes()) {
                return Err(format!(
                    "Unable to send message \"{}\" to {}",
                    message,
                    serial_port.name().unwrap_or(String::from("Unknown"))
                ));
            }
            if let Err(_) = serial_port.flush() {
                return Err(format!(
                    "Unable to flush message \"{}\" to {}",
                    message,
                    serial_port.name().unwrap_or(String::from("Unknown"))
                ));
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Writes a sync command
    ///
    /// Is required for issuing simulator commands
    pub fn write_sync(&mut self) -> Result<(), String> {
        self.write(general::SYNC)
    }
}

impl Endpoint for SerialEndpoint {
    fn write(&mut self, message: &str) -> Result<(), String> {
        // send message to device thread
        if let Some(tx) = self.tx_write.clone() {
            match tx.send(message.to_string()) {
                Ok(_) => Ok(()),
                Err(_) => Err(String::from(format!(
                    "Unable to send \"{}\" to {}",
                    message, self.port_name
                ))),
            }
        } else {
            return Err("Serial connection is not open!".to_string());
        }
    }

    /// Opens the serial connection and starts a new thread
    fn open(&mut self) -> Result<(), String> {
        // prevent opening a connection multiple times
        if let Some(_) = self.serial_thread {
            return Err("Serial device is already listening!".to_string());
        }

        self.open_serial_port();
        Ok(())
    }

    /// Closes existing serial connection
    fn close(&mut self) -> Result<(), String> {
        // reset all channels to serial thread
        // and wait until thread stops
        if let Some(tx) = &self.tx_close {
            match tx.send(true) {
                Ok(_) => {
                    self.tx_write = None;
                    self.tx_close = None;
                    self.serial_thread.take().map(JoinHandle::join);
                    Ok(())
                }
                Err(_) => Err(String::from(format!("Could not close {}", self.port_name))),
            }
        } else {
            Err(String::from(format!(
                "{} is already closed",
                self.port_name
            )))
        }
    }

    fn read_new_messages(&self, timeout: Duration) -> Vec<String> {
        let mut buffered_messages = Vec::new();

        // start timer for timeout check
        let read_start = Instant::now();

        // read one message in each step
        loop {
            if let Ok(message) = self.channel_read.1.recv_timeout(Duration::from_millis(1)) {
                buffered_messages.push(message);
            }

            // stop reading when timeout is reached
            if read_start.elapsed() >= timeout {
                break;
            }
        }

        return buffered_messages;
    }
}

impl Drop for SerialEndpoint {
    fn drop(&mut self) {
        // if serial thread is still active => serial connection still open
        // close the connection
        if let Some(_) = self.serial_thread {
            if let Err(_) = self.close() {
                panic!("Could not close serial endpoint");
            }
        }
    }
}
