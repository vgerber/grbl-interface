use grbli::device::response::version::*;


#[test]
fn from_parses_message_correctly() {
    let message_str = "[VER:0.1223d.234f:test]";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.version());
    assert_eq!(String::from("test"), *message.name())
}

#[test]
fn from_does_not_apply_trimming() {
    let message_str = "  [VER:0.1223d.234f:test]      ";
    let error = VersionResponse::from(message_str).err().unwrap();
    assert_eq!("Could not read version \"  [VER:0.1223d.234f:test]      \"", error);
}

#[test]
fn from_accepts_empty_name_section() {
    let message_str = "[VER:0.1223d.234f:]";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.version());
    assert_eq!(String::from(""), *message.name())
}

#[test]
fn from_accepts_colons_in_name() {
    let message_str = "[VER:0.1223d.234f:a:b:c:d:]";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.version());
    assert_eq!(String::from("a:b:c:d:"), *message.name())
}

#[test]
fn from_cannot_read_empty_messages() {
    let message_str = "[VER:]";
    let message_error = VersionResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid count of version strings \"\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_prefix() {
    let message_str = "0.1223d.234f:test]";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Could not read version \"0.1223d.234f:test]\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_suffix() {
    let message_str = "[VER:0.1223d.234f:test";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Could not read version \"[VER:0.1223d.234f:test\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_name_separator() {
    let message_str = "[VER:0.1223d.234f]";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Invalid count of version strings \"0.1223d.234f\"", &message_error[..])
}