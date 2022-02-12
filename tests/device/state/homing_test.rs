use grbli::device::{state::homing::HomingState, axis::Axis};


#[test]
fn from_parses_homed_correctly() {
    let homing = HomingState::from("H:1,3").unwrap();
    assert!(homing.is_homed());
    assert!(matches!(homing.homed_axes()[0], Axis::X));
    assert!(matches!(homing.homed_axes()[1], Axis::Y));
}

#[test]
fn from_parses_not_homed_correctly() {
    let homing = HomingState::from("H:0,3").unwrap();
    assert!(!homing.is_homed());
    assert!(matches!(homing.homed_axes()[0], Axis::X));
    assert!(matches!(homing.homed_axes()[1], Axis::Y));
}

#[test]
fn from_parses_all_homed_correctly() {
    let homing = HomingState::from("H:0").unwrap();
    assert!(!homing.is_homed());
    assert_eq!(6, homing.homed_axes().len());
    assert!(matches!(homing.homed_axes()[0], Axis::X));
    assert!(matches!(homing.homed_axes()[1], Axis::Y));
    assert!(matches!(homing.homed_axes()[2], Axis::Z));
    assert!(matches!(homing.homed_axes()[3], Axis::A));
    assert!(matches!(homing.homed_axes()[4], Axis::B));
    assert!(matches!(homing.homed_axes()[5], Axis::C));
}

#[test]
fn from_ignores_invalid_axis_bits() {
    let homing = HomingState::from("H:1,230").unwrap();
    assert!(homing.is_homed());
    assert_eq!(3, homing.homed_axes().len());
    assert!(matches!(homing.homed_axes()[0], Axis::Y));
    assert!(matches!(homing.homed_axes()[1], Axis::Z));
    assert!(matches!(homing.homed_axes()[2], Axis::C));
}

#[test]
fn from_fails_on_invalid_syntax() {
    let error_message = HomingState::from("H0").err().unwrap();
    assert_eq!("Cannot read homing state \"H0\"", error_message);
}

#[test]
fn from_fails_on_invalid_state() {
    let error_message = HomingState::from("H:3").err().unwrap();
    assert_eq!("Cannot read homing completion state \"3\"", error_message);
}

#[test]
fn from_fails_on_invalid_number_state() {
    let error_message = HomingState::from("H:a").err().unwrap();
    assert_eq!("Cannot read homing completion state \"a\"", error_message);
}
