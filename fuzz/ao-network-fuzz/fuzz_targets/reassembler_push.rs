//! Reassembly must never panic or exceed `max_frame` on a hostile chunk
//! stream. Completed frames go on to the frame decoder, as on a real link.
//!
//! Input is a sequence of records: `len u8 | advance_ms u8 | chunk[len]`.
//! A clear high bit in `advance_ms` pushes the chunk; a set one runs the
//! timer instead, so both timeout paths are reachable.
#![no_main]

use std::time::Duration;

use ao_network::{Frame, NetworkConfig, Reassembler};
use libfuzzer_sys::fuzz_target;

const MAX_FRAME: usize = 256;

fuzz_target!(|data: &[u8]| {
    // Fixed, known-valid values; a failure here is a harness bug, not input.
    let config = NetworkConfig::default()
        .with_max_frame(MAX_FRAME)
        .and_then(|c| c.with_reassembly_timeout(Duration::from_millis(200)))
        .expect("valid fuzz config");
    let mut reassembler = Reassembler::new(&config);
    let mut now = Duration::ZERO;
    let mut rest = data;
    while let [len, advance, tail @ ..] = rest {
        let Some((chunk, next)) = tail.split_at_checked(usize::from(*len)) else {
            return;
        };
        rest = next;
        now += Duration::from_millis(u64::from(advance & 0x7F));
        // Typed errors are the expected outcome; a panic is the bug.
        if advance & 0x80 != 0 {
            let _ = reassembler.poll_timeout(now);
        } else if let Ok(Some(frame)) = reassembler.push(now, chunk) {
            assert!(!frame.is_empty() && frame.len() <= MAX_FRAME);
            let _ = Frame::decode(&frame, MAX_FRAME);
        }
    }
});
