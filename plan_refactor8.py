# If we change to:
# match app_state.view_mode {
#     ViewMode::Grid | ViewMode::Kaleidoscope ... => {
#         match key_code { ... }
#     }
#     ViewMode::Genome => {
#         match key_code { ... }
#     }
#     ...
#     _ => {}
# }
#
# Wait, because of #[cfg(feature = "...")] on the variants, matching multiple variants with DIFFERENT cfg features in one line is impossible unless we wrap each in #[cfg], which is messy:
# ```rust
# #[cfg(feature = "nova")]
# ViewMode::Kaleidoscope |
# #[cfg(feature = "elektra")]
# ViewMode::Elektra => { ... }
# ```
# Rust DOES NOT ALLOW #[cfg] on individual OR patterns. You have to write multiple match arms.
# That's why the current code has multiple arms, one for each view, because they have different `cfg` attributes!

# So the current architecture:
# match key {
#   Down => match view {
#     #[cfg(feature = "nova")] ViewMode::Kaleidoscope => { ... }
#     #[cfg(feature = "elektra")] ViewMode::Elektra => { ... }
#     ViewMode::Grid => { ... }
#     ...
#   }
# }
# This is actually the most macro/cfg-friendly way.
