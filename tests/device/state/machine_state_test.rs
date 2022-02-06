use grbli::device::state::machine::state::{MachineState, MachineStateName};




#[test]
fn from_parses_status_and_sub_status_correctly() {
    let message_str = "Home:1";
    let machine_status = MachineState::from(message_str).unwrap();
    assert!(matches!(machine_status.status(), MachineStateName::Home));
    assert!(machine_status.sub_status().is_some());
    assert_eq!(1, machine_status.sub_status().unwrap())
}

#[test]
fn from_parses_status_correctly() {
    let message_str = "Home";
    let machine_status = MachineState::from(message_str).unwrap();
    assert!(matches!(machine_status.status(), MachineStateName::Home));
    assert!(machine_status.sub_status().is_none())
}

#[test]
fn from_cannot_read_unknown_status() {
    let message_str = "Homie";
    let machine_status = MachineState::from(message_str);
    assert!(machine_status.is_err());
    assert_eq!("Unknown status name \"Homie\"", machine_status.err().unwrap())
}

#[test]
fn from_cannot_read_empty_sub_status_status() {
    let message_str = "Home:";
    let machine_status = MachineState::from(message_str);
    assert!(machine_status.is_err());
    assert_eq!("Cannot read machine sub status \"\"", machine_status.err().unwrap())
}
