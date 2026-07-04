# Splice Surgeon Learning
- When combining dependencies like `physics-pbd` and `macroquad`, be very careful about internal types. `macroquad` has its own `glam` dependency that might conflict with the `glam` version used by the other physics crate. Make sure the cargo manifests are aligned, and disambiguate `::glam::Vec3` and `macroquad::prelude::*`.
