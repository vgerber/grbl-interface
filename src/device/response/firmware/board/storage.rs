
const STORAGE_PREFIX: &str = "[NVS STORAGE:";
const STORAGE_SUFFIX: &str = "]";
const EMULATED_SYMBOL: &str = "*";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Flash,
    FRAM,
    EEPROM,
}

pub fn parse_storage_type(storage_type: &str) -> Result<StorageType, String> {
    match storage_type {
        "FLASH" => Ok(StorageType::Flash),
        "FRAM" => Ok(StorageType::FRAM),
        "EEPROM" => Ok(StorageType::EEPROM),
        _ => Err(format!("Unknown storage type: \"{}\"", storage_type))
    }
}

/// On board storage
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Storage {
    /// Indicates if the storage type is emulated
    emulated: bool,
    storage_type: StorageType,
}

impl Storage {

    /// Reads the storage type line
    /// 
    /// # Error
    /// Returns an error when parsing fails
    /// 
    /// # Examples
    /// Basic usage:
    /// ```
    /// use grbli::device::response::firmware::board::storage::*;
    /// 
    /// let response = Storage::from("[NVS STORAGE:*FLASH]").unwrap();
    /// assert!(response.emulated())
    /// assert!(matches!(response.storage_type(), StorageType::Flash))
    /// ```
    pub fn from(message: &str) -> Result<Self, String> {
        if Storage::is_response(message) {
            let mut storage_type_str = message.strip_prefix(STORAGE_PREFIX).unwrap().strip_suffix(STORAGE_SUFFIX).unwrap();
            let emulated = storage_type_str.starts_with(EMULATED_SYMBOL);

            storage_type_str = match emulated {
                true => storage_type_str.strip_prefix(EMULATED_SYMBOL).unwrap(),
                false => storage_type_str,
            };

            let storage_type = match parse_storage_type(storage_type_str) {
                Ok(st) => st,
                Err(err) => return Err(err) 
            };

            return Ok(Storage {
                emulated,
                storage_type,
            })
        }

        Err(format!("Cannot read storage type message: \"{}\"", message))
    }

    /// Indicates if the message can be parsed as storage type
    pub fn is_response(message: &str) -> bool {
        message.starts_with(STORAGE_PREFIX) && message.ends_with(STORAGE_SUFFIX)
    }

    /// Get the storage's emulated.
    #[must_use]
    pub fn emulated(&self) -> bool {
        self.emulated
    }

    /// Get a reference to the storage's storage type.
    #[must_use]
    pub fn storage_type(&self) -> &StorageType {
        &self.storage_type
    }
}