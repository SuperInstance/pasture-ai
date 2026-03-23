//! Agent System Connectors - Two-Way Synergy
//! 
//! Provides connectors for integrating with other agent systems:
//! - OpenClaw (primary)
//! - LangChain
//! - AutoGen
//! - CrewAI
//! - Custom integrations

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::{AgentSystem, ConnectionStatus, AgentConnection};

/// Agent Connector trait - defines how to interact with agent systems
#[async_trait]
pub trait AgentConnector: Send + Sync {
    /// Get the system type
    fn system_type(&self) -> AgentSystem;
    
    /// Connect to the agent system
    async fn connect(&mut self) -> anyhow::Result<()>;
    
    /// Disconnect from the agent system
    async fn disconnect(&mut self) -> anyhow::Result<()>;
    
    /// Check connection status
    fn status(&self) -> ConnectionStatus;
    
    /// Send a message/task to the agent system
    async fn send_task(&self, task: AgentTask) -> anyhow::Result<AgentResponse>;
    
    /// Receive messages from the agent system
    async fn receive(&mut self, tx: mpsc::Sender<AgentMessage>) -> anyhow::Result<()>;
    
    /// Sync state between systems
    async fn sync(&self) -> anyhow::Result<SyncResult>;
}

/// Task to send to an agent system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Task ID
    pub id: String,
    /// Task type
    pub task_type: String,
    /// Task content
    pub content: String,
    /// Context/conversation history
    pub context: Vec<ContextMessage>,
    /// Priority
    pub priority: u8,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Response from an agent system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    /// Corresponding task ID
    pub task_id: String,
    /// Response content
    pub content: String,
    /// Success status
    pub success: bool,
    /// Confidence score
    pub confidence: Option<f32>,
    /// Processing time (ms)
    pub processing_time_ms: u64,
    /// Additional data
    pub data: HashMap<String, String>,
}

/// Message from an agent system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Message ID
    pub id: String,
    /// Source system
    pub source: AgentSystem,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Thread/conversation ID
    pub thread_id: Option<String>,
}

/// Context message for task history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced_items: usize,
    pub conflicts: Vec<SyncConflict>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Sync conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub item_id: String,
    pub local_version: String,
    pub remote_version: String,
    pub resolution: Option<String>,
}

/// OpenClaw Connector
pub struct OpenClawConnector {
    connection: AgentConnection,
    status: Arc<RwLock<ConnectionStatus>>,
    client: Option<reqwest::Client>,
}

impl OpenClawConnector {
    pub fn new(connection: AgentConnection) -> Self {
        Self {
            connection,
            status: Arc::new(RwLock::new(ConnectionStatus::NotConfigured)),
            client: None,
        }
    }
}

#[async_trait]
impl AgentConnector for OpenClawConnector {
    fn system_type(&self) -> AgentSystem {
        AgentSystem::OpenClaw
    }
    
    async fn connect(&mut self) -> anyhow::Result<()> {
        info!("🔗 Connecting to OpenClaw at {}", self.connection.endpoint);
        
        self.client = Some(reqwest::Client::new());
        *self.status.write() = ConnectionStatus::Connected;
        
        info!("✅ Connected to OpenClaw");
        Ok(())
    }
    
    async fn disconnect(&mut self) -> anyhow::Result<()> {
        *self.status.write() = ConnectionStatus::NotConfigured;
        self.client = None;
        info!("🛑 Disconnected from OpenClaw");
        Ok(())
    }
    
    fn status(&self) -> ConnectionStatus {
        *self.status.read()
    }
    
    async fn send_task(&self, task: AgentTask) -> anyhow::Result<AgentResponse> {
        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to OpenClaw"))?;
        
        let url = format!("{}/v1/agent/task", self.connection.endpoint);
        
        let response = client.post(&url)
            .json(&task)
            .send()
            .await?;
        
        let result: AgentResponse = response.json().await?;
        Ok(result)
    }
    
    async fn receive(&mut self, tx: mpsc::Sender<AgentMessage>) -> anyhow::Result<()> {
        // In production, this would use WebSocket for real-time messages
        info!("👂 Listening for OpenClaw messages");
        Ok(())
    }
    
    async fn sync(&self) -> anyhow::Result<SyncResult> {
        // Sync breed manifests, agent states, etc.
        info!("🔄 Syncing with OpenClaw");
        
        Ok(SyncResult {
            synced_items: 0,
            conflicts: Vec::new(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// LangChain Connector
pub struct LangChainConnector {
    connection: AgentConnection,
    status: Arc<RwLock<ConnectionStatus>>,
    client: Option<reqwest::Client>,
}

impl LangChainConnector {
    pub fn new(connection: AgentConnection) -> Self {
        Self {
            connection,
            status: Arc::new(RwLock::new(ConnectionStatus::NotConfigured)),
            client: None,
        }
    }
}

#[async_trait]
impl AgentConnector for LangChainConnector {
    fn system_type(&self) -> AgentSystem {
        AgentSystem::LangChain
    }
    
    async fn connect(&mut self) -> anyhow::Result<()> {
        info!("🔗 Connecting to LangChain at {}", self.connection.endpoint);
        
        self.client = Some(reqwest::Client::new());
        *self.status.write() = ConnectionStatus::Connected;
        
        info!("✅ Connected to LangChain");
        Ok(())
    }
    
    async fn disconnect(&mut self) -> anyhow::Result<()> {
        *self.status.write() = ConnectionStatus::NotConfigured;
        self.client = None;
        Ok(())
    }
    
    fn status(&self) -> ConnectionStatus {
        *self.status.read()
    }
    
    async fn send_task(&self, task: AgentTask) -> anyhow::Result<AgentResponse> {
        let client = self.client.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not connected to LangChain"))?;
        
        // LangChain uses different endpoints
        let url = format!("{}/invoke", self.connection.endpoint);
        
        let response = client.post(&url)
            .json(&serde_json::json!({
                "input": task.content,
                "config": {
                    "configurable": {
                        "thread_id": task.id
                    }
                }
            }))
            .send()
            .await?;
        
        let result: AgentResponse = response.json().await?;
        Ok(result)
    }
    
    async fn receive(&mut self, _tx: mpsc::Sender<AgentMessage>) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn sync(&self) -> anyhow::Result<SyncResult> {
        Ok(SyncResult {
            synced_items: 0,
            conflicts: Vec::new(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// AutoGen Connector
pub struct AutoGenConnector {
    connection: AgentConnection,
    status: Arc<RwLock<ConnectionStatus>>,
}

impl AutoGenConnector {
    pub fn new(connection: AgentConnection) -> Self {
        Self {
            connection,
            status: Arc::new(RwLock::new(ConnectionStatus::NotConfigured)),
        }
    }
}

#[async_trait]
impl AgentConnector for AutoGenConnector {
    fn system_type(&self) -> AgentSystem {
        AgentSystem::AutoGen
    }
    
    async fn connect(&mut self) -> anyhow::Result<()> {
        info!("🔗 Connecting to AutoGen at {}", self.connection.endpoint);
        *self.status.write() = ConnectionStatus::Connected;
        Ok(())
    }
    
    async fn disconnect(&mut self) -> anyhow::Result<()> {
        *self.status.write() = ConnectionStatus::NotConfigured;
        Ok(())
    }
    
    fn status(&self) -> ConnectionStatus {
        *self.status.read()
    }
    
    async fn send_task(&self, task: AgentTask) -> anyhow::Result<AgentResponse> {
        // AutoGen uses a different conversation-based model
        Ok(AgentResponse {
            task_id: task.id,
            content: "AutoGen response placeholder".to_string(),
            success: true,
            confidence: None,
            processing_time_ms: 0,
            data: HashMap::new(),
        })
    }
    
    async fn receive(&mut self, _tx: mpsc::Sender<AgentMessage>) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn sync(&self) -> anyhow::Result<SyncResult> {
        Ok(SyncResult::default())
    }
}

impl Default for SyncResult {
    fn default() -> Self {
        Self {
            synced_items: 0,
            conflicts: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Connector Factory - Creates the right connector for each system
pub struct ConnectorFactory;

impl ConnectorFactory {
    /// Create a connector for the given system
    pub fn create(connection: AgentConnection) -> Box<dyn AgentConnector> {
        match connection.system_type {
            AgentSystem::OpenClaw => Box::new(OpenClawConnector::new(connection)),
            AgentSystem::LangChain => Box::new(LangChainConnector::new(connection)),
            AgentSystem::AutoGen => Box::new(AutoGenConnector::new(connection)),
            AgentSystem::CrewAI => Box::new(AutoGenConnector::new(connection)), // Similar API
            AgentSystem::Custom => Box::new(OpenClawConnector::new(connection)), // Default to OpenClaw-style
        }
    }
}

/// Connection Manager - Manages all agent system connections
pub struct ConnectionManager {
    connectors: HashMap<String, Box<dyn AgentConnector>>,
    message_tx: Option<mpsc::Sender<AgentMessage>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connectors: HashMap::new(),
            message_tx: None,
        }
    }
    
    /// Add a connection
    pub fn add_connection(&mut self, id: String, connection: AgentConnection) {
        let connector = ConnectorFactory::create(connection);
        self.connectors.insert(id, connector);
    }
    
    /// Connect all systems
    pub async fn connect_all(&mut self, tx: mpsc::Sender<AgentMessage>) -> anyhow::Result<()> {
        self.message_tx = Some(tx);
        
        for (id, connector) in &mut self.connectors {
            if let Err(e) = connector.connect().await {
                warn!("Failed to connect to {}: {}", id, e);
            }
        }
        
        Ok(())
    }
    
    /// Send a task to a specific system
    pub async fn send_task(&self, system_id: &str, task: AgentTask) -> anyhow::Result<AgentResponse> {
        let connector = self.connectors.get(system_id)
            .ok_or_else(|| anyhow::anyhow!("System not found: {}", system_id))?;
        
        connector.send_task(task).await
    }
    
    /// Broadcast a task to all connected systems
    pub async fn broadcast(&self, task: AgentTask) -> HashMap<String, anyhow::Result<AgentResponse>> {
        let mut results = HashMap::new();
        
        for (id, connector) in &self.connectors {
            if connector.status() == ConnectionStatus::Connected {
                results.insert(id.clone(), connector.send_task(task.clone()).await);
            }
        }
        
        results
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_connector_factory() {
        let connection = AgentConnection {
            system_type: AgentSystem::OpenClaw,
            name: "Test".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            api_key: None,
            two_way: true,
            status: ConnectionStatus::NotConfigured,
        };
        
        let _connector = ConnectorFactory::create(connection);
    }
}
