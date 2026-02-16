#![cfg(feature = "nova")]

use crate::vm::{ChimeraVM, Value};
use regex::Regex;

/// Executes a Live Grid Parse.
///
/// Traces a path on the grid, consuming the input string.
/// Returns `true` if parsing was successful (reached a '!' end node with input consumed).
pub fn exec_live_parse(
    vm: &mut ChimeraVM,
    start_y: usize,
    start_x: usize,
    input: String,
) -> bool {
    let mut cursor_y = start_y;
    let mut cursor_x = start_x;
    let mut input_idx = 0;
    let input_chars: Vec<char> = input.chars().collect();
    let max_steps = 1000;
    let mut steps = 0;

    vm.babel_live_trace.clear();

    // Default direction: East
    let mut dy: i64 = 0;
    let mut dx: i64 = 1;

    while steps < max_steps {
        if !vm.is_valid_coord(cursor_y as i64, cursor_x as i64) {
            break;
        }

        vm.babel_live_trace.push((cursor_y, cursor_x));
        let cell = vm.grid[cursor_y][cursor_x].clone();

        match cell {
            Value::Str(s) => {
                let op_char = s.chars().next().unwrap_or('\0');
                match op_char {
                    '"' => {
                        // String literal mode: consume grid cells until closing quote
                        // and match against input.
                        let mut match_buffer = String::new();
                        let mut temp_x = cursor_x;
                        let mut temp_y = cursor_y;

                        // Advance one step first to enter string
                        if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                            temp_y = ny;
                            temp_x = nx;
                        } else {
                            break;
                        }

                        let mut matched = true;
                        loop {
                            vm.babel_live_trace.push((temp_y, temp_x));
                            if let Value::Str(ref char_s) = vm.grid[temp_y][temp_x] {
                                if char_s == "\"" {
                                    break;
                                }
                                if input_idx < input_chars.len() {
                                    let expected = char_s.chars().next().unwrap_or('\0');
                                    if input_chars[input_idx] == expected {
                                        input_idx += 1;
                                    } else {
                                        matched = false;
                                        // Continue traversing to update trace, but fail match
                                    }
                                } else {
                                    matched = false;
                                }
                            } else {
                                matched = false; // Non-string cell in string path
                            }

                            if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                                temp_y = ny;
                                temp_x = nx;
                            } else {
                                matched = false;
                                break;
                            }
                        }

                        if !matched {
                            return false;
                        }
                        cursor_y = temp_y;
                        cursor_x = temp_x;
                    }
                    '[' => {
                        // Regex mode
                        let mut regex_str = String::new();
                        let mut temp_x = cursor_x;
                        let mut temp_y = cursor_y;

                        // Advance
                        if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                            temp_y = ny;
                            temp_x = nx;
                        } else {
                            break;
                        }

                        loop {
                            vm.babel_live_trace.push((temp_y, temp_x));
                            if let Value::Str(ref char_s) = vm.grid[temp_y][temp_x] {
                                if char_s == "]" {
                                    break;
                                }
                                regex_str.push_str(char_s);
                            } else {
                                break;
                            }

                            if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                                temp_y = ny;
                                temp_x = nx;
                            } else {
                                break;
                            }
                        }

                        let anchored = format!("^{}", regex_str);
                        if let Ok(re) = Regex::new(&anchored) {
                            let remaining_input: String = input_chars[input_idx..].iter().collect();
                            if let Some(mat) = re.find(&remaining_input) {
                                let char_count = mat.as_str().chars().count();
                                input_idx += char_count;
                                cursor_y = temp_y;
                                cursor_x = temp_x;
                            } else {
                                return false;
                            }
                        } else {
                            return false; // Invalid Regex
                        }
                    }
                    '{' => {
                        // Action mode: Execute embedded ChimeraScript
                        let mut action_code = String::new();
                        let mut temp_x = cursor_x;
                        let mut temp_y = cursor_y;

                        // Advance
                        if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                            temp_y = ny;
                            temp_x = nx;
                        } else {
                            break;
                        }

                        loop {
                            vm.babel_live_trace.push((temp_y, temp_x));
                            if let Value::Str(ref char_s) = vm.grid[temp_y][temp_x] {
                                if char_s == "}" {
                                    break;
                                }
                                action_code.push_str(char_s);
                            } else {
                                break;
                            }

                            if let Some((ny, nx)) = vm.normalize_coords(temp_y as i64 + dy, temp_x as i64 + dx) {
                                temp_y = ny;
                                temp_x = nx;
                            } else {
                                break;
                            }
                        }

                        // Execute Action
                        let src = format!("strand action {{ {} }}", action_code);
                        if let Ok(dna) = crate::compiler::compile(&src, None) {
                            if let Some(strand) = dna.helix.strands.first() {
                                // Execute immediately
                                for gene in &strand.genes {
                                    let _ = vm.execute_gene_inner(gene.op.clone(), &gene.args);
                                    // If halted or died, stop parsing
                                    if vm.halted || vm.energy <= 0 {
                                        return false;
                                    }
                                }
                            }
                        } else {
                            vm.output.push(format!("BABEL ERROR: Failed to compile action '{}'", action_code));
                            return false;
                        }

                        cursor_y = temp_y;
                        cursor_x = temp_x;
                    }
                    '>' => { dy = 0; dx = 1; }
                    '<' => { dy = 0; dx = -1; }
                    '^' => { dy = -1; dx = 0; }
                    'v' => { dy = 1; dx = 0; }
                    '!' => {
                        return input_idx == input_chars.len();
                    }
                    '.' | '-' | '|' => {
                        // Pass-through connectors
                    }
                    _ => {
                        // Unknown symbol acts as wall or end?
                        // Treat as barrier.
                        return false;
                    }
                }
            }
            _ => {
                // Non-string value is a barrier
                return false;
            }
        }

        // Advance cursor
        if let Some((ny, nx)) = vm.normalize_coords(cursor_y as i64 + dy, cursor_x as i64 + dx) {
            cursor_y = ny;
            cursor_x = nx;
        } else {
            break;
        }

        steps += 1;
    }

    false
}
