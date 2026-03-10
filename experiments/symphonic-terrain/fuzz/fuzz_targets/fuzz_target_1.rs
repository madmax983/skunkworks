#![no_main]

use libfuzzer_sys::fuzz_target;
use symphonic_terrain::heightmap::generate_text_heightmap;

fuzz_target!(|data: &[u8]| {
    // Havoc: Fuzzing the font parser with raw entropy.
    // If rusttype panics on invalid bytes, the fuzzer will catch it.
    let _map = generate_text_heightmap("SYMPHONY", data, 100, 100);
});
