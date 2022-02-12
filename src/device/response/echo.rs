use std::result::Result;

const ECHO_PREFIX: &str = "[echo:";
const ECHO_SUFFIX: &str = "]";

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
            let message_payload = message.strip_prefix(ECHO_PREFIX).unwrap().strip_suffix(ECHO_SUFFIX).unwrap();
            return Ok(EchoResponse {
                echo: String::from(message_payload)
            })    
        }

        Err(format!("Cannot read echo \"{}\"", message))        
    }

    /// Indicates if message has required echo outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with(ECHO_PREFIX) && message.ends_with(ECHO_SUFFIX)
    }
    
    pub fn echo(&self) -> &String {
        &self.echo
    }
}