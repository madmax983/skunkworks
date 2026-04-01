with open("experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs", "r") as f:
    text = f.read()

import re

# We can group grid cursor movement like this:
# ```rust
# fn handle_grid_navigation(key_code: KeyCode, app_state: &mut AppState) -> bool {
#     match key_code {
#         KeyCode::Down => {
#             if app_state.grid_cursor.1 < 15 {
#                 app_state.grid_cursor.1 += 1;
#             }
#         }
#         KeyCode::Up => {
#             if app_state.grid_cursor.1 > 0 {
#                 app_state.grid_cursor.1 -= 1;
#             }
#         }
#         ...
#     }
#     true
# }
# ```
