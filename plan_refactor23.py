# Now add `is_grid_navigable` to `state.rs`.
with open("experiments/chimera-lang/src/tui/state.rs", "r") as f:
    state_text = f.read()

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
state_text = state_text.replace("pub enum ViewMode {", impl_block + "\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum ViewMode {")

with open("test_state.rs", "w") as f:
    f.write(state_text)
