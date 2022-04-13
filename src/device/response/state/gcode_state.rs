use std::result::Result;

const GCODE_PREFIX: &str = "[GC:";
const GCODE_SUFFIX: &str = "]";

#[derive(Clone)]
pub struct GCodeState {
    values: Vec<String>
}

impl GCodeState {

    /// Reads message string and returns message value
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores value ["G0", "G54", "G17", "G21"]
    /// let response = GCodeState::from("[GC:G0 G54 G17 G21]");
    /// ```
    pub fn from(message: &str) -> Result<GCodeState, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[GC:<value> <value> ... <value>]"
        if GCodeState::is_response(message) {
            // remove wrapper characters
            let message_payload = message.strip_prefix(GCODE_PREFIX).unwrap().strip_suffix(GCODE_SUFFIX).unwrap();

            // read all sub values in remaining message
            let message_values: Vec<String> = message_payload.split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
            
            return Ok(GCodeState {
                values: message_values
            })    
        }
        Err(format!("Cannot read gcode state message \"{}\"", message))        
    }

    /// Indicates if message has required gcode prefix
    pub fn is_response(message: &str) -> bool {
        message.starts_with(GCODE_PREFIX) && message.ends_with(GCODE_SUFFIX)
    }
    
    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}