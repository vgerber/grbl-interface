use std::result::Result;

/// Stores values from parsed help message "[HLP: ...\]"
pub struct HelpResponse {
    values: Vec<String>
}


impl HelpResponse {

    /// Reads message string and returns message value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value ["$$"" "$#" "$G" "$I" "$N"]
    /// let response = GCodeStateResponse::from("[HLP:$$ $# $G $I $N]");
    /// ```
    pub fn from(message: &str) -> Result<HelpResponse, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[HLP:<value> <value> ... <value>]"
        if HelpResponse::is_response(message) {
            // remove wrapper characters
            let message_end = message.len()-1;
            let message_payload = &message[5..message_end];

            // read all sub values in remaining message
            let message_values: Vec<String> = message_payload.split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
            
            return Ok(HelpResponse {
                values: message_values
            })    
        }
        Err(format!("Could not read help message \"{}\"", message))        
    }

    /// Indicates if message has required help outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with("[HLP:") && message.ends_with("]")
    }
    
    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}