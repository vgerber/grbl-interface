use std::result::Result;

const VERSION_PREFIX: &str = "[VER:";
const VERSION_SUFFIX: &str = "]";

/// Stores the version of the device firmware
#[derive(Clone)]
pub struct FirmwareVersion {
    version: String,
    name: String,
}

impl FirmwareVersion {

    /// Reads version and custom information
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores version "1.1" and name "grbl"
    /// let response = FirmwareVersion::from("[VER:1.1:grbl]");
    /// ```
    pub fn from(message: &str) -> Result<FirmwareVersion, String> {

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[Version:<value>]"
        if FirmwareVersion::is_response(&message) {
            let message_payload = message.strip_prefix(VERSION_PREFIX).unwrap().strip_suffix(VERSION_SUFFIX).unwrap();
            let version_segements: Vec<&str> = message_payload.split(":").collect();
            
            // expect <version>:<name>
            if version_segements.len() < 2 {
                return Err(format!("Invalid count of version strings \"{}\"", message_payload));    
            }

            // if name contains ":" join these sub strings
            let name = String::from(version_segements[1..].join(":"));
            let version = String::from(version_segements[0]);

            return Ok(FirmwareVersion {
                version,
                name,
            })    
        }
        Err(format!("Cannot read version \"{}\"", message))        
    }

    /// Indicates if message has required version outline
    pub fn is_response(message: &str) -> bool {
        message.starts_with(VERSION_PREFIX) && message.ends_with(VERSION_SUFFIX)
    }

    pub fn get_version_slice(message: &str) -> &str  {
        let start = VERSION_PREFIX.len();
        let end = message.len() - VERSION_SUFFIX.len();
        return &message[start..end];
    }
    
    pub fn version(&self) -> &String {
        &self.version
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}