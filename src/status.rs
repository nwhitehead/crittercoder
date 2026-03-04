enum AgentStatus {
    READY,
    WORKING,
    WAITING,
    BLOCKED,
}

struct AgentStatusRow {
    name: String,
    status: AgentStatus,
}

pub struct StatusWidget {
    agents: Vec<AgentStatusRow>,
}

impl StatusWidget {
    pub fn new() -> Self {
        Self { agents: vec![] }
    }
}
