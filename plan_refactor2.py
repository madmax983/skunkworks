with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

import re

# We can find all the branches where it does `if app_state.grid_cursor.1 < 15 ...`
matches = re.findall(r"(#\[cfg\(.*?\)\]\s*)?ViewMode::[A-Za-z]+\s*=>\s*\{(?:\s*if app_state\.(grid_cursor|pandemonium_cursor)\.[01] [<>] [0-9.]+ \{\s*app_state\.(grid_cursor|pandemonium_cursor)\.[01] [\+\-]= [0-9.]+;\s*\})?\s*\}", text)
print(len(matches))
