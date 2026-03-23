//! Telegram Channel - Integration with Telegram Bot API
//! 
//! Provides a Telegram bot interface for SuperInstance.
//! Supports:
//! - Text messages
//! - Inline keyboards
//! - Commands
//! - Group chats

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::{Channel, ChannelType, ChannelConfig, ChannelMessage, MessageSender, OutboundMessage};

/// Telegram Channel implementation
pub struct TelegramChannel {
    /// Channel configuration
    config: ChannelConfig,
    /// Connection state
    connected: Arc<RwLock<bool>>,
    /// Bot token
    token: String,
    /// Chat ID for messages
    chat_id: Option<i64>,
    /// Inbound message sender
    tx: Option<mpsc::Sender<ChannelMessage>>,
    /// Update offset for polling
    update_offset: Arc<RwLock<i64>>,
}

impl TelegramChannel {
    /// Create a new Telegram channel
    pub fn new(config: ChannelConfig) -> Result<Self> {
        let token = config.auth.token.clone()
            .ok_or_else(|| anyhow!("Telegram bot token required"))?;
        
        let chat_id = config.settings.get("chat_id")
            .and_then(|id| id.parse::<i64>().ok());
        
        Ok(Self {
            config,
            connected: Arc::new(RwLock::new(false)),
            token,
            chat_id,
            tx: None,
            update_offset: Arc::new(RwLock::new(0)),
        })
    }
    
    /// Get the bot API URL
    fn api_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }
    
    /// Send a message via Telegram API
    async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        let url = self.api_url("sendMessage");
        
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        });
        
        // In production, use reqwest to make the API call
        // let client = reqwest::Client::new();
        // let response = client.post(&url).json(&body).send().await?;
        
        debug!("📤 [Telegram] Sending to {}: {}", chat_id, 
            text.chars().take(50).as_str());
        
        Ok(())
    }
    
    /// Poll for updates
    async fn poll_updates(&self, tx: &mpsc::Sender<ChannelMessage>) -> Result<()> {
        let url = self.api_url("getUpdates");
        
        let offset = *self.update_offset.read();
        
        let body = serde_json::json!({
            "offset": offset + 1,
            "timeout": 30,
            "allowed_updates": ["message", "edited_message"]
        });
        
        // In production, use reqwest to make the API call
        // Parse updates and forward to channel manager
        
        Ok(())
    }
    
    /// Parse a Telegram update into a ChannelMessage
    fn parse_update(&self, update: TelegramUpdate) -> Option<ChannelMessage> {
        let message = update.message?;
        
        Some(ChannelMessage {
            id: message.message_id.to_string(),
            channel_id: message.chat.id.to_string(),
            channel_type: ChannelType::Telegram,
            sender: MessageSender {
                id: message.from.id.to_string(),
                username: message.from.username,
                display_name: Some(message.from.first_name),
                is_bot: message.from.is_bot,
            },
            content: message.text.unwrap_or_default(),
            timestamp: chrono::DateTime::from_timestamp(message.date as i64, 0)
                .unwrap_or_else(chrono::Utc::now),
            thread_id: None,
            reply_to: message.reply_to_message.map(|m| m.message_id.to_string()),
            attachments: Vec::new(),
        })
    }
    
    /// Format message for Telegram (Markdown)
    fn format_for_telegram(message: &OutboundMessage) -> String {
        let mut text = message.content.clone();
        
        // Telegram has a 4096 char limit
        if text.len() > 4000 {
            text = format!("{}...\n[truncated]", &text[..4000]);
        }
        
        // Escape special Markdown characters
        text = text.replace('_', "\\_")
            .replace('*', "\\*")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('~', "\\~")
            .replace('`', "\\`")
            .replace('#', "\\#")
            .replace('+', "\\+")
            .replace('-', "\\-")
            .replace('=', "\\=")
            .replace('|', "\\|")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('.', "\\.")
            .replace('!', "\\!");
        
        text
    }
}

#[async_trait]
impl Channel for TelegramChannel {
    fn id(&self) -> &str {
        &self.config.id
    }
    
    fn channel_type(&self) -> ChannelType {
        ChannelType::Telegram
    }
    
    async fn send(&self, message: OutboundMessage) -> Result<()> {
        if !*self.connected.read() {
            return Err(anyhow!("Telegram channel not connected"));
        }
        
        let chat_id = self.chat_id
            .ok_or_else(|| anyhow!("No chat ID configured"))?;
        
        let text = Self::format_for_telegram(&message);
        self.send_message(chat_id, &text).await?;
        
        Ok(())
    }
    
    async fn start(&mut self, tx: mpsc::Sender<ChannelMessage>) -> Result<()> {
        self.tx = Some(tx.clone());
        
        info!("🤖 Starting Telegram bot...");
        
        // In production, this would start polling for updates
        // tokio::spawn(async move {
        //     loop {
        //         if let Err(e) = self.poll_updates(&tx).await {
        //             warn!("Telegram poll error: {}", e);
        //         }
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //     }
        // });
        
        *self.connected.write() = true;
        
        info!("✅ Telegram bot connected (simulated)");
        
        // Simulate receiving a test message
        let test_msg = ChannelMessage {
            id: "1".to_string(),
            channel_id: self.chat_id.unwrap_or(123456789).to_string(),
            channel_type: ChannelType::Telegram,
            sender: MessageSender {
                id: "987654321".to_string(),
                username: Some("testuser".to_string()),
                display_name: Some("Test User".to_string()),
                is_bot: false,
            },
            content: "/hello from Telegram".to_string(),
            timestamp: chrono::Utc::now(),
            thread_id: None,
            reply_to: None,
            attachments: Vec::new(),
        };
        
        let _ = tx.send(test_msg).await;
        
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<()> {
        *self.connected.write() = false;
        info!("🛑 Telegram bot stopped");
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        *self.connected.read()
    }
}

/// Telegram Update structure
#[derive(Debug, Clone, Deserialize)]
struct TelegramUpdate {
    pub update_id: i64,
    pub message: Option<TelegramMessage>,
}

/// Telegram Message structure
#[derive(Debug, Clone, Deserialize)]
struct TelegramMessage {
    pub message_id: i64,
    pub from: TelegramUser,
    pub chat: TelegramChat,
    pub date: u64,
    pub text: Option<String>,
    pub reply_to_message: Option<Box<TelegramMessage>>,
}

/// Telegram User structure
#[derive(Debug, Clone, Deserialize)]
struct TelegramUser {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
}

/// Telegram Chat structure
#[derive(Debug, Clone, Deserialize)]
struct TelegramChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub title: Option<String>,
}

/// Telegram Inline Keyboard builder
pub struct TelegramKeyboard {
    buttons: Vec<Vec<TelegramButton>>,
}

/// Telegram inline button
#[derive(Debug, Clone, Serialize)]
pub struct TelegramButton {
    pub text: String,
    pub callback_data: Option<String>,
    pub url: Option<String>,
}

impl TelegramKeyboard {
    /// Create a new keyboard
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
    
    /// Add a row of buttons
    pub fn row(mut self, buttons: Vec<TelegramButton>) -> Self {
        self.buttons.push(buttons);
        self
    }
    
    /// Add a single button
    pub fn button(mut self, text: impl Into<String>, callback_data: impl Into<String>) -> Self {
        if self.buttons.is_empty() {
            self.buttons.push(Vec::new());
        }
        
        self.buttons.last_mut().unwrap().push(TelegramButton {
            text: text.into(),
            callback_data: Some(callback_data.into()),
            url: None,
        });
        
        self
    }
    
    /// Build the keyboard
    pub fn build(self) -> Vec<Vec<TelegramButton>> {
        self.buttons
    }
}

impl Default for TelegramKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_telegram_channel_creation() {
        let config = ChannelConfig {
            id: "telegram-main".to_string(),
            channel_type: ChannelType::Telegram,
            enabled: true,
            auth: super::ChannelAuth {
                token: Some("test_token".to_string()),
                ..Default::default()
            },
            settings: HashMap::new(),
        };
        
        let channel = TelegramChannel::new(config);
        assert!(channel.is_ok());
    }
    
    #[test]
    fn test_keyboard_builder() {
        let keyboard = TelegramKeyboard::new()
            .button("Yes", "yes")
            .button("No", "no")
            .build();
        
        assert_eq!(keyboard.len(), 1);
        assert_eq!(keyboard[0].len(), 2);
    }
}
