# The only tricky part is that NOT ALL views move the grid cursor.
# SOME views do other things.
# For example `ViewMode::Genome` does selected_strand/gene navigation.
# `ViewMode::Catalyst` scrolls.
# `ViewMode::Pandemonium` moves `pandemonium_cursor`.
# `ViewMode::Babel` moves `babel_focus`.
# `ViewMode::Graveyard` moves `selected_graveyard_strand`.
# `ViewMode::Dream` moves `selected_dream_trace`.
# `ViewMode::Alchemy` moves `alchemy_selection/shelf_idx/strand_idx`.
# `ViewMode::Laboratory` moves `lab_parent_a/b/method`.
# `ViewMode::Grimoire` moves `selected_sigil_index`.
# `ViewMode::Bestiary` moves `selected_organelle_index`.
# `ViewMode::Cortex` moves `selected_neuron_coords`.
# `ViewMode::Genesis` has `genesis_focus == 2` AND `grid_cursor` movement!
# `ViewMode::Quipu` moves `active_cord`.

# So we can simply say:
# ```rust
# match app_state.view_mode {
#     ViewMode::Genome => handle_genome_navigation(key_code, vm, app_state),
#     ViewMode::Catalyst => handle_catalyst_navigation(key_code, vm, app_state),
#     ViewMode::Pandemonium => handle_pandemonium_navigation(key_code, app_state),
#     ...
#     _ => handle_default_grid_navigation(key_code, app_state),
# }
# ```
# But wait, in `handle_default_grid_navigation`, we should only apply it if the view *actually* supports grid navigation! Wait, right now many views just do `{}`.
# So if a view does nothing, it will now do `grid_cursor` movement.
# Is that bad?
# Grid cursor movement doesn't hurt unless the view renders a cursor and we don't want it moving. But the grid cursor coordinates (`app_state.grid_cursor`) are simply numbers. If a view doesn't render it, updating the state variable silently is harmless and might even be desired (keeps the cursor where it was when switching back to grid view, or just moves it in the background).
# Wait, let's check if the current implementation intentionally ignores grid keys for those views.
# Yes, it explicitly has `{}` for them.
# So let's create `is_grid_navigable` to restrict it properly.
