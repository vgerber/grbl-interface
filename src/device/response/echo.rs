use std::result::Result;

/// Parses an echo response \[echo:<message>\].
pub struct EchoResponse {
    echo: String,
}

impl EchoResponse {

    /// Reads echo string and returns echo value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value "Hello"
    /// let response = MessageResponse::from("[echo:Hello]");
    /// ```
    pub fn from(message: &str) -> Result<EchoResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[echo:<value>]"
        if trimmed_message.starts_with("[echo:") && trimmed_message.ends_with("]") {
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[6..message_end];
            return Ok(EchoResponse {
                echo: String::from(message_payload)
            })    
        }

        Err(format!("Cannot read echo \"{}\"", trimmed_message))        
    }
    
    pub fn echo(&self) -> &String {
        &self.echo
    }
}