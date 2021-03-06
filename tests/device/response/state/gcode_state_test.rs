use grbli::device::response::state::gcode_state::*;


#[test]
fn from_parses_message_correctly() {
    let message_str = "[GC:AB CD F ' $H 'YO]";
    let expected_message_str = vec!["AB", "CD", "F", "'", "$H", "'YO"];
    let message = GCodeState::from(message_str).unwrap();
    assert_eq!(expected_message_str, *message.values())
}


#[test]
fn from_does_not_apply_trimming() {
    let message_str = "  [GC:AB CD F ' $H 'YO]                  ";
    let message_error = GCodeState::from(message_str).err().unwrap();
    assert_eq!("Cannot read gcode state message \"  [GC:AB CD F ' $H 'YO]                  \"", &message_error[..])
}

#[test]
fn from_removes_unecessary_value_whitespaces() {
    let message_str = "[GC: AB CD F ' $H 'YO ]";
    let expected_message_str = vec!["AB", "CD", "F", "'", "$H", "'YO"];
    let message = GCodeState::from(message_str).unwrap();
    assert_eq!(expected_message_str, *message.values())
}

#[test]
fn from_can_read_empty_messages() {
    let message_str = "[GC:]";
    let expected_message_str: Vec<String> = Vec::new();
    let message = GCodeState::from(message_str).unwrap();
    assert_eq!(expected_message_str, *message.values())
}

#[test]
fn from_fails_on_missing_prefix() {
    let message_str = "AB CD F ' $H 'YO]";
    let message = GCodeState::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Cannot read gcode state message \"AB CD F ' $H 'YO]\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_suffix() {
    let message_str = "[GC:AB CD F ' $H 'YO";
    let message = GCodeState::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Cannot read gcode state message \"[GC:AB CD F ' $H 'YO\"", &message_error[..])
}