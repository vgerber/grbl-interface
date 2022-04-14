const SETTING_DESC_PREFIX: &str = "[SETTING:";
const SETTING_DESC_SUFFIX: &str = "]";


/// Description of a single setting entry
pub struct DeviceSettingDescription {
    index: u32,
    group_index: u32,
    description: Option<String>,
    unit: Option<String>,
    value_type: u8,
    value_format: Option<String>,
    value_min: Option<String>,
    value_max: Option<String>,
}


impl DeviceSettingDescription {

    /// Reads a single setting description line and retruns its parsed content
    ///
    /// Some fields are not required (see the actual struct)
    /// # Error
    /// Returns an error if parsing failed at any point
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // setting for stepper pulse time
    /// let response = DeviceSettingDescription::from("[SETTING:0|27|Step pulse time|microseconds|6|#0.0|2.0|]");
    /// assert_eq!(response.index(), 0);
    /// assert_eq!(response.group_index(), 27);
    /// assert_eq!(response.description(), "Step pulse time");
    /// //...
    /// ```
    pub fn from(message: &str) -> Result<Self, String> {
        if DeviceSettingDescription::is_response(message) {
            let setting_str = message
                .strip_prefix(SETTING_DESC_PREFIX)
                .unwrap()
                .strip_suffix(SETTING_DESC_SUFFIX)
                .unwrap();
            let settings: Vec<&str> = setting_str.split("|").collect();

            let mut parser_index = 0usize;
            if settings.len() == 8 {

                // parse index
                let setting_index = match settings[parser_index].parse::<u32>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read setting index: \"{}\"",
                            settings[parser_index]
                        ))
                    }
                };
                parser_index += 1;

                // parse group index
                let group_index = match settings[parser_index].parse::<u32>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read group index: \"{}\"",
                            settings[parser_index]
                        ))
                    }
                };
                parser_index += 1;

                // parse description
                let description = match settings[parser_index].len() {
                    0 => None,
                    _ => Some(settings[parser_index].to_string()),
                };
                parser_index += 1;

                // parse value unit
                let unit = match settings[parser_index].len() {
                    0 => None,
                    _ => Some(settings[parser_index].to_string()),
                };
                parser_index += 1;

                // parse value data type
                let value_type = match settings[parser_index].parse::<u8>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read type index: \"{}\"",
                            settings[parser_index]
                        ))
                    }
                };
                parser_index += 1;

                // parse format
                let value_format = match settings[parser_index].len() {
                    0 => None,
                    _ => Some(settings[parser_index].to_string()),
                };
                parser_index += 1;

                // parse min
                let value_min = match settings[parser_index].len() {
                    0 => None,
                    _ => Some(settings[parser_index].to_string()),
                };
                parser_index += 1;

                // parse max
                let value_max = match settings[parser_index].len() {
                    0 => None,
                    _ => Some(settings[parser_index].to_string()),
                };

                // create struct from parsed values
                return Ok(DeviceSettingDescription {
                    index: setting_index,
                    group_index,
                    description,
                    unit: unit,
                    value_type,
                    value_format,
                    value_min,
                    value_max,
                });
            } else {
                return Err(format!(
                    "Expected 8 arguments for setting description: \"{}\"",
                    message
                ));
            }
        }
        Err(format!("Cannto read setting desc: \"{}\"", message))
    }

    /// Indicates if the response is a setting description
    pub fn is_response(message: &str) -> bool {
        message.starts_with(SETTING_DESC_PREFIX) && message.ends_with(SETTING_DESC_SUFFIX)
    }

    /// Get the device setting description's index.
    #[must_use]
    pub fn index(&self) -> &u32 {
        &self.index
    }

    /// Get the device setting description's group index.
    #[must_use]
    pub fn group_index(&self) -> &u32 {
        &self.group_index
    }

    /// Get a reference to the device setting description's description.
    #[must_use]
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Get a reference to the device setting description's unit.
    #[must_use]
    pub fn unit(&self) -> Option<&String> {
        self.unit.as_ref()
    }

    /// Get the device setting description's value type.
    #[must_use]
    pub fn value_type(&self) -> &u8 {
        &self.value_type
    }

    /// Get a reference to the device setting description's value format.
    #[must_use]
    pub fn value_format(&self) -> Option<&String> {
        self.value_format.as_ref()
    }

    /// Get a reference to the device setting description's value min.
    #[must_use]
    pub fn value_min(&self) -> Option<&String> {
        self.value_min.as_ref()
    }

    /// Get a reference to the device setting description's value max.
    #[must_use]
    pub fn value_max(&self) -> Option<&String> {
        self.value_max.as_ref()
    }
}