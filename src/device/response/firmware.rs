use self::{startup::FirmwareStartupResult, version::FirmwareVersion, compile_option::CompileOptions};

use super::state::compile::ExtendedCompileOption;

pub mod compile_option;
pub mod startup;
pub mod version;


#[derive(Clone, Debug)]
pub struct FirmwareInfo {
    startup_result: Option<FirmwareStartupResult>,
    version: Option<FirmwareVersion>,
    compile_options: Option<CompileOptions>,
    extended_compile_options: Option<Vec<ExtendedCompileOption>>,
}

impl FirmwareInfo {

    /// Creates a new empty firmware info
    pub fn new() -> Self {
        FirmwareInfo { startup_result: None, version: None, compile_options: None, extended_compile_options: None }
    }

    /// Creates the firmware info from given parameters
    ///
    /// At least one property is required
    pub fn from(
        startup_result: Option<FirmwareStartupResult>,
        version: Option<FirmwareVersion>,
        compile_options: Option<CompileOptions>,
        extended_compile_options: Option<Vec<ExtendedCompileOption>>,
    ) -> Result<Self, String> {
        if startup_result.is_none()
            && version.is_none()
            && compile_options.is_none()
            && extended_compile_options.is_none()
        {
            return Err("Cannot create firmware info from None's only".to_string());
        }

        Ok(FirmwareInfo {
            startup_result,
            version,
            compile_options,
            extended_compile_options,
        })
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
}
