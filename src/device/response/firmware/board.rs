use self::{storage::Storage, ports::AuxPorts};

pub mod ports;
pub mod name;
pub mod storage;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardInfo {
    name: Option<String>,
    storage: Option<Storage>,
    aux: Option<AuxPorts>,
}

impl BoardInfo {

    /// Creates a new empty board info
    pub fn new() -> Self {
        BoardInfo { name: None, storage: None, aux: None }
    }

    /// Get a reference to the board info's name.
    #[must_use]
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Set the board info's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Get a reference to the board info's storage.
    #[must_use]
    pub fn storage(&self) -> Option<&Storage> {
        self.storage.as_ref()
    }

    /// Set the board info's storage.
    pub fn set_storage(&mut self, storage: Option<Storage>) {
        self.storage = storage;
    }

    /// Get the board info's aux.
    #[must_use]
    pub fn aux(&self) -> Option<AuxPorts> {
        self.aux
    }

    /// Set the board info's aux.
    pub fn set_aux(&mut self, aux: Option<AuxPorts>) {
        self.aux = aux;
    }
}