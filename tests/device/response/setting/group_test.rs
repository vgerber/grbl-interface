use grbli::device::response::setting::group::DeviceSettingGroup;



#[test]
pub fn setting_group_is_parsed_correctly() {
    let group = DeviceSettingGroup::from("[SETTINGGROUP:30|29|X-axis]").unwrap();
    assert_eq!(*group.index(), 30);
    assert_eq!(*group.parent_group_index(), 29);
    assert_eq!(group.name(), "X-axis");
}

#[test]
pub fn setting_group_parsing_fails_on_invalid_prefix() {
    let error = DeviceSettingGroup::from("[SETTINGOUP:30|29|X-axis]").err().unwrap();
    assert_eq!(error, "Cannot read setting group: \"[SETTINGOUP:30|29|X-axis]\"")
}

#[test]
pub fn setting_group_parsing_fails_on_invalid_index() {
    let error = DeviceSettingGroup::from("[SETTINGGROUP:a30|29|X-axis]").err().unwrap();
    assert_eq!(error, "Cannot read setting group index: \"a30\"")
}


#[test]
pub fn setting_group_parsing_fails_on_invalid_group_index() {
    let error = DeviceSettingGroup::from("[SETTINGGROUP:30|2n9|X-axis]").err().unwrap();
    assert_eq!(error, "Cannot read setting group parent index: \"2n9\"")
}