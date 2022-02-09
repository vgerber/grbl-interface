use grbli::device::state::buffer::BufferState;




#[test]
fn from_reads_buffers_correctly() {
    let msg = "Bf:20,13";
    let buffer = BufferState::from(msg).unwrap();
    assert_eq!(20, buffer.block_buffers_free());
    assert_eq!(13, buffer.rx_characters_free());
}

#[test]
fn from_fails_on_invalid_prefix() {
    let msg = "BF:20,13";
    let error = BufferState::from(msg).err().unwrap();
    assert_eq!("Cannot read buffer state \"BF:20,13\"", error);
}

#[test]
fn from_fails_on_invalid_buffers_size() {
    let msg = "Bf:20";
    let error = BufferState::from(msg).err().unwrap();
    assert_eq!("Invalid buffer states count 1", error);
}

#[test]
fn from_fails_on_invalid_block_buffers() {
    let msg = "Bf:aa,0";
    let error = BufferState::from(msg).err().unwrap();
    assert_eq!("Cannot read block buffers free \"aa\"", error);
}

#[test]
fn from_fails_on_invalid_rx_buffers() {
    let msg = "Bf:20,bb";
    let error = BufferState::from(msg).err().unwrap();
    assert_eq!("Cannot read rx characters free \"bb\"", error);
}