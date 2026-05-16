import re

with open("experiments/chimera-lang/src/tui/app/router.rs", "r") as f:
    content = f.read()

# Replace multiple `=> {\n render_...(f, vm, app_state);\n true\n}` blocks with a single arm.
# Or wait, the original code had a match app_state.view_mode { ... } returning false for some and true for some.

# Let's write a python script to parse the match arms and group them by return value.
match_block = re.search(r'let bypasses_glitch = match app_state.view_mode \{(.*?)\};\n\n    if bypasses_glitch', content, re.DOTALL)
if match_block:
    print("Found match block")
    arms_content = match_block.group(1)

    # Actually this refactor might break feature flags.
    # It has a bunch of #[cfg(feature = "nova")] on each arm.
    # Grouping them might be hard because of the cfgs.
