with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

import re
empty_arm_pattern = r"^\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\}\n"
cleaned_text = re.sub(empty_arm_pattern, "", text, flags=re.MULTILINE)

with open("test_navigation.rs", "w") as f:
    f.write(cleaned_text)
