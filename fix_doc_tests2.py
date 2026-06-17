import re

with open('crates/gray-scott/src/lib.rs', 'r') as f:
    content = f.read()

# Make sure main ends correctly
content = re.sub(
r'''/// // Create a small dish and drop a single "spore" of chemical V in the center\.
/// let mut dish = GrayScott::new\(100, 100\);
/// dish\.add_chemical\(50, 50, 1\.0\);
///
/// for _ in 0\.\.10 \{
///     dish\.update\(0\.055, 0\.062, 1\.0\);
/// \}
/// ```''',
r'''/// // Create a small dish and drop a single "spore" of chemical V in the center.
/// let mut dish = GrayScott::new(100, 100);
/// dish.add_chemical(50, 50, 1.0);
///
/// for _ in 0..10 {
///     dish.update(0.055, 0.062, 1.0);
/// }
/// ```''', content)

with open('crates/gray-scott/src/lib.rs', 'w') as f:
    f.write(content)
