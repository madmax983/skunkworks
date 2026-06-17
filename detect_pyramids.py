import os
import re

def max_indentation(block):
    lines = block.split('\n')
    max_indent = 0
    for line in lines:
        if line.strip() and not line.strip().startswith("//"):
            indent = len(line) - len(line.lstrip())
            max_indent = max(max_indent, indent)
    return max_indent

for root, _, files in os.walk('.'):
    if "graveyard" in root or "target" in root or "node_modules" in root or ".git" in root:
        continue
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            # Simple heuristic: find 'fn' and count max indentation inside
            # Instead of parsing rust properly, we'll just check max indentation of the file
            lines = content.split('\n')
            max_indent = 0
            for line in lines:
                if line.strip() and not line.strip().startswith("//"):
                    indent = len(line) - len(line.lstrip())
                    max_indent = max(max_indent, indent)
            if max_indent >= 24: # 6+ levels of indentation (assuming 4 spaces)
                print(f"Deep nesting found in: {filepath} (Max Indent: {max_indent})")
