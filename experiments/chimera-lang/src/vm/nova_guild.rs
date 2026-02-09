#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildState {
    pub name: String,
    pub members: HashSet<usize>, // Strand indices
    pub treasury: i64,
    pub founder: usize,
    pub policies: HashMap<String, i64>,
}

#[cfg(feature = "nova")]
impl GuildState {
    pub fn new(name: String, founder: usize) -> Self {
        let mut members = HashSet::new();
        members.insert(founder);
        Self {
            name,
            members,
            treasury: 0,
            founder,
            policies: HashMap::new(),
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_guild(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for guild op".to_string());
        return None;
    }

    let arg_val = vm.stack.pop().unwrap();
    let name_val = vm.stack.pop().unwrap();
    let action_val = vm.stack.pop().unwrap();

    let action = match action_val {
        Value::Str(s) => s,
        _ => {
            vm.output
                .push("Error: Guild action must be a string".to_string());
            return None;
        }
    };

    let name = match name_val {
        Value::Str(s) => s,
        _ => {
            vm.output
                .push("Error: Guild name must be a string".to_string());
            return None;
        }
    };

    let caller = vm.ip.0;

    match action.as_str() {
        "Create" => {
            if vm.guilds.contains_key(&name) {
                vm.output
                    .push(format!("GUILD: Guild '{}' already exists", name));
            } else if vm.energy < 100 {
                vm.output
                    .push("GUILD: Insufficient energy to create guild".to_string());
            } else {
                vm.energy -= 100;
                let guild = GuildState::new(name.clone(), caller);
                vm.guilds.insert(name.clone(), guild);
                vm.strand_guild_map.insert(caller, name.clone());
                vm.output.push(format!("GUILD: Created '{}'", name));
            }
        }
        "Join" => {
            if vm.strand_guild_map.contains_key(&caller) {
                vm.output
                    .push(format!("GUILD: Strand {} is already in a guild", caller));
            } else if !vm.guilds.contains_key(&name) {
                vm.output.push(format!("GUILD: Guild '{}' not found", name));
            } else if vm.energy < 10 {
                vm.output
                    .push("GUILD: Insufficient energy to join".to_string());
            } else {
                vm.energy -= 10;
                {
                    let guild = vm.guilds.get_mut(&name).unwrap();
                    guild.members.insert(caller);
                    guild.treasury += 10;
                }
                vm.strand_guild_map.insert(caller, name.clone());
                vm.output.push(format!("GUILD: Joined '{}'", name));
            }
        }
        "Leave" => {
            if !vm.guilds.contains_key(&name) {
                vm.output.push(format!("GUILD: Guild '{}' not found", name));
                return None;
            }

            let removed = {
                let guild = vm.guilds.get_mut(&name).unwrap();
                guild.members.remove(&caller)
            };

            if removed {
                vm.strand_guild_map.remove(&caller);
                vm.output.push(format!("GUILD: Left '{}'", name));
            } else {
                vm.output
                    .push(format!("GUILD: Not a member of '{}'", name));
            }
        }
        "Deposit" => {
            if let Value::Int(amount) = arg_val {
                if amount <= 0 {
                    vm.output.push("GUILD: Invalid amount".to_string());
                    return None;
                }
                if vm.energy < amount {
                    vm.output.push("GUILD: Insufficient energy".to_string());
                    return None;
                }
                if !vm.guilds.contains_key(&name) {
                    vm.output.push(format!("GUILD: Guild '{}' not found", name));
                    return None;
                }

                let mut is_member = false;
                {
                    let guild = vm.guilds.get_mut(&name).unwrap();
                    if guild.members.contains(&caller) {
                        is_member = true;
                        guild.treasury += amount;
                    }
                }

                if is_member {
                    vm.energy -= amount;
                    vm.output
                        .push(format!("GUILD: Deposited {} to '{}'", amount, name));
                } else {
                    vm.output.push(format!(
                        "GUILD: Must be a member of '{}' to deposit",
                        name
                    ));
                }
            } else {
                vm.output
                    .push("Error: Deposit amount must be integer".to_string());
            }
        }
        "Withdraw" => {
            if let Value::Int(amount) = arg_val {
                if amount <= 0 {
                    vm.output.push("GUILD: Invalid amount".to_string());
                    return None;
                }
                if !vm.guilds.contains_key(&name) {
                    vm.output.push(format!("GUILD: Guild '{}' not found", name));
                    return None;
                }

                let mut success = false;
                let mut msg = String::new();

                {
                    let guild = vm.guilds.get_mut(&name).unwrap();
                    if !guild.members.contains(&caller) {
                        msg = format!("GUILD: Must be a member of '{}'", name);
                    } else if guild.founder != caller {
                        msg = "GUILD: Only founder can withdraw".to_string();
                    } else if guild.treasury < amount {
                        msg = "GUILD: Insufficient treasury funds".to_string();
                    } else {
                        guild.treasury -= amount;
                        success = true;
                    }
                }

                if success {
                    vm.energy += amount;
                    vm.output
                        .push(format!("GUILD: Withdrew {} from '{}'", amount, name));
                } else {
                    vm.output.push(msg);
                }
            } else {
                vm.output
                    .push("Error: Withdraw amount must be integer".to_string());
            }
        }
        _ => vm.output.push(format!("GUILD: Unknown action '{}'", action)),
    }

    None
}

#[cfg(feature = "nova")]
pub fn exec_charter(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for charter op".to_string());
        return None;
    }

    let guild_name_val = vm.stack.pop().unwrap();
    let arg_val = vm.stack.pop().unwrap();
    let action_val = vm.stack.pop().unwrap();

    let action = match action_val {
        Value::Str(s) => s,
        _ => {
            vm.output
                .push("Error: Charter action must be a string".to_string());
            return None;
        }
    };

    let name = match guild_name_val {
        Value::Str(s) => s,
        _ => {
            vm.output
                .push("Error: Guild name must be a string".to_string());
            return None;
        }
    };

    let caller = vm.ip.0;

    if !vm.guilds.contains_key(&name) {
        vm.output.push(format!("CHARTER: Guild '{}' not found", name));
        return None;
    }

    let mut kick_target = None;
    let mut msg = None;

    {
        let guild = vm.guilds.get_mut(&name).unwrap();
        if guild.founder != caller {
            msg = Some("CHARTER: Only founder can amend charter".to_string());
        } else {
            match action.as_str() {
                "Tax" => {
                    if let Value::Int(rate) = arg_val {
                        if rate >= 0 && rate <= 100 {
                            guild.policies.insert("Tax".to_string(), rate);
                            msg = Some(format!("CHARTER: Set tax rate to {}%", rate));
                        } else {
                            msg = Some("CHARTER: Invalid tax rate".to_string());
                        }
                    } else {
                        msg = Some("CHARTER: Tax rate must be integer".to_string());
                    }
                }
                "Kick" => {
                    if let Value::Int(target_id) = arg_val {
                        let target = target_id as usize;
                        if guild.members.contains(&target) {
                            kick_target = Some(target);
                        } else {
                            msg = Some(format!("CHARTER: Strand {} not in guild", target));
                        }
                    } else {
                        msg = Some("CHARTER: Target ID must be integer".to_string());
                    }
                }
                _ => msg = Some(format!("CHARTER: Unknown action '{}'", action)),
            }
        }
    }

    if let Some(m) = msg {
        vm.output.push(m);
    }

    if let Some(target) = kick_target {
        {
            let guild = vm.guilds.get_mut(&name).unwrap();
            guild.members.remove(&target);
        }
        vm.strand_guild_map.remove(&target);
        vm.output.push(format!("CHARTER: Kicked strand {}", target));
    }

    None
}
