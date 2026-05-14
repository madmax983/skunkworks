#!/bin/bash
cat << 'PATCH' > patch.diff
--- crates/origami/src/lib.rs
+++ crates/origami/src/lib.rs
@@ -156,7 +156,7 @@
     let vertices_pos = generate_miura_grid(params, grid_size, extension_factor);
     let (cols, rows) = grid_size;

-    if vertices_pos.len() > (isize::MAX as usize) / 32 {
+    if vertices_pos.len() > (isize::MAX as usize) / std::mem::size_of::<OrigamiVertex>() {
         return OrigamiMesh {
             vertices: Vec::new(),
             indices: Vec::new(),
PATCH
patch -p0 < patch.diff
