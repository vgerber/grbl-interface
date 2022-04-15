# Cross-Platform grbl-interface

The interface provides an easy access to all grblHAL devices. It parses and schedules messages autmatically and provides the user with a structurized representation of the grblHAL device


# Usage

```rust

// first start scanning for available serial devices
// ethernet will be implemented soon
// this function might return under linux ("/dev/ttyACM0", DeviceEndpointType::Serial)
let devices = DeviceService::get_available_devices();

// this service handles all communication with the controller
let mut service = DeviceService::new();

// lets open a device
// on my machine the following device is always available
let device_desc = ("/dev/ttyACM0".to_string(), DeviceEndpointType::Serial);

// now open the serial port
service.open_device(&device_desc).unwrap();

// start with retrieving some firmware information
service.write_device_command(&device_desc.0, format!("{}\n", state::GET_INFO_EXTENDED).as_str()).unwrap();

// next load the settings from the device
// this includes the settings, groups and description for each entry
service.write_device_command(&device_desc.0, format!("{}\n", settings::GET_ALL).as_str()).unwrap();
service.write_device_command(&device_desc.0, format!("{}\n", settings::GET_DETAILS).as_str()).unwrap();
service.write_device_command(&device_desc.0, format!("{}\n", settings::GET_GROUPS).as_str()).unwrap();

// sometimes the last message gets stuck in the device
// this messages flushes the last message
service.write_device_command(&device_desc.0, format!("{}\n", SYNC).as_str()).unwrap();

// wait until the controller has processes each request
thread::sleep(Duration::from_millis(100));

// load the state of the device information after all commands have been send
// and print the whole info object
let info =  service.get_device_info(&device_desc.0).unwrap();
println!("{:#?}", info);
```

The last println statement returns all the information we gathered from the last grbl commands. 
This structure will always be updated and contains all known states since the last start of the service.
E.g. in the following lines, all setting and firmware attributes can be inspected

```
DeviceInfo {
    id: "/dev/ttyACM0",
    firmware_info: FirmwareInfo {
        startup_result: None,
        version: Some(
            FirmwareVersion {
                version: "1.1f.20210715",
                name: "",
            },
        ),
        compile_options: Some(
            CompileOptions {
                options: [
                    VariableSpindleEnabled,
                    LineNumbersEnabled,
                    MistCoolantEnabled,
                    SoftwareDebounce,
                    AlarmStateOnPowerUpWhenHomingInitLock,
                ],
                unknown_options: [],
                block_buffer_size: 35,
                rx_buffer_size: 1024,
                axes_count: Some(
                    3,
                ),
                tool_table_entries_count: Some(
                    0,
                ),
            },
        ),
        extended_compile_options: Some(
            [
                CodeEnumerations,
                LegacyRealtimeCommands,
                EStopSignal,
                ManualToolChange,
                EthernetStreaming,
            ],
        ),
        driver_info: DriverInfo {
            name: Some(
                "iMXRT1062",
            ),
            version: Some(
                "210725",
            ),
            options: Some(
                [
                    "USB.2",
                ],
            ),
        },
        board_info: BoardInfo {
            name: Some(
                "T41U5XBB",
            ),
            storage: Some(
                Storage {
                    emulated: true,
                    storage_type: Flash,
                },
            ),
            aux: Some(
                AuxPorts {
                    digital_in: 1,
                    digital_out: 3,
                    analog_in: 0,
                    analog_out: 0,
                },
            ),
        },
    },
    machine_info: None,
    gcode_state: None,
    last_message: Some(
        Message {
            message: "Emergency stop",
        },
    ),
    last_echo_message: None,
    settings: DeviceSettings {
        settings: {
            44: DeviceSetting [44,4],
            342: DeviceSetting [342,30.0],
            160: DeviceSetting [160,0.000],
            35: DeviceSetting [35,0.0],
            82: DeviceSetting [82,0.000],
            344: DeviceSetting [344,200.0],
            132: DeviceSetting [132,200.000],
            2: DeviceSetting [2,0],
            14: DeviceSetting [14,0],
            120: DeviceSetting [120,10.000],
            18: DeviceSetting [18,0],
            347: DeviceSetting [347,5.0],
            348: DeviceSetting [348,2.500],
            5: DeviceSetting [5,0],
            370: DeviceSetting [370,0],
            400: DeviceSetting [400,0],
            402: DeviceSetting [402,4],
            4: DeviceSetting [4,0],
            12: DeviceSetting [12,0.002],
            34: DeviceSetting [34,0.0],
            305: DeviceSetting [305,23],
            349: DeviceSetting [349,25.000],
            7: DeviceSetting [7,0],
            27: DeviceSetting [27,1.000],
            341: DeviceSetting [341,0],
            24: DeviceSetting [24,25.0],
            19: DeviceSetting [19,0],
            101: DeviceSetting [101,250.000],
            121: DeviceSetting [121,10.000],
            307: DeviceSetting [307,80],
            16: DeviceSetting [16,0],
            15: DeviceSetting [15,0],
            112: DeviceSetting [112,500.000],
            300: DeviceSetting [300,GRBL],
            401: DeviceSetting [401,400],
            131: DeviceSetting [131,200.000],
            0: DeviceSetting [0,10.0],
            304: DeviceSetting [304,255.255.255.0],
            21: DeviceSetting [21,0],
            122: DeviceSetting [122,10.000],
            372: DeviceSetting [372,0],
            92: DeviceSetting [92,0.000],
            90: DeviceSetting [90,0.000],
            62: DeviceSetting [62,0],
            102: DeviceSetting [102,250.000],
            110: DeviceSetting [110,500.000],
            111: DeviceSetting [111,500.000],
            301: DeviceSetting [301,1],
            17: DeviceSetting [17,0],
            70: DeviceSetting [70,3],
            403: DeviceSetting [403,500],
            28: DeviceSetting [28,0.100],
            345: DeviceSetting [345,100.0],
            37: DeviceSetting [37,0],
            25: DeviceSetting [25,500.0],
            29: DeviceSetting [29,0.0],
            23: DeviceSetting [23,0],
            39: DeviceSetting [39,1],
            161: DeviceSetting [161,0.000],
            302: DeviceSetting [302,192.168.5.1],
            80: DeviceSetting [80,1.000],
            20: DeviceSetting [20,0],
            45: DeviceSetting [45,3],
            33: DeviceSetting [33,5000.0],
            31: DeviceSetting [31,0.000],
            100: DeviceSetting [100,250.000],
            91: DeviceSetting [91,0.000],
            11: DeviceSetting [11,0.010],
            64: DeviceSetting [64,0],
            32: DeviceSetting [32,0],
            6: DeviceSetting [6,0],
            130: DeviceSetting [130,200.000],
            22: DeviceSetting [22,0],
            43: DeviceSetting [43,1],
            3: DeviceSetting [3,0],
            84: DeviceSetting [84,0.000],
            85: DeviceSetting [85,10.000],
            26: DeviceSetting [26,250],
            162: DeviceSetting [162,0.000],
            10: DeviceSetting [10,511],
            81: DeviceSetting [81,0.010],
            95: DeviceSetting [95,0.000],
            46: DeviceSetting [46,0],
            303: DeviceSetting [303,192.168.5.1],
            1: DeviceSetting [1,25],
            343: DeviceSetting [343,25.0],
            13: DeviceSetting [13,0],
            65: DeviceSetting [65,0],
            30: DeviceSetting [30,1000.000],
            63: DeviceSetting [63,2],
            36: DeviceSetting [36,100.0],
            40: DeviceSetting [40,0],
        },
        setting_groups: {
            27: DeviceSettingGroup [27,0,Stepper],
            5: DeviceSettingGroup [5,0,Coolant],
            6: DeviceSettingGroup [6,0,Spindle],
            32: DeviceSettingGroup [32,29,Z-axis],
            31: DeviceSettingGroup [31,29,Y-axis],
            1: DeviceSettingGroup [1,0,General],
            9: DeviceSettingGroup [9,0,Tool change],
            18: DeviceSettingGroup [18,0,Aux ports],
            30: DeviceSettingGroup [30,29,X-axis],
            20: DeviceSettingGroup [20,0,Encoders],
            29: DeviceSettingGroup [29,0,Axis],
            15: DeviceSettingGroup [15,0,Networking],
            2: DeviceSettingGroup [2,0,Control signals],
            11: DeviceSettingGroup [11,0,Homing],
            12: DeviceSettingGroup [12,0,Probing],
            14: DeviceSettingGroup [14,0,Jogging],
            3: DeviceSettingGroup [3,0,Limits],
            21: DeviceSettingGroup [21,20,Encoder 0],
        },
        setting_descriptions: {
            80: DeviceSettingDescription [80,8,Spindle P-gain,-,6,###0.000,-,-],
            62: DeviceSettingDescription [62,1,Sleep enable,-,0,-,-,-],
            370: DeviceSettingDescription [370,18,Invert I/O Port inputs,-,1,Port 0,-,-],
            1: DeviceSettingDescription [1,27,Step idle delay,milliseconds,5,####0,-,65535],
            82: DeviceSettingDescription [82,8,Spindle D-gain,-,6,###0.000,-,-],
            2: DeviceSettingDescription [2,27,Step pulse invert,-,4,-,-,-],
            26: DeviceSettingDescription [26,11,Homing switch debounce delay,milliseconds,5,##0,-,-],
            24: DeviceSettingDescription [24,11,Homing locate feed rate,mm/min,6,#####0.0,-,-],
            25: DeviceSettingDescription [25,11,Homing search seek rate,mm/min,6,#####0.0,-,-],
            84: DeviceSettingDescription [84,8,Spindle PID max error,-,6,###0.000,-,-],
            132: DeviceSettingDescription [132,32,Z-axis maximum travel,mm,6,#####0.000,-,-],
            301: DeviceSettingDescription [301,15,IP Mode,-,3,Static,DHCP,AutoIP,-,-],
            27: DeviceSettingDescription [27,11,Homing switch pull-off distance,mm,6,#####0.000,-,-],
            305: DeviceSettingDescription [305,15,Telnet port,-,5,####0,1,65535],
            303: DeviceSettingDescription [303,15,Gateway,-,9,-,-,-],
            341: DeviceSettingDescription [341,9,Tool change mode,-,3,Normal,Manual touch off,Manual touch off @ G59.3,Automatic touch off @ G59.3,Ignore M6,-,-],
            345: DeviceSettingDescription [345,9,Tool change probe pull-off rate,mm/min,6,#####0.0,-,-],
            110: DeviceSettingDescription [110,30,X-axis maximum rate,mm/min,6,#####0.000,-,-],
            307: DeviceSettingDescription [307,15,Websocket port,-,5,####0,1,65535],
            64: DeviceSettingDescription [64,1,Force init alarm,-,0,-,-,-],
            112: DeviceSettingDescription [112,32,Z-axis maximum rate,mm/min,6,#####0.000,-,-],
            348: DeviceSettingDescription [348,4,Dual axis length fail min,mm,6,#####0.000,-,-],
            349: DeviceSettingDescription [349,4,Dual axis length fail max,mm,6,#####0.000,-,-],
            29: DeviceSettingDescription [29,27,Pulse delay,microseconds,6,#0.0,-,10],
            36: DeviceSettingDescription [36,6,Spindle PWM max value,percent,6,##0.0,-,100],
            122: DeviceSettingDescription [122,32,Z-axis acceleration,mm/sec^2,6,#####0.000,-,-],
            10: DeviceSettingDescription [10,1,Status report options,-,1,Position in machine coordinate,Buffer state,Line numbers,Feed & speed,Pin state,Work coordinate offset,Overrides,Probe coordinates,Buffer sync on WCO change,Parser state,Alarm substatus,Run substatus,-,-],
            347: DeviceSettingDescription [347,4,Dual axis length fail,percent,6,##0.0,0,100],
            400: DeviceSettingDescription [400,21,Encoder mode,-,3,Universal,Feed rate override,Rapid rate override,Spindle RPM override,-,-],
            304: DeviceSettingDescription [304,15,Netmask,-,9,-,-,-],
            23: DeviceSettingDescription [23,11,Homing direction invert,-,4,-,-,-],
            33: DeviceSettingDescription [33,6,Spindle PWM frequency,Hz,6,#####0,-,-],
            43: DeviceSettingDescription [43,11,Homing passes,-,5,##0,1,128],
            65: DeviceSettingDescription [65,12,Probing feed override,-,0,-,-,-],
            95: DeviceSettingDescription [95,7,Spindle sync PID max I error,-,6,###0.000,-,-],
            46: DeviceSettingDescription [46,11,Axes homing, third pass,-,4,-,-,-],
            121: DeviceSettingDescription [121,31,Y-axis acceleration,mm/sec^2,6,#####0.000,-,-],
            161: DeviceSettingDescription [161,31,Y-axis backlash compensation,mm,6,#####0.000,-,-],
            32: DeviceSettingDescription [32,1,Mode of operation,-,3,Normal,Laser mode,Lathe mode,-,-],
            45: DeviceSettingDescription [45,11,Axes homing, second pass,-,4,-,-,-],
            40: DeviceSettingDescription [40,14,Limit jog commands,-,0,-,-,-],
            28: DeviceSettingDescription [28,1,G73 Retract distance,mm,6,#####0.000,-,-],
            34: DeviceSettingDescription [34,6,Spindle PWM off value,percent,6,##0.0,-,100],
            7: DeviceSettingDescription [7,6,Disable spindle with zero speed,-,0,-,-,-],
            63: DeviceSettingDescription [63,1,Feed hold actions,-,1,Disable laser during hold,Restore spindle and coolant state on resume,-,-],
            100: DeviceSettingDescription [100,30,X-axis travel resolution,step/mm,6,#####0.000,-,-],
            302: DeviceSettingDescription [302,15,IP Address,-,9,-,-,-],
            401: DeviceSettingDescription [401,21,Encoder counts per revolution,-,5,###0,1,-],
            130: DeviceSettingDescription [130,30,X-axis maximum travel,mm,6,#####0.000,-,-],
            91: DeviceSettingDescription [91,7,Spindle sync I-gain,-,6,###0.000,-,-],
            14: DeviceSettingDescription [14,2,Invert control pins,-,1,N/A,Feed hold,Cycle start,N/A,N/A,N/A,EStop,-,-],
            85: DeviceSettingDescription [85,8,Spindle PID max I error,-,6,###0.000,-,-],
            17: DeviceSettingDescription [17,2,Pullup disable control pins,-,1,N/A,Feed hold,Cycle start,N/A,N/A,N/A,EStop,-,-],
            21: DeviceSettingDescription [21,3,Hard limits enable,-,2,Enable,Strict mode,-,-],
            6: DeviceSettingDescription [6,12,Invert probe pin,-,0,-,-,-],
            131: DeviceSettingDescription [131,31,Y-axis maximum travel,mm,6,#####0.000,-,-],
            13: DeviceSettingDescription [13,1,Report in inches,-,0,-,-,-],
            3: DeviceSettingDescription [3,27,Step direction invert,-,4,-,-,-],
            162: DeviceSettingDescription [162,32,Z-axis backlash compensation,mm,6,#####0.000,-,-],
            343: DeviceSettingDescription [343,9,Tool change locate feed rate,mm/min,6,#####0.0,-,-],
            344: DeviceSettingDescription [344,9,Tool change search seek rate,mm/min,6,#####0.0,-,-],
            37: DeviceSettingDescription [37,27,Steppers deenergize,-,4,-,-,-],
            18: DeviceSettingDescription [18,3,Pullup disable limit pins,-,4,-,-,-],
            31: DeviceSettingDescription [31,6,Minimum spindle speed,RPM,6,#####0.000,-,-],
            70: DeviceSettingDescription [70,15,Network Services,-,1,Telnet,Websocket,-,-],
            372: DeviceSettingDescription [372,18,Invert I/O Port outputs,-,1,Port 0,Port 1,Port 2,-,-],
            0: DeviceSettingDescription [0,27,Step pulse time,microseconds,6,#0.0,2.0,-],
            44: DeviceSettingDescription [44,11,Axes homing, first pass,-,4,-,-,-],
            12: DeviceSettingDescription [12,1,Arc tolerance,mm,6,#####0.000,-,-],
            39: DeviceSettingDescription [39,1,Enable legacy RT commands,-,0,-,-,-],
            101: DeviceSettingDescription [101,31,Y-axis travel resolution,step/mm,6,#####0.000,-,-],
            30: DeviceSettingDescription [30,6,Maximum spindle speed,RPM,6,#####0.000,-,-],
            16: DeviceSettingDescription [16,6,Invert spindle signals,-,1,Spindle enable,Spindle direction,-,-],
            35: DeviceSettingDescription [35,6,Spindle PWM min value,percent,6,##0.0,-,100],
            81: DeviceSettingDescription [81,8,Spindle I-gain,-,6,###0.000,-,-],
            403: DeviceSettingDescription [403,21,Encoder double click sensitivity,ms,5,##0,100,900],
            11: DeviceSettingDescription [11,1,Junction deviation,mm,6,#####0.000,-,-],
            22: DeviceSettingDescription [22,11,Homing cycle,-,2,Enable,Enable single axis commands,Homing on startup required,Set machine origin to 0,Two switches shares one input pin,Allow manual,Override locks,Keep homed status on reset,-,-],
            402: DeviceSettingDescription [402,21,Encoder counts per detent,-,5,#0,1,-],
            4: DeviceSettingDescription [4,27,Invert step enable pin(s),-,4,-,-,-],
            90: DeviceSettingDescription [90,7,Spindle sync P-gain,-,6,###0.000,-,-],
            15: DeviceSettingDescription [15,5,Invert coolant pins,-,1,Flood,Mist,-,-],
            102: DeviceSettingDescription [102,32,Z-axis travel resolution,step/mm,6,#####0.000,-,-],
            19: DeviceSettingDescription [19,12,Pullup disable probe pin,-,0,-,-,-],
            20: DeviceSettingDescription [20,3,Soft limits enable,-,0,-,-,-],
            92: DeviceSettingDescription [92,7,Spindle sync D-gain,-,6,###0.000,-,-],
            300: DeviceSettingDescription [300,15,Hostname,-,7,x(64),-,64],
            342: DeviceSettingDescription [342,9,Tool change probing distance,mm,6,#####0.0,-,-],
            120: DeviceSettingDescription [120,30,X-axis acceleration,mm/sec^2,6,#####0.000,-,-],
            5: DeviceSettingDescription [5,3,Invert limit pins,-,4,-,-,-],
            160: DeviceSettingDescription [160,30,X-axis backlash compensation,mm,6,#####0.000,-,-],
            111: DeviceSettingDescription [111,31,Y-axis maximum rate,mm/min,6,#####0.000,-,-],
        },
    },
}
```

