import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Let's see what features they have:
pattern = r"(?:#\[cfg\(feature = \"(.*?)\"\)\]\s*)?ViewMode::([A-Za-z]+)\s*=>\s*\{\s*if app_state\.grid_cursor"
matches = re.findall(pattern, text)

# Map ViewMode to its feature.
feature_map = {}
for feat, mode in set(matches):
    if mode not in feature_map:
        feature_map[mode] = set()
    if feat:
        feature_map[mode].add(feat)
    else:
        feature_map[mode].add(None)

for mode, feats in feature_map.items():
    print(mode, feats)
