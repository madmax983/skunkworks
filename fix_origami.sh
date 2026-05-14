#!/bin/bash
cat << 'PATCH' > patch.diff
--- crates/origami/src/lib.rs
+++ crates/origami/src/lib.rs
@@ -274,7 +274,7 @@
         .checked_add(1)
         .and_then(|r| cols.checked_add(1).and_then(|c| r.checked_mul(c)))
     {
-        Some(c) if c <= (isize::MAX as usize) / 32 => c,
+        Some(c) if c <= (isize::MAX as usize) / std::mem::size_of::<Vec3>() => c,
         _ => return Vec::new(),
     };
     let mut positions = Vec::with_capacity(capacity);
@@ -341,7 +341,7 @@
         .checked_add(1)
         .and_then(|r| cols.checked_add(1).and_then(|c| r.checked_mul(c)))
     {
-        Some(c) if c <= (isize::MAX as usize) / 32 => c,
+        Some(c) if c <= (isize::MAX as usize) / std::mem::size_of::<Vec3>() => c,
         _ => return Vec::new(),
     };
     let mut positions = Vec::with_capacity(capacity);
PATCH
patch -p0 < patch.diff
