//! Cattle Module - LLM inference via llama-cpp-rs
//! Handles model loading, downloading, and chat completion

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

// ============================================================================
// Types
// ============================================================================

/// Breed configuration from breed.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedConfig {
    pub name: String,
    pub genes: Vec<Gene>,
    pub config: Option<ModelConfig>,
    pub system_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    pub name: String,
    pub expression: f32,
    #[serde(default)]
    pub dominant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub context_size: Option<u32>,
}

/// Chat request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub prompt: String,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

/// Chat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
    pub tokens_used: Option<u32>,
}

/// Cattle state - manages the LLM
#[derive(Debug)]
pub struct CattleState {
    pub model_path: PathBuf,
    pub breed_config: Option<BreedConfig>,
    pub model_loaded: bool,
}

impl CattleState {
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            model_path,
            breed_config: None,
            model_loaded: false,
        }
    }

    /// Load breed configuration from pasture
    pub fn load_breed(&mut self, breed_path: &PathBuf) -> Result<()> {
        let content = std::fs::read_to_string(breed_path)?;
        let config: BreedConfig = toml::from_str(&content)?;
        self.breed_config = Some(config);
        info!("Loaded breed config: {:?}", self.breed_config.as_ref().map(|b| &b.name));
        Ok(())
    }
}

// ============================================================================
// Model Management
// ============================================================================

/// Default model URL
const MODEL_URL: &str = "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf";

/// Ensure model is downloaded
pub async fn ensure_model(model_path: &PathBuf) -> Result<()> {
    if model_path.exists() {
        info!("Model already exists at {:?}", model_path);
        return Ok(());
    }

    info!("Model not found, downloading phi3-mini.gguf...");
    download_model(model_path).await
}

/// Download the model file
async fn download_model(model_path: &PathBuf) -> Result<()> {
    use std::io::Write;

    // Create parent directory
    if let Some(parent) = model_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    info!("Downloading from {}", MODEL_URL);

    let response = reqwest::get(MODEL_URL).await?;

    if !response.status().is_success() {
        anyhow::bail!("Download failed: HTTP {}", response.status());
    }

    let bytes = response.bytes().await?;

    let mut file = std::fs::File::create(model_path)?;
    file.write_all(&bytes)?;

    info!("Model downloaded: {:?} ({} bytes)", model_path, bytes.len());
    Ok(())
}

// ============================================================================
// Chat Completion
// ============================================================================

/// Perform chat completion using llama-cpp-rs
pub async fn chat_completion(
    state: &Arc<Mutex<CattleState>>,
    request: ChatRequest,
) -> Result<ChatResponse> {
    let _model_path = {
        let s = state.lock().await;
        s.model_path.clone()
    };

    // Get breed config for system prompt
    let system_prompt = {
        let s = state.lock().await;
        s.breed_config.as_ref()
            .map(|b| b.system_prompt.clone())
            .unwrap_or_else(|| "You are a helpful assistant.".to_string())
    };

    let system = request.system.unwrap_or(system_prompt);
    let _temperature = request.temperature.unwrap_or(0.7);
    let _max_tokens = request.max_tokens.unwrap_or(512);

    // Try to use llama-cpp-rs if available
    #[cfg(feature = "llm")]
    {
        use llama_cpp_rs::Llama;

        // Ensure model is available (only download in LLM mode)
        ensure_model(&model_path).await?;

        info!("Loading model from {:?}", model_path);
        let llama = Llama::new(&model_path, 4096)?;

        info!("Running inference with temperature {}", _temperature);
        let response = llama.chat_completion(
            &system,
            &request.prompt,
            Some(_temperature),
            Some(_max_tokens as i32),
        )?;

        info!("Inference complete, response length: {}", response.len());

        Ok(ChatResponse {
            response,
            model: "phi3-mini".to_string(),
            tokens_used: Some(_max_tokens),
        })
    }

    #[cfg(not(feature = "llm"))]
    {
        // Stub response when llama-cpp-rs is not available
        // Skip model download in stub mode
        warn!("LLM feature not enabled, returning stub response (no model download)");

        let stub_response = format!(
            "[phi3-mini stub 🐄]\n\nSystem: {}\n\nYou said: \"{}\"\n\n\
            To enable actual inference, build with:\n\
            cargo build --features llm\n\n\
            Ensure llama.cpp native libraries are available.",
            system.chars().take(100).collect::<String>(),
            request.prompt.chars().take(100).collect::<String>()
        );

        Ok(ChatResponse {
            response: stub_response,
            model: "phi3-mini-stub".to_string(),
            tokens_used: None,
        })
    }
}

// ============================================================================
// Breed Loading
// ============================================================================

/// Load the default cattle-v1 breed from pasture
pub fn load_default_breed() -> Result<BreedConfig> {
    let breed_path = PathBuf::from("pasture/cattle-v1/breed.toml");

    if breed_path.exists() {
        let content = std::fs::read_to_string(&breed_path)?;
        let config: BreedConfig = toml::from_str(&content)?;
        info!("Loaded cattle-v1 breed from pasture");
        Ok(config)
    } else {
        // Fallback to default config
        Ok(BreedConfig {
            name: "cattle-v1".to_string(),
            genes: vec![
                Gene { name: "working".to_string(), expression: 0.9, dominant: true },
                Gene { name: "reliability".to_string(), expression: 0.95, dominant: true },
            ],
            config: Some(ModelConfig {
                model: Some("phi3-mini.gguf".to_string()),
                temperature: Some(0.7),
                max_tokens: Some(512),
                context_size: Some(4096),
            }),
            system_prompt: "You are a Cattle Dog assistant - tough, reliable, and direct.".to_string(),
        })
    }
}
