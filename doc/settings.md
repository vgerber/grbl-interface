# Settings for GRBL[HAL] Controller

## Setting Values

| Code | Name | Unit | Type | Description | Module |
| ----:| ---- | ---- | ---- | ----------- | ------ |
| 0 | StepPulseTime | Microseconds | Int |  |  |
| 1 | StepIdleDelay | Milliseconds | Int |  |  |
| 2 | StepPulsePinsInvert | Mask | Int |  |  |
| 3 | StepDirectionPinsInvert | Mask  | Int |  |  |
| 4 | StepEnablePinsInvert | Mask | Bool |  |  |
| 5 | LimitPinsInvert | Mask | Bool |  |  |
| 6 | ProbePinInvert | Mask | Bool |  |  |
| 10 | StatusReportOptions | Mask | Int | [Report Mask](#Report-Mask) |  |
| 11 | JunctionDeviation | Millimeters | Float | |  |
| 12 | ArcTolerance | Millimeters | Float |  |  |
| 13 | ReportInInches | Mask | Bool |  |  |
| 14 | ControlPinsInvert | Mask | Int | [Control Mask](#Control-Mask) | grblHAL  |
| 15 | CoolantPinsInvert | Mask | Int | [Control Mask](#Control-Mask) | grblHAL |
| 16 | SpindlePinsInvert | Mask | Int | [Spindle Mask](#Spindle-Mask) | grblHAL |
| 17 | ControlPullUpDisable | Mask | Int | Disables control pins pull up ([Control Mask](#Control-Mask)) | grblHAL |
| 18 | LimitPullDisable | Mask | Int | Disables limit pins pull up ([Axis Mask](#Axis-Mask)) | grblHAL |
| 19 | ProbePullUpDisable | Mask | Bool | Disables porbe pin pull up | grblHAL |
| 20 | SoftLimitsEnable | Mask | Bool |  |  |
| 21 | HardLimitsEnable | Mask | Int | [Hard Limit Mask](#Hard-Limit-Mask) | |
| 22 | HomingCycleEnable | Mask | Bool |  |  |
| 23 | HomingDirectionPinInvert | Mask | [Axis Mask](#Axis-Mask) |  |  |
| 24 | HomingFeedRate | mm/min | Float |  |  |
| 25 | HomingSeekRate | mm/min | Float |  |  |
| 26 | HomingDebounceDelay | Milliseconds | Int |  |  |
| 27 | HomingPullOfDistance | Millimeters | Float |  |  |
| 28 | G73RetractDistance | Millimeters | Float |  | grblHAL |
| 29 | StepPulseDelay | Microseconds | Int | 0-10 | grblHAL |
| 30 | SpindleSpeedMax | RPM | Int |  |  |
| 31 | SpindleSpeedMin | RPM | Int |  |  |
| 32 | LaserModeEnable | Mode | Int | [Laser Mode](#Laser-Mode) |  |
| 33 | SpindlePWMFreq | Hz | Float |  | grblHAL |
| 34 | SpindlePWMOffValue | Range | Int | 0 - 100 | grblHAL |
| 35 | SpindelPWMMinValue | Range | Int | 0 - 100 | grblHAL |
| 36 | SpindelPWMMaxValue | Range | Int | 0 - 100 | grblHAL |
| 37 | StepperDeenergize | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 38 | SpindlePPR | Pulses/Revolution | Int |  | grblHAL |
| 39 | RealtimeCommandsEnable | Mask | Bool |  | grblHAL |
| 40 | JogSoftLimitsEnable | Mask | Bool |  | grblHAL |
| 42 |  |  |  |  |  |
| 43 | HomingLocateCycles | Cycles | Int | 0 - 255 | grblHAL |
| 44 | HomingCycle1 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 45 | HomingCycle2 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 46 | HomingCycle3 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 47 | HomingCycle4 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 48 | HomingCycle5 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 49 | HomingCycle6 | Mask | Int | [Axis Mask](#Axis-Mask) | grblHAL |
| 50 | JogStepSpeed | mm/min | Float |  | grblHAL |
| 51 | JogSlowSpeed | mm/min | Float |  | grblHAL |
| 52 | JogFastSpeed | mm/min | Float |  | grblHAL |
| 53 | JogStepDistance | Millimeters | Float |  | grblHAL |
| 54 | JogSlowDistance | Millimeters | Float |  | grblHAL |
| 55 | JogFastDistance | Millimeters | Float |  | grblHAL |
| ... |  |  |  |  |  |
| 60 | RestoreOverridesAfterProgram | Mask | Bool |  | grblHAL |
| 61 | IgnoreSafetyDoorWhenIdle | Mask | Bool  |  | grblHAL |
| 62 | SleepEnable | Mask | Bool |  | grblHAL |
| 63 | LaserDisableWhenHold | Mask | Bool |  | grblHAL |
| 64 | ForceAlarmOnStartup | Mask | Bool |  | grblHAL |
| 65 | HomingRequiredOnStartup | Mask | Bool |  | grblHAL |
| ... |  |  |  |  |  |
| 70 | NetworkService | Mask | Int | [Network Service Mask](#Network-Service-Mask) | grblHAL |
| 71 | NetworkBluetoothName | Name | String | Max 32 characters | grblHAL |
| 72 | NetworkBluetoothServiceName | Name | String | Max 32 characters | grblHAL |
| 73 | NetworkWifiMode | Mode | Int | [Wifi Mode](#Wifi-Mode)  | grblHAL |
| 74 | NetworkWifiStationSSID | Name | String | Max 64 characters | grblHAL |
| 75 | NetworkWifiStationPW | Password | String | Max 32 characters | grblHAL |
| 76 | NetworkWifiAPSSID | Name | String | Max 64 characters | grblHAL |
| 77 | NetworkWifiAPPW | Password | String | Max 32 characters (Blank is open) | grblHAL |
| 78 | NetworkWifiAPCountry | ISO 3166-1 alpha-3 | String |  | grblHAL |
| 79 | NetworkWifiAPChannel | Channel Index | Int | 0-11 | grblHAL |
| 80 | SpindlePIDRegulatorProportionalGain | Gain | Float |  | grblHAL |
| 81 | SpindlePIDRegulatorIntegralGain | Gain | Float |  | grblHAL |
| 82 | SpindlePIDRegulatorDerivativeGain | Gain | Float |  | grblHAL |
| ... |  |  |  |  |  |
| 84 | SpindlePIDMaxOutputError | Value Error | Float |  | grblHAL |
| 85 | SpindlePIDRegulatorMaxIntegralError | Value Error | Float |  | grblHAL |
| ... |  |  |  |  |  |
| 90 | SpindleSyncMotionPIDRegulatorGain | Gain | Float |  | grblHAL |
| 91 | SpindleSyncMotionPIDRegulatorIntegralGain | Gain | Float |  | grblHAL |
| 92 | SpindleSyncMotionPIDRegulatorDerivativeGain | Gain | Float |  | grblHAL |
| ... |  |  |  |  |  |
| 100 | XAxisSteps | Millimeter | Float |  |  |
| 101 | YAxisSteps | Millimeter | Float |  |  |
| 102 | ZAxisSteps | Millimeter | Float |  |  |
| ... |  |  |  |  |  |
| 110 | XAxisMaxFeedRate | mm/min | Float |  |  |
| 111 | YAxisMaxFeedRate | mm/min | Float |  |  |
| 112 | ZAxisMaxFeedRate | mm/min | Float |  |  |
| ... |  |  |  |  |  |
| 120 | XAxisMaxAcceleration | mm/sec² | Float |  |  |
| 121 | YAxisMaxAcceleration | mm/sec² | Float |  |  |
| 122 | ZAxisMaxAcceleration | mm/sec² | Float |  |  |
| ... |  |  |  |  |  |
| 130 | XAxisMaxTravelDistance | Millimeters | Float |  |  |
| 131 | YAxisMaxTravelDistance | Millimeters | Float |  |  |
| 132 | ZAxisMaxTravelDistance | Millimeters | Float |  |  |
| ... |  |  |  |  |  |
| 300 | WifiStationOrEthernetHostName | Name | String |  | grblHAL |
| 301 | WifiStationOrEthernetIPMode | Mode | Int | [IP Mode](#IP-Mode) | grblHAL |
| 302 | WifiStationOrEthernetGateway | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 303 | WifiStationOrEthernetNetmask | IPv4/IPv6 | Int | up to 16 octests | grblHAL |
| 304 | WifiStationOrEthernetNetmask | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 305 | WifiStationOrEthernetTelnetPort | Port | Int | 1 - 65536 | grblHAL |
| 306 | WifiStationOrEthernetHttpPort | Port | Int | 1 - 65536 | grblHAL |
| 307 | WifiStationOrEthernetWebsocketPort | Port | Int | 1 - 65536 | grblHAL |
| ... |  |  |  |  |  |
| 310 | WifiAPHostName | Name | String |  | grblHAL |
| 311 | WifiAPIPMode | Mode | Int | [IP Mode](#IP-Mode) | grblHAL |
| 312 | WifiAPGateway | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 313 | WifiAPNetmask | IPv4/IPv6 | Int | up to 16 octests | grblHAL |
| 314 | WifiAPNetmask | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 315 | WifiAPTelnetPort | Port | Int | 1 - 65536 | grblHAL |
| 316 | WifiAPHttpPort | Port | Int | 1 - 65536 | grblHAL |
| 317 | WifiAPWebsocketPort | Port | Int | 1 - 65536 | grblHAL |
| ... |  |  |  |  |  |
| 320 | EthernetHostName | Name | String |  | grblHAL |
| 321 | EthernetIPMode | Mode | Int | [IP Mode](#IP-Mode) | grblHAL |
| 322 | EthernetGateway | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 323 | EthernetNetmask | IPv4/IPv6 | Int | up to 16 octests | grblHAL |
| 324 | EthernetNetmask | IPv4/IPv6 | Int | up to 16 octets | grblHAL |
| 325 | EthernetTelnetPort | Port | Int | 1 - 65536 | grblHAL |
| 326 | EthernetHttpPort | Port | Int | 1 - 65536 | grblHAL |
| 327 | EthernetWebsocketPort | Port | Int | 1 - 65536 | grblHAL |
| ... |  |  |  |  |  |
| 340 | SpindleSpeedTolerance | Percent | Int |  | grblHAL |
| 341 | SpindleToolChangeMode | Mode | Int | [Manual Tool Change Mode](#Manual-Tool-Change-Mode) | grblHAL |
| 342 | SpindleProbingDistance | Millimeters | Int |  | grblHAL |
| 343 | SpindleProbingSlowFeedRate | mm/min | Int |  | grblHAL |
| 344 | SpindleProbingSeekFeedrate | mm/min | Int |  | grblHAL |

## Masks
Bit-Index always starting at LSB

### Axis Mask
| Bit | Axis |
| --------- | ---- |
| 0 | X |
| 1 | Y |
| 2 | Z |
| 3 | A |
| 4 | B |

### Coolant Mask
| Bit | Name |
| --------- | ---- |
| 0 | Flood |
| 1 | Mist |


### Spindle Mask
| Bit | Name | Description |
| --- | ---- | ----------- |
| 0 | Spindle ON | |
| 1 | Spindle CCW | |
| 2 | Spindle PWM | |

### Control Mask
| Bit | Name | Description |
| --- | ---- | ----------- |
| 0 | Reset | |
| 1 | Feed Hold | |
| 2 | Cycle Start | |
| 3 | Safety Door | |
| 4 | Block Delete | |
| 5 | Stop Disable | |
| 6 | E-Stop | |
| 7 | Probe Connected | |

### Report Mask
| Bit | Name | Description |
| --- | ---- | ----------- |
| 0 | Machine Position | |
| 1 | Buffer State | |
| 2 | Line Number | |
| 3 | Feed Speed | |
| 4 | Pin State | |
| 5 | Work Coord Offste | |
| 6 | Overrides | |
| 7 | Probe Coordinates | |
| 8 | Sync On WCO Change | |
| 9 | Parser State | |
| 10 | Alarm Substate | |
| 11 | Run Substate | |

### Hard Limit Mask
| Bit | Name | Description |
| --- | ---- | ----------- |
| 0 | Hard Limits | |
| 1 | Strict Mode | |

## Network Service Mask
| Bit | Mode | Description |
| --- | ---- | ----------- |
| 0 | Telnet | |
| 1 | Websocket | |
| 2 | HTTP | |
| 3 | mDNS | |
| 4 | SSDP | |

## Mode

### Laser Mode Values
| Value | Mode | Description |
| --- | ---- | ----------- |
| 0 | Normal| |
| 1 | Laser | |
| 2 | Lathe | |

### Wifi Mode
| Value | Mode | Description |
| --- | ---- | ----------- |
| 0 | NULL | |
| 1 | STA | Station |
| 2 | AP | Access Point |
| 3 | APSTA | Access Point and Station |

### IP Mode
| Value | Mode | Description |
| --- | ---- | ----------- |
| 0 | Static | |
| 1 | DHCP | DHCP supplied address |
| 2 | Auto | Auto IP |

### Manual Tool Change Mode
| Value | Mode | Description |
| --- | ---- | ----------- |
| 0 | Normal | Manual tool change and touch off via jogging |
| 1 | Manual touch off | Initial move to linear axis home position for tool change, manual or automatic touch off with $TPW command |
| 2 | Manual touch off at G59.3 | Initial move to linear axis home position then to G59.3 position for tool change, manual or automatic touch off with $TPW command |
| 3 | Manual touch off at G59.3 | Initial move to linear axis home position for tool change then to G59.3 position for automatic touch off |
| 4 | Ignore M6 | Auto IP |

## Reference
[grbl Wiki](https://github.com/gnea/grbl/wiki/Grbl-v1.1-Interface)

[grblHal Wiki](https://github.com/grblHAL/core/wiki/Additional-or-extended-settings)