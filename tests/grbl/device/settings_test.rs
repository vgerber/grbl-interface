use grbli::grbl::device::settings::*;

#[test]
fn setting_stored_with_correct_values() {
    let value = SettingValue::String(String::from("Test"));
    let index = SettingIndex::StepEnablePinsInvert;

    let s = DeviceSetting::new(index, value);

    assert!(matches!(s.index(), SettingIndex::StepEnablePinsInvert));
    assert!(matches!(s.value(), SettingValue::String(_)));
    //s.value()
}