use self::{startup::FirmwareStartupResult, version::FirmwareVersion, compile_option::CompileOptions, driver::DriverInfo, board::BoardInfo};

use super::state::compile::ExtendedCompileOption;

pub mod compile_option;
pub mod startup;
pub mod version;
pub mod board;
pub mod driver;

#[derive(Clone, Debug, PartialEq)]
pub struct FirmwareInfo {
    startup_result: Option<FirmwareStartupResult>,
    version: Option<FirmwareVersion>,
    compile_options: Option<CompileOptions>,
    extended_compile_options: Option<Vec<ExtendedCompileOption>>,
    driver_info: DriverInfo,
    board_info: BoardInfo,
}

impl FirmwareInfo {

    /// Creates a new empty firmware info
    pub fn new() -> Self {
        FirmwareInfo { startup_result: None, version: None, compile_options: None, extended_compile_options: None, driver_info: DriverInfo::new(), board_info: BoardInfo::new() }
    }
    
    /// Get a reference to the firmware info's startup state.
    #[must_use]
    pub fn startup_result(&self) -> Option<&FirmwareStartupResult> {
        self.startup_result.as_ref()
    }

    /// Set the firmware info's startup state.
    pub fn set_startup_result(&mut self, startup_result: Option<FirmwareStartupResult>) {
        self.startup_result = startup_result;
    }

    /// Get a reference to the firmware info's version.
    #[must_use]
    pub fn version(&self) -> Option<&FirmwareVersion> {
        self.version.as_ref()
    }

    /// Set the firmware info's version.
    pub fn set_version(&mut self, version: Option<FirmwareVersion>) {
        self.version = version;
    }

    /// Get a reference to the firmware info's compile options.
    #[must_use]
    pub fn compile_options(&self) -> Option<&CompileOptions> {
        self.compile_options.as_ref()
    }

    /// Set the firmware info's compile options.
    pub fn set_compile_options(&mut self, compile_options: Option<CompileOptions>) {
        self.compile_options = compile_options;
    }

    /// Get a reference to the firmware info's extended compile options.
    #[must_use]
    pub fn extended_compile_options(&self) -> Option<&Vec<ExtendedCompileOption>> {
        self.extended_compile_options.as_ref()
    }

    /// Set the firmware info's extended compile options.
    pub fn set_extended_compile_options(
        &mut self,
        extended_compile_options: Option<Vec<ExtendedCompileOption>>,
    ) {
        self.extended_compile_options = extended_compile_options;
    }

    /// Get a reference to the firmware info's driver info.
    #[must_use]
    pub fn driver_info(&self) -> &DriverInfo {
        &self.driver_info
    }

    /// Get a reference to the firmware info's board info.
    #[must_use]
    pub fn board_info(&self) -> &BoardInfo {
        &self.board_info
    }

    /// Get a mutable reference to the firmware info's driver info.
    #[must_use]
    pub fn driver_info_mut(&mut self) -> &mut DriverInfo {
        &mut self.driver_info
    }

    /// Get a mutable reference to the firmware info's board info.
    #[must_use]
    pub fn board_info_mut(&mut self) -> &mut BoardInfo {
        &mut self.board_info
    }
}
