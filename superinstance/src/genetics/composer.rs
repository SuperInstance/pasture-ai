//! Gene Composer - Stacked LoRA Weight Composition
//! 
//! The "Mixing Board": Loads and composes multiple LoRA adapters based on
//! the genetic composition defined in breed.md.
//! 
//! Math: Output = BaseModel + (w1 × LoRA₁) + (w2 × LoRA₂) + ...
//! 
//! Supports:
//! - Linear composition
//! - Weighted interpolation
//! - SLERP (Spherical Linear Interpolation) for preserving geometric structure

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::{BreedManifest, Gene, GenePool, GeneWeight};
use crate::species::SpeciesType;

/// Gene Composer - Merges LoRA adapters based on genetic recipe
pub struct GeneComposer {
    /// Gene pool for looking up adapters
    gene_pool: Arc<GenePool>,
    /// Cache of composed weights
    weight_cache: Arc<RwLock<HashMap<String, ComposedWeights>>>,
    /// Configuration
    config: ComposerConfig,
}

/// Configuration for the composer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposerConfig {
    /// Maximum total weight before normalization
    pub max_total_weight: f32,
    /// Use GPU for composition (if available)
    pub use_gpu: bool,
    /// Cache composed weights in memory
    pub cache_enabled: bool,
    /// Composition method
    pub method: CompositionMethod,
}

impl Default for ComposerConfig {
    fn default() -> Self {
        Self {
            max_total_weight: 10.0,
            use_gpu: false,
            cache_enabled: true,
            method: CompositionMethod::Weighted,
        }
    }
}

/// Composition methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompositionMethod {
    /// Simple linear addition
    Linear,
    /// Weighted interpolation
    Weighted,
    /// SLERP for smooth transitions
    Slerp,
    /// TIES (Trim, Elect, Merge)
    Ties,
}

/// Composed weights result
#[derive(Debug, Clone)]
pub struct ComposedWeights {
    /// Breed ID
    pub breed_id: String,
    /// Composed weight tensors (mock for now)
    pub weights: HashMap<String, Vec<f32>>,
    /// Total VRAM required
    pub vram_bytes: u64,
    /// Genes used
    pub genes_used: Vec<String>,
    /// When composed
    pub composed_at: chrono::DateTime<chrono::Utc>,
}

/// Result of a composition operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionResult {
    pub breed_id: String,
    pub genes_composed: usize,
    pub total_weight: f32,
    pub vram_required_mb: f32,
    pub method: CompositionMethod,
    pub duration_ms: u64,
    pub success: bool,
}

impl GeneComposer {
    /// Create a new gene composer
    pub fn new(gene_pool: Arc<GenePool>) -> Self {
        Self {
            gene_pool,
            weight_cache: Arc::new(RwLock::new(HashMap::new())),
            config: ComposerConfig::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(gene_pool: Arc<GenePool>, config: ComposerConfig) -> Self {
        Self {
            gene_pool,
            weight_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// Compose genes according to breed manifest
    pub fn compose(&self, manifest: &BreedManifest) -> Result<CompositionResult> {
        let start = std::time::Instant::now();
        let breed_id = manifest.name.clone();
        
        info!("🧬 Composing genes for breed: {}", breed_id);
        
        // Check cache first
        if self.config.cache_enabled {
            let cache = self.weight_cache.read();
            if let Some(cached) = cache.get(&breed_id) {
                debug!("Using cached composition for {}", breed_id);
                return Ok(CompositionResult {
                    breed_id: breed_id.clone(),
                    genes_composed: cached.genes_used.len(),
                    total_weight: 1.0,
                    vram_required_mb: cached.vram_bytes as f32 / 1_000_000.0,
                    method: self.config.method,
                    duration_ms: start.elapsed().as_millis() as u64,
                    success: true,
                });
            }
        }
        
        // Load gene weights
        let mut loaded_genes: Vec<(GeneWeight, Gene, Vec<f32>)> = Vec::new();
        let mut total_weight: f32 = 0.0;
        let mut vram_total: u64 = 0;
        
        for gene_weight in &manifest.genetic_composition {
            // Look up gene in pool
            if let Some(gene) = self.gene_pool.get(&gene_weight.gene_id) {
                // Load adapter weights (mock - in production, load safetensors)
                let weights = self.load_gene_weights(&gene)?;
                
                total_weight += gene_weight.weight;
                vram_total += gene.size_bytes;
                
                loaded_genes.push((gene_weight.clone(), gene, weights));
                
                debug!(
                    "  Loaded gene: {} (weight: {:.2}, size: {} bytes)",
                    gene_weight.gene_id, gene_weight.weight, gene.size_bytes
                );
            } else {
                warn!("Gene not found in pool: {}", gene_weight.gene_id);
            }
        }
        
        if loaded_genes.is_empty() {
            return Err(anyhow!("No valid genes found for composition"));
        }
        
        // Normalize weights if needed
        if total_weight > self.config.max_total_weight {
            debug!("Normalizing weights (total: {:.2})", total_weight);
            for (gw, _, _) in &mut loaded_genes {
                gw.weight /= total_weight;
            }
        }
        
        // Compose based on method
        let composed = match self.config.method {
            CompositionMethod::Linear => self.compose_linear(&loaded_genes)?,
            CompositionMethod::Weighted => self.compose_weighted(&loaded_genes)?,
            CompositionMethod::Slerp => self.compose_slerp(&loaded_genes)?,
            CompositionMethod::Ties => self.compose_ties(&loaded_genes)?,
        };
        
        // Cache result
        if self.config.cache_enabled {
            let mut cache = self.weight_cache.write();
            cache.insert(breed_id.clone(), composed.clone());
        }
        
        let elapsed = start.elapsed();
        info!(
            "🧬 Composition complete: {} genes, {:.1}MB VRAM, {:?}",
            loaded_genes.len(),
            vram_total as f32 / 1_000_000.0,
            elapsed
        );
        
        Ok(CompositionResult {
            breed_id,
            genes_composed: loaded_genes.len(),
            total_weight,
            vram_required_mb: vram_total as f32 / 1_000_000.0,
            method: self.config.method,
            duration_ms: elapsed.as_millis() as u64,
            success: true,
        })
    }
    
    /// Load gene weights from adapter file
    fn load_gene_weights(&self, gene: &Gene) -> Result<Vec<f32>> {
        // In production, this would use safetensors to load actual weights
        // For now, return mock weights
        
        if !gene.adapter_path.exists() {
            debug!("Adapter file not found, using mock weights: {:?}", gene.adapter_path);
            return Ok(vec![0.0; 1024]); // Mock 1024 floats
        }
        
        // Mock loading - in production:
        // let tensors = safetensors::load(&gene.adapter_path)?;
        // Convert to f32 vector
        
        Ok(vec![0.5; 1024])
    }
    
    /// Linear composition: W = Σ(w_i × A_i)
    fn compose_linear(
        &self,
        genes: &[(GeneWeight, Gene, Vec<f32>)],
    ) -> Result<ComposedWeights> {
        let breed_id = genes.first()
            .map(|(_, g, _)| g.id.clone())
            .unwrap_or_default();
        
        // Find max length
        let max_len = genes.iter().map(|(_, _, w)| w.len()).max().unwrap_or(0);
        
        // Sum weighted values
        let mut result = vec![0.0f32; max_len];
        let genes_used: Vec<String> = genes.iter().map(|(gw, _, _)| gw.gene_id.clone()).collect();
        
        for (gene_weight, gene, weights) in genes {
            for (i, w) in weights.iter().enumerate() {
                if i < result.len() {
                    result[i] += gene_weight.weight * w;
                }
            }
        }
        
        let mut weights_map = HashMap::new();
        weights_map.insert("composed".to_string(), result);
        
        Ok(ComposedWeights {
            breed_id,
            weights: weights_map,
            vram_bytes: (max_len * 4) as u64,
            genes_used,
            composed_at: chrono::Utc::now(),
        })
    }
    
    /// Weighted interpolation with normalization
    fn compose_weighted(
        &self,
        genes: &[(GeneWeight, Gene, Vec<f32>)],
    ) -> Result<ComposedWeights> {
        let total_weight: f32 = genes.iter().map(|(gw, _, _)| gw.weight).sum();
        
        let breed_id = genes.first()
            .map(|(_, g, _)| g.id.clone())
            .unwrap_or_default();
        
        let max_len = genes.iter().map(|(_, _, w)| w.len()).max().unwrap_or(0);
        let mut result = vec![0.0f32; max_len];
        let genes_used: Vec<String> = genes.iter().map(|(gw, _, _)| gw.gene_id.clone()).collect();
        
        for (gene_weight, _, weights) in genes {
            let normalized_weight = gene_weight.weight / total_weight;
            for (i, w) in weights.iter().enumerate() {
                if i < result.len() {
                    result[i] += normalized_weight * w;
                }
            }
        }
        
        let mut weights_map = HashMap::new();
        weights_map.insert("composed".to_string(), result);
        
        Ok(ComposedWeights {
            breed_id,
            weights: weights_map,
            vram_bytes: (max_len * 4) as u64,
            genes_used,
            composed_at: chrono::Utc::now(),
        })
    }
    
    /// SLERP composition for smooth interpolation
    fn compose_slerp(
        &self,
        genes: &[(GeneWeight, Gene, Vec<f32>)],
    ) -> Result<ComposedWeights> {
        // SLERP is best for interpolating between two weight sets
        // For multiple genes, we do pairwise SLERP
        
        if genes.len() < 2 {
            return self.compose_linear(genes);
        }
        
        let breed_id = genes.first()
            .map(|(_, g, _)| g.id.clone())
            .unwrap_or_default();
        let genes_used: Vec<String> = genes.iter().map(|(gw, _, _)| gw.gene_id.clone()).collect();
        
        // Start with first gene
        let mut result = genes[0].2.clone();
        let mut current_weight = genes[0].0.weight;
        
        // SLERP with each subsequent gene
        for (gene_weight, _, weights) in &genes[1..] {
            result = self.slerp_vectors(&result, &weights, current_weight / (current_weight + gene_weight.weight));
            current_weight += gene_weight.weight;
        }
        
        let mut weights_map = HashMap::new();
        weights_map.insert("composed".to_string(), result);
        
        Ok(ComposedWeights {
            breed_id,
            weights: weights_map,
            vram_bytes: (genes[0].2.len() * 4) as u64,
            genes_used,
            composed_at: chrono::Utc::now(),
        })
    }
    
    /// SLERP between two vectors
    fn slerp_vectors(&self, v0: &[f32], v1: &[f32], t: f32) -> Vec<f32> {
        let len = v0.len().min(v1.len());
        let mut result = Vec::with_capacity(len);
        
        for i in 0..len {
            let a = v0[i];
            let b = v1[i];
            
            // Simplified SLERP for scalar values
            // θ = acos(dot) but for scalars, we use linear with angle preservation
            let dot = a * b;
            let theta = (dot / (a.abs() * b.abs() + 1e-8)).acos();
            
            if theta.abs() < 1e-6 {
                result.push(a * (1.0 - t) + b * t);
            } else {
                let sin_theta = theta.sin();
                result.push(
                    ((1.0 - t) * theta).sin() / sin_theta * a +
                    (t * theta).sin() / sin_theta * b
                );
            }
        }
        
        result
    }
    
    /// TIES composition (Trim, Elect, Merge)
    fn compose_ties(
        &self,
        genes: &[(GeneWeight, Gene, Vec<f32>)],
    ) -> Result<ComposedWeights> {
        // TIES: 
        // 1. Trim: Keep only top 20% by magnitude
        // 2. Elect: Choose sign direction by majority vote
        // 3. Merge: Average aligned weights
        
        let breed_id = genes.first()
            .map(|(_, g, _)| g.id.clone())
            .unwrap_or_default();
        let genes_used: Vec<String> = genes.iter().map(|(gw, _, _)| gw.gene_id.clone()).collect();
        
        let max_len = genes.iter().map(|(_, _, w)| w.len()).max().unwrap_or(0);
        
        // Trim: Calculate threshold for each gene
        let trimmed: Vec<Vec<f32>> = genes.iter().map(|(gw, _, weights)| {
            let threshold = self.calculate_threshold(weights, 0.2);
            weights.iter().map(|w| {
                if w.abs() >= threshold { *w * gw.weight } else { 0.0 }
            }).collect()
        }).collect();
        
        // Elect and Merge
        let mut result = vec![0.0f32; max_len];
        for i in 0..max_len {
            let values: Vec<f32> = trimmed.iter()
                .filter_map(|t| t.get(i).copied())
                .filter(|v| v.abs() > 0.0)
                .collect();
            
            if !values.is_empty() {
                // Elect sign by majority
                let positive: usize = values.iter().filter(|v| **v > 0.0).count();
                let sign = if positive > values.len() / 2 { 1.0 } else { -1.0 };
                
                // Merge aligned values
                let aligned: Vec<f32> = values.into_iter()
                    .filter(|v| v.signum() == sign)
                    .collect();
                
                if !aligned.is_empty() {
                    result[i] = aligned.iter().sum::<f32>() / aligned.len() as f32;
                }
            }
        }
        
        let mut weights_map = HashMap::new();
        weights_map.insert("composed".to_string(), result);
        
        Ok(ComposedWeights {
            breed_id,
            weights: weights_map,
            vram_bytes: (max_len * 4) as u64,
            genes_used,
            composed_at: chrono::Utc::now(),
        })
    }
    
    /// Calculate threshold for trimming
    fn calculate_threshold(&self, weights: &[f32], trim_ratio: f32) -> f32 {
        let mut sorted: Vec<f32> = weights.iter().map(|w| w.abs()).collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let idx = (sorted.len() as f32 * trim_ratio) as usize;
        *sorted.get(idx).unwrap_or(&0.0)
    }
    
    /// Clear the composition cache
    pub fn clear_cache(&self) {
        self.weight_cache.write().clear();
        info!("Composition cache cleared");
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        let cache = self.weight_cache.read();
        CacheStats {
            entries: cache.len(),
            total_vram_mb: cache.values().map(|w| w.vram_bytes as f64 / 1_000_000.0).sum(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub entries: usize,
    pub total_vram_mb: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_composer_creation() {
        let pool = Arc::new(GenePool::default());
        let composer = GeneComposer::new(pool);
        let stats = composer.cache_stats();
        assert_eq!(stats.entries, 0);
    }
}
