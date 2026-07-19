import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

# Let's see if arthropod-poincare is in the proposed crosses section.
print("Does arthropod-poincare exist in Proposed Crosses?", "arthropod-poincare" in content.split("## 🌸 Proposed Crosses")[1].split("## 🌿 Attempted Crosses")[0])
