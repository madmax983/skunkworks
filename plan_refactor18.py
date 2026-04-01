import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# I will write a script to completely parse the navigation.rs file, removing the repetitive ViewModes from the match arms.
# The modes to remove:
# 'Void', 'Kaleidoscope', 'Orca', 'Hologram', 'Grid', 'Logos', 'BioticChaos', 'Sovereignty', 'Chronos', 'Spectrogram', 'Signals', 'Ecology', 'Hydra', 'BioMesh', 'Virology', 'Elektra', 'Biolum', 'Reactor', 'Garden', 'Foundry', 'Metazoa'

# Also there are modes that do NOTHING! i.e. `{}`
# We can just use `_ => {}` as the default arm! Wait, `_ => {}` IS the default arm.
# Let's verify what views have `{}` arms and NO other logic!
# Oh, the match has `_ => {}` at the end already.
# So ALL empty arms `ViewMode::XYZ => {}` CAN BE REMOVED because `_ => {}` handles them!
