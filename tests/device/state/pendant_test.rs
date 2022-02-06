use grbli::device::state::pendant::*;


#[test]
fn parses_status_correctly() {
    let message_str = "MPG:1";
    let state = parse_pendant_control(message_str).unwrap();
    assert!(matches!(state, PendantControl::Taken));
}

#[test]
fn parsing_fails_on_invalid_state() {
    let message_str = "MPG:2";
    let error_message = parse_pendant_control(message_str).err().unwrap();
    assert_eq!("Unknown pendant control state 2", error_message);
}

#[test]
fn parsing_fails_on_non_numeric_state() {
    let message_str = "MPG:A";
    let error_message = parse_pendant_control(message_str).err().unwrap();
    assert_eq!("Cannot interpret pendant control state \"A\"", error_message);
}

#[test]
fn parsing_fails_on_invalid_message_syntax() {
    let message_str = "MP:A";
    let error_message = parse_pendant_control(message_str).err().unwrap();
    assert_eq!("Cannot read pendant control message \"MP:A\"", error_message);
}