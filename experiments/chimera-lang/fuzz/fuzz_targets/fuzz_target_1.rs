#![no_main]
use libfuzzer_sys::fuzz_target;
use chimera_lang::vm::paradox::Paradox;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut paradox = Paradox::new();
        let _ = paradox.parse_rule(s);
    }
});
