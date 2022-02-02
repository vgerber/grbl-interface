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

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[echo:<value>]"
        if EchoResponse::is_response(message) {
            let message_end = message.len()-1;
            let message_payload = &message[6..message_end];
            return Ok(EchoResponse {
                echo: String::from(message_payload)
            })    
        }

        Err(format!("Cannot read echo \"{}\"", message))        
    }

    /// Indicates if message has required echo outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with("[echo:") && message.ends_with("]")
    }
    
    pub fn echo(&self) -> &String {
        &self.echo
    }
}