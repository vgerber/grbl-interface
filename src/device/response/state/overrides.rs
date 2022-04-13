
const OVERRIDE_VALUES_PREFIX: &str = "Ov:";

#[derive(Clone, Copy)]
pub struct Overrides {
    feed_rate_percentage: i32,
    rapids_percentage: i32,
    spindle_speed_percentage: i32
}

impl Overrides {
    
    /// Creates overrides from \<int\>,\<int\>,\<int\>
    /// 
    /// # Examples
    /// ```
    /// let overrides = OverridesValues::from("Ov:100,100,20")
    /// ```
    pub fn from(message: &str) -> Result<Overrides, String> {
        if Overrides::is_overrides_values(message) {
            let values: Vec<&str> = (&message[OVERRIDE_VALUES_PREFIX.len()..]).split(",").collect();
            if values.len() != 3 {
                return Err(format!("Invalid count of override values \"{}\"", message))
            }
            let feed_rate: i32 = match values[0].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read feed rate override \"{}\"", values[0]))
            };
            let rapids: i32 = match values[1].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read rapids override \"{}\"", values[1]))
            };
            let spindle_speed: i32 = match values[2].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read spindle speed override \"{}\"", values[2]))
            };
            return Ok(Overrides {
                feed_rate_percentage: feed_rate,
                rapids_percentage: rapids,
                spindle_speed_percentage: spindle_speed,
            })
        }
        Err(format!("Cannot read overrides \"{}\"", message))
    }

    pub fn is_overrides_values(message: &str) -> bool {
        message.starts_with(OVERRIDE_VALUES_PREFIX)
    }

    /// Get a reference to the overrides values's feed rate percentage.
    pub fn feed_rate_percentage(&self) -> i32 {
        self.feed_rate_percentage
    }

    /// Get a reference to the overrides values's rapids percentage.
    pub fn rapids_percentage(&self) -> i32 {
        self.rapids_percentage
    }

    /// Get a reference to the overrides values's spindle speed percentage.
    pub fn spindle_speed_percentage(&self) -> i32 {
        self.spindle_speed_percentage
    }
}