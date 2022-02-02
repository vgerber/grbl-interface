use std::result::Result;


pub struct MessageResponse {
    message: String,
}

impl MessageResponse {

    /// Reads message string and returns message value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value "Hello"
    /// let response = MessageResponse::from("[MSG:Hello]");
    /// ```
    pub fn from(message: &str) -> Result<MessageResponse, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[MSG:<value>]"
        if MessageResponse::is_response(message) {
            let message_end = message.len()-1;
            let message_payload = &message[5..message_end];
            return Ok(MessageResponse {
                message: String::from(message_payload)
            })    
        }
        Err(format!("Could not read message \"{}\"", message))        
    }

    /// Indicates if message has required message outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with("[MSG:") && message.ends_with("]")
    }
    
    pub fn message(&self) -> &String {
        &self.message
    }
}