use std::result::Result;

const MESSAGE_PREFIX: &str = "[MSG:";
const MESSAGE_SUFFIX: &str = "]";

#[derive(Clone, Debug)]
pub struct Message {
    message: String,
}

impl Message {

    /// Reads message string and returns message value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value "Hello"
    /// let response = Message::from("[MSG:Hello]");
    /// ```
    pub fn from(message: &str) -> Result<Message, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[MSG:<value>]"
        if Message::is_response(message) {
            let message_payload = message.strip_prefix(MESSAGE_PREFIX).unwrap().strip_suffix(MESSAGE_SUFFIX).unwrap();
            return Ok(Message {
                message: String::from(message_payload)
            })    
        }
        Err(format!("Cannot read message \"{}\"", message))        
    }

    /// Indicates if message has required message outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with(MESSAGE_PREFIX) && message.ends_with(MESSAGE_SUFFIX)
    }
    
    pub fn message(&self) -> &String {
        &self.message
    }
}