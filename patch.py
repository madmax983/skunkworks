import re

with open("Cargo.toml", "r") as f:
    cargo = f.read()

# Replace the [patch.crates-io] section to handle both versions if possible, or force everything to use bevy's glam version
cargo = re.sub(
    r"\[patch\.crates-io\]\n.*",
    """[patch.crates-io]
bevy_reflect = { git = "https://github.com/bevyengine/bevy", tag = "v0.14.2" }""",
    cargo, flags=re.DOTALL
)

with open("Cargo.toml", "w") as f:
    f.write(cargo)
