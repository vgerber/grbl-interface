use crate::device::util::axis::{MAX_AXES, MIN_AXES, Axis, get_axis};

const POSITION_LOCAL_PREFIX: &str = "WPos:";
const POSITION_LOCAL_OFFSET_PREFIX: &str = "WCO:";
const POSITION_GLOBAL_PREFIX: &str = "MPos:";
const COORDINATE_SYSTEM_PREFIX: &str = "WCS:";
const SCALED_AXES_PREFIX: &str = "Sc:";

pub type MachinePosition = Vec<f32>;

/// Creates position from "\<float\>,\<float\>,...."
/// 
/// # Examples
/// ```
/// let position_3d = parse_position("3.21,2.0,-1")
/// let position_4d = parse_position("3.21,2.0,-1,0.0")
/// let position_5d = parse_position("3.21,2.0,-1,0.0,15")
/// ```
pub fn parse_position(position: &str) -> Result<MachinePosition, String> {
    let axis_strings: Vec<&str> = position.split(",").collect();
    let mut axis_values: MachinePosition = Vec::new();
    if axis_strings.len() >= MIN_AXES && axis_strings.len() <= MAX_AXES {
        for (axis_index, axis_string) in axis_strings.iter().enumerate() {
            axis_values.push(match axis_string.parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read axis:{} \"{}\"", axis_index, axis_string))
            })
        }
        return Ok(axis_values);
    }
    Err(format!("Invalid count of axis in \"{}\" ({} <= x <= {})", position, MIN_AXES, MAX_AXES))
}

/// Creates position from "WPos:\<float\>,\<float\>,\<float\>,..."
/// 
/// # Examples
/// ```
/// let position = parse_local_position("WPos:3.21,2.0,-1")
/// ```
pub fn parse_local_position(position: &str) -> Result<MachinePosition, String> {
    if is_local_position(position) {
        return parse_position(&position[POSITION_LOCAL_PREFIX.len()..]);
    }
    Err(format!("Position is not a local position message \"{}\"", position))
}   

/// Indicates if position starts with "WPos:"
pub fn is_local_position(position: &str) -> bool {
    position.starts_with(POSITION_LOCAL_PREFIX)
}

/// Creates position from "MPos:\<float\>,\<float\>,\<float\>,..."
/// 
/// Also known as machine position
/// 
/// # Examples
/// ```
/// let position = parse_global_position("MPos:3.21,2.0,-1")
/// ```
pub fn parse_global_position(position: &str) -> Result<MachinePosition, String> {
    if is_global_position(position) {
        return parse_position(&position[POSITION_GLOBAL_PREFIX.len()..]);
    }
    Err(format!("Position is not a global position message \"{}\"", position))
} 

/// Indicates if position starts with "MPos:"
pub fn is_global_position(position: &str) -> bool {
    position.starts_with(POSITION_GLOBAL_PREFIX)
}

/// Creates offset position from "WCO:\<float\>,\<float\>,\<float\>,..."
/// 
/// # Examples
/// ```
/// let position = parse_local_position_offset("WCO:3.21,2.0,-1")
/// ```
pub fn parse_local_position_offset(position: &str) -> Result<MachinePosition, String> {
    if is_local_position_offset(position) {
        return parse_position(&position[POSITION_LOCAL_OFFSET_PREFIX.len()..]);
    }
    Err(format!("Position is not a local position offset message \"{}\"", position))
}   

/// Indicates if position starts with "WCO:"
pub fn is_local_position_offset(position: &str) -> bool {
    position.starts_with(POSITION_LOCAL_OFFSET_PREFIX)
}

/// Returns coordinate system from "WCS:G.."
/// 
/// [Coordinate systems](https://linuxcnc.org/docs/html/gcode/g-code.html#gcode:g54-g59.3)
/// # Examples
/// ```
/// let coordinate_system = parse_coordinate_system("WCS:G54")
/// ```
pub fn parse_coordinate_system(message: &str) -> Result<String, String> {
    if is_coordinate_system(message) {
        let coordinate_system = &message[COORDINATE_SYSTEM_PREFIX.len()..];
        return match coordinate_system.starts_with("G") {
            true => Ok(coordinate_system.to_string()),
            false => Err(format!("Cannot read coordinate system \"{}\"", coordinate_system)),
        }
    }
    Err(format!("Cannot read coordinate system message \"{}\"", message))
}

/// Indicates if message has coordinate system syntax
pub fn is_coordinate_system(message: &str) -> bool {
    message.starts_with(COORDINATE_SYSTEM_PREFIX)
}

/// Returns scaled axes from "Sc:XYZABC"
/// 
/// # Examples
/// ```
/// let scaled_axes = parse_scaled_axes("Sc:XZ");
/// assert_eq!(vec![Axis::X, Axis::Z], scaled_axes)
/// ```
pub fn parse_scaled_axes(message: &str) -> Result<Vec<Axis>, String> {
    if is_scaled_axes(message) {
        let scaled_axes_message = &message[SCALED_AXES_PREFIX.len()..];
        return parse_scaled_axes_values(scaled_axes_message);
    }
    Err(format!("Cannot read scaled axes message \"{}\"", message))
}

/// Parses signals value string and returns all signals or an error if a signal is unknown
fn parse_scaled_axes_values(scaled_axes_values: &str) -> Result<Vec<Axis>, String> {
    let mut scaled_axes: Vec<Axis> = Vec::new();

    // parses accessory states as ascii chars
    // returns an error if a state is unknown
    for scaled_axis_byte in scaled_axes_values.bytes() {
        let scaled_axis = match String::from_utf8(vec![scaled_axis_byte]) {
            Ok(string) => string,
            Err(_) => String::from("Invalid Symbol")
        }; 
        match get_axis(&scaled_axis[..]) {
            Ok(axis) => scaled_axes.push(axis),
            Err(_) => return Err(format!("Unknown scaled axis \"{}\"", scaled_axis)),
        }
    }
    Ok(scaled_axes)
}

/// Indicates if message has scaled axes syntax
pub fn is_scaled_axes(message: &str) -> bool {
    message.starts_with(SCALED_AXES_PREFIX)
}