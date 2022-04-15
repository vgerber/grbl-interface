use crate::device::response::parser::string_parser::parse_string_property;


const DRIVER_VERSION_PREFIX: &str = "[DRIVER VERSION:";
const DRIVER_VERSION_SUFFIX: &str = "]";


/// Reads the driver version line
/// 
/// # Error
/// Returns an error when parsing fails
/// 
/// # Examples
/// Basic usage:
/// ```
/// use grbli::device::response::firmware::driver::version::*;
/// let driver_version = parse_driver_version("[DRIVER VERSION:210725]").unwrap();
/// assert_eq!(driver_version, "210725")
/// ```
pub fn parse_driver_version(message: &str) -> Result<String, String> {
    parse_string_property(message, DRIVER_VERSION_PREFIX, DRIVER_VERSION_SUFFIX, "driver version")
}

/// Indicates if the message can be parsed as driver name
pub fn is_driver_version_response(message: &str) -> bool {
    message.starts_with(DRIVER_VERSION_PREFIX) && message.ends_with(DRIVER_VERSION_SUFFIX)
}