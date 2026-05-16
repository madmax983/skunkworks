import os
import re

def fix_file(filepath):
    with open(filepath, "r") as f:
        content = f.read()

    # Find structs/enums with `pub fn new() -> Self`
    matches = re.finditer(r'impl(?:<[^>]*>)?\s+([A-Za-z0-9_]+)(?:<[^>]*>)?\s*\{[^}]*?pub\s+fn\s+new\(\)\s*->\s*Self', content, re.DOTALL)

    replacements = {}
    for match in matches:
        struct_name = match.group(1)

        has_derive = re.search(r'#\[derive\([^\]]*Default[^\]]*\)\]\s*(?:pub\s+)?(?:struct|enum)\s+' + struct_name, content)
        has_impl = re.search(r'impl(?:<[^>]*>)?\s+Default\s+for\s+' + struct_name, content)

        if not has_derive and not has_impl:
            replacements[struct_name] = True

    if replacements:
        for struct_name in replacements.keys():
            # append to content
            content += f"\nimpl Default for {struct_name} {{\n    fn default() -> Self {{\n        Self::new()\n    }}\n}}\n"

        with open(filepath, "w") as f:
            f.write(content)
        print(f"Fixed {filepath}")

for root, dirs, files in os.walk("experiments/chimera-lang"):
    for file in files:
        if file.endswith(".rs"):
            fix_file(os.path.join(root, file))
