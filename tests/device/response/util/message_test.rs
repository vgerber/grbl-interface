use grbli::device::response::util::message::Message;



#[test]
fn from_parses_message_correctly() {
    let message_str = "[MSG:ABCDF'$H'YO]";
    let expected_message_str = "ABCDF'$H'YO";
    let message = Message::from(message_str).unwrap();
    assert_eq!(String::from(expected_message_str), *message.message())
}

#[test]
fn from_does_not_apply_trimming() {
    let message_str = "  [MSG:ABCDF'$H'YO]      ";
    let message_error = Message::from(message_str).err().unwrap();
    assert_eq!("Cannot read message \"  [MSG:ABCDF'$H'YO]      \"", message_error)
}

#[test]
fn from_can_read_empty_messages() {
    let message_str = "[MSG:]";
    let expected_message_str = "";
    let message = Message::from(message_str).unwrap();
    assert_eq!(String::from(expected_message_str), *message.message())
}

#[test]
fn from_fails_on_missing_prefix() {
    let message_str = "ABCDF'$H'YO]";
    let message = Message::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Cannot read message \"ABCDF'$H'YO]\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_suffix() {
    let message_str = "[MSG:ABCDF'$H'YO";
    let message = Message::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Cannot read message \"[MSG:ABCDF'$H'YO\"", &message_error[..])
}