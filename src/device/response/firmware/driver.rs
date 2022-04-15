pub mod name;
pub mod option;
pub mod version;

#[derive(Clone, Debug)]
pub struct DriverInfo {
    name: Option<String>,
    version: Option<String>,
    options: Option<Vec<String>>,
}

impl DriverInfo {

    /// Creates a new empty driver info
    pub fn new() -> Self {
        DriverInfo { name: None, version: None, options: None }
    }

    /// Get a reference to the driver info's name.
    #[must_use]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Set the driver info's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Get a reference to the driver info's version.
    #[must_use]
    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Set the driver info's version.
    pub fn set_version(&mut self, version: Option<String>) {
        self.version = version;
    }

    /// Get a reference to the driver info's options.
    #[must_use]
    pub fn options(&self) -> Option<&Vec<String>> {
        self.options.as_ref()
    }

    /// Set the driver info's options.
    pub fn set_options(&mut self, options: Option<Vec<String>>) {
        self.options = options;
    }
}