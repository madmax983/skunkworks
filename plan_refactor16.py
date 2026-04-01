# In `state.rs`, add:
# ```rust
# impl ViewMode {
#     pub fn is_grid_navigable(&self) -> bool {
#         match self {
#             ViewMode::Grid | ViewMode::BioticChaos => true,
#             #[cfg(feature = "nova")]
#             ViewMode::Kaleidoscope | ViewMode::Metazoa | ViewMode::Chronos | ViewMode::Logos
#             | ViewMode::Void | ViewMode::Signals | ViewMode::Sovereignty | ViewMode::Spectrogram
#             | ViewMode::Garden | ViewMode::Orca | ViewMode::Hydra | ViewMode::Hologram
#             | ViewMode::Virology | ViewMode::BioMesh | ViewMode::Reactor | ViewMode::Biolum
#             | ViewMode::Ecology => true,
#             #[cfg(feature = "elektra")]
#             ViewMode::Elektra => true,
#             #[cfg(feature = "silicon")]
#             ViewMode::Foundry => true,
#             _ => false,
#         }
#     }
# }
# ```
# Then in `navigation.rs`, I can extract all that redundant code!
