//! Random data from `/dev/urandom`
//!
//! Before `getentropy` was standardized in 2024, UNIX didn't have a standardized
//! way of getting random data, so systems just followed the precedent set by
//! Linux and exposed random devices at `/dev/random` and `/dev/urandom`. Thus,
//! for the few systems that support neither `arc4random_buf` nor `getentropy`
//! yet, we just read from the file.

pub fn fill_bytes(bytes: &mut [u8]) {
    println!("@MS fill_bytes called nto");



    // fill the buffer with some dummy data for testing
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (i % 256) as u8;

}
