use std::result::Result;

// All available compile options for grbl 1.1 firmware
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompileOption {
    VariableSpindleEnabled,
    LineNumbersEnabled,
    MistCoolantEnabled,
    CoreXYEnabled,
    ParkingMotionEnabled,
    HomingForceOriginEnabled,
    HomingSingleAxisEnabled,
    TwoLimitSwitchOnAxisEnabled,
    AllowFeedRateOverridesInProbeCycles,
    RestoreAllEEPROMDisabled,
    RestoreEEPROMDollarSettingsDisabled,
    RestoreEEPROMParameterDataDisabled,
    BuildInfoWriteUserStringDisabled,
    ForceSyncEEPROMWriteDisabled,
    ForceSyncWorkCoordinateOffsetChangeDisabled,
    AlarmStateOnPowerUpWhenHomingInitLock,
    DualAxisMotorsWithSelfSquaringEnabled,
    SoftwareDebounce
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExtendedCompileOption {
    AutomaticToolChange,
    BlockDeleteSignal,
    BluetoothStreaming,
    CodeEnumerations,
    EStopSignal,
    EthernetStreaming,
    Homing,
    LatheMode,
    MPGMode,
    NoProbeInput,
    Odometer,
    OptionalStopSignal,
    ProbeConnectedSignal,
    PIDLog,
    LegacyRealtimeCommands,
    RealtimeCommands,
    SettingsDescriptions,
    SDCardStreaming,
    SpindelSync,
    ManualToolChange,
    WifiStreaming,
}

/// Get compile option enum from option value
pub fn get_compile_option(option: &str) -> Result<CompileOption, String> {
    match option {
        "V" => Ok(CompileOption::VariableSpindleEnabled),
        "N" => Ok(CompileOption::LineNumbersEnabled),
        "M" => Ok(CompileOption::MistCoolantEnabled),
        "C" => Ok(CompileOption::CoreXYEnabled),
        "P" => Ok(CompileOption::ParkingMotionEnabled),
        "Z" => Ok(CompileOption::HomingForceOriginEnabled),
        "H" => Ok(CompileOption::HomingSingleAxisEnabled),
        "T" => Ok(CompileOption::TwoLimitSwitchOnAxisEnabled),
        "A" => Ok(CompileOption::AllowFeedRateOverridesInProbeCycles),
        "*" => Ok(CompileOption::RestoreAllEEPROMDisabled),
        "$" => Ok(CompileOption::RestoreEEPROMDollarSettingsDisabled),
        "#" => Ok(CompileOption::RestoreEEPROMParameterDataDisabled),
        "I" => Ok(CompileOption::BuildInfoWriteUserStringDisabled),
        "E" => Ok(CompileOption::ForceSyncEEPROMWriteDisabled),
        "W" => Ok(CompileOption::ForceSyncWorkCoordinateOffsetChangeDisabled),
        "L" => Ok(CompileOption::AlarmStateOnPowerUpWhenHomingInitLock),
        "2" => Ok(CompileOption::DualAxisMotorsWithSelfSquaringEnabled),
        "S" => Ok(CompileOption::SoftwareDebounce),
        o => Err(format!("Invalid option {}", o)),
    }
}

/// Get extended compile option enum from option value
pub fn get_extended_compile_option(option: &str) -> Result<ExtendedCompileOption, String> {
    match option {
        "ATC" => Ok(ExtendedCompileOption::AutomaticToolChange),
        "BD" => Ok(ExtendedCompileOption::BlockDeleteSignal),
        "BT" => Ok(ExtendedCompileOption::BluetoothStreaming),
        "ENUMS" => Ok(ExtendedCompileOption::CodeEnumerations),
        "ES" => Ok(ExtendedCompileOption::EStopSignal),
        "ETH" => Ok(ExtendedCompileOption::EthernetStreaming),
        "HOME" => Ok(ExtendedCompileOption::Homing),
        "LATHE" => Ok(ExtendedCompileOption::LatheMode),
        "MPG" => Ok(ExtendedCompileOption::MPGMode),
        "NOPROBE" => Ok(ExtendedCompileOption::NoProbeInput),
        "ODO" => Ok(ExtendedCompileOption::Odometer),
        "OS" => Ok(ExtendedCompileOption::OptionalStopSignal),
        "PC" => Ok(ExtendedCompileOption::ProbeConnectedSignal),
        "PID" => Ok(ExtendedCompileOption::PIDLog),
        "RT+" => Ok(ExtendedCompileOption::LegacyRealtimeCommands),
        "RT-" => Ok(ExtendedCompileOption::RealtimeCommands),
        "SED" => Ok(ExtendedCompileOption::SettingsDescriptions),
        "SD" => Ok(ExtendedCompileOption::SDCardStreaming),
        "SS" => Ok(ExtendedCompileOption::SpindelSync),
        "TC" => Ok(ExtendedCompileOption::ManualToolChange),
        "WIFI" => Ok(ExtendedCompileOption::WifiStreaming),
        o => Err(format!("Invalid option \"{}\"", o)),
    }
}

