use grbli::device::response::startup::*;


#[test]
fn from_parses_message_correctly() {
    let message_str = ">ABC:ok";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from("ABC"), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Ok))
}

#[test]
fn from_parses_error_correctly() {
    let message_str = ">ABC:error:10";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from("ABC"), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Error(10)))
}

#[test]
fn from_parses_error_only_correctly() {
    let message_str = ">:error:10";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from(""), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Error(10)))
}

#[test]
fn from_parses_error_without_code_correctly() {
    let message_str = ">:error";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from(""), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Error(-1)))
}

#[test]
fn from_applys_trimming() {
    let message_str = "  >ABC:ok      ";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from("ABC"), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Ok))
}

#[test]
fn from_accepts_empty_executed_line() {
    let message_str = ">:ok";
    let message = StartupResponse::from(message_str).unwrap();
    assert_eq!(String::from(""), *message.executed_line());
    assert!(matches!(*message.result(), StartupResult::Ok))
}

#[test]
fn from_cannot_read_empty_messages() {
    let message_str = ">";
    let message_error = StartupResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid count of startup segments \"\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_prefix() {
    let message_str = "G23:ok";
    let message = StartupResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Could not read startup \"G23:ok\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_result_section() {
    let message_str = ">G66";
    let message = StartupResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Invalid count of startup segments \"G66\"", &message_error[..])
}