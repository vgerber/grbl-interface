
const BUFFER_STATE_PREFIX: &str = "Bf:";

pub struct BufferState {
    block_buffers_free: i32,

    /// Amount of characters until buffer on device is full
    rx_characters_free: i32,
}


impl BufferState {
    

    /// Creates buffer states from "Bf:\<block buffers free\>,\<rx characters free\>"
    /// 
    /// # Examples
    /// ```
    /// let buffer_state = BufferState::from("Bf:20,13")
    /// ```
    pub fn from(message: &str) -> Result<BufferState, String> {
        if BufferState::is_buffer_state(message) {
            let values: Vec<&str> = (&message[BUFFER_STATE_PREFIX.len()..]).split(",").collect();
            
            let block_buffers: i32 = match values[0].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read block buffers free \"{}\"", values[0]))
            };
            let rx_characters: i32 = match values[1].parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Cannot read rx characters free \"{}\"", values[1]))
            };

            return Ok(BufferState {
                block_buffers_free: block_buffers,
                rx_characters_free: rx_characters,
            });
        }
        Err(format!("Cannot read buffer state \"{}\"", message))
    }

    /// Indicates if message starts with "Bf:"
    pub fn is_buffer_state(message: &str) -> bool {
        message.starts_with(BUFFER_STATE_PREFIX)
    }

    /// Get a reference to the buffer state's block buffers free.
    pub fn block_buffers_free(&self) -> i32 {
        self.block_buffers_free
    }

    /// Get a reference to the buffer state's rx characters free.
    pub fn rx_characters_free(&self) -> i32 {
        self.rx_characters_free
    }
}
