use crate::device::response::parser::string_parser::parse_string_property;


const DRIVER_OPTIONS_PREFIX: &str = "[DRIVER OPTIONS:";
const DRIVER_OPTIONS_SUFFIX: &str = "]";

/// Reads the driver options line
/// 
/// # Error
/// Returns an error when parsing fails
/// 
/// # Examples
/// Basic usage:
/// ```
/// use grbli::device::response::firmware::driver::option::parse_driver_options;
/// let options = parse_driver_options("[DRIVER OPTIONS:USB.2,Explode]").unwrap();
/// assert_eq!(options, vec!["USB.2", "Explode"])
/// ```
pub fn parse_driver_options(message: &str) -> Result<Vec<String>, String> {
    match parse_string_property(message, DRIVER_OPTIONS_PREFIX, DRIVER_OPTIONS_SUFFIX, "driver options") {
        // split csv string -> remove empty strings -> take ownership
        Ok(options) => Ok(options.split(",").filter(|opt| opt.len() > 0).map(|opt| opt.to_string()).collect()),
        Err(err) => Err(err)
    }
}

pub fn is_driver_options_response(message: &str) -> bool {
    message.starts_with(DRIVER_OPTIONS_PREFIX) && message.ends_with(DRIVER_OPTIONS_SUFFIX)
}