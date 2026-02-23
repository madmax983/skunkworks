use super::normalize_coords;
use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EchoBuffer {
    pub buffer: Vec<Value>,
    pub index: usize,
    pub recording: bool,
    pub playing: bool,
    pub reversed: bool,
    pub distorted: bool,
    #[serde(default)]
    pub last_tick: u64,
}

pub fn apply_echo_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    echoes: &mut HashMap<(usize, usize), EchoBuffer>,
    tick: u64,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };

    match rune {
        "(" => {
            // Record
            let echo = echoes.entry((y, x)).or_default();

            // Only process state changes once per tick
            if echo.last_tick != tick {
                if let Some(Value::Int(1)) = n_sig {
                    echo.recording = !echo.recording;
                    if echo.recording {
                        echo.buffer.clear();
                        echo.index = 0;
                    }
                }
                echo.last_tick = tick;

                // Recording happens once per tick too
                if echo.recording {
                    if let Some(val) = w_sig.clone() {
                        echo.buffer.push(val);
                    }
                }
            }

            // Propagation logic (Output) should happen every iteration if input is present?
            // "Pass through to allow chaining"
            // If we output to next_signals, we need to ensure we don't spam?
            // Propagation loop handles stability. If we output same value, it's fine.
            if echo.recording {
                if let Some(val) = w_sig {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(val);
                            changes = true;
                        }
                    }
                }
            }
        }
        ")" => {
            // Play
            let echo = echoes.entry((y, x)).or_default();

            if echo.last_tick != tick {
                if let Some(Value::Int(1)) = w_sig {
                    echo.playing = !echo.playing;
                    if echo.playing {
                        echo.index = 0;
                    }
                }

                // Advance index ONLY once per tick
                if echo.playing && !echo.buffer.is_empty() {
                    echo.index += 1;
                }

                echo.last_tick = tick;
            }

            if echo.playing && !echo.buffer.is_empty() {
                let len = echo.buffer.len();
                // Use previous index (since we incremented for next tick?)
                // Or increment at end?
                // Let's increment at end of tick processing?
                // But we are inside loop.
                // We used `echo.index` then `echo.index += 1` in previous code.
                // Here we incremented inside `if echo.last_tick != tick`.
                // So `echo.index` is already next. We should use `index - 1`.

                let current_idx = if echo.index == 0 {
                    len - 1
                } else {
                    echo.index - 1
                };

                let idx = if echo.reversed {
                    len - 1 - (current_idx % len)
                } else {
                    current_idx % len
                };

                let mut val = echo.buffer[idx].clone();

                if echo.distorted {
                    if let Value::Int(n) = val {
                        val = Value::Int(n ^ (current_idx as i64));
                    }
                }

                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if next_signals[sy][sx].is_none() {
                        next_signals[sy][sx] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        "{" => {
            // Reverse
            let echo = echoes.entry((y, x)).or_default();
            if echo.last_tick != tick {
                if let Some(Value::Int(1)) = w_sig {
                    echo.reversed = !echo.reversed;
                }
                echo.last_tick = tick;
            }
        }
        "}" => {
            // Distort
            let echo = echoes.entry((y, x)).or_default();
            if echo.last_tick != tick {
                if let Some(Value::Int(1)) = w_sig {
                    echo.distorted = !echo.distorted;
                }
                echo.last_tick = tick;
            }
        }
        _ => {}
    }
    changes
}
