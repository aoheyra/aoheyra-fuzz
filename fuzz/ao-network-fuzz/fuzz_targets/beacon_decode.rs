//! Beacon decoding must never panic on hostile radio bytes.
#![no_main]

use ao_network::Beacon;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Typed errors are the expected outcome; a panic is the bug.
    let _ = Beacon::decode(data);
});
