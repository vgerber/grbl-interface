use grbli::device::response::setting::DeviceSetting;



#[test]
fn setting_stored_with_correct_values() {
    let setting = DeviceSetting::from("$12=ab0.1").unwrap();
    assert_eq!(*setting.index(), 12u32);
    assert_eq!(setting.value(), "ab0.1");
}

#[test]
fn setting_fails_on_invalid_message() {
    let error = DeviceSetting::from("12=ab0.1").err().unwrap();
    assert_eq!(error, "Cannot read setting: \"12=ab0.1\"");
}

#[test]
fn setting_fails_on_invalid_index() {
    let error = DeviceSetting::from("$a12=ab0.1").err().unwrap();
    assert_eq!(error, "Cannot read setting index: \"a12\"");
}