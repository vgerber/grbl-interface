use grbli::device::response::setting::description::DeviceSettingDescription;


#[test]
fn setting_desc_is_parsed_correctly() {
    let desc = DeviceSettingDescription::from("[SETTING:0|27|Step pulse time|microseconds|6|#0.0|2.0|3.0]").unwrap();
    assert_eq!(*desc.index(), 0);
    assert_eq!(*desc.group_index(), 27);
    assert_eq!(desc.description().unwrap(), "Step pulse time");
    assert_eq!(desc.unit().unwrap(), "microseconds");
    assert_eq!(*desc.value_type(), 6);
    assert_eq!(desc.value_format().unwrap(), "#0.0");
    assert_eq!(desc.value_min().unwrap(), "2.0");
    assert_eq!(desc.value_max().unwrap(), "3.0");
}

#[test]
fn setting_desc_accepts_missing_fields() {
    let desc = DeviceSettingDescription::from("[SETTING:0|27|||6|||]").unwrap();
    assert_eq!(*desc.index(), 0);
    assert_eq!(*desc.group_index(), 27);
    assert!(desc.description().is_none());
    assert!(desc.unit().is_none());
    assert_eq!(*desc.value_type(), 6);
    assert!(desc.value_format().is_none());
    assert!(desc.value_min().is_none());
    assert!(desc.value_max().is_none());
}

#[test]
fn setting_desc_parsing_fails_on_invalid_index() {
    let err = DeviceSettingDescription::from("[SETTING:0k|27|Step pulse time|microseconds|6|#0.0|2.0|3.0]").err().unwrap();
    assert_eq!(err, "Cannot read setting index: \"0k\"")
}

#[test]
fn setting_desc_parsing_fails_on_invalid_group_index() {
    let err = DeviceSettingDescription::from("[SETTING:0|2_7|Step pulse time|microseconds|6|#0.0|2.0|3.0]").err().unwrap();
    assert_eq!(err, "Cannot read group index: \"2_7\"")
}

#[test]
fn setting_desc_parsing_fails_on_invalid_type() {
    let err = DeviceSettingDescription::from("[SETTING:0|27|Step pulse time|microseconds|999h|#0.0|2.0|3.0]").err().unwrap();
    assert_eq!(err, "Cannot read type index: \"999h\"")
}