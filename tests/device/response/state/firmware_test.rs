use grbli::device::response::state::firmware::parse_firmware;



#[test]
fn firmware_is_parsed_correctly() {
    let msg = "FW:grblHAL";
    let fw = parse_firmware(msg).unwrap();
    assert_eq!("grblHAL", fw);
}

#[test]
fn firmware_ccepts_empty_strings() {
    let msg = "FW:";
    let fw = parse_firmware(msg).unwrap();
    assert_eq!("", fw);
}

#[test]
fn parse_fails_on_invaid_suffix() {
    let msg = "Fw:grblHAL";
    let error = parse_firmware(msg).err().unwrap();
    assert_eq!("Cannot read firmware message \"Fw:grblHAL\"", error);
}