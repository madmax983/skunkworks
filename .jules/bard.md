## 2024-10-24 - SystemMonitor Cold Start
**Confusion:** `SystemMonitor` metrics (CPU, Memory) were staying at 0.0 for the first second of execution, causing visualizations to "pop" in suddenly.
**Clarification:** `sysinfo` requires two polls to calculate CPU usage (differential). The `SystemMonitor` was waiting 1.0s before the first poll, and then another 1.0s for the second poll.
**Fix:** Changed `SystemMonitor::new()` to initialize `last_update` to `-2.0`, forcing an immediate poll on the first frame (t=0). This, combined with an initial `refresh_cpu` in the constructor, reduces the warm-up time significantly.

## 2025-03-08 - OpCode Documentation
**Confusion:** Failing tests in `song_test`, `nova_chronos_local_test`, `nova_fractal_test`, `nova_quantum_scribe_test`, and `prologue_logos_test`. Many core `OpCode` variants were missing documentation explaining their behavior.
**Clarification:** The opcodes like `Choir`, `TimeWarp`, `Mandelbrot`, `Julia`, `Zoom`, `QuantumScan`, and `QuantumScribe` required clear descriptions and working doctests. The `process_choir_organelle` had tick offset bugs. The Prologue `Logos` engine parsing logic needed to respect string literals.
**Fix:** Added descriptive explanations and executable doctests (using `Gene::new`) to these variants in `opcode.rs`. Updated the execution loop to handle them.
