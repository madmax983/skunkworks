export RUSTDOCFLAGS="-D warnings"
cargo doc --no-deps --workspace 2>&1 | grep "missing documentation"
