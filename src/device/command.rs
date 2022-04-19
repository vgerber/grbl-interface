pub mod util;

pub mod general {
    pub const SYNC: &str = "\0\n";

    pub const STATUS_REPORT: &str = "?";
    pub const CYCLE_START: &str = "~";
    pub const FEED_HOLD: &str = "!";
    pub const UNLOCK: &str = "$X";
    
    pub mod home {
        pub const HOME_ALL: &str = "$H\n";
        pub const HOME_X: &str = "$HX\n";
        pub const HOME_Y: &str = "$HY\n";
        pub const HOME_Z: &str = "$HZ\n";
        pub const HOME_A: &str = "$HA\n";
        pub const HOME_B: &str = "$HB\n";
        pub const HOME_C: &str = "$HC\n";
    }

    pub const CHECK: &str = "$C";
    
    pub const GET_NGC_PARAMETERS: &str = "$#";
    pub const GET_STARTUP_LINES: &str = "$N";    
    
    pub const GET_PROGRAM_DEMARCATION: &str = "%";
    pub const NO_TOOL: &str = "None";    
}

pub mod state {
    pub const GET_PARSER_STATE: &str = "$G\n";
    pub const GET_INFO: &str = "$I\n";
    pub const GET_INFO_EXTENDED: &str = "$I+\n";
    pub const GET_ALARM_CODES: &str = "$EA\n";
    pub const GET_ERROR_CODES: &str = "$EE\n";
}

pub mod settings {
    pub const GET: &str = "$$\n";
    pub const GET_ALL: &str = "$+\n";
    pub const GET_DETAILS: &str = "$ES\n";
    pub const GET_GROUPS: &str = "$EG\n";
    pub const SETTINGS_RESET: &str = "RST\n";
}

pub mod sdcard {
    pub const MOUNT: &str = "FM\n";
    pub const DIR: &str = "F\n";
    pub const REWIND: &str = "FR\n";
    pub const RUN: &str = "F=\n";
    pub const UNLINK: &str = "FD=\n";
    pub const DUMP: &str = "F<=\n";
}

pub mod format {
    pub const METRIC: &str = "###0.000";
    pub const IMPERIAL: &str = "##0.0000";
}

pub mod signals {
    pub const BASE: &str = "XYZABCEPRDHSBTOW";
    pub const THC: &str = "AERTOVHDU";
}