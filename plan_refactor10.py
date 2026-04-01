# Look at `ViewMode::Genesis` in Up/Left/Right:
import re
with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

print("Genesis matches in Down:")
print(re.findall(r"ViewMode::Genesis => \{[^{}]*\}", text))

# Wait, `ViewMode::Genesis` doesn't exist in Up, Left, Right? Let's check:
