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
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[MSG:<value>]"
        if trimmed_message.starts_with("[MSG:") && trimmed_message.ends_with("]") {
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[5..message_end];
            return Ok(MessageResponse {
                message: String::from(message_payload)
            })    
        }

        let mut error_str = String::from("Could not read message: ");
        error_str.push_str(&trimmed_message);
        Err(error_str)        
    }
    
    pub fn get_message(&self) -> &String {
        &self.message
    }
}