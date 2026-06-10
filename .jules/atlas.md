**Dependency Tangles (Version Conflicts)**
**Tangle:** Conflicting transitive versions of a core mathematical crate (`glam`) due to varying versions of UI/rendering frameworks (`bevy` and `macroquad`). This caused workspace-wide compilation failures due to ambiguous struct resolutions.
**Blueprint:** Instead of forcing unsafe individual upgrades, standardize the workspace on a unified framework version stack (e.g., downgrading `bevy` to `0.13.2` to match `macroquad`'s `glam 0.25` bounds) and adapt API calls globally to restore build harmony.

**The Leak (Module Visibility)**
**Tangle:** Internal modules incorrectly marked as `pub(crate)` caused `private in public` integration test failures and cross-crate dependency blockers.
**Blueprint:** Encapsulate gracefully, but enforce `pub mod` for modules explicitly re-exported or accessed by tests/binaries.
