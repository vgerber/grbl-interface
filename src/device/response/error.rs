use std::collections::HashMap;

const ERROR_CODE_PREFIX: &str = "[ERRORCODE:";
const ERROR_CODE_SUFFIX: &str = "]";
const ALARM_CODE_PREFIX: &str = "[ALARMCODE:";
const ALARM_CODE_SUFFIX: &str = "]";
const STATUS_CODE_DELIMETER: &str = "|";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusCodes {
    error_codes: HashMap<u16, ErrorCode>,
    alarm_codes: HashMap<u16, AlarmCode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorCode {
    code: u16,
    description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlarmCode {
    code: u16,
    description: String,
}

impl StatusCodes {

    /// Creates a empty status code collection
    pub fn new() -> Self {
        StatusCodes { error_codes: HashMap::new(), alarm_codes: HashMap::new() }
    }

    /// Inserts a new error code or replaces the old value
    pub fn put_error_code(&mut self, error_code: ErrorCode) {
        self.error_codes.insert(error_code.code(), error_code);
    }

    /// Inserts a new alarm code or replaces the old value
    pub fn put_alarm_code(&mut self, alarm_code: AlarmCode) {
        self.alarm_codes.insert(alarm_code.code(), alarm_code);
    }

    /// Get a reference to the status codes's error codes.
    #[must_use]
    pub fn error_codes(&self) -> &HashMap<u16, ErrorCode> {
        &self.error_codes
    }

    /// Get a reference to the status codes's alarm codes.
    #[must_use]
    pub fn alarm_codes(&self) -> &HashMap<u16, AlarmCode> {
        &self.alarm_codes
    }
}

/// Parses a \<prefix\>\<code\>||\<description\>\<suffix\> message and returns the code and description as tuple
///
/// # Errors
/// Returns an error when parsing fails
fn parse_status_code(
    message: &str,
    prefix: &str,
    suffix: &str,
    property: &str,
) -> Result<(u16, String), String> {
    if message.starts_with(prefix) && message.ends_with(suffix) {
        let status_message: Vec<&str> = message
            .strip_prefix(prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
            .split(STATUS_CODE_DELIMETER)
            .collect();
        if status_message.len() != 3 {
            return Err(format!(
                "Invalid count of substrings in {}: \"{}\"",
                property, message
            ));
        }

        // parse the code values as integer
        let code = match status_message[0].parse::<u16>() {
            Ok(code) => code,
            Err(_) => {
                return Err(format!(
                    "Cannot read status code: \"{}\"",
                    status_message[0]
                ))
            }
        };

        // the second field is always empty and the third field contains the description
        let description = status_message[2];
        return Ok((code, description.to_string()));
    }
    Err(format!("Cannot read {}: \"{}\"", property, message))
}

impl AlarmCode {

    /// Reads an alarm code description message and stores the description for the specific code
    ///
    /// # Errors
    /// Returns an error when parsing fails
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// let alarm = AlarmCode::from("[ALARMCODE:6||Homing fail. The active homing cycle was reset.]").unwrap();
    /// assert_eq!(alarm.code(), 6u16);
    /// assert_eq!(alarm.description(), "Homing fail. The active homing cycle was reset.");
    /// ```
    pub fn from(message: &str) -> Result<Self, String> {
        match parse_status_code(message, ALARM_CODE_PREFIX, ALARM_CODE_SUFFIX, "alarm code") {
            Ok(alarm_code) => Ok(AlarmCode {
                code: alarm_code.0,
                description: alarm_code.1,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn is_response(message: &str) -> bool {
        message.starts_with(ALARM_CODE_PREFIX) && message.ends_with(ALARM_CODE_SUFFIX)
    }

    /// Get the alarm code's code.
    #[must_use]
    pub fn code(&self) -> u16 {
        self.code
    }

    /// Get a reference to the alarm code's description.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
}

impl ErrorCode {

    /// Reads an error code description message and stores the description for the specific code
    ///
    /// # Errors
    /// Returns an error when parsing fails
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// let error = ErrorCode::from("[ERRORCODE:54||Retract position is less than drill depth.]").unwrap();
    /// assert_eq!(error.code(), 54u16);
    /// assert_eq!(error.description(), "Retract position is less than drill depth.");
    /// ```
    pub fn from(message: &str) -> Result<Self, String> {
        match parse_status_code(message, ERROR_CODE_PREFIX, ERROR_CODE_SUFFIX, "error code") {
            Ok(alarm_code) => Ok(ErrorCode {
                code: alarm_code.0,
                description: alarm_code.1,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn is_response(message: &str) -> bool {
        message.starts_with(ERROR_CODE_PREFIX) && message.ends_with(ERROR_CODE_SUFFIX)
    }

    /// Get the error code's code.
    #[must_use]
    pub fn code(&self) -> u16 {
        self.code
    }

    /// Get a reference to the error code's description.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
}
