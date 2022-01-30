use std::collections::HashMap;

pub enum SettingIndex {
    // Stepper 
    StepPulseMicroseconds = 0,
    StepIdleDelayMilliseconds = 1,
    StepPulsePinsInvert = 2,
    StepDirectionPinsInvert = 3,
    StepEnablePinsInvert = 4,
    StepPulseDelayMicroseconds = 29, // grblHal

    // Limit & Probe
    LimitPinsInvert = 5,
    ProbePinInvert = 6,
    StatusReportOptions = 10,
    JunctionDeviation = 11,
    ArcTolerance = 12,
    ReportInInches = 13,
    ControlPinsInvert = 14, // grblHAL
    CoolantPinsInvert = 15, // grblHAL
    SpindlePinsInvert = 16, // grblHAL
    ControlPullUpDisable = 17, // grblHAL
    LimitPullUpDisable = 18, // grblHAL
    ProbePullUpDisable = 19, // grblHAL
    SoftLimitsEnable = 20,
    HardLimitsEnable = 21,
    
    // Homing
    HomingCycleEnable = 22,
    HomingDirectionPinInvert = 23,
    HomingFeedRate = 24,
    HomingSeekRate = 25,
    HomingDebounceDelay = 26,
    HomingPullOffDistance = 27,
    HomingLocateCycles = 43, // grblHal
    HomingCycle1 = 44, // grblHal
    HomingCycle2 = 45, // grblHal
    HomingCycle3 = 46, // grblHal
    HomingCycle4 = 47, // grblHal
    HomingCycle5 = 48, // grblHal 
    HomingCycle6 = 49, // grblHal

    // Spindle Control
    G73RetractDistance = 28, // grblHal
    SpindleSpeedMax = 30,
    SpindleSpeedMin = 31,
    LaserModeEnable = 32, 
    SpindlePWMFreq = 33, // grblHal
    SpindlePWMOffValue = 34, // grblHal
    SpindlePWMMinValue = 35, // grblHal
    SpindlePWMMaxValue = 36, // grblHal
    StepperDeenergizeMask = 37,
    SpindlePPR = 38,
    EnableRealtimeCommands = 39,

    // Probing TPW (Tool Probe Workpiece)
    SpindleSpeedTolerance = 340,
    SpindleToolChangeMode = 341,
    SpindleProbingDistance = 342,
    SpindleProbingSlowFeedRate = 343,
    SpindleProbingSeekFeedrate = 344,

    // Jog Settings
    JogEnableSoftLimits = 40,
    JogStepSpeed = 50,
    JogSlowSpeed = 51,
    JogFastSpeed = 52,
    JogStepDistance = 53,
    JogSlowDistance = 54,
    JogFastDistance = 55,

    // Per axis settings
    XAxisSteps = 100,
    YAxisSteps = 101,
    ZAxisSteps = 102,
    XAxisMaxFeedRate = 110,
    YAxisMaxFeedRate = 111,
    ZAxisMaxFeedRate = 112,
    XAxisAcceleration = 120,
    YAxisAcceleration = 121,
    ZAxisAcceleration = 122,
    XAxisMaxTravelDistance = 130,
    YAxisMaxTravelDistance = 131,
    ZAxisMaxTravelDistance = 132,

    MotorCurrentBase = 140,
    MicroStepsBase = 150,
    StallGuardBase = 200,
    // End per axis settings

    RestoreOverridesAfterProgram = 60,
    IgnoreSafteyDoorWhenIdle = 61,
    SleepEnable = 62,
    LaserDisableWhenHold = 63,
    ForceAlarmOnStartup = 64,
    HomingRequiredOnStartup = 65,

    // Network
    NetworkService = 70,
    NetworkBluetoothName = 71,
    NetworkBluetoothServiceName = 72,
    NetworkWifiMode = 73,
    NetworkWifiStationSSID = 74,
    NetworkWifiStationPW = 75,
    NetworkWifiAPSSID = 76,
    NetworkWifiAPPW = 77,
    NetworkWifiAPCountry = 78,
    NetworkWifiAPChannel = 79,

    // PID
    SpindlePIDRegulatorProportionalGain = 80,
    SpindlePIDRegulatorIntegralGain = 81,
    SpindlePIDRegulatorDerivativeGain = 82,
    SpindlePIDMaxOutputError = 84,
    SpindlePIDRegulatorMaxIntegralError = 85,
    SpindleSyncMotionPIDRegulatorGain = 90,
    SpindleSyncMotionPIDRegulatorIntegralGain = 91,
    SpindleSyncMotionPIDRegulatorDerivativeGain = 92,

    
}

pub enum SettingValue {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
}

pub struct DeviceSetting {
    index: SettingIndex,
    value: SettingValue,
}


impl DeviceSetting {
    
    pub fn new(index: SettingIndex, value: SettingValue) -> Self {
        DeviceSetting {
            index,
            value,
        }
    }

    pub fn index(&self) -> &SettingIndex {
        &self.index
    }

    pub fn value(&self) -> &SettingValue {
        &self.value
    }
}

pub struct DeviceSettings {

    settings: HashMap<i32, DeviceSetting>
}


impl DeviceSettings {
    
    pub fn new(&mut self) {
        self.settings = HashMap::new();

        
    }
}