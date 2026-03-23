//! Channels - External Communication Connectors
//! 
//! Provides connectors for external messaging platforms:
//! - Discord
//! - Telegram  
//! - WhatsApp
//! - Slack
//! - Generic Webhook
//! 
//! These enable the Border Collie to interact with users through their
//! preferred channels, following OpenClaw's gateway pattern.

mod discord;
mod telegram;
mod webhook;
mod types;

pub use discord::*;
pub use telegram::*;
pub use webhook::*;
pub use types::*;

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::species::Intent;

/// Channel Manager - Manages all active channel connections
pub struct ChannelManager {
    /// Active channels
    channels: Arc<RwLock<HashMap<String, Box<dyn Channel>>>>,
    /// Inbound message receiver
    inbound_rx: mpsc::Receiver<ChannelMessage>,
    /// Inbound message sender (cloned for each channel)
    inbound_tx: mpsc::Sender<ChannelMessage>,
    /// Channel configurations
    configs: HashMap<String, ChannelConfig>,
}

/// A communication channel
#[async_trait]
pub trait Channel: Send + Sync {
    /// Get channel ID
    fn id(&self) -> &str;
    
    /// Get channel type
    fn channel_type(&self) -> ChannelType;
    
    /// Send a message through this channel
    async fn send(&self, message: OutboundMessage) -> anyhow::Result<()>;
    
    /// Start listening for incoming messages
    async fn start(&mut self, tx: mpsc::Sender<ChannelMessage>) -> anyhow::Result<()>;
    
    /// Stop the channel
    async fn stop(&mut self) -> anyhow::Result<()>;
    
    /// Check if channel is connected
    fn is_connected(&self) -> bool;
}

/// Types of channels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChannelType {
    Discord,
    Telegram,
    WhatsApp,
    Slack,
    Webhook,
    CLI,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Channel ID
    pub id: String,
    /// Channel type
    pub channel_type: ChannelType,
    /// Enabled
    pub enabled: bool,
    /// Authentication config (platform-specific)
    pub auth: ChannelAuth,
    /// Channel-specific settings
    pub settings: HashMap<String, String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelAuth {
    /// Bot token
    pub token: Option<String>,
    /// API key
    pub api_key: Option<String>,
    /// Webhook URL
    pub webhook_url: Option<String>,
    /// Client ID
    pub client_id: Option<String>,
    /// Client secret
    pub client_secret: Option<String>,
}

/// Message from a channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    /// Message ID
    pub id: String,
    /// Source channel
    pub channel_id: String,
    /// Channel type
    pub channel_type: ChannelType,
    /// Sender information
    pub sender: MessageSender,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Thread/conversation ID
    pub thread_id: Option<String>,
    /// Reply-to message ID
    pub reply_to: Option<String>,
    /// Attachments
    pub attachments: Vec<MessageAttachment>,
}

/// Sender information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSender {
    pub id: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub is_bot: bool,
}

/// Message attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub id: String,
    pub filename: String,
    pub url: String,
    pub content_type: String,
    pub size_bytes: u64,
}

/// Outbound message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundMessage {
    /// Target channel
    pub channel_id: String,
    /// Thread/conversation ID
    pub thread_id: Option<String>,
    /// Reply-to message ID
    pub reply_to: Option<String>,
    /// Content
    pub content: String,
    /// Embeds (for Discord)
    pub embeds: Vec<Embed>,
    /// Attachments
    pub attachments: Vec<AttachmentData>,
}

/// Embed for rich messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
}

/// Embed field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

/// Embed footer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

/// Attachment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentData {
    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

impl ChannelManager {
    /// Create a new channel manager
    pub fn new() -> Self {
        let (inbound_tx, inbound_rx) = mpsc::channel(100);
        
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            inbound_rx,
            inbound_tx,
            configs: HashMap::new(),
        }
    }
    
    /// Add a channel configuration
    pub fn add_config(&mut self, config: ChannelConfig) {
        self.configs.insert(config.id.clone(), config);
    }
    
    /// Initialize channels from configs
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        for (id, config) in &self.configs {
            if !config.enabled {
                debug!("Skipping disabled channel: {}", id);
                continue;
            }
            
            match config.channel_type {
                ChannelType::Discord => {
                    let channel = DiscordChannel::new(config.clone())?;
                    self.channels.write().insert(id.clone(), Box::new(channel));
                    info!("✅ Added Discord channel: {}", id);
                }
                ChannelType::Telegram => {
                    #[cfg(feature = "telegram")]
                    {
                        let channel = TelegramChannel::new(config.clone())?;
                        self.channels.write().insert(id.clone(), Box::new(channel));
                        info!("✅ Added Telegram channel: {}", id);
                    }
                    #[cfg(not(feature = "telegram"))]
                    {
                        warn!("Telegram support not compiled in");
                    }
                }
                ChannelType::Webhook => {
                    let channel = WebhookChannel::new(config.clone());
                    self.channels.write().insert(id.clone(), Box::new(channel));
                    info!("✅ Added Webhook channel: {}", id);
                }
                _ => {
                    warn!("Unsupported channel type: {:?}", config.channel_type);
                }
            }
        }
        
        Ok(())
    }
    
    /// Start all channels
    pub async fn start_all(&mut self) -> anyhow::Result<()> {
        let tx = self.inbound_tx.clone();
        
        for channel in self.channels.write().values_mut() {
            channel.start(tx.clone()).await?;
        }
        
        info!("📡 All channels started");
        Ok(())
    }
    
    /// Send a message through a channel
    pub async fn send(&self, message: OutboundMessage) -> anyhow::Result<()> {
        let channels = self.channels.read();
        
        if let Some(channel) = channels.get(&message.channel_id) {
            channel.send(message).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Channel not found: {}", message.channel_id))
        }
    }
    
    /// Receive the next inbound message
    pub async fn receive(&mut self) -> Option<ChannelMessage> {
        self.inbound_rx.recv().await
    }
    
    /// Get connected channels
    pub fn connected_channels(&self) -> Vec<String> {
        self.channels.read().iter()
            .filter(|(_, c)| c.is_connected())
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Convert channel message to intent
    pub fn message_to_intent(&self, message: &ChannelMessage) -> Intent {
        Intent {
            kind: "chat".to_string(),
            payload: message.content.clone(),
            priority: 5,
            timestamp: message.timestamp,
        }
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}
