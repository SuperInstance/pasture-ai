//! Onboarding Configuration - Settings and Defaults

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Full SuperInstance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperInstanceConfig {
    /// Ranch configuration
    pub ranch: RanchConfig,
    /// Model configuration
    pub model: ModelConfig,
    /// Evolution configuration
    pub evolution: EvolutionConfig,
    /// Channel configurations
    pub channels: Vec<ChannelSettings>,
    /// Agent connections
    pub connections: Vec<ConnectionSettings>,
    /// User preferences
    pub preferences: HashMap<String, String>,
}

impl Default for SuperInstanceConfig {
    fn default() -> Self {
        Self {
            ranch: RanchConfig::default(),
            model: ModelConfig::default(),
            evolution: EvolutionConfig::default(),
            channels: Vec::new(),
            connections: Vec::new(),
            preferences: HashMap::new(),
        }
    }
}

/// Ranch-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RanchConfig {
    /// Ranch name
    pub name: String,
    /// Maximum VRAM in GB
    pub max_vram_gb: u64,
    /// Maximum concurrent agents
    pub max_agents: usize,
    /// Dashboard enabled
    pub dashboard_enabled: bool,
    /// Dashboard refresh rate (ms)
    pub dashboard_refresh_ms: u64,
}

impl Default for RanchConfig {
    fn default() -> Self {
        Self {
            name: "SuperInstance Ranch".to_string(),
            max_vram_gb: 6,
            max_agents: 20,
            dashboard_enabled: true,
            dashboard_refresh_ms: 100,
        }
    }
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Base model name
    pub base_model: String,
    /// Model path
    pub model_path: String,
    /// Context length
    pub context_length: usize,
    /// KV cache size
    pub kv_cache_size: usize,
    /// Use GPU
    pub use_gpu: bool,
    /// Number of GPU layers
    pub gpu_layers: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            base_model: "phi-3-mini".to_string(),
            model_path: "pasture/base_models/phi-3-mini".to_string(),
            context_length: 4096,
            kv_cache_size: 256,
            use_gpu: true,
            gpu_layers: 32,
        }
    }
}

/// Evolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Night School enabled
    pub night_school_enabled: bool,
    /// Night School time (HH:MM)
    pub night_school_time: String,
    /// Cull threshold
    pub cull_threshold: f32,
    /// Minimum breeding pairs
    pub min_breeding_pairs: usize,
    /// Maximum offspring per night
    pub max_offspring: usize,
    /// Quarantine period (days)
    pub quarantine_days: u32,
    /// Cloud distillation enabled
    pub cloud_distillation: bool,
    /// Merge method
    pub merge_method: String,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            night_school_enabled: true,
            night_school_time: "02:00".to_string(),
            cull_threshold: 0.4,
            min_breeding_pairs: 2,
            max_offspring: 4,
            quarantine_days: 1,
            cloud_distillation: false,
            merge_method: "weighted".to_string(),
        }
    }
}

/// Channel settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    /// Channel ID
    pub id: String,
    /// Channel type
    pub channel_type: String,
    /// Display name
    pub name: String,
    /// Enabled
    pub enabled: bool,
    /// Authentication settings
    pub auth: HashMap<String, String>,
    /// Other settings
    pub settings: HashMap<String, String>,
}

/// Connection settings for agent systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionSettings {
    /// System type
    pub system_type: String,
    /// Connection name
    pub name: String,
    /// Endpoint URL
    pub endpoint: String,
    /// API key
    pub api_key: Option<String>,
    /// Two-way communication
    pub two_way: bool,
    /// Sync interval (seconds)
    pub sync_interval: u64,
}

impl SuperInstanceConfig {
    /// Load from file
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save to file
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create from onboarding state
    pub fn from_onboarding(state: &super::OnboardingState) -> Self {
        let mut config = Self::default();
        
        // Apply preferences
        if let Some(vram) = state.preferences.get("max_vram_gb") {
            config.ranch.max_vram_gb = vram.parse().unwrap_or(6);
        }
        
        if let Some(model) = state.preferences.get("base_model") {
            config.model.base_model = model.clone();
        }
        
        if let Some(time) = state.preferences.get("night_school_time") {
            config.evolution.night_school_time = time.clone();
        }
        
        // Add channels
        for channel in &state.configured_channels {
            config.channels.push(ChannelSettings {
                id: channel.id.clone(),
                channel_type: channel.channel_type.clone(),
                name: channel.name.clone(),
                enabled: channel.enabled,
                auth: channel.settings.clone(),
                settings: HashMap::new(),
            });
        }
        
        // Add connections
        for conn in &state.connected_systems {
            config.connections.push(ConnectionSettings {
                system_type: format!("{:?}", conn.system_type),
                name: conn.name.clone(),
                endpoint: conn.endpoint.clone(),
                api_key: conn.api_key.clone(),
                two_way: conn.two_way,
                sync_interval: 60,
            });
        }
        
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = SuperInstanceConfig::default();
        assert_eq!(config.ranch.max_vram_gb, 6);
    }
}
