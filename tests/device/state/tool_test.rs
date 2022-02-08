use grbli::device::state::tool::parse_tool_length_reference;



#[test]
fn parses_tlr_1_correctly() {
    let message_str = "TLR:1";
    let tlr = parse_tool_length_reference(message_str).unwrap();
    assert!(tlr);
}

#[test]
fn parses_tlr_0_correctly() {
    let message_str = "TLR:0";
    let tlr = parse_tool_length_reference(message_str).unwrap();
    assert!(!tlr);
}

#[test]
fn parse_tlr_fails_on_invalid_prefix() {
    let message_str = "TL:";
    let error = parse_tool_length_reference(message_str).err().unwrap();
    assert_eq!("Cannot read tool reference length \"TL:\"", error);
}

#[test]
fn parse_tlr_is_false_on_none_one_values() {
    let message_str = "TLR:3";
    let value = parse_tool_length_reference(message_str).unwrap();
    assert!(!value)
}

#[test]
fn parse_tlr_is_false_on_invalid_value() {
    let message_str = "TLR:a";
    let error = parse_tool_length_reference(message_str).err().unwrap();
    assert_eq!("Cannot interpret tool reference length offset set value \"a\"", error);
}
