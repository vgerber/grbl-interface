pub mod general {
    pub const STATUS_REPORT: &str = "?";
    pub const CYCLE_START: &str = "~";
    pub const FEED_HOLD: &str = "!";
    pub const UNLOCK: &str = "$X";
    pub const START_HOMING: &str = "$H";
    pub const CHECK: &str = "$C";
    
    pub const GET_NGC_PARAMETERS: &str = "$#";
    pub const GET_STARTUP_LINES: &str = "$N";    
    
    pub const GET_PROGRAM_DEMARCATION: &str = "%";
    pub const NO_TOOL: &str = "None";    
}

pub mod state {
    pub const GET_PARSER_STATE: &str = "$G";
    pub const GET_INFO: &str = "$I";
    pub const GET_INFO_EXTENDED: &str = "$I+";
    pub const GET_ALARM_CODES: &str = "$EA";
    pub const GET_ERROR_CODES: &str = "$EE";
}

pub mod settings {
    pub const GET: &str = "$$";
    pub const GET_ALL: &str = "$+";
    pub const GET_DETAILS: &str = "$ES";
    pub const GET_GROUPS: &str = "$EG";
}

pub mod sdcard {
    pub const MOUNT: &str = "FM";
    pub const DIR: &str = "F";
    pub const REWIND: &str = "FR";
    pub const RUN: &str = "F=";
    pub const UNLINK: &str = "FD=";
    pub const DUMP: &str = "F<=";
}

pub mod format {
    pub const METRIC: &str = "###0.000";
    pub const IMPERIAL: &str = "##0.0000";
}

pub mod signals {
    pub const BASE: &str = "XYZABCEPRDHSBTOW";
    pub const THC: &str = "AERTOVHDU";
}