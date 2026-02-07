mod cli;
mod core;

use clap::Parser;
use cli::{Cli, Commands, RigCommands, CrewCommands, ConvoyCommands, MayorCommands};
use core::config::Config;
use core::convoy::Convoy;
use core::agent::Agent;
use core::task::generate_id;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut config = Config::load().context("Failed to load configuration")?;

    match args.command {
        Commands::Install { path, git } => {
            println!("Initializing workspace at {}", path);
            if git {
                println!("Initializing with git support...");
            }
            // Create directory if not exists
            std::fs::create_dir_all(&path)?;
            println!("Workspace created.");
        }
        Commands::Rig { command } => match command {
            RigCommands::Add { name, repo } => {
                println!("Adding rig {} from {}", name, repo);
                config.rigs.insert(name, repo);
                config.save()?;
            }
            RigCommands::List => {
                println!("Rigs:");
                for (name, repo) in &config.rigs {
                    println!("  {} -> {}", name, repo);
                }
            }
        },
        Commands::Crew { command } => match command {
            CrewCommands::Add { name, rig } => {
                if !config.rigs.contains_key(&rig) {
                    eprintln!("Error: Rig '{}' not found.", rig);
                    return Ok(());
                }
                println!("Creating crew workspace '{}' for rig '{}'", name, rig);
                // In a real implementation, this would create a worktree
            }
            CrewCommands::List => {
                println!("Crews (Not implemented yet)");
            }
        },
        Commands::Convoy { command } => match command {
            ConvoyCommands::Create { name, issues, notify: _, human: _ } => {
                let id = generate_id("cv");
                println!("Creating convoy '{}' ({})", name, id);
                let convoy = Convoy {
                    id: id.clone(),
                    name,
                    tasks: issues,
                };
                config.convoys.insert(id.clone(), convoy);
                config.current_convoy = Some(id);
                config.save()?;
            }
            ConvoyCommands::List => {
                println!("Convoys:");
                for (id, convoy) in &config.convoys {
                    println!("  {} ({}) - {} tasks", convoy.name, id, convoy.tasks.len());
                }
            }
            ConvoyCommands::Show { id } => {
                if let Some(convoy) = config.convoys.get(&id) {
                    println!("Convoy: {} ({})", convoy.name, convoy.id);
                    println!("Tasks:");
                    for task in &convoy.tasks {
                        println!("  - {}", task);
                    }
                } else {
                    println!("Convoy not found.");
                }
            }
            ConvoyCommands::Add { convoy_id, issues } => {
                 if let Some(convoy) = config.convoys.get_mut(&convoy_id) {
                     convoy.tasks.extend(issues);
                     println!("Added tasks to convoy {}.", convoy_id);
                     config.save()?;
                 } else {
                     println!("Convoy not found.");
                 }
            }
        },
        Commands::Sling { bead_id, rig, agent } => {
             let agent_name = agent.unwrap_or_else(|| "default".to_string());
             println!("Slinging bead {} to agent {} on rig {}", bead_id, agent_name, rig);
             // Create mock agent if not exists
             let agent_id = generate_id("ag");
             let agent_obj = Agent {
                 id: agent_id.clone(),
                 name: agent_name.clone(),
                 status: "Working".to_string(),
                 current_task: Some(bead_id),
             };
             config.agents.insert(agent_id, agent_obj);
             config.save()?;
        },
        Commands::Agents => {
            println!("Agents:");
            for (id, agent) in &config.agents {
                println!("  {} ({}) - Status: {}, Task: {:?}", agent.name, id, agent.status, agent.current_task);
            }
        },
        Commands::Mayor { command } => match command {
            MayorCommands::Attach => {
                println!("Attaching to Mayor session... (Press Ctrl+C to exit)");
                // Mock REPL loop
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
            MayorCommands::Start { agent } => {
                println!("Starting Mayor with agent {:?}...", agent);
            }
        }
    }

    Ok(())
}
