use grbli::device::state::modes::{parse_arc_mode, ArcMode};

#[test]
fn parses_mode_radius_correctly() {
    let message_str = "D:0";
    let mode = parse_arc_mode(message_str).unwrap();
    assert!(matches!(mode, ArcMode::Radius));
}

#[test]
fn parses_mode_diameter_correctly() {
    let message_str = "D:1";
    let mode = parse_arc_mode(message_str).unwrap();
    assert!(matches!(mode, ArcMode::Diameter));
}

#[test]
fn parsing_fails_on_invalid_arc_mode() {
    let message_str = "D:3";
    let error_message = parse_arc_mode(message_str).err().unwrap();
    assert_eq!("Unknown arc mode \"3\"", error_message);
}


#[test]
fn parsing_fails_on_invalid_message_syntax() {
    let message_str = "D0";
    let error_message = parse_arc_mode(message_str).err().unwrap();
    assert_eq!("Cannot read arc mode message \"D0\"", error_message);
}