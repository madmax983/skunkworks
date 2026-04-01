# Now let's remove the "only moves grid_cursor" logic.
import re
with open("test_navigation.rs", "r") as f:
    text = f.read()

# Down: if app_state.grid_cursor.1 < 15 { app_state.grid_cursor.1 += 1; }
grid_down = r"^\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.1 < 15 \{\n\s*app_state\.grid_cursor\.1 \+= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_down, "", text, flags=re.MULTILINE)

# Up: if app_state.grid_cursor.1 > 0 { app_state.grid_cursor.1 -= 1; }
grid_up = r"^\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.1 > 0 \{\n\s*app_state\.grid_cursor\.1 \-= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_up, "", text, flags=re.MULTILINE)

# Right: if app_state.grid_cursor.0 < 15 { app_state.grid_cursor.0 += 1; }
grid_right = r"^\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.0 < 15 \{\n\s*app_state\.grid_cursor\.0 \+= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_right, "", text, flags=re.MULTILINE)

# Left: if app_state.grid_cursor.0 > 0 { app_state.grid_cursor.0 -= 1; }
grid_left = r"^\s*(?:#\[cfg\(feature = \"[^\"]+\"\)\]\n)?\s*ViewMode::[A-Za-z]+\s*=>\s*\{\n\s*if app_state\.grid_cursor\.0 > 0 \{\n\s*app_state\.grid_cursor\.0 \-= 1;\n\s*\}\n\s*\}\n"
text = re.sub(grid_left, "", text, flags=re.MULTILINE)

with open("test_navigation_2.rs", "w") as f:
    f.write(text)
