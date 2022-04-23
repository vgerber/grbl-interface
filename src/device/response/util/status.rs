use std::result::Result;

/// Reads startup line response and status
/// 
/// # Examples
/// Basic usage:
/// ```
/// use grbli::device::response::util::status::*;
/// 
/// // stores executed line and status
/// let response = parse_response_status("ok").unwrap();
/// assert!(response.is_ok());
/// 
/// let response = parse_response_status("error:2").unwrap();
/// assert_eq!(response.err().unwrap(), 2)
/// ```
pub fn parse_response_status(message: &str) -> Result<Result<(), i32>, String> {

    // check if message has the correct syntax
    // and return the unwrapped value
    // "status[:code]"
    if is_response_status(&message) {
        let segments: Vec<&str> = message.split(":").collect();
        
        // expect {<status> [,code]}
        if segments.is_empty() {
            return Err(format!("Invalid count response status segments: \"{}\"", message));    
        }

        let status_type = segments[0];

        if status_type == "ok" {
            return Ok(Ok(()))
        } else if status_type == "error" && segments.len() >= 2 {
            // parse the error code
            let result_code = match segments[1].parse::<i32>() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot parse response status status code: \"{}\"", segments[1])),
            };
            return Ok(Err(result_code))
        }
    }
    Err(format!("Cannot read response status \"{}\"", message))        
}

/// Indicates if message has required startup prefix
pub fn is_response_status(message: &str) -> bool {
    message == "ok" || message.starts_with("error")
}