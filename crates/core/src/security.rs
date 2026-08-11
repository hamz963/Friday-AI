use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionMode {
    Assistant,  // Ask user before every action
    Operator,   // Auto-execute safe operations; ask before dangerous ones
    Autonomous, // Perform missions independently within strict sandbox
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunReport {
    pub mission_id: String,
    pub target_files_modified: Vec<PathBuf>,
    pub commands_to_execute: Vec<String>,
    pub risk_score: u8, // 1 to 10
    pub requires_external_api: bool,
    pub estimated_token_cost: f32,
    pub recommendation: String,
}

pub struct SecurityKernel {
    pub mode: PermissionMode,
    pub blocked_command_patterns: Vec<String>,
}

impl SecurityKernel {
    pub fn new(mode: PermissionMode) -> Self {
        Self {
            mode,
            blocked_command_patterns: vec![
                "rm -rf /".to_string(),
                "format".to_string(),
                "drop database".to_string(),
                "sudo rm".to_string(),
                "chmod 777 /".to_string(),
            ],
        }
    }

    pub fn sanitize_untrusted_input(&self, untrusted_content: &str) -> String {
        // Strip potential system prompt overrides & instructions from external web/pdf
        let mut sanitized = untrusted_content.replace("System:", "External Content:")
            .replace("USER_REQUEST:", "EXTERNAL_DATA:")
            .replace("Ignore previous instructions", "[Filtered untrusted instruction]");
        
        if sanitized.len() > 15000 {
            sanitized.truncate(15000);
            sanitized.push_str("\n[Truncated untrusted payload for security]");
        }
        sanitized
    }

    pub fn generate_dry_run(&self, mission_id: &str, files: Vec<PathBuf>, commands: Vec<String>) -> DryRunReport {
        let mut risk = 1;
        for cmd in &commands {
            for pattern in &self.blocked_command_patterns {
                if cmd.to_lowercase().contains(pattern) {
                    risk += 5;
                }
            }
        }

        let recommendation = if risk > 5 {
            "CRITICAL: Require explicit user authorization before executing."
        } else {
            "SAFE: Proceed with automated execution and checkpoint snapshot."
        };

        DryRunReport {
            mission_id: mission_id.to_string(),
            target_files_modified: files,
            commands_to_execute: commands,
            risk_score: risk.min(10),
            requires_external_api: false,
            estimated_token_cost: 0.05,
            recommendation: recommendation.to_string(),
        }
    }

    pub fn is_command_permitted(&self, cmd: &str) -> bool {
        let lower = cmd.to_lowercase();
        for pattern in &self.blocked_command_patterns {
            if lower.contains(pattern) {
                return false;
            }
        }
        true
    }
}
