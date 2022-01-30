use std::result::Result;


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

pub fn get_axis_mask(axis: Axis) -> i32 {
    match axis {
        X => 0b1,
        Y => 0b10,
        Z => 0b100,
        A => 0b1000,
        B => 0b10000,
        C => 0b100000,
    }
}

/// Returns true if bit for axis is set
pub fn is_axis_enabled(mask: i32, axis: Axis) -> bool {
    (mask & get_axis_mask(axis)) > 0
} 