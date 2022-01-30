use std::result::Result;

// All available compile options for grbl 1.1 firmware
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
        o => Err(format!("Invalid option {}", o)),
    }
}

/// Stores values from parsed help message "[OPT: ...\]"
pub struct CompileOptionsResponse {
    options: Vec<CompileOption>,
    unknown_options: Vec<String>,
    block_buffer_size: i32,
    rx_buffer_size: i32,
}


impl CompileOptionsResponse {

    /// Reads compile options string and returns its sub elements
    /// 
    /// Performs trimming before syntax checks
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// // stores:
    /// // compile options = "VL"
    /// // block buffer size = 15
    /// // rx buffer size = 128
    /// let response = CompileOptionsResponse::from("[OPT:VL,15,128]");
    /// ```
    pub fn from(message: &str) -> Result<CompileOptionsResponse, String> {
        let trimmed_message = String::from(message).trim().to_owned();

        // check if message has the correct syntax
        // and return the unwrapped value
        // "[OPT:<options>,<block size>,<rx size>]"
        if trimmed_message.starts_with("[OPT:") && trimmed_message.ends_with("]") {
            // remove wrapper characters
            let message_end = trimmed_message.len()-1;
            let message_payload = &trimmed_message[5..message_end];

            // read all sub values in remaining message
            // compile options could be empty and therefore is not filtered out
            let message_values: Vec<String> = message_payload.split(",").map(|s| s.to_string()).collect();
            
            // format should only contain the three defined options properties
            if message_values.len() != 3 {
                return Err(format!("Invalid compile options string \"{}\"", message_payload));
            }
            
            // read all compile options
            let compile_options = CompileOptionsResponse::parse_compile_options(&message_values[0][..]);
            
            // read block buffer size
            let block_buffer_size = match message_values[1].parse::<i32>() {
                Ok(buffer_size) => buffer_size,
                Err(_) => return Err(format!("Invalid block buffer size \"{}\"", message_values[1]))
            };
            
            // read rx buffer size
            let rx_buffer_size = match message_values[2].parse::<i32>() {
                Ok(buffer_size) => buffer_size,
                Err(_) => return Err(format!("Invalid rx buffer size \"{}\"", message_values[2]))
            };
            
            return Ok(CompileOptionsResponse {
                options: compile_options.0,
                unknown_options: compile_options.1,
                block_buffer_size,
                rx_buffer_size
            })    
        }
        Err(format!("Could not read compile options \"{}\"", trimmed_message))        
    }

    /// Parses options string and returns interpreted compile options and unkown compile options
    fn parse_compile_options(options: &str) -> (Vec<CompileOption>, Vec<String>) {
        let mut compile_options: Vec<CompileOption> = Vec::new();
        let mut unknown_compile_options: Vec<String> = Vec::new();

        // parses options as ascii chars
        // adds valid options to compile_options
        // adds unknown/invalid options to unknown compile options
        for option_byte in options.bytes() {
            let option = match String::from_utf8(vec![option_byte]) {
                Ok(string) => string,
                Err(_) => String::from("Invalid Symbol")
            }; 
            match get_compile_option(&option[..]) {
                Ok(compile_option) => compile_options.push(compile_option),
                Err(_) => unknown_compile_options.push(option),
            }
        }

        (compile_options, unknown_compile_options)
    }
    
    pub fn options(&self) -> &Vec<CompileOption> {
        &self.options
    }

    pub fn unknown_options(&self) -> &Vec<String> {
        &self.unknown_options
    }

    pub fn block_buffer_size(&self) -> i32 {
        self.block_buffer_size
    }

    pub fn rx_buffer_size(&self) -> i32 {
        self.rx_buffer_size
    }
}