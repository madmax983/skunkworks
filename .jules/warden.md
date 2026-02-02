# Warden's Journal 🔒

**2025-02-18 - Fugue State Panic & Karman Render Safety**
**Threat:** Denial of Service (DoS) via panic injection.
**Defense:** Replaced `unwrap()` on user input path in `fugue-state` and rendering bounds in `karman-text-street` with safe handling.

**2025-05-23 - Hyperbolic-FS Symlink Loop Hardening**
**Threat:** DoS via symlink loops in `hyperbolic-fs`, causing redundant processing or potentially unbounded recursion.
**Defense:** Implemented cycle detection in `scan_depth` using `canonicalize` and `ancestors` set.

**2025-05-24 - CPU Casino NaN Hardening**
**Threat:** DoS and logic corruption via NaN injection in `cpu-casino` auction bids, allowing invalid bids to win and propagating NaN to credits.
**Defense:** Implemented explicit NaN handling in `resolve_auction` sort logic (pushing NaN to end) and sanitized clearing price calculation.
