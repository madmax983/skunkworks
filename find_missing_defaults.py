import os
import re

missing_defaults = []

for root, dirs, files in os.walk("crates"):
    for file in files:
        if file.endswith(".rs"):
            filepath = os.path.join(root, file)
            with open(filepath, "r") as f:
                content = f.read()

            # Find structs/enums with `pub fn new() -> Self`
            matches = re.finditer(r'impl(?:<[^>]*>)?\s+(\w+)(?:<[^>]*>)?\s*\{[^}]*?pub\s+fn\s+new\(\)\s*->\s*Self', content, re.DOTALL)
            for match in matches:
                struct_name = match.group(1)

                # Check if it has an explicit Default implementation or derive(Default)
                has_derive = re.search(r'#\[derive\([^\]]*Default[^\]]*\)\]\s*(?:pub\s+)?(?:struct|enum)\s+' + struct_name, content)
                has_impl = re.search(r'impl(?:<[^>]*>)?\s+Default\s+for\s+' + struct_name, content)

                if not has_derive and not has_impl:
                    missing_defaults.append((filepath, struct_name))

for root, dirs, files in os.walk("experiments"):
    for file in files:
        if file.endswith(".rs"):
            filepath = os.path.join(root, file)
            with open(filepath, "r") as f:
                content = f.read()

            # Find structs/enums with `pub fn new() -> Self`
            matches = re.finditer(r'impl(?:<[^>]*>)?\s+(\w+)(?:<[^>]*>)?\s*\{[^}]*?pub\s+fn\s+new\(\)\s*->\s*Self', content, re.DOTALL)
            for match in matches:
                struct_name = match.group(1)

                # Check if it has an explicit Default implementation or derive(Default)
                has_derive = re.search(r'#\[derive\([^\]]*Default[^\]]*\)\]\s*(?:pub\s+)?(?:struct|enum)\s+' + struct_name, content)
                has_impl = re.search(r'impl(?:<[^>]*>)?\s+Default\s+for\s+' + struct_name, content)

                if not has_derive and not has_impl:
                    missing_defaults.append((filepath, struct_name))

for filepath, struct_name in missing_defaults:
    print(f"{filepath}: {struct_name}")
