use grbli::device::{response::compile_option::*, state::compile::{CompileOption, ExtendedCompileOption}};


#[test]
fn co_from_parses_message_correctly() {
    let message_str = "[OPT:$2L,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(3, compile_option.options().len());
    assert_eq!(0, compile_option.unknown_options().len());
    assert!(matches!(compile_option.options()[0], CompileOption::RestoreEEPROMDollarSettingsDisabled));
    assert!(matches!(compile_option.options()[1], CompileOption::DualAxisMotorsWithSelfSquaringEnabled));
    assert!(matches!(compile_option.options()[2], CompileOption::AlarmStateOnPowerUpWhenHomingInitLock));
}

#[test]
fn co_from_parses_unknown_options_correctly() {
    let message_str = "[OPT:$2Lc,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(3, compile_option.options().len());
    assert!(matches!(compile_option.options()[0], CompileOption::RestoreEEPROMDollarSettingsDisabled));
    assert!(matches!(compile_option.options()[1], CompileOption::DualAxisMotorsWithSelfSquaringEnabled));
    assert!(matches!(compile_option.options()[2], CompileOption::AlarmStateOnPowerUpWhenHomingInitLock));

    assert_eq!(1, compile_option.unknown_options().len());
    assert_eq!("c", compile_option.unknown_options()[0])
}

#[test]
fn co_from_accepts_an_empty_options_section() {
    let message_str = "[OPT:,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(0, compile_option.options().len());
    assert_eq!(0, compile_option.unknown_options().len())
}

#[test]
fn co_from_cannot_read_empty_message() {
    let message_str = "[OPT:]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \"\"", &message_error[..])
}

#[test]
fn co_from_cannot_read_non_numeric_block_size() {
    let message_str = "[OPT:,a,3]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid block buffer size \"a\"", &message_error[..])
}

#[test]
fn co_from_cannot_read_non_numeric_rx_size() {
    let message_str = "[OPT:,10,a]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid rx buffer size \"a\"", &message_error[..])
}

#[test]
fn co_from_cannot_read_less_than_three_segements() {
    let message_str = "[OPT:10,3]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \"10,3\"", &message_error[..])
}

#[test]
fn co_from_cannot_read_more_than_five_segements() {
    let message_str = "[OPT:,10,12,3,3,10]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \",10,12,3,3,10\"", &message_error[..])
}

#[test]
fn co_from_cfails_on_invalid_prefix() {
    let message_str = "OPT:10,12,3,3,10]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Could not read compile options \"OPT:10,12,3,3,10]\"", &message_error[..])
}

#[test]
fn co_from_cfails_on_invalid_suffix() {
    let message_str = "[OPT:10,12,3,3,10";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Could not read compile options \"[OPT:10,12,3,3,10\"", &message_error[..])
}

#[test]
fn eco_parses_message_correctly() {
    let message_str = "[NEWOPT:ATC,SS,SD]";
    let compile_option = parse_extended_compile_options(message_str).unwrap();
    assert!(matches!(compile_option[0], ExtendedCompileOption::AutomaticToolChange));
    assert!(matches!(compile_option[1], ExtendedCompileOption::SpindelSync));
    assert!(matches!(compile_option[2], ExtendedCompileOption::SDCardStreaming));
}

#[test]
fn eco_parses_empty_option_correctly() {
    let message_str = "[NEWOPT:]";
    let compile_option = parse_extended_compile_options(message_str).unwrap();
    assert_eq!(0, compile_option.len());
}

#[test]
fn eco_parses_fails_on_empty_option() {
    let message_str = "[NEWOPT:ATC,,SS,SD]";
    let message_error = parse_extended_compile_options(message_str).err().unwrap();
    assert_eq!("Cannot read extended compile options: \"Invalid option \"\"\"", &message_error[..])
}

#[test]
fn eco_parses_fails_on_invalid_prefix() {
    let message_str = "[NEOPT:ATC,SS,SD]";
    let message_error = parse_extended_compile_options(message_str).err().unwrap();
    assert_eq!("Cannot read extended compile options \"[NEOPT:ATC,SS,SD]\"", &message_error[..])
}

#[test]
fn eco_parses_fails_on_invalid_suffix() {
    let message_str = "[NEWOPT:ATC,SS,SD";
    let message_error = parse_extended_compile_options(message_str).err().unwrap();
    assert_eq!("Cannot read extended compile options \"[NEWOPT:ATC,SS,SD\"", &message_error[..])
}