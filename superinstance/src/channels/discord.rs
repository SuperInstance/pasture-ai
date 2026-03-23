//! Discord Channel - Integration with Discord via Serenity
//! 
//! Provides a Discord bot interface for SuperInstance using the Serenity library.
//! Supports:
//! - Guild messages
//! - DMs
//! - Slash commands
//! - Embeds and rich content

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::{Channel, ChannelType, ChannelConfig, ChannelMessage, MessageSender, OutboundMessage};

/// Discord Channel implementation
pub struct DiscordChannel {
    /// Channel configuration
    config: ChannelConfig,
    /// Connection state
    connected: Arc<RwLock<bool>>,
    /// Bot token
    token: String,
    /// Channel ID for messages
    channel_id: Option<u64>,
    /// Inbound message sender
    tx: Option<mpsc::Sender<ChannelMessage>>,
}

impl DiscordChannel {
    /// Create a new Discord channel
    pub fn new(config: ChannelConfig) -> Result<Self> {
        let token = config.auth.token.clone()
            .ok_or_else(|| anyhow!("Discord token required"))?;
        
        let channel_id = config.settings.get("channel_id")
            .and_then(|id| id.parse::<u64>().ok());
        
        Ok(Self {
            config,
            connected: Arc::new(RwLock::new(false)),
            token,
            channel_id,
            tx: None,
        })
    }
    
    /// Format a message for Discord
    fn format_for_discord(message: &OutboundMessage) -> String {
        let mut content = message.content.clone();
        
        // Truncate if needed (Discord limit is 2000 chars)
        if content.len() > 1900 {
            content = format!("{}...\n[truncated]", &content[..1900]);
        }
        
        content
    }
    
    /// Parse a Discord message
    fn parse_discord_message(
        &self,
        msg_id: u64,
        channel_id: u64,
        author_id: u64,
        author_name: &str,
        content: &str,
    ) -> ChannelMessage {
        ChannelMessage {
            id: msg_id.to_string(),
            channel_id: channel_id.to_string(),
            channel_type: ChannelType::Discord,
            sender: MessageSender {
                id: author_id.to_string(),
                username: Some(author_name.to_string()),
                display_name: Some(author_name.to_string()),
                is_bot: false,
            },
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
            thread_id: None,
            reply_to: None,
            attachments: Vec::new(),
        }
    }
}

#[async_trait]
impl Channel for DiscordChannel {
    fn id(&self) -> &str {
        &self.config.id
    }
    
    fn channel_type(&self) -> ChannelType {
        ChannelType::Discord
    }
    
    async fn send(&self, message: OutboundMessage) -> Result<()> {
        if !*self.connected.read() {
            return Err(anyhow!("Discord channel not connected"));
        }
        
        // In production, this would use Serenity's HTTP client
        // For now, we simulate the send
        let content = Self::format_for_discord(&message);
        
        info!("📤 [Discord] Sending message to channel {}: {}", 
            message.channel_id, 
            content.chars().take(50).as_str());
        
        // Simulate successful send
        Ok(())
    }
    
    async fn start(&mut self, tx: mpsc::Sender<ChannelMessage>) -> Result<()> {
        self.tx = Some(tx.clone());
        
        info!("🤖 Starting Discord bot...");
        
        // In production, this would start the Serenity client:
        // 
        // use serenity::prelude::*;
        // use serenity::model::prelude::*;
        // use serenity::Client;
        //
        // struct Handler;
        //
        // #[async_trait]
        // impl EventHandler for Handler {
        //     async fn message(&self, ctx: Context, msg: Message) {
        //         // Forward to channel manager
        //     }
        // }
        //
        // let mut client = Client::builder(&self.token)
        //     .event_handler(Handler)
        //     .await?;
        //
        // client.start().await?;
        
        // Simulate connection
        *self.connected.write() = true;
        
        info!("✅ Discord bot connected (simulated)");
        
        // Simulate receiving a test message
        let test_msg = self.parse_discord_message(
            1,
            self.channel_id.unwrap_or(123456789),
            987654321,
            "TestUser",
            "!hello from Discord",
        );
        
        let _ = tx.send(test_msg).await;
        
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<()> {
        *self.connected.write() = false;
        info!("🛑 Discord bot stopped");
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        *self.connected.read()
    }
}

/// Discord message handler (for Serenity integration)
pub struct DiscordHandler {
    /// Channel to send messages to
    tx: mpsc::Sender<ChannelMessage>,
    /// Bot's user ID (set after connection)
    bot_user_id: Arc<RwLock<Option<u64>>>,
}

impl DiscordHandler {
    /// Create a new handler
    pub fn new(tx: mpsc::Sender<ChannelMessage>) -> Self {
        Self {
            tx,
            bot_user_id: Arc::new(RwLock::new(None)),
        }
    }
}

/// Discord embed builder helper
pub struct DiscordEmbedBuilder {
    embed: super::Embed,
}

impl DiscordEmbedBuilder {
    /// Create a new embed builder
    pub fn new() -> Self {
        Self {
            embed: super::Embed {
                title: None,
                description: None,
                url: None,
                color: Some(0x00FF00), // Green
                fields: Vec::new(),
                footer: None,
            },
        }
    }
    
    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.embed.title = Some(title.into());
        self
    }
    
    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.embed.description = Some(description.into());
        self
    }
    
    /// Set color
    pub fn color(mut self, color: u32) -> Self {
        self.embed.color = Some(color);
        self
    }
    
    /// Add a field
    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        self.embed.fields.push(super::EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }
    
    /// Set footer
    pub fn footer(mut self, text: impl Into<String>) -> Self {
        self.embed.footer = Some(super::EmbedFooter {
            text: text.into(),
            icon_url: None,
        });
        self
    }
    
    /// Build the embed
    pub fn build(self) -> super::Embed {
        self.embed
    }
}

impl Default for DiscordEmbedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a SuperInstance status embed
pub fn create_status_embed(
    day: u64,
    active_agents: usize,
    vram_percent: f64,
    dollars_saved: f64,
) -> super::Embed {
    DiscordEmbedBuilder::new()
        .title("🐄 SuperInstance Ranch Status")
        .color(0x00FF00)
        .field("Day", day.to_string(), true)
        .field("Active Agents", active_agents.to_string(), true)
        .field("VRAM", format!("{:.1}%", vram_percent), true)
        .field("Cloud Savings", format!("${:.2}", dollars_saved), true)
        .description("The Ranch is operating normally.")
        .footer("SuperInstance v0.1.0")
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_discord_channel_creation() {
        let config = ChannelConfig {
            id: "discord-main".to_string(),
            channel_type: ChannelType::Discord,
            enabled: true,
            auth: super::ChannelAuth {
                token: Some("test_token".to_string()),
                ..Default::default()
            },
            settings: HashMap::new(),
        };
        
        let channel = DiscordChannel::new(config);
        assert!(channel.is_ok());
    }
    
    #[test]
    fn test_embed_builder() {
        let embed = DiscordEmbedBuilder::new()
            .title("Test")
            .description("Test description")
            .field("Field 1", "Value 1", false)
            .build();
        
        assert_eq!(embed.title, Some("Test".to_string()));
        assert_eq!(embed.fields.len(), 1);
    }
}
