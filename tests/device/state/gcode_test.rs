use grbli::device::state::gcode::parse_line_number;

#[test]
fn parses_mode_radius_correctly() {
    assert_eq!(32, parse_line_number("Ln:32").unwrap());
    assert_eq!(0, parse_line_number("Ln:0").unwrap());
    assert_eq!(-32, parse_line_number("Ln:-32").unwrap());
}

#[test]
fn parsing_fails_on_invalid_value() {
    let message_str = "Ln:a";
    let error_message = parse_line_number(message_str).err().unwrap();
    assert_eq!("Cannot read line number \"a\"", error_message);
}


#[test]
fn parsing_fails_on_invalid_message_syntax() {
    let message_str = "Ln32";
    let error_message = parse_line_number(message_str).err().unwrap();
    assert_eq!("Cannot read line number \"Ln32\"", error_message);
}