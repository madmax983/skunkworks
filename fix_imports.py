import os
import glob

for filepath in glob.glob("experiments/chimera-lang/src/tui/app/handlers/editing/*.rs"):
    with open(filepath, "r") as f:
        content = f.read()

    # Fix import paths
    content = content.replace("super::super::state", "super::super::super::state")
    content = content.replace("super::super::get_all_views", "super::super::super::get_all_views")
    content = content.replace("super::super::parse_grid_value", "super::super::super::parse_grid_value")

    with open(filepath, "w") as f:
        f.write(content)
