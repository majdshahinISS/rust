
pub fn fill_bytes(bytes: &mut [u8]) {
    println!("@MS fill_bytes called nuttx");
    // fill the buffer with some dummy data for testing
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

}
