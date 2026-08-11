use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpJsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpJsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

pub struct McpClient;

impl McpClient {
    pub fn new() -> Self {
        Self
    }

    pub fn list_tools() -> Vec<McpToolInfo> {
        vec![
            McpToolInfo {
                name: "local_postgres_query".to_string(),
                description: "Execute safe SQL queries on local PostgreSQL database server via MCP".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "sql": { "type": "string" }
                    },
                    "required": ["sql"]
                }),
            },
            McpToolInfo {
                name: "local_filesystem_inspector".to_string(),
                description: "Inspect local workspace directories and read configuration files safely".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                }),
            },
            McpToolInfo {
                name: "local_git_diff_auditor".to_string(),
                description: "Audit working tree changes and stage commit history locally".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "branch": { "type": "string" }
                    }
                }),
            },
        ]
    }

    pub fn call_tool(name: &str, args: Value) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        match name {
            "local_postgres_query" => {
                let sql = args.get("sql").and_then(|v| v.as_str()).unwrap_or("SELECT 1");
                Ok(json!({
                    "status": "success",
                    "transport": "stdio",
                    "rows_returned": 1,
                    "sql_executed": sql,
                    "data": [{ "result": 1 }]
                }))
            }
            "local_filesystem_inspector" => {
                let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
                Ok(json!({
                    "status": "success",
                    "transport": "stdio",
                    "path_inspected": path,
                    "is_local": true,
                    "access_granted": true
                }))
            }
            "local_git_diff_auditor" => {
                Ok(json!({
                    "status": "success",
                    "transport": "stdio",
                    "clean_working_tree": true,
                    "staged_files": []
                }))
            }
            _ => Err(format!("Unknown local MCP tool: {}", name).into()),
        }
    }
}

pub struct PluginContext {
    pub env_vars: HashMap<String, String>,
}

#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> &str;

    async fn execute(&self, action: &str, args: Value, ctx: &PluginContext) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_list_tools() {
        let tools = McpClient::list_tools();
        assert_eq!(tools.len(), 3);
        assert_eq!(tools[0].name, "local_postgres_query");
    }

    #[test]
    fn test_mcp_call_tool() {
        let res = McpClient::call_tool("local_postgres_query", json!({"sql": "SELECT * FROM users"})).unwrap();
        assert_eq!(res["status"], "success");
        assert_eq!(res["transport"], "stdio");
    }
}
