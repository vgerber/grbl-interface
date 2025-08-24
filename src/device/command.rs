pub mod util;

/// Indicates if the executor of this command should expect a status code
///
/// # Status Codes
/// ```text
/// // Command executed successful
/// $ Command
/// ok
///
/// // Command failed
/// $ Command
/// error:2
/// ```
///
pub fn has_status_response(command: &str) -> bool {
    match command {
        "?" | "~" | "!" => true,
        _ => false,
    }
}

pub mod realtime {
    pub const STATUS_REPORT: &str = "?";
    pub const CYCLE_START: &str = "~";
    pub const FEED_HOLD: &str = "!";
}

pub mod general {
    pub const SYNC: &str = "\r\n";

    pub mod home {
        pub const HOME_ALL: &str = "$H\r";
        pub const HOME_X: &str = "$HX\r";
        pub const HOME_Y: &str = "$HY\r";
        pub const HOME_Z: &str = "$HZ\r";
        pub const HOME_A: &str = "$HA\r";
        pub const HOME_B: &str = "$HB\r";
        pub const HOME_C: &str = "$HC\r";
    }

    pub const UNLOCK: &str = "$X\r";
    pub const CHECK: &str = "$C\r";

    pub const GET_NGC_PARAMETERS: &str = "$#\r";
    pub const GET_STARTUP_LINES: &str = "$N\r";

    pub const NO_TOOL: &str = "None";
}

pub mod state {
    pub const GET_PARSER_STATE: &str = "$G\r";
    pub const GET_INFO: &str = "$I\r";
    pub const GET_INFO_EXTENDED: &str = "$I+\r";
    pub const GET_ALARM_CODES: &str = "$EA\r";
    pub const GET_ERROR_CODES: &str = "$EE\r";
}

pub mod settings {
    pub const GET: &str = "$$\r";
    pub const GET_ALL: &str = "$+\r";
    pub const GET_DETAILS: &str = "$ES\r";
    pub const GET_GROUPS: &str = "$EG\r";
    pub const SETTINGS_RESET: &str = "RST\r";
}

pub mod sdcard {
    pub const MOUNT: &str = "FM\r";
    pub const DIR: &str = "F\r";
    pub const REWIND: &str = "FR\r";
    pub const RUN: &str = "F=\r";
    pub const UNLINK: &str = "FD=\r";
    pub const DUMP: &str = "F<=\r";
}

pub mod gcode {
    pub const PROGRAM_DEMARCATION: &str = "%";
}

pub mod format {
    pub const METRIC: &str = "###0.000";
    pub const IMPERIAL: &str = "##0.0000";
}

pub mod signals {
    pub const BASE: &str = "XYZABCEPRDHSBTOW";
    pub const THC: &str = "AERTOVHDU";
}
