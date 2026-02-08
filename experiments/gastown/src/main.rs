mod cli;
mod core;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConvoyCommands, CrewCommands, MayorCommands, RigCommands};
use core::agent::list_agents;
use core::config::{Config, Convoy, Task};
use core::task::generate_id;
use std::process::Command;

fn main() -> Result<()> {
    let args = Cli::parse();

    // Load config, ignore error if it doesn't exist yet for Install
    let mut config = Config::load().unwrap_or_default();

    match args.command {
        Commands::Install { path, git } => {
            println!("Initializing Gas Town workspace at {}", path);
            std::fs::create_dir_all(&path)?;
            if git {
                let status = Command::new("git")
                    .arg("init")
                    .current_dir(&path)
                    .status()?;
                if !status.success() {
                    eprintln!("Failed to initialize git repository");
                }
            }
            // Ensure config exists
            config.save()?;
        }
        Commands::Rig { command } => match command {
            RigCommands::Add { name, repo } => {
                config.rigs.insert(name.clone(), repo.clone());
                println!("Added rig: {} -> {}", name, repo);
                config.save()?;
            }
            RigCommands::List => {
                if config.rigs.is_empty() {
                    println!("No rigs found.");
                } else {
                    println!("Rigs:");
                    for (name, repo) in &config.rigs {
                        println!("  - {}: {}", name, repo);
                    }
                }
            }
        },
        Commands::Crew { command } => match command {
            CrewCommands::Add { name, rig } => {
                // Simplified: just check if rig exists
                if !config.rigs.contains_key(&rig) {
                    eprintln!("Rig '{}' not found. Please add it first.", rig);
                } else {
                    println!("Created crew workspace '{}' for rig '{}'", name, rig);
                    // In real implementation, this would create a worktree or directory
                }
            }
            CrewCommands::List => {
                println!("Crews functionality not fully implemented yet.");
            }
        },
        Commands::Mayor { command } => match command {
            MayorCommands::Attach => {
                println!("Mayor attached. (Press Ctrl+C to exit)");
                // Simple REPL simulation
                loop {
                    use std::io::{self, Write};
                    print!("gastown> ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    if io::stdin().read_line(&mut input)? == 0 {
                        break;
                    }
                    let input = input.trim();
                    if input == "exit" {
                        break;
                    }
                    if !input.is_empty() {
                        println!("Mayor received: {}", input);
                        // Here we would parse natural language or commands
                    }
                }
            }
            MayorCommands::Start { agent } => {
                println!("Mayor started with agent: {:?}", agent);
            }
        },
        Commands::Convoy { command } => match command {
            ConvoyCommands::Create {
                name,
                issues,
                notify: _,
                human: _,
            } => {
                let id = generate_id();
                let convoy = Convoy {
                    id: id.clone(),
                    name: name.clone(),
                    tasks: issues.clone(),
                };
                config.convoys.insert(id.clone(), convoy);
                config.current_convoy = Some(id.clone());

                // Add tasks to global task list if they are new IDs?
                // For simplicity, we assume issues are just strings for now.
                // Or we generate task objects for them.
                for issue in issues {
                    if !config.tasks.contains_key(&issue) {
                        config.tasks.insert(
                            issue.clone(),
                            Task {
                                id: issue.clone(),
                                description: format!("Task {}", issue),
                                status: "pending".into(),
                            },
                        );
                    }
                }

                println!("Created convoy '{}' ({})", name, id);
                config.save()?;
            }
            ConvoyCommands::List => {
                if config.convoys.is_empty() {
                    println!("No convoys found.");
                } else {
                    println!("Convoys:");
                    for convoy in config.convoys.values() {
                        println!(
                            "  - {} ({}) [{} tasks]",
                            convoy.name,
                            convoy.id,
                            convoy.tasks.len()
                        );
                    }
                }
            }
            ConvoyCommands::Show { id } => {
                if let Some(convoy) = config.convoys.get(&id) {
                    println!("Convoy: {} ({})", convoy.name, convoy.id);
                    println!("Tasks:");
                    for task_id in &convoy.tasks {
                        let status = config
                            .tasks
                            .get(task_id)
                            .map(|t| t.status.as_str())
                            .unwrap_or("unknown");
                        println!("  - {} [{}]", task_id, status);
                    }
                } else {
                    eprintln!("Convoy '{}' not found", id);
                }
            }
            ConvoyCommands::Add { convoy_id, issues } => {
                if let Some(convoy) = config.convoys.get_mut(&convoy_id) {
                    for issue in issues {
                        convoy.tasks.push(issue.clone());
                        if !config.tasks.contains_key(&issue) {
                            config.tasks.insert(
                                issue.clone(),
                                Task {
                                    id: issue.clone(),
                                    description: format!("Task {}", issue),
                                    status: "pending".into(),
                                },
                            );
                        }
                    }
                    println!("Added tasks to convoy '{}'", convoy.name);
                    config.save()?;
                } else {
                    eprintln!("Convoy '{}' not found", convoy_id);
                }
            }
        },
        Commands::Sling {
            bead_id,
            rig,
            agent,
        } => {
            // Find a free agent or use specified
            let agent_name = agent.unwrap_or_else(|| "default".to_string());
            println!(
                "Slung bead '{}' to agent '{}' on rig '{}'",
                bead_id, agent_name, rig
            );

            // Update task status
            if let Some(task) = config.tasks.get_mut(&bead_id) {
                task.status = "in-progress".into();
                config.save()?;
            } else {
                // Create if not exists
                config.tasks.insert(
                    bead_id.clone(),
                    Task {
                        id: bead_id.clone(),
                        description: format!("Task {}", bead_id),
                        status: "in-progress".into(),
                    },
                );
                config.save()?;
            }
        }
        Commands::Agents => {
            let agents = list_agents();
            println!("Available Agents:");
            for agent in agents {
                println!("  - {} ({})", agent.name, agent.status);
            }
        }
    }

    Ok(())
}
