use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_forge_and_invoke() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let input_path = dir.path().join("hello.wasm");
    let rune_path = dir.path().join("rune.png");

    // 1. Create a minimal WASM that prints "Hello, World!"
    // Using WASI preview1 imports
    let wasm_wat = r#"
        (module
            (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
            (memory 1)
            (export "memory" (memory 0))
            (data (i32.const 8) "Hello, World!\n")

            (func $main (export "_start")
                (i32.store (i32.const 0) (i32.const 8))  ;; iov.base
                (i32.store (i32.const 4) (i32.const 14)) ;; iov.len

                (call $fd_write
                    (i32.const 1)  ;; stdout
                    (i32.const 0)  ;; iovs ptr
                    (i32.const 1)  ;; iovs len
                    (i32.const 20) ;; nwritten ptr
                )
                drop
            )
        )
    "#;

    let wasm_bytes = wat::parse_str(wasm_wat)?;
    fs::write(&input_path, wasm_bytes)?;

    // 2. Run `forge`
    let mut cmd = Command::cargo_bin("wasm-runes")?;
    cmd.arg("forge")
       .arg(input_path.to_str().unwrap())
       .arg(rune_path.to_str().unwrap())
       .assert()
       .success()
       .stdout(predicate::str::contains("Rune forged successfully"));

    assert!(rune_path.exists());

    // 3. Run `invoke`
    let mut cmd = Command::cargo_bin("wasm-runes")?;
    cmd.arg("invoke")
       .arg(rune_path.to_str().unwrap())
       .assert()
       .success()
       .stdout(predicate::str::contains("Hello, World!"));

    Ok(())
}
