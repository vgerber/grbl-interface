use std::result::Result;

use crate::device::{axis::MAX_AXES, state::compile::{CompileOption, get_compile_option, ExtendedCompileOption, get_extended_compile_option}};

const COMPILE_OPTION_SUFFIX: &str = "]";
const COMPILE_OPTION_PREFIX: &str = "[OPT:";
const EXTENDED_COMPILE_OPTION_SUFFIX: &str = "]";
const EXTENDED_COMPILE_OPTION_PREFIX: &str = "[NEWOPT:";

// All available compile options for grbl 1.1 firmware


/// Stores values from parsed help message "[OPT: ...\]"
pub struct CompileOptionsResponse {
    options: Vec<CompileOption>,
    unknown_options: Vec<String>,
    block_buffer_size: i32,
    rx_buffer_size: i32,
    axes_count: Option<i32>,
    tool_table_entries_count: Option<i32>
}


impl CompileOptionsResponse {

    /// Reads compile options string and returns its sub elements
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
        // check if message has the correct syntax
        // and return the unwrapped value
        // "[OPT:<options>,<block size>,<rx size>{,<axes>,<tools>}]"
        if CompileOptionsResponse::is_compile_options(message) {
            // remove wrapper characters
            let message_end = message.len()-1;
            let message_payload = &message[5..message_end];

            // read all sub values in remaining message
            // compile options could be empty and therefore is not filtered out
            let message_values: Vec<String> = message_payload.split(",").map(|s| s.to_string()).collect();
            
            // format should only contain the three defined options properties
            if message_values.len() > 5 || message_values.len() < 3 {
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

            // read axes count
            let mut axes_count: Option<i32> = None;
            if message_values.len() >= 4 {
                let axes_count_value = match message_values[3].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid axes count value \"{}\"", message_values[3])),
                };

                if axes_count_value < 1 && axes_count_value > MAX_AXES as i32 {
                    return Err(format!("Invalid axes count {}", axes_count_value));
                }
                axes_count = Some(axes_count_value)
            }

            // read tool table entries
            let mut tool_table_entries_count: Option<i32> = None;
            if message_values.len() >= 5 {
                let tool_table_entries_count_value = match message_values[4].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Invalid tool table entries count value \"{}\"", message_values[4])),
                };

                if tool_table_entries_count_value < 1 && tool_table_entries_count_value > MAX_AXES as i32 {
                    return Err(format!("Invalid axes count {}", tool_table_entries_count_value));
                }
                tool_table_entries_count = Some(tool_table_entries_count_value)
            }
            
            return Ok(CompileOptionsResponse {
                options: compile_options.0,
                unknown_options: compile_options.1,
                block_buffer_size,
                rx_buffer_size,
                axes_count,
                tool_table_entries_count
            })    
        }
        Err(format!("Could not read compile options \"{}\"", message))        
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

    /// Indicates if message has compile options syntax
    pub fn is_compile_options(message: &str) -> bool {
        message.starts_with(COMPILE_OPTION_PREFIX) && message.ends_with(COMPILE_OPTION_SUFFIX)
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

    /// Get a reference to the compile options response's axes count.
    pub fn axes_count(&self) -> Option<i32> {
        self.axes_count
    }

    /// Get a reference to the compile options response's tool table entries count.
    pub fn tool_table_entries_count(&self) -> Option<i32> {
        self.tool_table_entries_count
    }
}

/// Reads extended compile options string and returns all options
/// 
/// # Examples
/// Basic usage:
/// ```
/// // stores:
/// // Ethernet enabled, Wifi enabeld, Homing enabled
/// let response = CompileOptionsResponse::from("[NEWOPT:ETH,WIFI,HOME]");
/// ```
pub fn parse_extended_compile_options(message: &str) -> Result<Vec<ExtendedCompileOption>, String> {
    if is_extended_compile_options(message) {
        // parse comma seperate list of compile options
        // quit on error
        let mut compile_options: Vec<ExtendedCompileOption> = Vec::new();
        let options_message = &message[EXTENDED_COMPILE_OPTION_PREFIX.len()..message.len()-1];
        if options_message.len() == 0 {
            return Ok(compile_options);
        }

        let options: Vec<&str> = options_message.split(",").collect();
        for option in options {
            compile_options.push(match get_extended_compile_option(option) {
                Ok(o) => o,
                Err(error) => return Err(format!("Cannot read extended compile options: \"{}\"", error))
            });
        }
        return Ok(compile_options);

    }
    Err(format!("Cannot read extended compile options \"{}\"", message))
}

/// Indicates if message has extended compile options syntax
pub fn is_extended_compile_options(message: &str) -> bool {
    message.starts_with(EXTENDED_COMPILE_OPTION_PREFIX) && message.ends_with(EXTENDED_COMPILE_OPTION_SUFFIX)
}