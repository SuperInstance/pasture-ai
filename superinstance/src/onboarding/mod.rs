//! Onboarding System - Interactive Setup Experience
//! 
//! Provides an OpenClaw-like onboarding experience for new users.
//! Features:
//! - Interactive CLI setup wizard
//! - Modular feature selection
//! - Channel configuration
//! - Agent system connections

pub mod wizard;
pub mod config;
pub mod connectors;

pub use wizard::*;
pub use config::*;
pub use connectors::*;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Onboarding state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingState {
    /// Whether onboarding is complete
    pub complete: bool,
    /// Current step
    pub current_step: OnboardingStep,
    /// Selected features
    pub selected_features: Vec<Feature>,
    /// Configured channels
    pub configured_channels: Vec<ChannelConfig>,
    /// Connected agent systems
    pub connected_systems: Vec<AgentConnection>,
    /// User preferences
    pub preferences: HashMap<String, String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for OnboardingState {
    fn default() -> Self {
        Self {
            complete: false,
            current_step: OnboardingStep::Welcome,
            selected_features: Vec::new(),
            configured_channels: Vec::new(),
            connected_systems: Vec::new(),
            preferences: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Onboarding steps
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OnboardingStep {
    Welcome,
    Features,
    Channels,
    AgentConnections,
    Configuration,
    Complete,
}

/// Features that can be enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub requires: Vec<String>,
    pub category: FeatureCategory,
}

/// Feature categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeatureCategory {
    Core,
    Communication,
    Intelligence,
    Evolution,
    Integration,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub id: String,
    pub channel_type: String,
    pub name: String,
    pub enabled: bool,
    pub auth_configured: bool,
    pub settings: HashMap<String, String>,
}

/// Agent system connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConnection {
    pub system_type: AgentSystem,
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub two_way: bool,
    pub status: ConnectionStatus,
}

/// Supported agent systems
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AgentSystem {
    OpenClaw,
    LangChain,
    AutoGen,
    CrewAI,
    Custom,
}

/// Connection status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectionStatus {
    NotConfigured,
    Configured,
    Connected,
    Error,
}

/// Onboarding manager
pub struct OnboardingManager {
    /// Onboarding state file path
    state_path: PathBuf,
    /// Current state
    state: OnboardingState,
    /// Available features
    available_features: Vec<Feature>,
}

impl OnboardingManager {
    /// Create a new onboarding manager
    pub fn new(state_path: PathBuf) -> Self {
        let available_features = Self::default_features();
        
        Self {
            state_path,
            state: OnboardingState::default(),
            available_features,
        }
    }
    
    /// Get default features
    fn default_features() -> Vec<Feature> {
        vec![
            Feature {
                id: "open_genomics".to_string(),
                name: "Open Genomics".to_string(),
                description: "Edit agent DNA in Markdown files".to_string(),
                enabled: true,
                requires: vec![],
                category: FeatureCategory::Core,
            },
            Feature {
                id: "night_school".to_string(),
                name: "Night School".to_string(),
                description: "Automatic agent evolution and breeding".to_string(),
                enabled: true,
                requires: vec!["open_genomics".to_string()],
                category: FeatureCategory::Evolution,
            },
            Feature {
                id: "discord_channel".to_string(),
                name: "Discord Integration".to_string(),
                description: "Connect to Discord servers and DMs".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Communication,
            },
            Feature {
                id: "telegram_channel".to_string(),
                name: "Telegram Integration".to_string(),
                description: "Connect to Telegram chats and groups".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Communication,
            },
            Feature {
                id: "whatsapp_channel".to_string(),
                name: "WhatsApp Integration".to_string(),
                description: "Connect to WhatsApp conversations".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Communication,
            },
            Feature {
                id: "openclaw_connection".to_string(),
                name: "OpenClaw Connection".to_string(),
                description: "Two-way synergy with OpenClaw agents".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Integration,
            },
            Feature {
                id: "langchain_connection".to_string(),
                name: "LangChain Integration".to_string(),
                description: "Connect to LangChain workflows".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Integration,
            },
            Feature {
                id: "terminal_dashboard".to_string(),
                name: "Terminal Dashboard".to_string(),
                description: "Rich TUI for monitoring the Ranch".to_string(),
                enabled: true,
                requires: vec![],
                category: FeatureCategory::Core,
            },
            Feature {
                id: "voice_wake".to_string(),
                name: "Voice Wake".to_string(),
                description: "Wake the Collie with voice commands".to_string(),
                enabled: false,
                requires: vec![],
                category: FeatureCategory::Intelligence,
            },
            Feature {
                id: "cloud_distillation".to_string(),
                name: "Cloud Distillation".to_string(),
                description: "Learn from cloud API calls overnight".to_string(),
                enabled: false,
                requires: vec!["night_school".to_string()],
                category: FeatureCategory::Evolution,
            },
        ]
    }
    
    /// Load existing state
    pub fn load(&mut self) -> Result<()> {
        if self.state_path.exists() {
            let content = std::fs::read_to_string(&self.state_path)?;
            self.state = serde_json::from_str(&content)?;
            info!("Loaded onboarding state from {:?}", self.state_path);
        }
        Ok(())
    }
    
    /// Save current state
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(&self.state_path, content)?;
        info!("Saved onboarding state to {:?}", self.state_path);
        Ok(())
    }
    
    /// Check if onboarding is needed
    pub fn needs_onboarding(&self) -> bool {
        !self.state.complete
    }
    
    /// Get current step
    pub fn current_step(&self) -> OnboardingStep {
        self.state.current_step
    }
    
    /// Get available features
    pub fn available_features(&self) -> &[Feature] {
        &self.available_features
    }
    
    /// Enable a feature
    pub fn enable_feature(&mut self, feature_id: &str) -> Result<()> {
        // Check if feature exists
        let feature = self.available_features.iter()
            .find(|f| f.id == feature_id)
            .ok_or_else(|| anyhow::anyhow!("Feature not found: {}", feature_id))?
            .clone();
        
        // Check dependencies
        for req in &feature.requires {
            if !self.state.selected_features.iter().any(|f| &f.id == req) {
                return Err(anyhow::anyhow!("Feature {} requires {}", feature_id, req));
            }
        }
        
        // Add to selected if not already there
        if !self.state.selected_features.iter().any(|f| &f.id == feature_id) {
            let mut enabled_feature = feature.clone();
            enabled_feature.enabled = true;
            self.state.selected_features.push(enabled_feature);
        }
        
        Ok(())
    }
    
    /// Disable a feature
    pub fn disable_feature(&mut self, feature_id: &str) {
        self.state.selected_features.retain(|f| &f.id != feature_id);
        
        // Also disable features that depend on this one
        let dependents: Vec<String> = self.state.selected_features.iter()
            .filter(|f| f.requires.iter().any(|r| r == feature_id))
            .map(|f| f.id.clone())
            .collect();
        
        for dep in dependents {
            self.disable_feature(&dep);
        }
    }
    
    /// Add a channel configuration
    pub fn add_channel(&mut self, config: ChannelConfig) {
        self.state.configured_channels.push(config);
    }
    
    /// Add an agent connection
    pub fn add_agent_connection(&mut self, connection: AgentConnection) {
        self.state.connected_systems.push(connection);
    }
    
    /// Set a preference
    pub fn set_preference(&mut self, key: String, value: String) {
        self.state.preferences.insert(key, value);
    }
    
    /// Advance to next step
    pub fn advance(&mut self) {
        self.state.current_step = match self.state.current_step {
            OnboardingStep::Welcome => OnboardingStep::Features,
            OnboardingStep::Features => OnboardingStep::Channels,
            OnboardingStep::Channels => OnboardingStep::AgentConnections,
            OnboardingStep::AgentConnections => OnboardingStep::Configuration,
            OnboardingStep::Configuration => OnboardingStep::Complete,
            OnboardingStep::Complete => OnboardingStep::Complete,
        };
        
        if self.state.current_step == OnboardingStep::Complete {
            self.state.complete = true;
        }
        
        self.state.timestamp = chrono::Utc::now();
    }
    
    /// Complete onboarding
    pub fn complete(&mut self) -> Result<()> {
        self.state.complete = true;
        self.state.current_step = OnboardingStep::Complete;
        self.save()
    }
    
    /// Get the state
    pub fn state(&self) -> &OnboardingState {
        &self.state
    }
    
    /// Generate a summary of the configuration
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str("# SuperInstance Configuration Summary\n\n");
        
        summary.push_str("## Enabled Features\n");
        for feature in &self.state.selected_features {
            summary.push_str(&format!("- ✅ {} ({})\n", feature.name, feature.id));
        }
        
        summary.push_str("\n## Configured Channels\n");
        for channel in &self.state.configured_channels {
            let status = if channel.auth_configured { "✅" } else { "⚠️" };
            summary.push_str(&format!("- {} {} ({})\n", status, channel.name, channel.channel_type));
        }
        
        summary.push_str("\n## Connected Agent Systems\n");
        for conn in &self.state.connected_systems {
            let status = match conn.status {
                ConnectionStatus::Connected => "✅",
                ConnectionStatus::Configured => "⚠️",
                _ => "❌",
            };
            let two_way = if conn.two_way { " (two-way)" } else { "" };
            summary.push_str(&format!("- {} {:?}{} - {}\n", status, conn.system_type, two_way, conn.endpoint));
        }
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_onboarding_manager() {
        let manager = OnboardingManager::new(PathBuf::from("test_state.json"));
        assert!(manager.needs_onboarding());
    }
    
    #[test]
    fn test_feature_enable() {
        let mut manager = OnboardingManager::new(PathBuf::from("test_state.json"));
        manager.enable_feature("open_genomics").unwrap();
        assert!(manager.state().selected_features.iter().any(|f| f.id == "open_genomics"));
    }
}
