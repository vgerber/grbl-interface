use std::result::Result;


pub enum MachineStatusName {
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

pub fn get_machine_status_name(name: &str) -> Result<MachineStatusName, String> {
    match name {
        "Idle" => Ok(MachineStatusName::Idle),
        "Run" => Ok(MachineStatusName::Run),
        "Hold" => Ok(MachineStatusName::Hold),
        "Jog" => Ok(MachineStatusName::Jog),
        "Alarm" => Ok(MachineStatusName::Alarm),
        "Door" => Ok(MachineStatusName::Door),
        "Check" => Ok(MachineStatusName::Check),
        "Home" => Ok(MachineStatusName::Home),
        "Sleep" => Ok(MachineStatusName::Sleep),
        "Tool" => Ok(MachineStatusName::Tool),
        _ => Err(format!("Unknown status name \"{}\"", name))
    }
}

pub struct MachineStatus {
    status: MachineStatusName,
    sub_status: Option<i8>,
}

impl MachineStatus {
    
    pub fn from(message: &str) -> Result<MachineStatus, String> {

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

            return Ok(MachineStatus {
                status,
                sub_status
            })

        }

        Err(format!("Cannot read machine status \"{}\"", message))
    }
}