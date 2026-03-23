//! File Watcher - Hot-Reload for breed.md Changes
//! 
//! Watches the pasture/ directory for changes to breed.md files.
//! When a change is detected, the Collie immediately updates the running agent.
//! 
//! This enables real-time "genetic mutation" - users can edit their agent's
//! DNA and see results instantly without restarting.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::{DebouncedEvent, Debouncer, new_debouncer};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use super::{BreedManifest, Breed};
use crate::species::SpeciesType;

/// File Watcher - Monitors breed.md files for changes
pub struct BreedWatcher {
    /// Path to pasture directory
    pasture_path: PathBuf,
    /// Loaded breeds cache
    breeds: Arc<RwLock<HashMap<String, Breed>>>,
    /// Channel for change notifications
    change_tx: mpsc::Sender<BreedChangeEvent>,
    /// Debouncer for handling rapid changes
    _debouncer: Option<Debouncer<notify::RecommendedWatcher>>,
}

/// Event when a breed changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedChangeEvent {
    /// Breed ID
    pub breed_id: String,
    /// Type of change
    pub change_type: ChangeType,
    /// Path to changed file
    pub path: PathBuf,
    /// New manifest (if loaded successfully)
    pub manifest: Option<BreedManifest>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Type of file change
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Error,
}

impl BreedWatcher {
    /// Create a new breed watcher
    pub fn new(
        pasture_path: PathBuf,
        change_tx: mpsc::Sender<BreedChangeEvent>,
    ) -> Self {
        Self {
            pasture_path,
            breeds: Arc::new(RwLock::new(HashMap::new())),
            change_tx,
            _debouncer: None,
        }
    }
    
    /// Start watching for file changes
    pub fn start(&mut self) -> Result<()> {
        info!("👀 Starting breed watcher on {:?}", self.pasture_path);
        
        // Create the pasture directory if it doesn't exist
        if !self.pasture_path.exists() {
            std::fs::create_dir_all(&self.pasture_path)?;
            info!("Created pasture directory: {:?}", self.pasture_path);
        }
        
        // Scan existing breeds
        self.scan_existing_breeds()?;
        
        // Create debounced watcher
        let tx = self.change_tx.clone();
        let breeds = Arc::clone(&self.breeds);
        
        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |result: Result<Vec<DebouncedEvent>, Vec<notify::Error>>| {
                match result {
                    Ok(events) => {
                        for event in events {
                            handle_file_event(&event, &tx, &breeds);
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            warn!("Watch error: {:?}", error);
                        }
                    }
                }
            },
        )?;
        
        // Start watching
        debouncer.watch(&self.pasture_path, RecursiveMode::Recursive)?;
        
        self._debouncer = Some(debouncer);
        
        info!("👀 Breed watcher active - hot-reload enabled");
        Ok(())
    }
    
    /// Scan for existing breed.md files
    fn scan_existing_breeds(&self) -> Result<()> {
        if !self.pasture_path.exists() {
            return Ok(());
        }
        
        let mut count = 0;
        
        for entry in walkdir::WalkDir::new(&self.pasture_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.file_name().map(|f| f == "breed.md").unwrap_or(false) {
                if let Ok(manifest) = BreedManifest::load(path) {
                    let breed = Breed {
                        id: manifest.name.clone(),
                        species: manifest.species,
                        path: path.parent().unwrap().to_path_buf(),
                        manifest,
                        modified_at: chrono::Utc::now(),
                    };
                    
                    self.breeds.write().insert(breed.id.clone(), breed);
                    count += 1;
                }
            }
        }
        
        info!("👀 Found {} existing breeds", count);
        Ok(())
    }
    
    /// Get a breed by ID
    pub fn get_breed(&self, id: &str) -> Option<Breed> {
        self.breeds.read().get(id).cloned()
    }
    
    /// Get all loaded breeds
    pub fn list_breeds(&self) -> Vec<Breed> {
        self.breeds.read().values().cloned().collect()
    }
    
    /// Get breeds by species
    pub fn breeds_by_species(&self, species: SpeciesType) -> Vec<Breed> {
        self.breeds.read().values()
            .filter(|b| b.species == species)
            .cloned()
            .collect()
    }
    
    /// Reload a specific breed
    pub fn reload_breed(&self, path: &Path) -> Result<Breed> {
        let manifest = BreedManifest::load(path)?;
        
        let breed = Breed {
            id: manifest.name.clone(),
            species: manifest.species,
            path: path.parent().unwrap().to_path_buf(),
            manifest,
            modified_at: chrono::Utc::now(),
        };
        
        self.breeds.write().insert(breed.id.clone(), breed.clone());
        
        Ok(breed)
    }
    
    /// Stop watching
    pub fn stop(&mut self) {
        if let Some(_debouncer) = self._debouncer.take() {
            // Debouncer is dropped, stopping the watcher
            info!("👀 Breed watcher stopped");
        }
    }
}

/// Handle a file system event
fn handle_file_event(
    event: &DebouncedEvent,
    tx: &mpsc::Sender<BreedChangeEvent>,
    breeds: &Arc<RwLock<HashMap<String, Breed>>>,
) {
    let path = &event.paths[0];
    
    // Only care about breed.md files
    if !path.file_name().map(|f| f == "breed.md").unwrap_or(false) {
        return;
    }
    
    debug!("📁 Breed file event: {:?} on {:?}", event.kind, path);
    
    let change_type = match event.kind {
        notify::EventKind::Create(_) => ChangeType::Created,
        notify::EventKind::Modify(_) => ChangeType::Modified,
        notify::EventKind::Remove(_) => ChangeType::Deleted,
        _ => return,
    };
    
    // Try to load the manifest
    let manifest = if change_type != ChangeType::Deleted {
        match BreedManifest::load(path) {
            Ok(m) => {
                // Update cache
                let breed = Breed {
                    id: m.name.clone(),
                    species: m.species,
                    path: path.parent().unwrap().to_path_buf(),
                    manifest: m.clone(),
                    modified_at: chrono::Utc::now(),
                };
                
                breeds.write().insert(breed.id.clone(), breed);
                
                info!("🧬 Genetic update detected: {} ({:?})", m.name, change_type);
                Some(m)
            }
            Err(e) => {
                warn!("Failed to load breed {:?}: {}", path, e);
                None
            }
        }
    } else {
        // Remove from cache on delete
        let id = path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        breeds.write().remove(id);
        None
    };
    
    // Send notification
    let breed_id = manifest.as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| {
            path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string()
        });
    
    let event = BreedChangeEvent {
        breed_id,
        change_type,
        path: path.clone(),
        manifest,
        timestamp: chrono::Utc::now(),
    };
    
    // Try to send, but don't block if receiver is full
    let _ = tx.try_send(event);
}

/// Walkdir stub (we'll use the actual crate in production)
mod walkdir {
    use std::path::Path;
    
    pub struct WalkDir {
        path: std::path::PathBuf,
    }
    
    impl WalkDir {
        pub fn new<P: AsRef<Path>>(path: P) -> Self {
            Self {
                path: path.as_ref().to_path_buf(),
            }
        }
        
        pub fn follow_links(self, _follow: bool) -> Self {
            self
        }
        
        pub fn into_iter(self) -> impl Iterator<Item = Result<DirEntry, std::io::Error>> {
            // Simplified: just return empty for now
            std::iter::empty()
        }
    }
    
    pub struct DirEntry {
        path: std::path::PathBuf,
    }
    
    impl DirEntry {
        pub fn path(&self) -> &std::path::Path {
            &self.path
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_watcher_creation() {
        let (tx, _rx) = mpsc::channel(100);
        let watcher = BreedWatcher::new(PathBuf::from("pasture"), tx);
        assert!(watcher.list_breeds().is_empty());
    }
}
