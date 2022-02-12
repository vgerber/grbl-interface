use grbli::device::state::input::*;

#[test]
fn parses_mode_radius_correctly() {
    assert!(!parse_input_wait_result("In:-1").unwrap());
    assert!(parse_input_wait_result("In:0").unwrap());
    assert!(parse_input_wait_result("In:1").unwrap());
}

#[test]
fn parsing_fails_on_invalid_value() {
    let message_str = "In:3";
    let error_message = parse_input_wait_result(message_str).err().unwrap();
    assert_eq!("Cannot interpret input wait result \"3\"", error_message);
}


#[test]
fn parsing_fails_on_invalid_message_syntax() {
    let message_str = "In0";
    let error_message = parse_input_wait_result(message_str).err().unwrap();
    assert_eq!("Cannot read input wait result \"In0\"", error_message);
}