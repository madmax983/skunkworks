## 2024-10-24 - SystemMonitor Cold Start
**Confusion:** `SystemMonitor` metrics (CPU, Memory) were staying at 0.0 for the first second of execution, causing visualizations to "pop" in suddenly.
**Clarification:** `sysinfo` requires two polls to calculate CPU usage (differential). The `SystemMonitor` was waiting 1.0s before the first poll, and then another 1.0s for the second poll.
**Fix:** Changed `SystemMonitor::new()` to initialize `last_update` to `-2.0`, forcing an immediate poll on the first frame (t=0). This, combined with an initial `refresh_cpu` in the constructor, reduces the warm-up time significantly.

## 2026-03-03 - PBD Tuning Parameters
**Confusion:** Users were unsure how to make Position Based Dynamics constraints stiffer.
**Clarification:** PBD is an iterative solver. Setting `stiffness` to 1.0 is not enough if multiple constraints affect the same point.
**Fix:** Added extensive documentation to `physics-pbd::PbdSystem::step` explaining the relationship between the `iterations` argument and constraint rigidity.
