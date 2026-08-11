use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionStatus {
    Planned,
    Running,
    WaitingForApproval,
    Blocked,
    Failed,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionTask {
    pub task_id: String,
    pub description: String,
    pub agent_role: String,
    pub status: MissionStatus,
    pub verification_result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub mission_id: String,
    pub objective: String,
    pub status: MissionStatus,
    pub tasks: Vec<MissionTask>,
    pub current_step: usize,
}

impl Mission {
    pub fn new(objective: &str) -> Self {
        let tasks = vec![
            MissionTask {
                task_id: "task-01".to_string(),
                description: format!("Analyze requirement: {}", objective),
                agent_role: "Planner Agent".to_string(),
                status: MissionStatus::Planned,
                verification_result: None,
            },
            MissionTask {
                task_id: "task-02".to_string(),
                description: "Execute code/document generation".to_string(),
                agent_role: "Coding Agent".to_string(),
                status: MissionStatus::Planned,
                verification_result: None,
            },
            MissionTask {
                task_id: "task-03".to_string(),
                description: "Run automated self-verification & testing".to_string(),
                agent_role: "Critic Agent".to_string(),
                status: MissionStatus::Planned,
                verification_result: None,
            },
        ];

        Self {
            mission_id: format!("mission-{}", uuid::Uuid::new_v4().simple()),
            objective: objective.to_string(),
            status: MissionStatus::Planned,
            tasks,
            current_step: 0,
        }
    }

    pub fn advance_step(&mut self) {
        if self.current_step < self.tasks.len() {
            self.tasks[self.current_step].status = MissionStatus::Completed;
            self.tasks[self.current_step].verification_result = Some("VERIFIED: Clean execution".to_string());
            self.current_step += 1;
        }

        if self.current_step >= self.tasks.len() {
            self.status = MissionStatus::Completed;
        } else {
            self.status = MissionStatus::Running;
        }
    }
}
