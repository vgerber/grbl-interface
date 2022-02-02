use std::result::Result;


pub struct VersionResponse {
    version: String,
    name: String,
}

impl VersionResponse {

    /// Reads version and custom information
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores version "1.1" and name "grbl"
    /// let response = VersionResponse::from("[VER:1.1:grbl]");
    /// ```
    pub fn from(message: &str) -> Result<VersionResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[Version:<value>]"
        if VersionResponse::is_response(&trimmed_message) {
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[5..message_end];
            let version_segements: Vec<&str> = message_payload.split(":").collect();
            
            // expect <version>:<name>
            if version_segements.len() < 2 {
                return Err(format!("Invalid count of version strings: \"{}\"", message_payload));    
            }

            // if name contains ":" join these sub strings
            let name = String::from(version_segements[1..].join(":"));
            let version = String::from(version_segements[0]);

            return Ok(VersionResponse {
                version,
                name,
            })    
        }
        Err(format!("Could not read version: {}", message))        
    }

    /// Indicates if message has required version outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with("[VER:") && message.ends_with("]")
    }
    
    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}