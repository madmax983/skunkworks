## [Reduction]
**Bloat:** `Option<Option<(usize, usize)>>` used as a pseudo-Result in VM Dispatch
**Cut:** Introduced an explicit `Dispatch` enum (`Handled`, `Jump`, `Unhandled`) and flattened the return paths in `core_dispatch` and `nova_dispatch`.
**Saved:** Eliminated mental overhead of decoding nested generic Options; code is now strictly declarative.
