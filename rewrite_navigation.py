import re

with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

# Let's count how many lines we can save if we write a custom `match app_state.view_mode` that handles `key_code` internally:

# Wait, `handle_navigation_input` basically takes `key_code`.
# We can just change it to match on `app_state.view_mode` FIRST, and then inside that, match on `key_code`.
#
# match app_state.view_mode {
#     ViewMode::Grid | #[cfg(feature="nova")] ViewMode::Kaleidoscope | ... => {
#         match key_code {
#             KeyCode::Down => if app_state.grid_cursor.1 < 15 { app_state.grid_cursor.1 += 1; },
#             KeyCode::Up => if app_state.grid_cursor.1 > 0 { app_state.grid_cursor.1 -= 1; },
#             ...
#         }
#     }
#     ViewMode::Genome => {
#         match key_code {
#             KeyCode::Down => ...
#             ...
#         }
#     }
#     ...
# }

# Does `#[cfg(feature="nova")] ViewMode::Kaleidoscope | ViewMode::Grid =>` compile in Rust?
# NO, Rust DOES NOT allow `#[cfg(..)]` on individual OR (`|`) patterns in a single match arm.
# You have to write:
# ```rust
# #[cfg(feature = "nova")]
# ViewMode::Kaleidoscope | ViewMode::Metazoa | ... => { ... }
# #[cfg(feature = "elektra")]
# ViewMode::Elektra => { ... }
# ViewMode::Grid | ViewMode::BioticChaos => { ... }
# ```
