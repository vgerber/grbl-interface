use crate::device::response::parser::string_parser::parse_string_property;


const DRIVER_NAME_PREFIX: &str = "[DRIVER:";
const DRIVER_NAME_SUFFIX: &str = "]";


/// Reads the driver name line
/// 
/// # Error
/// Returns an error when parsing fails
/// 
/// # Examples
/// Basic usage:
/// ```
/// use grbli::device::response::firmware::driver::name::*;
/// let driver_name = parse_driver_name("[DRIVER:iMXRT1062]").unwrap();
/// assert_eq!(driver_name, "iMXRT1062")
/// ```
pub fn parse_driver_name(message: &str) -> Result<String, String> {
    parse_string_property(message, DRIVER_NAME_PREFIX, DRIVER_NAME_SUFFIX, "driver name")
}

/// Indicates if the message can be parsed as driver name
pub fn is_driver_name_response(message: &str) -> bool {
    message.starts_with(DRIVER_NAME_PREFIX) && message.ends_with(DRIVER_NAME_SUFFIX)
}