import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# All views that only contain grid moving logic in Down
grid_views_down = re.findall(r"(?:#\[cfg\(.*?\)\]\s*)?ViewMode::([A-Za-z]+)\s*=>\s*\{\s*if app_state\.grid_cursor\.1 < 15 \{\s*app_state\.grid_cursor\.1 \+= 1;\s*\}\s*\}", text)
print("Grid views (Down):", grid_views_down)

empty_views_down = re.findall(r"(?:#\[cfg\(.*?\)\]\s*)?ViewMode::([A-Za-z]+)\s*=>\s*\{\s*\}", text)
print("Empty views (Down):", empty_views_down)
