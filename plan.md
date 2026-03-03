Plan:
1. Modify `crates/resonance-audio/src/audio.rs`:
   - Hoist the `AudioSnapshot` creation out of the hot audio callback loop (`for sample in output.iter_mut()`).
   - Introduce a `needs_snapshot` boolean inside the sample loop. Set it to `true` when `self.sample_counter % 735 == 0`.
   - After the sample loop, check `if needs_snapshot && !self.snapshot_tx.is_full()`.
   - Only create and `try_send` the `AudioSnapshot` if both conditions are met.
2. Complete Pre-Commit steps.
3. Submit the changes.
