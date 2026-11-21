//! Frame decoding must never panic on hostile link bytes, and anything it
//! accepts must re-encode to the same bytes.
#![no_main]

use ao_network::{Frame, NetworkConfig};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let max_frame = NetworkConfig::default().max_frame();
    if let Ok(frame) = Frame::decode(data, max_frame) {
        assert_eq!(frame.encode(), data);
    }
});
