git reset --hard
cd crates/market-sim
cat << 'INNER_EOF' >> Cargo.toml

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "market_bench"
harness = false
INNER_EOF
cat << 'INNER_EOF2' > patch.diff
--- src/lib.rs
+++ src/lib.rs
@@ -233,10 +233,10 @@
         }

-        // Clone scan_x to avoid borrowing self while mutating it in the loop
-        let scan_order = self.scan_x.clone();
-
         // Pass 1: Bids (Up)
         for y in 0..self.height {
-            for &x in &scan_order {
+            for i in 0..self.width {
+                let x = self.scan_x[i];
                 let idx = y * self.width + x;
                 if self.updated[idx] {
                     continue;
@@ -250,7 +250,8 @@

         // Pass 2: Asks (Down)
         for y in (0..self.height).rev() {
-            for &x in &scan_order {
+            for i in 0..self.width {
+                let x = self.scan_x[i];
                 let idx = y * self.width + x;
                 if self.updated[idx] {
                     continue;
INNER_EOF2
patch src/lib.rs < patch.diff
cargo bench
