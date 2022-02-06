use grbli::device::{response::report::ReportResponse, state::{machine::state::MachineStateName, signal::MachineSignal, accessory::AccessoryState, pendant::PendantControl, modes::ArcMode}, axis::Axis};



#[test]
pub fn from_parses_all_attributes_correctly() {
    // test report with all possible fields set
    let message = "<Idle:2|WPos:3.32,67|Bf:100,10|Ln:9|FS:100,23,20|PN:DRH|WCO:23.2,0|WCS:G55|Ov:10,12,115|A:TF|MPG:0|H:1,7|D:1|Sc:XYZABC|TLR:0|FW:test|In:-1>";
    let report = ReportResponse::from(message).unwrap();

    let machine_state = report.machine_state();
    assert!(matches!(machine_state.status(), MachineStateName::Idle));
    assert_eq!(2, machine_state.sub_status().unwrap());
    
    assert!(matches!(report.global_position(), None));
    let machine_pos = report.local_position().unwrap();
    assert_eq!(3.32, machine_pos[0]);
    assert_eq!(67f32, machine_pos[1]);

    let buffer_state = report.buffer_state().unwrap();
    assert_eq!(100, buffer_state.block_buffers_free());
    assert_eq!(10, buffer_state.rx_characters_free());

    assert_eq!(9, report.line_number().unwrap());

    let machine_speed = report.machine_speed().unwrap();
    assert_eq!(100, machine_speed.feed_rate());
    assert_eq!(23, machine_speed.spindle_programmed_rpm());
    assert_eq!(20, machine_speed.spindle_actual_rpm().unwrap());

    let machine_signals = report.machine_signals().unwrap();
    assert!(matches!(machine_signals[0], MachineSignal::DoorSwitchAsserted));
    assert!(matches!(machine_signals[1], MachineSignal::ResetSwitchAsserted));
    assert!(matches!(machine_signals[2], MachineSignal::FeedHoldSwitchAsserted));

    let local_pos_offset = report.local_offset().unwrap();
    assert_eq!(23.2, local_pos_offset[0]);
    assert_eq!(0f32, local_pos_offset[1]);

    assert_eq!("G55", report.machine_coordinate_system().unwrap());

    let overrides = report.override_values().unwrap();
    assert_eq!(10, overrides.feed_rate_percentage());
    assert_eq!(12, overrides.rapids_percentage());
    assert_eq!(115, overrides.spindle_speed_percentage());

    let accessories = report.accessory_state().unwrap();
    assert!(matches!(accessories[0], grbli::device::state::accessory::AccessoryState::ToolChangePending));
    assert!(matches!(accessories[1], AccessoryState::FloodCoolantEnabled));

    assert!(matches!(report.pendant_control().unwrap(), PendantControl::Released));

    let homing = report.homing_state().unwrap();
    assert!(homing.is_homed());
    assert!(matches!(homing.homed_axes()[0], Axis::X));
    assert!(matches!(homing.homed_axes()[1], Axis::Y));
    assert!(matches!(homing.homed_axes()[2], Axis::Z));

    assert!(matches!(report.arc_mode().unwrap(), ArcMode::Diameter));

    let scaled_axes = report.scaled_axes().unwrap();
    assert!(matches!(scaled_axes[0], Axis::X));
    assert!(matches!(scaled_axes[1], Axis::Y));
    assert!(matches!(scaled_axes[2], Axis::Z));
    assert!(matches!(scaled_axes[3], Axis::A));
    assert!(matches!(scaled_axes[4], Axis::B));
    assert!(matches!(scaled_axes[5], Axis::C));

    assert!(report.tool_length_reference_offset_set().unwrap() == false);

    assert_eq!("test", report.firmware().unwrap());

    assert!(report.input_wait_result_succeeded().unwrap() == false);
}