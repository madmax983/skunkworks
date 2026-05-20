🦠 Threat: `Geodesic::euclidean_circle` propagates `NaN` when coordinates are malformed, crashing processes like `havoc_geodesic` natively.
🛡️ Defense: Added `if det.is_nan() { return None; }` before calculation.
💥 Severity: Moderate - mathematical propagation.
🧪 Verification: Fuzzing Havoc property test `havoc_test_geodesic_nan_poison` now passes successfully by catching the NaN early.
