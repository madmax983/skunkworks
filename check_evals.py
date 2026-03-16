import re

with open("MUTATIONS.md", "r") as f:
    lines = f.readlines()

current_hybrid = None
has_eval = False

for line in lines:
    if line.startswith("### "):
        if current_hybrid and not has_eval:
            print(f"Missing evaluation: {current_hybrid}")
        current_hybrid = line.strip()
        has_eval = False
    elif line.startswith("- **Evaluation**:"):
        has_eval = True

if current_hybrid and not has_eval:
    print(f"Missing evaluation: {current_hybrid}")
