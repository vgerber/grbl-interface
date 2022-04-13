use crate::device::util::axis::{Axis, get_axes_from_mask, get_all_grbl_axes};

const HOMING_STATE_PREFIX: &str = "H:";

#[derive(Clone)]
pub struct HomingState {
    /// True if homed_axes are homed
    homed: bool,

    /// Homed axis
    homed_axes: Vec<Axis>
}

impl HomingState {
    
    /// Creates homing state from "H:1,3"
    /// 
    /// # Examples
    /// ```
    /// let homing_state = HomingState::from("H:1,3");
    /// assert!(homing_state.is_homed());
    /// assert_eq!(vec![Axis::X, Axis::Y], homing_state.homed_axes())
    /// ```
    pub fn from(message: &str) -> Result<HomingState, String> {
        if HomingState::is_homing_state(message) {
            let homing_state_message = &message[HOMING_STATE_PREFIX.len()..];
            let homing_state_strings: Vec<&str> = homing_state_message.split(",").collect();
            if homing_state_message.len() < 1 && homing_state_message.len() > 2 {
                return Err(format!("Invalid count of homing states \"{}\"", homing_state_message));
            }
            
            let homed = match homing_state_strings[0].parse::<i8>() {
                Ok(1) => true,
                Ok(0) => false,
                Ok(_) => return Err(format!("Cannot read homing completion state \"{}\"", homing_state_strings[0])),
                Err(_) => return Err(format!("Cannot read homing completion state \"{}\"", homing_state_strings[0])),
            };

            // read all axes from mask or
            // add all axes when no axes is present
            // first option is only present if axes are homed in seperate steps
            let homed_axes: Vec<Axis>;
            if homing_state_strings.len() == 2 {
                let homed_axes_mask = match homing_state_strings[1].parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Cannot read homed axis \"{}\"", homing_state_strings[1])),
                };
                homed_axes = get_axes_from_mask(homed_axes_mask);
            } else {
                homed_axes = get_all_grbl_axes();
            }

            return Ok(HomingState {
                homed,
                homed_axes,
            })

        }
        Err(format!("Cannot read homing state \"{}\"", message))
    }

    /// Indicates if message has homing state syntax
    pub fn is_homing_state(message: &str) -> bool {
        return message.starts_with(HOMING_STATE_PREFIX);
    }

    /// Indicates if machine is homed for given axes.
    pub fn is_homed(&self) -> bool {
        self.homed
    }

    /// Get a reference to the homing state's homed axis.
    pub fn homed_axes(&self) -> &[Axis] {
        self.homed_axes.as_ref()
    }
}

