## 2024-10-24 - SystemMonitor Cold Start
**Confusion:** `SystemMonitor` metrics (CPU, Memory) were staying at 0.0 for the first second of execution, causing visualizations to "pop" in suddenly.
**Clarification:** `sysinfo` requires two polls to calculate CPU usage (differential). The `SystemMonitor` was waiting 1.0s before the first poll, and then another 1.0s for the second poll.
**Fix:** Changed `SystemMonitor::new()` to initialize `last_update` to `-2.0`, forcing an immediate poll on the first frame (t=0). This, combined with an initial `refresh_cpu` in the constructor, reduces the warm-up time significantly.

## 2024-05-15 - [The Ghost Example]
**Confusion:** How to correctly instantiate complex internal geometry structures like `Geodesic` or `PhysicsGrid` without an explicit example.
**Clarification:** Added executable doctests to `Geodesic::new`, `PhysicsGrid::step`, and `AudioModel::process` to clearly demonstrate initialization and usage.
