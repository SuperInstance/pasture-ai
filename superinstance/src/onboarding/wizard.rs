//! Onboarding Wizard - Interactive CLI Setup
//! 
//! Provides an interactive command-line wizard for setting up SuperInstance.
//! Uses dialoguer for beautiful prompts and progress indicators.

use std::path::PathBuf;

use anyhow::Result;
use console::{style, Emoji};
use dialoguer::{Input, Select, MultiSelect, Confirm, theme::ColorfulTheme};
use tracing::info;

use super::{OnboardingManager, OnboardingStep, Feature, ChannelConfig, AgentConnection, AgentSystem, ConnectionStatus};

static CHECK: Emoji<'_, '_> = Emoji("✅", "[OK]");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", ">>");
static COW: Emoji<'_, '_> = Emoji("🐄", "C");
static COLLEGE: Emoji<'_, '_> = Emoji("🎓", ">>");

/// Run the interactive onboarding wizard
pub async fn run_wizard(state_path: PathBuf) -> Result<()> {
    let mut manager = OnboardingManager::new(state_path);
    manager.load()?;
    
    // Skip if already complete
    if !manager.needs_onboarding() {
        println!("\n{} SuperInstance is already configured!", CHECK);
        println!("Run with --reconfigure to change settings.\n");
        return Ok(());
    }
    
    println!("\n{}", style("╔═══════════════════════════════════════════════════════════╗").cyan());
    println!("{}", style("║        SUPERINSTANCE - The Ranch Ecosystem                ║").cyan());
    println!("{}", style("║                                                           ║").cyan());
    println!("{}", style("║   \"Not a Superintelligence, but a loyal team              ║").cyan());
    println!("{}", style("║    that evolves every night.\"                             ║").cyan());
    println!("{}", style("╚═══════════════════════════════════════════════════════════╝").cyan());
    println!();
    
    // Step 1: Welcome
    welcome_step(&mut manager)?;
    
    // Step 2: Features
    features_step(&mut manager)?;
    
    // Step 3: Channels
    channels_step(&mut manager)?;
    
    // Step 4: Agent Connections
    agent_connections_step(&mut manager)?;
    
    // Step 5: Configuration
    configuration_step(&mut manager)?;
    
    // Complete
    complete_step(&manager)?;
    
    manager.save()?;
    
    Ok(())
}

/// Welcome step
fn welcome_step(manager: &mut OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", COLLEGE, style("WELCOME TO THE RANCH").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    println!("This wizard will help you set up your SuperInstance Ranch.");
    println!();
    println!("You'll configure:");
    println!("  {} Which features to enable", style("•").dim());
    println!("  {} Communication channels (Discord, Telegram, etc.)", style("•").dim());
    println!("  {} Connections to other agent systems", style("•").dim());
    println!("  {} Your base model and preferences", style("•").dim());
    println!();
    
    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Ready to begin?")
        .default(true)
        .interact()?;
    
    if proceed {
        manager.advance();
    }
    
    Ok(())
}

/// Feature selection step
fn features_step(manager: &mut OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", COW, style("FEATURE SELECTION").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    println!("Select the features you want to enable:");
    println!();
    
    let features = manager.available_features();
    let items: Vec<String> = features.iter()
        .map(|f| {
            let status = if f.enabled { "✓" } else { " " };
            let deps = if f.requires.is_empty() {
                String::new()
            } else {
                format!(" (requires: {})", f.requires.join(", "))
            };
            format!("[{}] {} - {}{}", status, f.name, f.description, deps)
        })
        .collect();
    
    let defaults: Vec<bool> = features.iter().map(|f| f.enabled).collect();
    
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select features (Space to toggle, Enter to confirm)")
        .items(&items)
        .defaults(&defaults)
        .interact()?;
    
    // Clear and re-add selected features
    for feature in features {
        let selected = selections.contains(&features.iter().position(|f| f.id == feature.id).unwrap());
        
        if selected {
            let _ = manager.enable_feature(&feature.id);
        } else {
            manager.disable_feature(&feature.id);
        }
    }
    
    println!();
    println!("{} Features configured!", CHECK);
    
    manager.advance();
    Ok(())
}

/// Channel configuration step
fn channels_step(manager: &mut OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", style("📡").cyan(), style("CHANNEL CONFIGURATION").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    println!("Connect your SuperInstance to communication channels:");
    println!();
    
    let channels = ["Discord", "Telegram", "WhatsApp", "Webhook", "Skip for now"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a channel to configure")
        .default(4)
        .items(&channels)
        .interact()?;
    
    if selection < channels.len() - 1 {
        let channel_type = channels[selection];
        
        println!("\n{} Configuration for {}", style("→").cyan(), channel_type);
        
        match channel_type {
            "Discord" => configure_discord(manager)?,
            "Telegram" => configure_telegram(manager)?,
            "WhatsApp" => configure_whatsapp(manager)?,
            "Webhook" => configure_webhook(manager)?,
            _ => {}
        }
    }
    
    println!();
    println!("{} Channels configured!", CHECK);
    
    manager.advance();
    Ok(())
}

/// Configure Discord channel
fn configure_discord(manager: &mut OnboardingManager) -> Result<()> {
    println!("\nTo connect to Discord, you need a Bot Token.");
    println!("1. Go to https://discord.com/developers/applications");
    println!("2. Create a new application and bot");
    println!("3. Copy the bot token\n");
    
    let token = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your Discord Bot Token")
        .allow_empty(true)
        .interact()?;
    
    if !token.is_empty() {
        let channel_id = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the default Channel ID (optional)")
            .allow_empty(true)
            .interact()?;
        
        manager.add_channel(ChannelConfig {
            id: format!("discord-{}", chrono::Utc::now().timestamp()),
            channel_type: "discord".to_string(),
            name: "Discord Main".to_string(),
            enabled: true,
            auth_configured: true,
            settings: {
                let mut s = std::collections::HashMap::new();
                s.insert("token".to_string(), token);
                if !channel_id.is_empty() {
                    s.insert("channel_id".to_string(), channel_id);
                }
                s
            },
        });
        
        println!("{} Discord configured!", CHECK);
    }
    
    Ok(())
}

/// Configure Telegram channel
fn configure_telegram(manager: &mut OnboardingManager) -> Result<()> {
    println!("\nTo connect to Telegram, you need a Bot Token.");
    println!("1. Open Telegram and search for @BotFather");
    println!("2. Send /newbot and follow instructions");
    println!("3. Copy the bot token\n");
    
    let token = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your Telegram Bot Token")
        .allow_empty(true)
        .interact()?;
    
    if !token.is_empty() {
        let chat_id = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter your Chat ID (optional)")
            .allow_empty(true)
            .interact()?;
        
        manager.add_channel(ChannelConfig {
            id: format!("telegram-{}", chrono::Utc::now().timestamp()),
            channel_type: "telegram".to_string(),
            name: "Telegram Main".to_string(),
            enabled: true,
            auth_configured: true,
            settings: {
                let mut s = std::collections::HashMap::new();
                s.insert("token".to_string(), token);
                if !chat_id.is_empty() {
                    s.insert("chat_id".to_string(), chat_id);
                }
                s
            },
        });
        
        println!("{} Telegram configured!", CHECK);
    }
    
    Ok(())
}

/// Configure WhatsApp channel
fn configure_whatsapp(manager: &mut OnboardingManager) -> Result<()> {
    println!("\nWhatsApp integration requires WhatsApp Business API.");
    println!("This feature is coming soon.\n");
    
    println!("{} WhatsApp integration coming soon!", style("ℹ").yellow());
    
    Ok(())
}

/// Configure Webhook channel
fn configure_webhook(manager: &mut OnboardingManager) -> Result<()> {
    println!("\nConfigure a generic webhook for integrations.\n");
    
    let webhook_url = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your Webhook URL")
        .allow_empty(true)
        .interact()?;
    
    if !webhook_url.is_empty() {
        manager.add_channel(ChannelConfig {
            id: format!("webhook-{}", chrono::Utc::now().timestamp()),
            channel_type: "webhook".to_string(),
            name: "Custom Webhook".to_string(),
            enabled: true,
            auth_configured: true,
            settings: {
                let mut s = std::collections::HashMap::new();
                s.insert("webhook_url".to_string(), webhook_url);
                s
            },
        });
        
        println!("{} Webhook configured!", CHECK);
    }
    
    Ok(())
}

/// Agent connections step
fn agent_connections_step(manager: &mut OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", style("🔗").cyan(), style("AGENT SYSTEM CONNECTIONS").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    println!("Connect to other agent systems for two-way synergy:");
    println!("  {} OpenClaw - Local AI agent gateway", style("•").dim());
    println!("  {} LangChain - LLM application framework", style("•").dim());
    println!("  {} AutoGen - Multi-agent conversations", style("•").dim());
    println!("  {} CrewAI - Team of AI agents", style("•").dim());
    println!();
    
    let connect = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to connect to another agent system?")
        .default(false)
        .interact()?;
    
    if connect {
        let systems = ["OpenClaw", "LangChain", "AutoGen", "CrewAI", "Custom"];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select agent system")
            .default(0)
            .items(&systems)
            .interact()?;
        
        let system_type = match selection {
            0 => AgentSystem::OpenClaw,
            1 => AgentSystem::LangChain,
            2 => AgentSystem::AutoGen,
            3 => AgentSystem::CrewAI,
            _ => AgentSystem::Custom,
        };
        
        let endpoint = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the endpoint URL")
            .default("http://localhost:8080".to_string())
            .interact()?;
        
        let api_key = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter API key (optional)")
            .allow_empty(true)
            .interact()?;
        
        let two_way = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable two-way communication?")
            .default(true)
            .interact()?;
        
        manager.add_agent_connection(AgentConnection {
            system_type,
            name: systems[selection].to_string(),
            endpoint,
            api_key: if api_key.is_empty() { None } else { Some(api_key) },
            two_way,
            status: ConnectionStatus::Configured,
        });
        
        println!("{} Agent system connected!", CHECK);
    }
    
    manager.advance();
    Ok(())
}

/// Configuration step
fn configuration_step(manager: &mut OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", style("⚙").cyan(), style("FINAL CONFIGURATION").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    // Base model selection
    let models = ["Phi-3 Mini (2.5GB)", "Mamba (2GB)", "Llama-3-8B (6GB)"];
    
    let model_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your base model")
        .default(0)
        .items(&models)
        .interact()?;
    
    manager.set_preference("base_model".to_string(), models[model_selection].to_string());
    
    // VRAM limit
    let vram = Input::<u64>::with_theme(&ColorfulTheme::default())
        .with_prompt("Maximum VRAM to use (GB)")
        .default(6)
        .interact()?;
    
    manager.set_preference("max_vram_gb".to_string(), vram.to_string());
    
    // Night school time
    let night_time = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Night School time (HH:MM, 24h)")
        .default("02:00".to_string())
        .interact()?;
    
    manager.set_preference("night_school_time".to_string(), night_time);
    
    println!();
    println!("{} Configuration complete!", CHECK);
    
    manager.advance();
    Ok(())
}

/// Complete step
fn complete_step(manager: &OnboardingManager) -> Result<()> {
    println!("\n{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!("{} {}", ROCKET, style("SETUP COMPLETE!").cyan().bold());
    println!("{}", style("═══════════════════════════════════════════════════════════").cyan());
    println!();
    
    println!("{}", manager.summary());
    println!();
    
    println!("Your Ranch is ready to go!");
    println!();
    println!("Next steps:");
    println!("  {} Run 'superinstance' to start the Ranch", style("1.").dim());
    println!("  {} Edit breed.md files to customize your agents", style("2.").dim());
    println!("  {} Watch as your agents evolve in Night School", style("3.").dim());
    println!();
    
    Ok(())
}
