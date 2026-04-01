import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Let's see what features are used for each view in the navigation match.
features = re.findall(r"(?:#\[cfg\(.*?\)\]\s*)?ViewMode::[A-Za-z]+", text)

# Forge refactoring: `handle_navigation_input` is 966 lines!
# I will use a helper struct or a pattern match on view mode first, THEN key code, because there are ~100 view modes but only 4 keys.
# Actually, the file structure is:
# match key {
#   Down => match view { ... }
#   Up => match view { ... }
#   Right => match view { ... }
#   Left => match view { ... }
# }
# Reversing it (match view { Down =>, Up => ... }) would deduplicate all the Views that just move grid cursor.
