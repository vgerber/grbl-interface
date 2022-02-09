use std::result::Result;

pub enum StartupResult {
    Ok,
    Error(i32)
}

pub struct StartupResponse {
    executed_line: String,
    result: StartupResult,
}

impl StartupResponse {

    /// Reads startup line reponse and status
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores executed line and status
    /// let response = StartupResponse::from(">G54G20:ok");
    /// ```
    pub fn from(message: &str) -> Result<StartupResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // ">line:status:code"
        if StartupResponse::is_response(&trimmed_message) {
            let message_payload = &trimmed_message[1..];
            let segments: Vec<&str> = message_payload.split(":").collect();
            
            // expect <line>:<status>[:code]
            if segments.len() < 2 {
                return Err(format!("Invalid count of startup segments \"{}\"", message_payload));    
            }
            
            // reead <:code> value
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

            return Ok(StartupResponse {
                executed_line,
                result,
            })
        }
        Err(format!("Could not read startup \"{}\"", message))        
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