const INPUT_WAIT_RESULT_PREFIX: &str = "In:";


/// Returns status of input wait e.g. "In:1"
/// 
/// # Examples
/// ```
/// // Input wait has failed
/// let input_wait_result = parse_input_wait_result_succeded("In:-1");
/// 
/// // Input wait has succeeded
/// let input_wait_result = parse_input_wait_result_succeded("In:0");
/// let input_wait_result = parse_input_wait_result_succeded("In:1");
/// ```
pub fn parse_input_wait_result(message: &str) -> Result<bool, String> {
    if is_input_wait_result(message) {
        let input_wait_result_message: &str = &message[INPUT_WAIT_RESULT_PREFIX.len()..];
        let mut input_wait_result: i8 = match input_wait_result_message.parse() {
            Ok(value) => value,
            Err(_) => return Err(format!("Cannot read input wait result \"{}\"", input_wait_result_message))
        };

        input_wait_result = match input_wait_result {
            -1..=1 => input_wait_result,
            _ => return Err(format!("Cannot interpret input wait result \"{}\"", input_wait_result)),
        };

        return Ok(input_wait_result == 0 || input_wait_result == 1);
    }



    Err(format!("Cannot read input wait result \"{}\"", message))
}

/// Indicates if message has input wait result syntax
pub fn is_input_wait_result(message: &str) -> bool {
    message.starts_with(INPUT_WAIT_RESULT_PREFIX)
}