use grbli::device::response::state::overrides::Overrides;

#[test]
fn from_parses_message_correctly() {
    let message_str = "Ov:10,22,1";
    let overrides = Overrides::from(message_str).unwrap();
    assert_eq!(10, overrides.feed_rate_percentage());
    assert_eq!(22, overrides.rapids_percentage());
    assert_eq!(1, overrides.spindle_speed_percentage());
}

#[test]
fn from_cannot_read_empty_message() {
    let message_str = "Ov:";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Invalid count of override values \"Ov:\"", &message_error[..])
}

#[test]
fn from_fails_on_more_than_three_options() {
    let message_str = "Ov:1,2,3,4";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Invalid count of override values \"Ov:1,2,3,4\"", &message_error[..])
}

#[test]
fn from_fails_on_less_than_three_options() {
    let message_str = "Ov:1,2";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Invalid count of override values \"Ov:1,2\"", &message_error[..])
}

#[test]
fn from_fails_on_invalid_feed_rate() {
    let message_str = "Ov:a,1,2";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Cannot read feed rate override \"a\"", &message_error[..])
}

#[test]
fn from_fails_on_invalid_rapids() {
    let message_str = "Ov:1,b,2";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Cannot read rapids override \"b\"", &message_error[..])
}

#[test]
fn from_fails_on_invalid_spindle_speed() {
    let message_str = "Ov:1,2,c";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Cannot read spindle speed override \"c\"", &message_error[..])
}

#[test]
fn from_fails_on_invalid_prefix() {
    let message_str = "v:1,2,3";
    let message_error = Overrides::from(message_str).err().unwrap();
    assert_eq!("Cannot read overrides \"v:1,2,3\"", &message_error[..])
}