with open("experiments/chimera-lang/src/tui/state.rs", "r") as f:
    text = f.read()

impl_block = """
impl ViewMode {
    pub fn is_grid_navigable(&self) -> bool {
        match self {
            ViewMode::Grid | ViewMode::BioticChaos => true,
            #[cfg(feature = "nova")]
            ViewMode::Kaleidoscope | ViewMode::Metazoa | ViewMode::Chronos | ViewMode::Logos
            | ViewMode::Void | ViewMode::Signals | ViewMode::Sovereignty | ViewMode::Spectrogram
            | ViewMode::Garden | ViewMode::Orca | ViewMode::Hydra | ViewMode::Hologram
            | ViewMode::Virology | ViewMode::BioMesh | ViewMode::Reactor | ViewMode::Biolum
            | ViewMode::Ecology => true,
            #[cfg(feature = "elektra")]
            ViewMode::Elektra => true,
            #[cfg(feature = "silicon")]
            ViewMode::Foundry => true,
            _ => false,
        }
    }
}
"""
text = text.replace("#[derive(Debug, PartialEq, Clone, Copy)]\npub enum ViewMode {", impl_block + "\n#[derive(Debug, PartialEq, Clone, Copy)]\npub enum ViewMode {")

with open("test_state.rs", "w") as f:
    f.write(text)
