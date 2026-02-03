# Warden's Journal 🔒

**2025-02-18 - Fugue State Panic & Karman Render Safety**
**Threat:** Denial of Service (DoS) via panic injection.
**Defense:** Replaced `unwrap()` on user input path in `fugue-state` and rendering bounds in `karman-text-street` with safe handling.

**2025-05-23 - Hyperbolic-FS Symlink Loop Hardening**
**Threat:** DoS via symlink loops in `hyperbolic-fs`, causing redundant processing or potentially unbounded recursion.
**Defense:** Implemented cycle detection in `scan_depth` using `canonicalize` and `ancestors` set.

**2026-02-03 - Quipu-Renderer UTF-8 Panic**
**Threat:** DoS via panic in `quipu-renderer` when parsing strings containing '≡'. Code attempted to slice at byte index 1 of a 3-byte character.
**Defense:** Removed unsafe slicing logic and ensured all string operations respect UTF-8 boundaries.
