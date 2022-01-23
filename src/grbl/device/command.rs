mod general {
    const STATUS_REPORT: &str = "?";
    const CYCLE_START: &str = "~";
    const FEED_HOLD: &str = "!";
    const UNLOCK: &str = "$X";
    const START_HOMING: &str = "$H";
    const CHECK: &str = "$C";
    
    const GET_NGC_PARAMETERS: &str = "$#";
    const GET_STARTUP_LINES: &str = "$N";    
    
    const GET_PROGRAM_DEMARCATION: &str = "%";
    const NO_TOOL: &str = "None";    
}

mod state {
    const GET_PARSER_STATE: &str = "$G";
    const GET_INFO: &str = "$I";
    const GET_INFO_EXTENDED: &str = "$I+";
    const GET_ALARM_CODES: &str = "$EA";
    const GET_ERROR_CODES: &str = "$EE";
}

mod settings {
    const GET: &str = "$$";
    const GET_ALL: &str = "$+";
    const GET_DETAILS: &str = "$ES";
    const GET_GROUPS: &str = "$EG";
}

mod sdcard {
    const MOUNT: &str = "FM";
    const DIR: &str = "F";
    const REWIND: &str = "FR";
    const RUN: &str = "F=";
    const UNLINK: &str = "FD=";
    const DUMP: &str = "F<=";
}

mod format {
    const METRIC: &str = "###0.000";
    const IMPERIAL: &str = "##0.0000";
}

mod signals {
    const BASE: &str = "XYZABCEPRDHSBTOW";
    const THC: &str = "AERTOVHDU";
}