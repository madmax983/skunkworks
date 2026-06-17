import re

with open("crates/git-associates/src/lib.rs", "r") as f:
    content = f.read()

print("Vec::with_capacity" in content)
