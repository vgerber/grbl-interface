use std::result::Result;

const STARTUP_PREFIX: &str = ">";

#[derive(Clone, Debug)]
pub enum StartupResult {
    Ok,
    Error(i32)
}

/// Stores the reponse from the firmware startup process
#[derive(Clone, Debug)]
pub struct FirmwareStartupResult {
    executed_line: String,
    result: StartupResult,
}

impl FirmwareStartupResult {

    /// Reads startup line reponse and status
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores executed line and status
    /// let response = FirmwareStartupResult::from(">G54G20:ok");
    /// ```
    pub fn from(message: &str) -> Result<FirmwareStartupResult, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // ">line:status:code"
        if FirmwareStartupResult::is_response(&message) {
            let message_payload = message.strip_prefix(STARTUP_PREFIX).unwrap();
            let segments: Vec<&str> = message_payload.split(":").collect();
            
            // expect <line>:<status>[:code]
            if segments.len() < 2 {
                return Err(format!("Invalid count of startup segments \"{}\"", message_payload));    
            }
            
            // read <:code> value
            let mut result_code = -1;
            if segments.len() >= 3 {
                result_code = match segments[2].parse() {
                    Ok(value) => value,
                    Err(_) => -1
                };
            }
            
            // read status message
            let executed_line = String::from(segments[0]);
            let result = match segments[1] {
                "ok" => StartupResult::Ok,
                "error" => StartupResult::Error(result_code),
                _ => return Err(format!("Invalid result \"{}\"", segments[1])),
            };

            return Ok(FirmwareStartupResult {
                executed_line,
                result,
            })
        }
        Err(format!("Cannot read startup \"{}\"", message))        
    }

    /// Indicates if message has required startup prefix
    pub fn is_response(message: &str) -> bool {
        message.starts_with(">")
    }
    
    pub fn executed_line(&self) -> &String {
        &self.executed_line
    }

    pub fn result(&self) -> &StartupResult {
        &self.result
    }
}