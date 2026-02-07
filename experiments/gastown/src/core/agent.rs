use crate::core::config::Agent;

pub fn list_agents() -> Vec<Agent> {
    vec![
        Agent {
            id: "agent-1".into(),
            name: "claude".into(),
            status: "idle".into(),
            current_task: None
        },
        Agent {
            id: "agent-2".into(),
            name: "codex".into(),
            status: "idle".into(),
            current_task: None
        },
    ]
}
