use std::collections::HashMap;

const SETTINGS_PREFIX: &str = "$";
const SETTING_DESC_PREFIX: &str = "[SETTING:";
const SETTING_DESC_SUFFIX: &str = "]";
const SETTING_GROUP_PREFIX: &str = "[SETTINGGROUP:";
const SETTING_GROUP_SUFFIX: &str = "]";

#[derive(Clone, Debug)]
pub struct DeviceSetting {
    index: u32,
    value: String,
}

pub struct DeviceSettingGroup {
    index: u32,
    parent_group_index: u32,
    name: String,
}

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

/// Stores all settings and meta data
pub struct DeviceSettings {
    /// Stored settings
    settings: HashMap<u32, DeviceSetting>,
}

impl DeviceSetting {

    /// Creates a new setting
    pub fn new(index: u32, value: String) -> Self {
        DeviceSetting { index, value }
    }

    /// Reads a single setting line and retruns its value and index
    ///
    /// # Error
    /// Returns an error if parsing failed at any point
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores:
    /// // Index and value as string
    /// let response = DeviceSetting::from("$32=72.001");
    /// assert_eq!(response.index(), 32);
    /// assert_eq!(response.value(), "72.001");
    /// ```
    pub fn from(message: String) -> Result<Self, String> {
        if DeviceSetting::is_response(message.as_str()) {
            // expect $<index>=<value>
            let setting_pair: Vec<&str> = message
                .strip_prefix(SETTINGS_PREFIX)
                .unwrap()
                .split("=")
                .collect();
            if setting_pair.len() == 2 {
                // expect [<index>,<value>]
                let index = match setting_pair[0].parse::<u32>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read setting index: \"{}\"",
                            setting_pair[0]
                        ))
                    }
                };

                let value = setting_pair[1];
                return Ok(DeviceSetting {
                    index,
                    value: value.to_string(),
                });
            }
        }
        Err(format!("Cannot read setting: \"{}\"", message))
    }

    /// Indicates if the response is a setting
    pub fn is_response(message: &str) -> bool {
        message.starts_with(SETTINGS_PREFIX)
    }

    /// Get the index of the setting
    pub fn index(&self) -> &u32 {
        &self.index
    }

    /// Get the setting value
    pub fn value(&self) -> &String {
        &self.value
    }
}

impl DeviceSettingGroup {
    /// Reads a single setting group line and retruns its value and index
    ///
    /// # Error
    /// Returns an error if parsing failed at any point
    ///
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores:
    /// // index, parent index and name
    /// let response = DeviceSettingGroup::from("[SETTINGGROUP:30|29|X-axis]");
    /// assert_eq!(response.index(), 30);
    /// assert_eq!(response.parent_group_index(), "29");
    /// assert_eq!(response.name(), "X-axis");
    /// ```
    pub fn from(message: String) -> Result<Self, String> {
        if DeviceSettingGroup::is_response(message.as_str()) {
            // expect $<index>=<value>
            let setting_pair: Vec<&str> = message
                .strip_prefix(SETTING_GROUP_PREFIX).unwrap()
                .strip_suffix(SETTING_GROUP_SUFFIX).unwrap()
                .split("|")
                .collect();
            if setting_pair.len() == 3 {
                // expect [<index>,<parent_index>,<name>]
                let index = match setting_pair[0].parse::<u32>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read setting group index: \"{}\"",
                            setting_pair[0]
                        ))
                    }
                };

                let parent_index = match setting_pair[1].parse::<u32>() {
                    Ok(index) => index,
                    Err(_) => {
                        return Err(format!(
                            "Cannot read setting group parent index: \"{}\"",
                            setting_pair[1]
                        ))
                    }
                };

                let name = setting_pair[2].to_string();
                return Ok(DeviceSettingGroup {
                    index,
                    parent_group_index: parent_index,
                    name,
                });
            }
        }
        Err(format!("Cannot read setting group: \"{}\"", message))
    }

    /// Indicates if the response is a setting group
    pub fn is_response(message: &str) -> bool {
        message.starts_with(SETTING_GROUP_PREFIX) && message.ends_with(SETTING_GROUP_SUFFIX)
    }

    /// Get the index of the group
    pub fn index(&self) -> &u32 {
        &self.index
    }

    /// Get the index of the parent group
    pub fn parent_group_index(&self) -> &u32 {
        &self.parent_group_index
    }

    /// Get the group name
    pub fn name(&self) -> &String {
        &self.name
    }
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
    pub fn from(message: String) -> Result<Self, String> {
        if DeviceSettingDescription::is_response(message.as_str()) {
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
                let description = Some(settings[parser_index].to_string());
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
                            "Cannot read group index: \"{}\"",
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
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get the device setting description's group index.
    #[must_use]
    pub fn group_index(&self) -> u32 {
        self.group_index
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
    pub fn value_type(&self) -> u8 {
        self.value_type
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

impl DeviceSettings {
    pub fn new(&mut self) {
        self.settings = HashMap::new();
    }

    /// Stores the new setting and overrides the old value
    pub fn put_setting(&mut self, setting: DeviceSetting) {
        self.settings.insert(setting.index().clone(), setting);
    }

    /// Get setting for specific setting if present
    pub fn get_setting(&self, index: &u32) -> Option<&DeviceSetting> {
        self.settings.get(index)
    }

    /// Get all stored settings
    pub fn get_settings(&self) -> HashMap<u32, DeviceSetting> {
        self.settings.clone()
    }
}
