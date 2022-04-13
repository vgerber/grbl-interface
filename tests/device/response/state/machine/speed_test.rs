use grbli::device::response::state::machine::speed::MachineSpeed;



#[test]
fn from_reads_full_machine_speed() {
    let msg = "FS:100,3000,1677";
    let speed = MachineSpeed::from(msg).unwrap();
    assert_eq!(100, speed.feed_rate());
    assert_eq!(3000, speed.spindle_programmed_rpm());
    assert_eq!(1677, speed.spindle_actual_rpm().unwrap());
}

#[test]
fn from_ignore_actual_spindle_rpm() {
    let msg = "FS:100,3000";
    let speed = MachineSpeed::from(msg).unwrap();
    assert_eq!(100, speed.feed_rate());
    assert_eq!(3000, speed.spindle_programmed_rpm());
    assert!(speed.spindle_actual_rpm().is_none());
}

#[test]
fn from_fails_on_invalid_prefix() {
    let msg = "FS100,3000";
    let error = MachineSpeed::from(msg).err().unwrap();
    assert_eq!("Cannot read machine speed \"FS100,3000\"", error)
}

#[test]
fn from_fails_on_invalid_feed_rate() {
    let msg = "FS:a0,3000";
    let error = MachineSpeed::from(msg).err().unwrap();
    assert_eq!("Cannot read feed rate \"a0\"", error)
}

#[test]
fn from_fails_on_invalid_spindle_prg_rpm() {
    let msg = "FS:0,c00";
    let error = MachineSpeed::from(msg).err().unwrap();
    assert_eq!("Cannot read spindle programmed rpm \"c00\"", error)
}

#[test]
fn from_fails_on_invalid_spindle_act_rpm() {
    let msg = "FS:0,0,vb";
    let error = MachineSpeed::from(msg).err().unwrap();
    assert_eq!("Cannot read spindle actual rpm \"vb\"", error)
}