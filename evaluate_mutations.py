import re

mutations_file = "MUTATIONS.md"

with open(mutations_file, "r") as f:
    lines = f.readlines()

for line in lines:
    if line.startswith("### "):
        print(line.strip())
