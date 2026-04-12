import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

# Replace Attempted Crosses block with updated content
print(content[:500])
