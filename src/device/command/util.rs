use super::{settings, state, general, realtime};



pub fn load_device_metadata_commands() -> Vec<String> {
    vec![
        // read all setting info
        settings::GET_ALL.to_string(),
        settings::GET_DETAILS.to_string(),
        settings::GET_GROUPS.to_string(),

        // read firmware info
        state::GET_INFO_EXTENDED.to_string(),

        // read alarm / error codes info
        state::GET_ERROR_CODES.to_string(),
        state::GET_ALARM_CODES.to_string(),

        // query the current device state
        general::GET_NGC_PARAMETERS.to_string(),
        general::GET_STARTUP_LINES.to_string(),
        realtime::STATUS_REPORT.to_string(),
        general::SYNC.to_string()
    ] 
}