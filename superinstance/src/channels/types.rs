//! Channel Types - Shared types for channel communications

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ChannelType, MessageSender};

/// Re-export channel message for convenience
pub use super::{ChannelMessage, OutboundMessage, MessageAttachment, AttachmentData};

/// Embed for rich messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedImage>,
    pub author: Option<EmbedAuthor>,
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

/// Embed image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Embed author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

/// Channel status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStatus {
    pub id: String,
    pub channel_type: ChannelType,
    pub connected: bool,
    pub last_message: Option<DateTime<Utc>>,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub errors: u64,
}

/// Channel statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelStats {
    pub total_channels: usize,
    pub connected_channels: usize,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub total_errors: u64,
    pub uptime_seconds: u64,
}

impl ChannelStats {
    /// Create new stats
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a sent message
    pub fn message_sent(&mut self) {
        self.total_messages_sent += 1;
    }
    
    /// Add a received message
    pub fn message_received(&mut self) {
        self.total_messages_received += 1;
    }
    
    /// Add an error
    pub fn error(&mut self) {
        self.total_errors += 1;
    }
}

/// Message reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub emoji: String,
    pub count: u32,
    pub me: bool,
}

/// Message reference (for replies)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
}

/// Channel permission
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    Read,
    Write,
    Admin,
    Owner,
}

/// Channel member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMember {
    pub user: MessageSender,
    pub nickname: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: DateTime<Utc>,
    pub permissions: Vec<Permission>,
}

/// Typing indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingIndicator {
    pub channel_id: String,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
}

/// Presence update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdate {
    pub user_id: String,
    pub status: UserStatus,
    pub activities: Vec<Activity>,
}

/// User status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UserStatus {
    Online,
    Idle,
    DoNotDisturb,
    Offline,
}

/// User activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub url: Option<String>,
    pub details: Option<String>,
}

/// Activity type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivityType {
    Playing,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
}

/// Channel event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelEvent {
    Message(ChannelMessage),
    MessageDeleted { channel_id: String, message_id: String },
    MessageUpdated { channel_id: String, message_id: String, content: String },
    ReactionAdded { channel_id: String, message_id: String, reaction: MessageReaction },
    ReactionRemoved { channel_id: String, message_id: String, reaction: MessageReaction },
    Typing(TypingIndicator),
    MemberJoined { channel_id: String, member: ChannelMember },
    MemberLeft { channel_id: String, user_id: String },
    PresenceUpdate(PresenceUpdate),
    ChannelCreated { channel_id: String, channel_type: ChannelType },
    ChannelDeleted { channel_id: String },
    Error { channel_id: String, error: String },
}

/// Command received from a channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCommand {
    pub command: String,
    pub args: Vec<String>,
    pub channel_id: String,
    pub message_id: String,
    pub sender: MessageSender,
    pub timestamp: DateTime<Utc>,
}

impl ChannelCommand {
    /// Parse a message as a command
    pub fn from_message(prefix: &str, message: &ChannelMessage) -> Option<Self> {
        let content = message.content.trim();
        
        if !content.starts_with(prefix) {
            return None;
        }
        
        let without_prefix = content.trim_start_matches(prefix).trim();
        let parts: Vec<&str> = without_prefix.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }
        
        Some(Self {
            command: parts[0].to_lowercase(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
            channel_id: message.channel_id.clone(),
            message_id: message.id.clone(),
            sender: message.sender.clone(),
            timestamp: message.timestamp,
        })
    }
    
    /// Get an argument by index
    pub fn arg(&self, index: usize) -> Option<&str> {
        self.args.get(index).map(|s| s.as_str())
    }
    
    /// Get all arguments as a single string
    pub fn args_string(&self) -> String {
        self.args.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_parsing() {
        let message = ChannelMessage {
            id: "1".to_string(),
            channel_id: "test".to_string(),
            channel_type: ChannelType::Discord,
            sender: MessageSender {
                id: "123".to_string(),
                username: Some("testuser".to_string()),
                display_name: Some("Test User".to_string()),
                is_bot: false,
            },
            content: "!hello world".to_string(),
            timestamp: chrono::Utc::now(),
            thread_id: None,
            reply_to: None,
            attachments: Vec::new(),
        };
        
        let cmd = ChannelCommand::from_message("!", &message);
        assert!(cmd.is_some());
        
        let cmd = cmd.unwrap();
        assert_eq!(cmd.command, "hello");
        assert_eq!(cmd.args, vec!["world"]);
    }
    
    #[test]
    fn test_stats() {
        let mut stats = ChannelStats::new();
        stats.message_sent();
        stats.message_received();
        stats.message_received();
        
        assert_eq!(stats.total_messages_sent, 1);
        assert_eq!(stats.total_messages_received, 2);
    }
}
