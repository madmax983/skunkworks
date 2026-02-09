
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running,
    Waiting,
    Completed,
    Starving,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: usize,
    pub name: String,
    pub work_remaining: usize,
    pub credits: f32,
    pub column: usize,
    pub state: ProcessState,
    pub starvation_level: usize,
}

impl Process {
    pub fn new(id: usize, name: String, work: usize, credits: f32, column: usize) -> Self {
        Self {
            id,
            name,
            work_remaining: work,
            credits,
            column,
            state: ProcessState::Waiting,
            starvation_level: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.work_remaining > 0
    }
}
