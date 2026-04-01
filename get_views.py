import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Down
down_matches = re.findall(r"ViewMode::([a-zA-Z]+) => \{\s*if app_state\.grid_cursor\.1 < 15 \{\s*app_state\.grid_cursor\.1 \+= 1;\s*\}\s*\}", text)
print("Down matches:", down_matches)

# Up
up_matches = re.findall(r"ViewMode::([a-zA-Z]+) => \{\s*if app_state\.grid_cursor\.1 > 0 \{\s*app_state\.grid_cursor\.1 \-= 1;\s*\}\s*\}", text)
print("Up matches:", up_matches)

# Right
right_matches = re.findall(r"ViewMode::([a-zA-Z]+) => \{\s*if app_state\.grid_cursor\.0 < 15 \{\s*app_state\.grid_cursor\.0 \+= 1;\s*\}\s*\}", text)
print("Right matches:", right_matches)

# Left
left_matches = re.findall(r"ViewMode::([a-zA-Z]+) => \{\s*if app_state\.grid_cursor\.0 > 0 \{\s*app_state\.grid_cursor\.0 \-= 1;\s*\}\s*\}", text)
print("Left matches:", left_matches)
