use grbli::device::{state::machine::position::{parse_position, parse_local_position, parse_global_position, parse_local_position_offset, parse_coordinate_system, parse_scaled_axes}, axis::Axis};



#[test]
fn parse_position_returns_all_axis() {
    let message_str = "3,000.1,23.11,7,12.5,18";
    let position = parse_position(message_str).unwrap();
    assert_eq!(3f32, position[0]);
    assert_eq!(0.1, position[1]);
    assert_eq!(23.11, position[2]);
    assert_eq!(7f32, position[3]);
    assert_eq!(12.5, position[4]);
    assert_eq!(18f32, position[5]);
}

#[test]
fn parse_position_requires_at_least_one_axis() {
    let message_str = "";
    let error = parse_position(message_str).err().unwrap();
    assert_eq!("Cannot read axis:0 \"\"", error)
}

#[test]
fn parse_position_fails_on_invalid_axis_value() {
    let message_str = "1,2,v,4";
    let error = parse_position(message_str).err().unwrap();
    assert_eq!("Cannot read axis:2 \"v\"", error)
}

#[test]
fn parse_position_accept_max_six_axes() {
    let message_str = "1,2,3,4,5,6,7";
    let error = parse_position(message_str).err().unwrap();
    assert_eq!("Invalid count of axis in \"1,2,3,4,5,6,7\" (1 <= x <= 6)", error)
}

#[test]
fn parse_local_position_returns_position() {
    let message_str = "WPos:3,000.1,23.11,7,12.5,18";
    let position = parse_local_position(message_str).unwrap();
    assert_eq!(3f32, position[0]);
    assert_eq!(0.1, position[1]);
    assert_eq!(23.11, position[2]);
    assert_eq!(7f32, position[3]);
    assert_eq!(12.5, position[4]);
    assert_eq!(18f32, position[5]);
}

#[test]
fn parse_local_position_fails_on_wrong_prefix() {
    let message_str = "WPos3,000.1,23.11,7,12.5,18";
    let error = parse_local_position(message_str).err().unwrap();
    assert_eq!("Position is not a local position message \"WPos3,000.1,23.11,7,12.5,18\"", error);
}

#[test]
fn parse_global_position_returns_position() {
    let message_str = "MPos:3,000.1,23.11,7,12.5,18";
    let position = parse_global_position(message_str).unwrap();
    assert_eq!(3f32, position[0]);
    assert_eq!(0.1, position[1]);
    assert_eq!(23.11, position[2]);
    assert_eq!(7f32, position[3]);
    assert_eq!(12.5, position[4]);
    assert_eq!(18f32, position[5]);
}

#[test]
fn parse_global_position_fails_on_wrong_prefix() {
    let message_str = "MPos3,000.1,23.11,7,12.5,18";
    let error = parse_global_position(message_str).err().unwrap();
    assert_eq!("Position is not a global position message \"MPos3,000.1,23.11,7,12.5,18\"", error);
}

#[test]
fn parse_local_position_offset_returns_position() {
    let message_str = "WCO:3,000.1,23.11,7,12.5,18";
    let position = parse_local_position_offset(message_str).unwrap();
    assert_eq!(3f32, position[0]);
    assert_eq!(0.1, position[1]);
    assert_eq!(23.11, position[2]);
    assert_eq!(7f32, position[3]);
    assert_eq!(12.5, position[4]);
    assert_eq!(18f32, position[5]);
}

#[test]
fn parse_local_position_offset_fails_on_wrong_prefix() {
    let message_str = "WCO3,000.1,23.11,7,12.5,18";
    let error = parse_local_position_offset(message_str).err().unwrap();
    assert_eq!("Position is not a local position offset message \"WCO3,000.1,23.11,7,12.5,18\"", error);
}

#[test]
fn parse_coordinate_system_returns_cs() {
    let message_str = "WCS:G66";
    let cs = parse_coordinate_system(message_str).unwrap();
    assert_eq!("G66", cs);
}

#[test]
fn parse_coording_system_fails_on_wrong_prefix() {
    let message_str = "WCSBV";
    let error = parse_coordinate_system(message_str).err().unwrap();
    assert_eq!("Cannot read coordinate system message \"WCSBV\"", error);
}

#[test]
fn parse_scaled_axes_returns_axes() {
    let message_str = "Sc:XYBA";
    let axes = parse_scaled_axes(message_str).unwrap();
    assert!(matches!(axes[0], Axis::X));
    assert!(matches!(axes[1], Axis::Y));
    assert!(matches!(axes[2], Axis::B));
    assert!(matches!(axes[3], Axis::A));
}


#[test]
fn parse_scaled_axes_fails_on_wrong_prefix() {
    let message_str = "Sc";
    let error = parse_scaled_axes(message_str).err().unwrap();
    assert_eq!("Cannot read scaled axes message \"Sc\"", error);
}