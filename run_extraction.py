import re

with open('experiments/chimera-lang/src/tui/app/handlers/normal.rs', 'r') as f:
    text = f.read()

start_idx = text.find('match key.code {')
end_idx = text.rfind('Ok(false)') - 1

header = text[:start_idx]
match_block = text[start_idx:end_idx]
footer = text[end_idx:]

match_body = match_block[16:]

arms = []
current_arm = ""
depth = 0
in_pattern = True

for i, c in enumerate(match_body):
    current_arm += c

    if c == '{': depth += 1
    elif c == '}': depth -= 1

    if depth == 0 and not in_pattern:
        if c == ',':
            arms.append(current_arm.strip())
            current_arm = ""
            in_pattern = True
        elif c == '}':
            if match_body[i+1:].strip() == '':
                arms.append(current_arm[:-1].strip())
                break
            else:
                arms.append(current_arm.strip())
                current_arm = ""
                in_pattern = True

    if depth == 0 and in_pattern and current_arm.endswith('=>'):
        in_pattern = False

cleaned_arms = []
for arm in arms:
    if arm == '': continue
    cleaned_arms.append(arm)

char_arms = []
nav_arms = []
action_arms = []

for arm in cleaned_arms:
    if "KeyCode::Char" in arm.split('=>')[0]:
        char_arms.append(arm)
    elif "KeyCode::Up" in arm.split('=>')[0] or "KeyCode::Down" in arm.split('=>')[0] or "KeyCode::Left" in arm.split('=>')[0] or "KeyCode::Right" in arm.split('=>')[0]:
        nav_arms.append(arm)
    else:
        action_arms.append(arm)

with open('experiments/chimera-lang/src/tui/app/handlers/normal/chars.rs', 'w') as f:
    f.write("use crate::vm::ChimeraVM;\n")
    f.write("use crate::tui::state::{AppState, InputMode, ViewMode};\n")
    f.write("use anyhow::Result;\n")
    f.write("use crossterm::event::KeyCode;\n\n")
    f.write("pub(crate) fn handle_char_input(c: char, vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {\n")
    f.write("    match KeyCode::Char(c) {\n")
    for arm in char_arms:
        f.write(f"        {arm}\n")
    f.write("        _ => {}\n")
    f.write("    }\n")
    f.write("    Ok(false)\n")
    f.write("}\n")

with open('experiments/chimera-lang/src/tui/app/handlers/normal/navigation.rs', 'w') as f:
    f.write("use crate::vm::ChimeraVM;\n")
    f.write("use crate::tui::state::{AppState, InputMode, ViewMode};\n")
    f.write("use anyhow::Result;\n")
    f.write("use crossterm::event::KeyCode;\n\n")
    f.write("pub(crate) fn handle_navigation_input(key_code: KeyCode, vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {\n")
    f.write("    match key_code {\n")
    for arm in nav_arms:
        f.write(f"        {arm}\n")
    f.write("        _ => {}\n")
    f.write("    }\n")
    f.write("    Ok(false)\n")
    f.write("}\n")

with open('experiments/chimera-lang/src/tui/app/handlers/normal/actions.rs', 'w') as f:
    f.write("use crate::vm::ChimeraVM;\n")
    f.write("use crate::tui::state::{AppState, InputMode, ViewMode};\n")
    f.write("use anyhow::Result;\n")
    f.write("use crossterm::event::KeyCode;\n\n")
    f.write("pub(crate) fn handle_action_input(key_code: KeyCode, vm: &mut ChimeraVM, app_state: &mut AppState) -> Result<bool> {\n")
    f.write("    match key_code {\n")
    for arm in action_arms:
        f.write(f"        {arm}\n")
    f.write("        _ => {}\n")
    f.write("    }\n")
    f.write("    Ok(false)\n")
    f.write("}\n")

mod_content = """pub(crate) mod chars;
pub(crate) mod navigation;
pub(crate) mod actions;

""" + header + """
                if let KeyCode::Char(c) = key.code {
                    if crate::tui::app::handlers::normal::chars::handle_char_input(c, vm, app_state)? {
                        return Ok(true);
                    }
                } else if matches!(key.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) {
                    if crate::tui::app::handlers::normal::navigation::handle_navigation_input(key.code, vm, app_state)? {
                        return Ok(true);
                    }
                } else {
                    if crate::tui::app::handlers::normal::actions::handle_action_input(key.code, vm, app_state)? {
                        return Ok(true);
                    }
                }
""" + footer

with open('experiments/chimera-lang/src/tui/app/handlers/normal/mod.rs', 'w') as f:
    f.write(mod_content)

import os
os.remove('experiments/chimera-lang/src/tui/app/handlers/normal.rs')
