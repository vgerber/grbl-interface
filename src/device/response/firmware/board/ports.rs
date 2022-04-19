 
const AUX_PREFIX: &str = "[AUX IO:";
const AUX_SUFFIX: &str = "]";

/// Available auxillary ports
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuxPorts {
    digital_in: u16,
    digital_out: u16,
    analog_in: u16,
    analog_out: u16,
}

impl AuxPorts {

    /// Reads the aux ports line
    /// 
    /// # Error
    /// Returns an error when parsing fails
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// use grbli::device::response::firmware::board::aux::*;
    /// 
    /// let aux = AuxPorts::from("[AUX IO:1,3,0,5]").unwrap();
    /// assert_eq!(aux.digital_in(), 1);
    /// assert_eq!(aux.digital_out(), 3);
    /// assert_eq!(aux.analog_in(), 0);
    /// assert_eq!(aux.analog_out(), 5);
    /// ```
    pub fn from(message: &str) -> Result<Self, String> {
        if AuxPorts::is_response(message) {

            let ports: Vec<&str> = message.strip_prefix(AUX_PREFIX).unwrap().strip_suffix(AUX_SUFFIX).unwrap().split(",").collect();

            if ports.len() != 4 {
                return Err(format!("Invalid count aux ports: \"{}\"", message));
            }

            let digital_in = match ports[0].parse::<u16>() {
                Ok(count) => count,
                Err(_) => return Err(format!("Cannot parse digital in port count: \"{}\"", ports[0]))
            };

            let digital_out = match ports[1].parse::<u16>() {
                Ok(count) => count,
                Err(_) => return Err(format!("Cannot parse digital out port count: \"{}\"", ports[1]))
            };

            let analog_in = match ports[2].parse::<u16>() {
                Ok(count) => count,
                Err(_) => return Err(format!("Cannot parse analog in port count: \"{}\"", ports[2]))
            };

            let analog_out = match ports[3].parse::<u16>() {
                Ok(count) => count,
                Err(_) => return Err(format!("Cannot parse analog out port count: \"{}\"", ports[3]))
            };

            return Ok(AuxPorts {
                digital_in,
                digital_out,
                analog_in,
                analog_out
            });
        }

        Err(format!("Cannot read aux ports: \"{}\"", message))
    }

    pub fn is_response(message: &str) -> bool {
        message.starts_with(AUX_PREFIX) && message.ends_with(AUX_SUFFIX)
    }

    /// Get the aux ports's digital in.
    #[must_use]
    pub fn digital_in(&self) -> u16 {
        self.digital_in
    }

    /// Get the aux ports's digital out.
    #[must_use]
    pub fn digital_out(&self) -> u16 {
        self.digital_out
    }

    /// Get the aux ports's analog in.
    #[must_use]
    pub fn analog_in(&self) -> u16 {
        self.analog_in
    }

    /// Get the aux ports's analog out.
    #[must_use]
    pub fn analog_out(&self) -> u16 {
        self.analog_out
    }
}