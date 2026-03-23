//! Anatomy - Weight Metadata Inspection
//! 
//! Provides tools for inspecting and documenting LoRA adapter structure.
//! Generates meta.json files that document what each layer does.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Anatomy of a LoRA adapter - layer-by-layer documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterAnatomy {
    /// Adapter ID
    pub id: String,
    /// Total size in bytes
    pub size_bytes: u64,
    /// Number of layers
    pub layer_count: usize,
    /// Layer documentation
    pub layers: HashMap<String, LayerAnatomy>,
    /// Overall function
    pub primary_function: String,
    /// Sensitivity score (how much it affects output)
    pub sensitivity: Sensitivity,
    /// Tags
    pub tags: Vec<String>,
}

/// Documentation for a single layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerAnatomy {
    /// Layer name
    pub name: String,
    /// What this layer does
    pub function: String,
    /// Sensitivity (Low, Medium, High)
    pub sensitivity: Sensitivity,
    /// Weight dimensions
    pub dimensions: (usize, usize),
    /// Parameter count
    pub param_count: usize,
    /// Layer index
    pub index: usize,
}

/// Sensitivity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Sensitivity {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}

/// Anatomy Inspector - Analyzes LoRA adapters
pub struct AnatomyInspector {
    /// Path to gene pool
    gene_pool_path: PathBuf,
}

impl AnatomyInspector {
    /// Create a new inspector
    pub fn new(gene_pool_path: PathBuf) -> Self {
        Self { gene_pool_path }
    }
    
    /// Analyze a LoRA adapter and generate anatomy documentation
    pub fn analyze(&self, adapter_path: &Path) -> Result<AdapterAnatomy> {
        let id = adapter_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        info!("🔍 Analyzing adapter anatomy: {}", id);
        
        // In production, this would use safetensors to inspect actual weights
        // For now, generate mock anatomy
        
        let mut layers = HashMap::new();
        
        // Common layer patterns in transformers
        let layer_patterns = [
            ("q_proj", "Query projection - attention mechanism", Sensitivity::High),
            ("k_proj", "Key projection - attention mechanism", Sensitivity::High),
            ("v_proj", "Value projection - attention mechanism", Sensitivity::High),
            ("o_proj", "Output projection - attention aggregation", Sensitivity::Medium),
            ("gate_proj", "Gate projection - FFN routing", Sensitivity::Medium),
            ("up_proj", "Up projection - FFN expansion", Sensitivity::Medium),
            ("down_proj", "Down projection - FFN compression", Sensitivity::Medium),
            ("embed_tokens", "Token embeddings - vocabulary", Sensitivity::Critical),
            ("lm_head", "Language model head - output generation", Sensitivity::Critical),
        ];
        
        for (idx, (name, function, sensitivity)) in layer_patterns.iter().enumerate() {
            layers.insert(
                name.to_string(),
                LayerAnatomy {
                    name: name.to_string(),
                    function: function.to_string(),
                    sensitivity: *sensitivity,
                    dimensions: (4096, 4096), // Typical dimensions
                    param_count: 16_777_216,
                    index: idx,
                },
            );
        }
        
        // Determine primary function from name patterns
        let primary_function = determine_primary_function(&id);
        
        // Determine overall sensitivity
        let sensitivity = determine_overall_sensitivity(&layers);
        
        // Generate tags
        let tags = generate_tags(&id, &primary_function);
        
        let anatomy = AdapterAnatomy {
            id,
            size_bytes: 50_000_000, // ~50MB mock
            layer_count: layers.len(),
            layers,
            primary_function,
            sensitivity,
            tags,
        };
        
        debug!("🔍 Anatomy complete: {} layers, sensitivity: {:?}", 
            anatomy.layer_count, anatomy.sensitivity);
        
        Ok(anatomy)
    }
    
    /// Save anatomy documentation to meta.json
    pub fn save_anatomy(&self, anatomy: &AdapterAnatomy, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(anatomy)?;
        std::fs::write(output_path, json)?;
        info!("📝 Saved anatomy to {:?}", output_path);
        Ok(())
    }
    
    /// Load anatomy from meta.json
    pub fn load_anatomy(&self, path: &Path) -> Result<AdapterAnatomy> {
        let content = std::fs::read_to_string(path)?;
        let anatomy: AdapterAnatomy = serde_json::from_str(&content)?;
        Ok(anatomy)
    }
    
    /// Compare two adapters
    pub fn compare(&self, a: &AdapterAnatomy, b: &AdapterAnatomy) -> AnatomyComparison {
        let mut comparison = AnatomyComparison {
            adapter_a: a.id.clone(),
            adapter_b: b.id.clone(),
            common_layers: Vec::new(),
            unique_to_a: Vec::new(),
            unique_to_b: Vec::new(),
            sensitivity_overlap: 0.0,
            functional_similarity: 0.0,
        };
        
        // Find common and unique layers
        for (name, layer) in &a.layers {
            if b.layers.contains_key(name) {
                comparison.common_layers.push(name.clone());
            } else {
                comparison.unique_to_a.push(LayerAnatomy {
                    name: name.clone(),
                    ..layer.clone()
                });
            }
        }
        
        for (name, layer) in &b.layers {
            if !a.layers.contains_key(name) {
                comparison.unique_to_b.push(LayerAnatomy {
                    name: name.clone(),
                    ..layer.clone()
                });
            }
        }
        
        // Calculate similarity metrics
        let total_layers = a.layers.len() + b.layers.len();
        if total_layers > 0 {
            comparison.sensitivity_overlap = 
                (2 * comparison.common_layers.len()) as f64 / total_layers as f64;
        }
        
        // Functional similarity based on primary function
        comparison.functional_similarity = if a.primary_function == b.primary_function {
            1.0
        } else if a.tags.iter().any(|t| b.tags.contains(t)) {
            0.5
        } else {
            0.0
        };
        
        comparison
    }
}

/// Result of comparing two adapter anatomies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnatomyComparison {
    pub adapter_a: String,
    pub adapter_b: String,
    pub common_layers: Vec<String>,
    pub unique_to_a: Vec<LayerAnatomy>,
    pub unique_to_b: Vec<LayerAnatomy>,
    pub sensitivity_overlap: f64,
    pub functional_similarity: f64,
}

/// Determine primary function from adapter name
fn determine_primary_function(name: &str) -> String {
    let lower = name.to_lowercase();
    
    if lower.contains("code") || lower.contains("rust") || lower.contains("python") {
        "Code generation and understanding".to_string()
    } else if lower.contains("email") || lower.contains("mail") {
        "Email composition and triage".to_string()
    } else if lower.contains("polite") || lower.contains("formal") || lower.contains("tone") {
        "Style and tone adjustment".to_string()
    } else if lower.contains("json") || lower.contains("format") || lower.contains("struct") {
        "Structured output formatting".to_string()
    } else if lower.contains("summar") || lower.contains("condense") {
        "Text summarization".to_string()
    } else if lower.contains("chat") || lower.contains("convers") {
        "Conversational interaction".to_string()
    } else if lower.contains("debug") || lower.contains("fix") {
        "Debugging and error fixing".to_string()
    } else if lower.contains("reason") || lower.contains("think") || lower.contains("logic") {
        "Deep reasoning and analysis".to_string()
    } else {
        "General purpose assistance".to_string()
    }
}

/// Determine overall sensitivity from layer analysis
fn determine_overall_sensitivity(layers: &HashMap<String, LayerAnatomy>) -> Sensitivity {
    let has_critical = layers.values().any(|l| matches!(l.sensitivity, Sensitivity::Critical));
    let has_high = layers.values().any(|l| matches!(l.sensitivity, Sensitivity::High));
    
    if has_critical {
        Sensitivity::Critical
    } else if has_high {
        Sensitivity::High
    } else {
        Sensitivity::Medium
    }
}

/// Generate tags from adapter name and function
fn generate_tags(name: &str, function: &str) -> Vec<String> {
    let mut tags = Vec::new();
    
    let lower = name.to_lowercase();
    
    // Name-based tags
    if lower.contains("code") { tags.push("coding".to_string()); }
    if lower.contains("email") { tags.push("email".to_string()); }
    if lower.contains("polite") { tags.push("politeness".to_string()); }
    if lower.contains("rust") { tags.push("rust".to_string()); }
    if lower.contains("python") { tags.push("python".to_string()); }
    if lower.contains("json") { tags.push("formatting".to_string()); }
    if lower.contains("debug") { tags.push("debugging".to_string()); }
    
    // Function-based tags
    let func_lower = function.to_lowercase();
    if func_lower.contains("code") { tags.push("code-gen".to_string()); }
    if func_lower.contains("email") { tags.push("communication".to_string()); }
    if func_lower.contains("style") { tags.push("style".to_string()); }
    if func_lower.contains("reason") { tags.push("reasoning".to_string()); }
    
    tags.sort();
    tags.dedup();
    
    if tags.is_empty() {
        tags.push("general".to_string());
    }
    
    tags
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_adapter() {
        let inspector = AnatomyInspector::new(PathBuf::from("genetics/traits"));
        let anatomy = inspector.analyze(Path::new("genetics/traits/test/adapter.safetensors"));
        assert!(anatomy.is_ok());
    }
    
    #[test]
    fn test_determine_function() {
        let func = determine_primary_function("email-cow-v1");
        assert!(func.contains("Email"));
    }
}
