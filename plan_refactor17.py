# There is no `impl ViewMode` yet. I'll add it right below `enum ViewMode`.
# What about `ViewMode::Genesis`? It has:
# `if app_state.genesis_focus == 2 && app_state.grid_cursor.1 < 15 { ... }`
# I can just leave `Genesis` in the match loop.
# But wait! If I leave it in the match loop, then I STILL have the big match loop!
# Yes, but it will have only ~15 variants instead of ~100!
