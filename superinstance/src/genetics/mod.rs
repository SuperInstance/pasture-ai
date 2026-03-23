//! Open Genomics - Markdown-as-DNA for Agent Configuration
//! 
//! The breakthrough interface: Instead of binary config files, users edit
//! `breed.md` Markdown files that define agent behavior (Phenotype),
//! history (Lineage), and knowledge (Genetics).
//! 
//! The Border Collie treats these files as source-of-truth instructions,
//! hot-reloading them instantly when changed.

mod manifest;
mod composer;
mod watcher;
mod anatomy;

pub use manifest::*;
pub use composer::*;
pub use watcher::*;
pub use anatomy::*;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::species::SpeciesType;

/// Gene Pool - Collection of atomic LoRA traits
pub struct GenePool {
    /// Path to gene pool directory
    path: PathBuf,
    /// Loaded genes
    genes: Arc<RwLock<Vec<Gene>>>,
}

/// A single gene (atomic LoRA trait)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    /// Unique gene ID
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this gene does
    pub description: String,
    /// Path to the LoRA adapter
    pub adapter_path: PathBuf,
    /// Size in bytes
    pub size_bytes: u64,
    /// Compatible species
    pub compatible_species: Vec<SpeciesType>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// A breed (agent) defined by its breed.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breed {
    /// Unique breed ID
    pub id: String,
    /// Species type
    pub species: SpeciesType,
    /// Path to breed directory
    pub path: PathBuf,
    /// Parsed manifest from breed.md
    pub manifest: BreedManifest,
    /// Last modification time
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

impl GenePool {
    /// Create a new gene pool
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            genes: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Scan the gene pool directory for available genes
    pub fn scan(&self) -> Result<Vec<Gene>> {
        let mut genes = Vec::new();
        
        if !self.path.exists() {
            info!("Gene pool directory does not exist: {:?}", self.path);
            return Ok(genes);
        }
        
        for entry in std::fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Look for meta.json and adapter.safetensors
                let meta_path = path.join("meta.json");
                let adapter_path = path.join("adapter.safetensors");
                
                if meta_path.exists() {
                    if let Ok(meta_content) = std::fs::read_to_string(&meta_path) {
                        if let Ok(mut gene) = serde_json::from_str::<Gene>(&meta_content) {
                            gene.adapter_path = adapter_path;
                            genes.push(gene);
                        }
                    }
                }
            }
        }
        
        *self.genes.write() = genes.clone();
        info!("Found {} genes in gene pool", genes.len());
        
        Ok(genes)
    }
    
    /// Get a gene by ID
    pub fn get(&self, id: &str) -> Option<Gene> {
        self.genes.read().iter().find(|g| g.id == id).cloned()
    }
    
    /// List all genes
    pub fn list(&self) -> Vec<Gene> {
        self.genes.read().clone()
    }
    
    /// List genes compatible with a species
    pub fn for_species(&self, species: SpeciesType) -> Vec<Gene> {
        self.genes.read().iter()
            .filter(|g| g.compatible_species.is_empty() || g.compatible_species.contains(&species))
            .cloned()
            .collect()
    }
}

impl Default for GenePool {
    fn default() -> Self {
        Self::new(PathBuf::from("genetics/traits"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gene_pool_creation() {
        let pool = GenePool::default();
        assert!(pool.list().is_empty());
    }
}
