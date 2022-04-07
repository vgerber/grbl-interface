use std::time::Duration;

pub mod serial;

/// Endpoint for device communication
pub trait Endpoint {

    /// Opens the connection to the device
    /// 
    /// Returns an error if open failed
    fn open(&mut self) -> Result<(), String>;

    /// Closes the connection to the device
    /// 
    /// Returns an error if close failed
    fn close(&mut self) -> Result<(), String>;

    /// Writes the message to the device stream
    /// 
    /// Returns an error if write fails
    fn write(&mut self, message: &str) -> Result<(), String>;

    /// Returns all received messages since last read call
    /// 
    /// Timeout stops read if message stream is continuous
    /// 
    /// Returns all messages in message buffer or until timeout triggered
    fn read_new_messages(&self, timeout: Duration) -> Vec<String>;
}