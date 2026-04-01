# Let's verify which view modes are grid-navigable.
# We found:
# 'Void', 'Kaleidoscope', 'Orca', 'Hologram', 'Grid', 'Logos', 'BioticChaos', 'Sovereignty', 'Chronos', 'Spectrogram', 'Signals', 'Ecology', 'Hydra', 'BioMesh', 'Virology', 'Elektra', 'Biolum', 'Reactor', 'Garden', 'Foundry'
# Metazoa is down only? Let's check Metazoa again.
import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

print("Metazoa matches:", len(re.findall(r"ViewMode::Metazoa", text)))

# If it's only 1, that means it's missing Up/Left/Right.
