
const LINE_NUMBER_PREFIX: &str = "Ln:";


/// Returns the line number from message "Ln:\<number\>"
/// 
/// # Examples
/// ```
/// let line_number = parse_line_number("Ln:32");
/// assert_eq!(32, line_number)
/// ```
pub fn parse_line_number(message: &str) -> Result<i32, String> {
    if is_line_number(message) {
        let line_number_str = &message[LINE_NUMBER_PREFIX.len()..];
        return match line_number_str.parse() {
            Ok(line_number) => Ok(line_number),
            Err(_) => Err(format!("Cannot read line number \"{}\"", line_number_str))
        };
    }
    Err(format!("Cannot read line number \"{}\"", message))
}

/// Indicates if message starts with "Ln:"
pub fn is_line_number(message: &str) -> bool {
    message.starts_with(LINE_NUMBER_PREFIX)
}