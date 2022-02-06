
const MACHINE_SPEED_PREFIX: &str = "FS:";

pub struct MachineSpeed {
    feed_rate: i32,
    spindle_programmed_rpm: i32,
    spindle_actual_rpm: Option<i32>
}


impl MachineSpeed {
    
    /// Creates machine speed from "FS:\<feed rate\>,\<spindle programmed rpm\>,\<spindle actual rpm\>"
    /// 
    /// # Examples
    /// ```
    /// let machine_speed = MachineSpeed::from("FS:100,3000,1677")
    /// ```
    pub fn from(message: &str) -> Result<MachineSpeed, String> {
        if MachineSpeed::is_machine_speed(message) {
            let values: Vec<&str> = (&message[MACHINE_SPEED_PREFIX.len()..]).split(",").collect();
            if values.len() < 2 || values.len() > 3 {
                return Err(format!("Invalid count of machine speed values \"{}\"", message))
            }

            let feed_rate: i32 = match values[0].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read feed rate \"{}\"", values[0]))
            };
            let spindle_programmed_rpm: i32 = match values[1].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read spindle programmed rpm \"{}\"", values[1]))
            };

            let mut spindle_actual_rpm: Option<i32> = None;
            if values.len() == 3 {
                spindle_actual_rpm = match values[2].parse() {
                    Ok(value) => Some(value),
                    Err(_) => return Err(format!("Cannot read spindle actual rpm \"{}\"", values[2]))
                };
            }

            return Ok(MachineSpeed {
                feed_rate,
                spindle_programmed_rpm,
                spindle_actual_rpm,
            })
        }
        Err(format!("Cannot read machine speed \"{}\"", message))
    }

    /// Indicates if message starts with "FS:"
    pub fn is_machine_speed(message: &str) -> bool {
        message.starts_with(MACHINE_SPEED_PREFIX)
    }

    /// Get a reference to the machine speed's feed rate.
    pub fn feed_rate(&self) -> i32 {
        self.feed_rate
    }

    /// Get a reference to the machine speed's spindle programmed rpm.
    pub fn spindle_programmed_rpm(&self) -> i32 {
        self.spindle_programmed_rpm
    }

    /// Get a reference to the machine speed's spindle actual rpm.
    pub fn spindle_actual_rpm(&self) -> Option<i32> {
        self.spindle_actual_rpm
    }
}