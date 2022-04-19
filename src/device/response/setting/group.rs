use std::fmt;

const SETTING_GROUP_PREFIX: &str = "[SETTINGGROUP:";
const SETTING_GROUP_SUFFIX: &str = "]";

/// Group information for multiple setting entries
#[derive(Clone, PartialEq, Eq)]
pub struct DeviceSettingGroup {
    index: u32,
    parent_group_index: u32,
    name: String,
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
    pub fn from(message: &str) -> Result<Self, String> {
        if DeviceSettingGroup::is_response(message) {
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

impl fmt::Debug for DeviceSettingGroup {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(format!("DeviceSettingGroup [{},{},{}]", self.index(), self.parent_group_index(), self.name()).as_str()).finish()
    }
}