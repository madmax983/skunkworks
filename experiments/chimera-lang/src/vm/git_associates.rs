#![cfg(feature = "git")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_git_associates_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::GitHistory => {
            // stack: count (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(count) = val {
                    let limit = count.clamp(1, 100) as usize;
                    match git_associates::GitModel::open(".") {
                        Ok(model) => match model.history(limit) {
                            Ok(history) => {
                                let len = history.len();
                                vm.stack.push(Value::Int(len as i64));
                                for commit in history {
                                    vm.stack.push(Value::Str(commit.short_hash));
                                }
                                vm.energy = vm.energy.saturating_sub(5);
                                vm.output
                                    .push(format!("GITHISTORY: Retrieved {} commits", len));
                            }
                            Err(e) => {
                                vm.output.push(format!("GITHISTORY ERROR: {}", e));
                                vm.stack.push(Value::Int(0));
                            }
                        },
                        Err(e) => {
                            vm.output
                                .push(format!("GITHISTORY ERROR: Failed to open repo: {}", e));
                            vm.stack.push(Value::Int(0));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for githistory".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for githistory".to_string());
            }
            None
        }
        OpCode::GitDiffWorkspace => {
            // stack: nothing popped
            match git_associates::GitModel::open(".") {
                Ok(model) => match model.diff_workdir() {
                    Ok(stats) => {
                        vm.stack.push(Value::Int(stats.total_added as i64));
                        vm.stack.push(Value::Int(stats.total_removed as i64));
                        vm.stack.push(Value::Int(stats.files.len() as i64));
                        vm.energy = vm.energy.saturating_sub(10);
                        vm.output.push(format!(
                            "GITDIFFWORKSPACE: {} insertions, {} deletions in {} files",
                            stats.total_added,
                            stats.total_removed,
                            stats.files.len()
                        ));
                    }
                    Err(e) => {
                        vm.output.push(format!("GITDIFFWORKSPACE ERROR: {}", e));
                        vm.stack.push(Value::Int(0));
                    }
                },
                Err(e) => {
                    vm.output.push(format!(
                        "GITDIFFWORKSPACE ERROR: Failed to open repo: {}",
                        e
                    ));
                    vm.stack.push(Value::Int(0));
                }
            }
            None
        }
        _ => None,
    }
}
