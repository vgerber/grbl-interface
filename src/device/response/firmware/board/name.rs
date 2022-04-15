use crate::device::response::parser::string_parser::parse_string_property;


const BOARD_NAME_PREFIX: &str = "[BOARD:";
const BOARD_NAME_SUFFIX: &str = "]";


/// Reads the board name line
/// 
/// # Error
/// Returns an error when parsing fails
/// 
/// # Examples
/// Basic usage:
/// ```
/// use grbli::device::response::firmware::board::name::*;
/// 
/// let board_name = parse_board_name("[BOARD:T41U5XBB]").unwrap();
/// assert_eq!(board_name, "T41U5XBB")
/// ```
pub fn parse_board_name(message: &str) -> Result<String, String> {
    parse_string_property(message, BOARD_NAME_PREFIX, BOARD_NAME_SUFFIX, "board name")
}

/// Indicates if the message can be parsed as board name
pub fn is_board_name_response(message: &str) -> bool {
    message.starts_with(BOARD_NAME_PREFIX) && message.ends_with(BOARD_NAME_SUFFIX)
}