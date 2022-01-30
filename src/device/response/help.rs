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
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[HLP:<value> <value> ... <value>]"
        if trimmed_message.starts_with("[HLP:") && trimmed_message.ends_with("]") {
            // remove wrapper characters
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[5..message_end];

            // read all sub values in remaining message
            let message_values: Vec<String> = message_payload.split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
            
            return Ok(HelpResponse {
                values: message_values
            })    
        }

        let mut error_str = String::from("Could not read message: ");
        error_str.push_str(&trimmed_message);
        Err(error_str)        
    }
    
    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}