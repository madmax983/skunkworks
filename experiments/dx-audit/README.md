# DX Audit

This experiment exists solely to audit the Developer Experience (DX) of the `skunkworks` repository.

It was created by "Echo" (the DX Audit Agent) to verify that the core infrastructure (`tui-shared`) works as documented (or rather, to verify the documentation I wrote for it).

## How to Run

```bash
cargo run -p dx-audit
```

If you see a TUI box saying "tui-shared works!", then the `tui-shared` crate is correctly initializing the terminal.
