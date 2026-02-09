#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tissue {
    pub id: usize,
    pub members: Vec<u64>, // IDs of Organelles
}

pub fn exec_bond(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., direction ] -> [ ..., tissue_id ]
    // Direction: 0=N, 1=E, 2=S, 3=W
    if let Some(Value::Int(dir)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let (dy, dx) = match dir.rem_euclid(4) {
            0 => (-1, 0), // N
            1 => (0, 1),  // E
            2 => (1, 0),  // S
            3 => (0, -1), // W
            _ => (0, 0),
        };

        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            // Find current organelle
            let mut current_org_idx = None;
            for (i, org) in vm.organelles.iter().enumerate() {
                if org.context_loc == (cy, cx) {
                    current_org_idx = Some(i);
                    break;
                }
            }

            // Find neighbor organelle
            let mut neighbor_org_idx = None;
            for (i, org) in vm.organelles.iter().enumerate() {
                if org.context_loc == (ny, nx) {
                    neighbor_org_idx = Some(i);
                    break;
                }
            }

            if let (Some(c_idx), Some(n_idx)) = (current_org_idx, neighbor_org_idx) {
                let c_id = vm.organelles[c_idx].id;
                let n_id = vm.organelles[n_idx].id;
                let c_tissue = vm.organelles[c_idx].tissue_id;
                let n_tissue = vm.organelles[n_idx].tissue_id;

                let final_tissue_id = match (c_tissue, n_tissue) {
                    (None, None) => {
                        // Create new tissue
                        let new_id = vm.tissues.keys().max().map(|k| k + 1).unwrap_or(1);
                        let tissue = Tissue {
                            id: new_id,
                            members: vec![c_id, n_id],
                        };
                        vm.tissues.insert(new_id, tissue);
                        vm.organelles[c_idx].tissue_id = Some(new_id);
                        vm.organelles[n_idx].tissue_id = Some(new_id);
                        new_id
                    }
                    (Some(tid), None) => {
                        // Add neighbor to current tissue
                        if let Some(tissue) = vm.tissues.get_mut(&tid) {
                            tissue.members.push(n_id);
                        }
                        vm.organelles[n_idx].tissue_id = Some(tid);
                        tid
                    }
                    (None, Some(tid)) => {
                        // Add current to neighbor tissue
                        if let Some(tissue) = vm.tissues.get_mut(&tid) {
                            tissue.members.push(c_id);
                        }
                        vm.organelles[c_idx].tissue_id = Some(tid);
                        tid
                    }
                    (Some(tid_c), Some(tid_n)) => {
                        if tid_c == tid_n {
                            tid_c
                        } else {
                            // Merge tissues (keep smaller ID)
                            let (keep_id, merge_id) = if tid_c < tid_n {
                                (tid_c, tid_n)
                            } else {
                                (tid_n, tid_c)
                            };

                            // Move members from merge_id to keep_id
                            if let Some(merge_tissue) = vm.tissues.remove(&merge_id) {
                                let members_to_move = merge_tissue.members;
                                if let Some(keep_tissue) = vm.tissues.get_mut(&keep_id) {
                                    keep_tissue.members.extend(&members_to_move);
                                }

                                // Update organelle pointers
                                for org in &mut vm.organelles {
                                    if org.tissue_id == Some(merge_id) {
                                        org.tissue_id = Some(keep_id);
                                    }
                                }
                            }
                            keep_id
                        }
                    }
                };

                vm.stack.push(Value::Int(final_tissue_id as i64));
                vm.energy = vm.energy.saturating_sub(10);
                vm.output.push(format!("BOND: Joined tissue {}", final_tissue_id));
            } else {
                vm.stack.push(Value::Int(0)); // Fail
                vm.output.push("BOND: No neighbor found".to_string());
            }
        } else {
            vm.stack.push(Value::Int(0));
            vm.output.push("BOND: Boundary reached".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for bond".to_string());
    }
    None
}

pub fn exec_unbond(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ... ] -> [ ... ] (Just leaves current tissue)
    // Actually, maybe argument is unused?
    // "Severs the bond with a neighbor" implied directional severing, but tissue model is a set.
    // So "Unbond" removes SELF from tissue.

    // Find current organelle
    let (cy, cx) = vm.context_loc;
    let mut current_org_idx = None;
    for (i, org) in vm.organelles.iter().enumerate() {
        if org.context_loc == (cy, cx) {
            current_org_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = current_org_idx {
        if let Some(tid) = vm.organelles[idx].tissue_id {
            vm.organelles[idx].tissue_id = None;
            if let Some(tissue) = vm.tissues.get_mut(&tid) {
                let my_id = vm.organelles[idx].id;
                tissue.members.retain(|&id| id != my_id);
                if tissue.members.is_empty() {
                    vm.tissues.remove(&tid);
                }
            }
            vm.energy = vm.energy.saturating_sub(5);
            vm.output.push(format!("UNBOND: Left tissue {}", tid));
        } else {
            vm.output.push("UNBOND: Not in a tissue".to_string());
        }
    }
    None
}

pub fn exec_signify(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., value ]
    if let Some(val) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        // Find current organelle
        let mut current_org_info = None;
        for org in &vm.organelles {
            if org.context_loc == (cy, cx) {
                current_org_info = Some((org.id, org.tissue_id));
                break;
            }
        }

        if let Some((my_id, Some(tid))) = current_org_info {
            if let Some(tissue) = vm.tissues.get(&tid) {
                let members = tissue.members.clone();
                let mut count = 0;
                for &member_id in &members {
                    if member_id == my_id { continue; }
                    // Find member
                    for org in &mut vm.organelles {
                        if org.id == member_id {
                            org.stack.push(val.clone());
                            count += 1;
                        }
                    }
                }
                vm.energy = vm.energy.saturating_sub(count * 2);
                vm.output.push(format!("SIGNIFY: Sent to {} members", count));
            }
        } else {
            vm.output.push("SIGNIFY: Not in a tissue".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for signify".to_string());
    }
    None
}

pub fn exec_tissue(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let mut tid = 0;
    for org in &vm.organelles {
        if org.context_loc == (cy, cx) {
            if let Some(id) = org.tissue_id {
                tid = id as i64;
            }
            break;
        }
    }
    vm.stack.push(Value::Int(tid));
    None
}

pub fn cleanup_tissues(vm: &mut ChimeraVM) {
    let active_ids: HashSet<u64> = vm.organelles.iter().map(|o| o.id).collect();
    let mut empty_tissues = Vec::new();

    for (tid, tissue) in &mut vm.tissues {
        tissue.members.retain(|id| active_ids.contains(id));
        if tissue.members.is_empty() {
            empty_tissues.push(*tid);
        }
    }

    for tid in empty_tissues {
        vm.tissues.remove(&tid);
    }
}
