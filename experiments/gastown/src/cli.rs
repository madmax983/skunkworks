use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gastown")]
#[command(about = "Rust version of Gas Town AI orchestration tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize workspace
    Install {
        /// Path to install
        path: String,
        /// Initialize with git
        #[arg(long)]
        git: bool,
    },
    /// Manage projects (rigs)
    Rig {
        #[command(subcommand)]
        command: RigCommands,
    },
    /// Manage crew workspaces
    Crew {
        #[command(subcommand)]
        command: CrewCommands,
    },
    /// Start Mayor session
    Mayor {
        #[command(subcommand)]
        command: MayorCommands,
    },
    /// Manage convoys
    Convoy {
        #[command(subcommand)]
        command: ConvoyCommands,
    },
    /// Assign work to an agent
    Sling {
        /// Bead ID (Task ID)
        bead_id: String,
        /// Rig name
        rig: String,
        /// Agent to use
        #[arg(long)]
        agent: Option<String>,
    },
    /// List active agents
    Agents,
}

#[derive(Subcommand)]
pub enum RigCommands {
    Add {
        name: String,
        repo: String,
    },
    List,
}

#[derive(Subcommand)]
pub enum CrewCommands {
    Add {
        name: String,
        #[arg(long)]
        rig: String,
    },
    List,
}

#[derive(Subcommand)]
pub enum MayorCommands {
    Attach,
    Start {
        #[arg(long)]
        agent: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ConvoyCommands {
    Create {
        name: String,
        #[arg(num_args = 0..)]
        issues: Vec<String>,
        #[arg(long)]
        notify: bool,
        #[arg(long)]
        human: bool,
    },
    List,
    Show {
        id: String,
    },
    Add {
        convoy_id: String,
        #[arg(num_args = 1..)]
        issues: Vec<String>,
    },
}
