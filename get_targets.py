import re
import os

def check_file(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    pattern = re.compile(r"impl Default for (\w+)\s*\{\s*fn default\(\)\s*->\s*Self\s*\{\s*Self::new\(\)\s*\}\s*\}", re.MULTILINE)
    matches = pattern.finditer(content)

    for match in matches:
        struct_name = match.group(1)
        # Check if Self::new() returns exactly what Default should return.
        # We need to find `pub fn new() -> Self` and see what it does.
        # We can look for `pub fn new() -> Self {` or `pub fn new() -> Result<Self` or `Option`

        # If the struct itself can be derived, we'll try to apply #[derive(Default)] and remove `impl Default`.
        # For now let's just list them to understand the scale and if they can be safely derived.
        print(f"File: {file_path}, Struct: {struct_name}")


files = [os.path.join(dp, f) for dp, dn, filenames in os.walk('.') for f in filenames if f.endswith('.rs')]
for f in files:
    check_file(f)
