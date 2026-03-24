import re

with open("MUTATIONS.md", "r") as f:
    content = f.read()

# Update Breeding Strategy
strategy_match = re.search(r"(\*\*Strategy Shift:\*\*\n.*?)(?=\n---\n)", content, re.DOTALL)
if strategy_match:
    old_strategy = strategy_match.group(1)
    new_strategy = old_strategy + "\nAdditionally, the success of translating complex abstract graphs (like codebases) into purely kinetic/acoustic systems (like mnem-strings) confirms that bridging static structural information with dynamic physical media (sound/magnetism) yields extraordinary hybrid vigor. We will continue exploring cross-domain translations (structure to physics)."
    content = content.replace(old_strategy, new_strategy)

# Check if mnem-strings is in proposed crosses
if "### mnem-strings" in content and "## 🌸 Proposed Crosses" in content:
    proposed_match = re.search(r"(## 🌸 Proposed Crosses\n.*?)(?=---)", content, re.DOTALL)
    if proposed_match:
        proposed_section = proposed_match.group(1)
        if "### mnem-strings" in proposed_section:
            # It's in proposed, let's remove it from there.
            pass # Actually it wasn't in proposed, I created it conceptually.

# Let's just write back the updated strategy
with open("MUTATIONS.md", "w") as f:
    f.write(content)
