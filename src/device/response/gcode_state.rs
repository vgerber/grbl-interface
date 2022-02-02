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

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[GC:<value> <value> ... <value>]"
        if GCodeStateResponse::is_response(message) {
            // remove wrapper characters
            let message_end = message.len()-1;
            let message_payload = &message[4..message_end];

            // read all sub values in remaining message
            let message_values: Vec<String> = message_payload.split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
            
            return Ok(GCodeStateResponse {
                values: message_values
            })    
        }
        Err(format!("Could not read gcode state message \"{}\"", message))        
    }

    /// Indicates if message has required gcode prefix
    pub fn is_response(message: &str) -> bool {
        message.starts_with("[GC:") && message.ends_with("]")
    }
    
    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}