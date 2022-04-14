use std::collections::HashMap;

pub mod description;
pub mod group;

const SETTINGS_PREFIX: &str = "$";

#[derive(Clone, Debug)]
pub struct DeviceSetting {
    index: u32,
    value: String,
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
    pub fn from(message: &str) -> Result<Self, String> {
        if DeviceSetting::is_response(message) {
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