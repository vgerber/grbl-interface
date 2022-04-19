use std::result::Result;

/// Maximum amount of axes, available in grbl
pub const MAX_AXES: usize = 6;

/// Minimum amount of axes, available in grbl
pub const MIN_AXES: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
    A,
    B,
    C,
}

/// Get axis by name (X,Y,Z,A,B,C)
pub fn get_axis(axis: &str) -> Result<Axis, String> {
    match axis {
        "X" => Ok(Axis::X),
        "Y" => Ok(Axis::Y),
        "Z" => Ok(Axis::Z),
        "A" => Ok(Axis::A),
        "B" => Ok(Axis::B),
        "C" => Ok(Axis::C),
        _ => Err(format!("Unknown axis \"{}\"", axis))
    }
}

/// Returns list of all available axis for grbl firmware (X,Y,Z,A,B,C)
pub fn get_all_grbl_axes() -> Vec<Axis> {
    vec![Axis::X, Axis::Y, Axis::Z, Axis::A, Axis::B, Axis::C]
}

// Get axis by axis index 0 - 5
pub fn get_axis_by_index(axis_index: usize) -> Result<Axis, String> {
    match axis_index {
        0 => Ok(Axis::X),
        1 => Ok(Axis::Y),
        2 => Ok(Axis::Z),
        3 => Ok(Axis::A),
        4 => Ok(Axis::B),
        5 => Ok(Axis::C),
        _ => Err(format!("Unknown axis index \"{}\"", axis_index))
    }
}

/// Returns name for axis
pub fn get_axis_name(axis: &Axis) -> String {
    match axis {
        Axis::X => String::from("X"),
        Axis::Y => String::from("Y"),
        Axis::Z => String::from("Z"),
        Axis::A => String::from("A"),
        Axis::B => String::from("B"),
        Axis::C => String::from("C"),
    }
}

/// Get axis name by index 0 - 5
pub fn get_axis_name_by_index(axis_index: usize) -> Result<String, String> {
    let axis = get_axis_by_index(axis_index);
    if axis.is_err() {
        return Err(axis.err().unwrap());
    }
    
    match axis.unwrap() {
        Axis::X => Ok(String::from("X")),
        Axis::Y => Ok(String::from("Y")),
        Axis::Z => Ok(String::from("Z")),
        Axis::A => Ok(String::from("A")),
        Axis::B => Ok(String::from("B")),
        Axis::C => Ok(String::from("C")),
    }
}

/// Returns mask bit for given axis
pub fn get_axis_mask(axis: &Axis) -> i32 {
    match axis {
        Axis::X => 0b1,
        Axis::Y => 0b10,
        Axis::Z => 0b100,
        Axis::A => 0b1000,
        Axis::B => 0b10000,
        Axis::C => 0b100000,
    }
}

/// Returns combined masks from all axis
pub fn get_combined_axes_mask(axes: &Vec<Axis>) -> i32 {
    let mut axes_mask: i32 = 0;
    for ax in axes {
        axes_mask |= get_axis_mask(ax);
    }
    axes_mask
}

/// Returns true if bit for axis is set
pub fn is_axis_enabled(mask: i32, axis: &Axis) -> bool {
    (mask & get_axis_mask(axis)) > 0
}

/// Returns all axes which are set as one in the bit mask
/// 
/// # Examples
/// ```
/// let axes = get_axes_from_mask(0b101);
/// assert_eq!(vec![Axis::X, Axis::Z], axes)
/// ```
/// 
pub fn get_axes_from_mask(mask: i32) -> Vec<Axis> {
    let mut axes_in_mask = Vec::new();
    for axis in get_all_grbl_axes() {
        if is_axis_enabled(mask, &axis) {
            axes_in_mask.push(axis);
        }
    }
    axes_in_mask
} 