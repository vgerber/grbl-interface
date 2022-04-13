use std::result::Result;

const ECHO_PREFIX: &str = "[echo:";
const ECHO_SUFFIX: &str = "]";

/// Parses an echo response \[echo:<message>\].
#[derive(Clone)]
pub struct EchoMessage {
    echo: String,
}

impl EchoMessage {

    /// Reads echo string and returns echo value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value "Hello"
    /// let response = EchoMessage::from("[echo:Hello]");
    /// ```
    pub fn from(message: &str) -> Result<EchoMessage, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[echo:<value>]"
        if EchoMessage::is_response(message) {
            let message_payload = message.strip_prefix(ECHO_PREFIX).unwrap().strip_suffix(ECHO_SUFFIX).unwrap();
            return Ok(EchoMessage {
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