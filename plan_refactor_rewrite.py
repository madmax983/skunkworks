import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Remove empty arms
empty_arm_pattern = r"\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\}\n"
text = re.sub(empty_arm_pattern, "\n", text, flags=re.MULTILINE)

# Remove the specific grid movement patterns
# Down
grid_down = r"\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.1 < 15 \{\n\s*app_state\.grid_cursor\.1 \+= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_down, "\n", text, flags=re.MULTILINE)

# Up
grid_up = r"\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.1 > 0 \{\n\s*app_state\.grid_cursor\.1 \-= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_up, "\n", text, flags=re.MULTILINE)

# Right
grid_right = r"\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.0 < 15 \{\n\s*app_state\.grid_cursor\.0 \+= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_right, "\n", text, flags=re.MULTILINE)

# Left
grid_left = r"\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.0 > 0 \{\n\s*app_state\.grid_cursor\.0 \-= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_left, "\n", text, flags=re.MULTILINE)

text = re.sub(r"\n\n+", "\n", text)

with open("test_navigation_clean.rs", "w") as f:
    f.write(text)
