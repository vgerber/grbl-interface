use std::{collections::BTreeMap, fmt};

use self::{group::DeviceSettingGroup, description::DeviceSettingDescription};

pub mod description;
pub mod group;

const SETTINGS_PREFIX: &str = "$";

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceSetting {
    index: u32,
    value: String,
}

/// Stores all settings and meta data
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceSettings {
    /// Stored settings
    settings: BTreeMap<u32, DeviceSetting>,
    setting_groups: BTreeMap<u32, DeviceSettingGroup>,
    setting_descriptions: BTreeMap<u32, DeviceSettingDescription>,
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

impl fmt::Debug for DeviceSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("DeviceSetting [{},{}]", self.index, self.value).as_str()).finish()
    }
}

impl DeviceSettings {

    pub fn new() -> Self {
        DeviceSettings {
            settings: BTreeMap::new(),
            setting_descriptions: BTreeMap::new(),
            setting_groups: BTreeMap::new(),
        }
    }

    /// Stores the new setting and overrides the old value
    pub fn put_setting(&mut self, setting: DeviceSetting) {
        self.settings.insert(*setting.index(), setting);
    }

    /// Stores the new setting and overrides the old value
    pub fn put_setting_group(&mut self, group: DeviceSettingGroup) {
        self.setting_groups.insert(*group.index(), group);
    }

    /// Stores the new setting and overrides the old value
    pub fn put_setting_description(&mut self, description: DeviceSettingDescription) {
        self.setting_descriptions.insert(*description.index(), description);
    }

    /// Get value for specific setting if present
    pub fn get_setting(&self, index: &u32) -> Option<&DeviceSetting> {
        self.settings.get(index)
    }

    /// Get the group by index if present
    pub fn get_setting_group(&self, index: &u32) -> Option<&DeviceSettingGroup> {
        self.setting_groups.get(index)
    }

    /// Get teh description by index if present
    pub fn get_setting_description(&self, index: &u32) -> Option<&DeviceSettingDescription> {
        self.setting_descriptions.get(index)
    }

    /// Get all stored settings
    pub fn get_settings(&self) -> &BTreeMap<u32, DeviceSetting> {
        &self.settings
    }

    /// Get a reference to the device settings's setting groups.
    #[must_use]
    pub fn setting_groups(&self) -> &BTreeMap<u32, DeviceSettingGroup> {
        &self.setting_groups
    }

    /// Get a reference to the device settings's setting descriptions.
    #[must_use]
    pub fn setting_descriptions(&self) -> &BTreeMap<u32, DeviceSettingDescription> {
        &self.setting_descriptions
    }
}