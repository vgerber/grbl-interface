mod mask {
    const OFF: u32                  = 0b00000000000000001u32;
    const LIMIT_X: u32              = 0b00000000000000010u32;
    const LIMIT_Y: u32              = 0b00000000000000100u32;
    const LIMIT_Z: u32              = 0b00000000000001000u32;
    const LIMIT_A: u32              = 0b00000000000010000u32;
    const LIMIT_B: u32              = 0b00000000000100000u32;
    const LIMIT_C: u32              = 0b00000000001000000u32;
    const E_STOP: u32               = 0b00000000010000000u32;
    const PROBE: u32                = 0b00000000100000000u32;
    const RESET: u32                = 0b00000001000000000u32;
    const SAFETY_DOOR: u32          = 0b00000010000000000u32;
    const HOLD: u32                 = 0b00000100000000000u32;
    const CYCLE_START: u32          = 0b00001000000000000u32;
    const BLOCK_DELETE: u32         = 0b00010000000000000u32;
    const OPTIONAL_STOP: u32        = 0b00100000000000000u32;
    const PROBE_DISCONNECTED: u32   = 0b01000000000000000u32;
    const MOTOR_WARNING: u32        = 0b10000000000000000u32;
}