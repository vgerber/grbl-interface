use grbli::device::response::error::*;

#[test]
fn alarm_code_is_parsed_correctly() {
    let alarm = AlarmCode::from("[ALARMCODE:6||Homing fail. The active homing cycle was reset.]").unwrap();
    assert_eq!(alarm.code(), 6u16);
    assert_eq!(alarm.description(), "Homing fail. The active homing cycle was reset.");
}

#[test]
fn error_code_is_parsed_correctly() {
    let error = ErrorCode::from("[ERRORCODE:54||Retract position is less than drill depth.]").unwrap();
    assert_eq!(error.code(), 54u16);
    assert_eq!(error.description(), "Retract position is less than drill depth.");
}

#[test]
fn error_code_stored_correctly() {
    let error = ErrorCode::from("[ERRORCODE:54||Retract position is less than drill depth.]").unwrap();
    let mut status_codes = StatusCodes::new();
    status_codes.put_error_code(error.clone());
    assert_eq!(error, *status_codes.error_codes().get(&error.code()).unwrap())
}

#[test]
fn alarm_code_stored_correctly() {
    let alarm = AlarmCode::from("[ALARMCODE:6||Homing fail. The active homing cycle was reset.]").unwrap();
    let mut status_codes = StatusCodes::new();
    status_codes.put_alarm_code(alarm.clone());
    assert_eq!(alarm, *status_codes.alarm_codes().get(&alarm.code()).unwrap())
}