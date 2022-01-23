use std::result::Result;

pub struct GCodeStateResponse {
    values: Vec<String>
}

impl GCodeStateResponse {

    /// Reads message string and returns message value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value ["G0", "G54", "G17", "G21"]
    /// let response = GCodeStateResponse::from("[GC:G0 G54 G17 G21]");
    /// ```
    pub fn from(message: &str) -> Result<GCodeStateResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[GC:<value> <value> ... <value>]"
        if trimmed_message.starts_with("[GC:") && trimmed_message.ends_with("]") {
            // remove wrapper characters
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[4..message_end];

            // read all sub values in remaining message
            let message_values: Vec<String> = message_payload.split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
            
            return Ok(GCodeStateResponse {
                values: message_values
            })    
        }

        let mut error_str = String::from("Could not read message: ");
        error_str.push_str(&trimmed_message);
        Err(error_str)        
    }
    
    pub fn get_values(&self) -> &Vec<String> {
        &self.values
    }
}