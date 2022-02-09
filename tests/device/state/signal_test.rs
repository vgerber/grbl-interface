use grbli::device::state::{ signal::{parse_machine_signal, MachineSignal}};



#[test]
fn parses_signals_correctly() {
    let message_str = "PN:POX";
    let signals = parse_machine_signal(message_str).unwrap();
    assert!(matches!(signals[0], MachineSignal::ProbeTriggered));
    assert!(matches!(signals[1], MachineSignal::ProbeDisconnected));
    assert!(matches!(signals[2], MachineSignal::XLimitSwitchAsserted));
}

#[test]
fn parses_empty_signals_correctly() {
    let message_str = "PN:";
    let signals = parse_machine_signal(message_str).unwrap();
    assert_eq!(signals.len(), 0);
}

#[test]
fn parse_signals_fails_on_invalid_prefix() {
    let message_str = "P:";
    let error = parse_machine_signal(message_str).err().unwrap();
    assert_eq!("Cannot read machine signal \"P:\"", error);
}

#[test]
fn parse_signals_is_false_on_invalid_value() {
    let message_str = "PN:m";
    let error = parse_machine_signal(message_str).err().unwrap();
    assert_eq!("Unknown machine signal \"m\"", error);
}
