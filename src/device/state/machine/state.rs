use std::result::Result;


pub enum MachineStateName {
    Idle,
    Run,
    Hold,
    Jog,
    Alarm,
    Door,
    Check,
    Home,
    Sleep,
    Tool,
}

pub fn get_machine_status_name(name: &str) -> Result<MachineStateName, String> {
    match name {
        "Idle" => Ok(MachineStateName::Idle),
        "Run" => Ok(MachineStateName::Run),
        "Hold" => Ok(MachineStateName::Hold),
        "Jog" => Ok(MachineStateName::Jog),
        "Alarm" => Ok(MachineStateName::Alarm),
        "Door" => Ok(MachineStateName::Door),
        "Check" => Ok(MachineStateName::Check),
        "Home" => Ok(MachineStateName::Home),
        "Sleep" => Ok(MachineStateName::Sleep),
        "Tool" => Ok(MachineStateName::Tool),
        _ => Err(format!("Unknown status name \"{}\"", name))
    }
}

pub struct MachineState {
    status: MachineStateName,
    sub_status: Option<i8>,
}

impl MachineState {
    
    /// Reads report status section
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores status "Hold" and sub status "0"
    /// let response = MachineStatus::from("Hold:0");
    /// ```
    pub fn from(message: &str) -> Result<MachineState, String> {

        let status_segments: Vec<&str> = message.split(":").collect();
        if status_segments.len() > 0 {

            let status = match get_machine_status_name(status_segments[0]) {
                Ok(status) => status,
                Err(err) => return Err(err),
            };

            let mut sub_status: Option<i8> = None;
            if status_segments.len() > 1 {
                sub_status = match status_segments[1].parse() {
                    Ok(sub_status) => Some(sub_status),
                    Err(_) => return Err(format!("Cannot read machine sub status \"{}\"", status_segments[1]))
                };
            }

            return Ok(MachineState {
                status,
                sub_status
            })

        }

        Err(format!("Cannot read machine status \"{}\"", message))
    }

    pub fn status(&self) -> &MachineStateName {
        &self.status
    } 

    pub fn sub_status(&self) -> &Option<i8> {
        &self.sub_status
    }
}