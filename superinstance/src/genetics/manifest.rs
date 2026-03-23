//! Breed Manifest Parser - Parses breed.md Markdown files
//! 
//! The core of Open Genomics: A Markdown parser that extracts:
//! - System Prompt (Genetic Code)
//! - Settings (Phenotype)
//! - Genetic Composition (LoRA weights recipe)
//! - Tool Access
//! - Lineage information

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::species::SpeciesType;

/// Parsed breed manifest from breed.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedManifest {
    /// Breed name (from heading)
    pub name: String,
    /// Species type
    pub species: SpeciesType,
    /// Lineage information
    pub lineage: Lineage,
    /// Runtime settings
    pub phenotype: Phenotype,
    /// Genetic composition (LoRA recipe)
    pub genetic_composition: Vec<GeneWeight>,
    /// System prompt (the soul)
    pub system_prompt: String,
    /// Tool access permissions
    pub tool_access: HashMap<String, bool>,
    /// Path to the breed.md file
    pub source_path: PathBuf,
}

/// Lineage tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Lineage {
    pub generation: u32,
    pub sire: Option<String>,
    pub dam: Option<String>,
    pub last_bred: Option<String>,
    pub created_at: Option<String>,
}

/// Runtime phenotype settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phenotype {
    /// Temperature (0.0 to 2.0)
    pub temperature: f32,
    /// Max tokens to generate
    pub max_tokens: u32,
    /// Priority level (Low, Normal, High, Critical)
    pub priority: Priority,
    /// Response style
    pub tone: String,
    /// Additional settings
    pub extra: HashMap<String, String>,
}

impl Default for Phenotype {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 500,
            priority: Priority::Normal,
            tone: "Professional".to_string(),
            extra: HashMap::new(),
        }
    }
}

/// Priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Priority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

/// A gene with its weight in the composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneWeight {
    /// Gene ID (e.g., "polite_tone")
    pub gene_id: String,
    /// Weight (0.0 to 1.0+)
    pub weight: f32,
    /// Description (from table)
    pub description: Option<String>,
}

impl BreedManifest {
    /// Load and parse a breed.md file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content, path.to_path_buf())
    }
    
    /// Parse breed.md content
    pub fn parse(content: &str, source_path: PathBuf) -> Result<Self> {
        let parser = Parser::new(content);
        
        let mut manifest = BreedManifest {
            name: String::new(),
            species: SpeciesType::Cattle,
            lineage: Lineage::default(),
            phenotype: Phenotype::default(),
            genetic_composition: Vec::new(),
            system_prompt: String::new(),
            tool_access: HashMap::new(),
            source_path,
        };
        
        let mut current_section = Section::None;
        let mut current_text = String::new();
        let mut in_code_block = false;
        let mut in_table = false;
        let mut table_rows: Vec<Vec<String>> = Vec::new();
        
        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    // Save previous section
                    manifest = Self::save_section(manifest, current_section, &current_text, &table_rows)?;
                    current_text.clear();
                    table_rows.clear();
                    
                    if level == HeadingLevel::H1 {
                        current_section = Section::Title;
                    }
                }
                
                Event::End(TagEnd::Heading { .. }) => {
                    if current_section == Section::Title {
                        manifest.name = current_text.trim().to_string();
                        manifest.species = Self::extract_species(&manifest.name);
                        current_section = Section::None;
                    }
                    current_text.clear();
                }
                
                Event::Start(Tag::Heading { level: HeadingLevel::H2, .. }) => {
                    manifest = Self::save_section(manifest, current_section, &current_text, &table_rows)?;
                    current_text.clear();
                    table_rows.clear();
                    current_section = Section::None;
                }
                
                Event::Text(text) => {
                    let text_str = text.to_string();
                    
                    // Detect section from heading text
                    if current_section == Section::None {
                        current_section = Self::detect_section(&text_str);
                    }
                    
                    // Check for specific headings
                    if text_str.contains("Lineage") {
                        current_section = Section::Lineage;
                    } else if text_str.contains("Phenotype") {
                        current_section = Section::Phenotype;
                    } else if text_str.contains("Genetic") || text_str.contains("Genetic Composition") {
                        current_section = Section::Genetic;
                    } else if text_str.contains("Genetic Code") || text_str.contains("System Soul") || text_str.contains("System Prompt") {
                        current_section = Section::SystemPrompt;
                    } else if text_str.contains("Tool Access") {
                        current_section = Section::ToolAccess;
                    }
                    
                    current_text.push_str(&text_str);
                }
                
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    current_text.clear();
                }
                
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    if current_section == Section::SystemPrompt {
                        manifest.system_prompt = current_text.trim().to_string();
                    }
                    current_text.clear();
                }
                
                Event::Start(Tag::Table(_)) => {
                    in_table = true;
                    table_rows.clear();
                }
                
                Event::End(TagEnd::Table) => {
                    in_table = false;
                    manifest = Self::parse_table(manifest, current_section, &table_rows)?;
                    table_rows.clear();
                }
                
                Event::Start(Tag::TableRow) => {
                    table_rows.push(Vec::new());
                }
                
                Event::Start(Tag::TableCell) => {
                    current_text.clear();
                }
                
                Event::End(TagEnd::TableCell) => {
                    if let Some(row) = table_rows.last_mut() {
                        row.push(current_text.trim().to_string());
                    }
                    current_text.clear();
                }
                
                Event::Start(Tag::List(_)) => {
                    // List items for lineage or tool access
                }
                
                Event::Start(Tag::Item) => {
                    current_text.clear();
                }
                
                Event::End(TagEnd::Item) => {
                    Self::parse_list_item(&mut manifest, current_section, &current_text)?;
                    current_text.clear();
                }
                
                Event::SoftBreak | Event::HardBreak => {
                    current_text.push(' ');
                }
                
                _ => {}
            }
        }
        
        // Save final section
        manifest = Self::save_section(manifest, current_section, &current_text, &table_rows)?;
        
        info!("Parsed breed manifest: {} ({:?})", manifest.name, manifest.species);
        debug!("  System prompt length: {} chars", manifest.system_prompt.len());
        debug!("  Genes: {} traits", manifest.genetic_composition.len());
        
        Ok(manifest)
    }
    
    /// Detect section from heading text
    fn detect_section(text: &str) -> Section {
        let lower = text.to_lowercase();
        if lower.contains("lineage") {
            Section::Lineage
        } else if lower.contains("phenotype") {
            Section::Phenotype
        } else if lower.contains("genetic") && !lower.contains("code") {
            Section::Genetic
        } else if lower.contains("tool") {
            Section::ToolAccess
        } else {
            Section::None
        }
    }
    
    /// Extract species type from breed name
    fn extract_species(name: &str) -> SpeciesType {
        let lower = name.to_lowercase();
        if lower.contains("cow") || lower.contains("cattle") {
            SpeciesType::Cattle
        } else if lower.contains("sheep") {
            SpeciesType::Sheep
        } else if lower.contains("duck") {
            SpeciesType::Duck
        } else if lower.contains("goat") {
            SpeciesType::Goat
        } else if lower.contains("hog") || lower.contains("pig") {
            SpeciesType::Hog
        } else if lower.contains("chicken") {
            SpeciesType::Chicken
        } else if lower.contains("horse") {
            SpeciesType::Horse
        } else {
            SpeciesType::Cattle
        }
    }
    
    /// Save accumulated text to the appropriate section
    fn save_section(
        mut manifest: BreedManifest,
        section: Section,
        text: &str,
        table_rows: &[Vec<String>],
    ) -> Result<BreedManifest> {
        match section {
            Section::Phenotype => {
                manifest = Self::parse_phenotype_text(manifest, text)?;
            }
            Section::Lineage => {
                manifest = Self::parse_lineage_text(manifest, text)?;
            }
            _ => {}
        }
        
        if !table_rows.is_empty() {
            manifest = Self::parse_table(manifest, section, table_rows)?;
        }
        
        Ok(manifest)
    }
    
    /// Parse phenotype settings from text
    fn parse_phenotype_text(mut manifest: BreedManifest, text: &str) -> Result<BreedManifest> {
        let temp_re = Regex::new(r"(?i)temperature[:\s]+([0-9.]+)")?;
        let tokens_re = Regex::new(r"(?i)max_tokens?[:\s]+(\d+)")?;
        let tone_re = Regex::new(r"(?i)tone[:\s]+(\w+)")?;
        let priority_re = Regex::new(r"(?i)priority[:\s]+(\w+)")?;
        
        if let Some(caps) = temp_re.captures(text) {
            if let Ok(val) = caps[1].parse::<f32>() {
                manifest.phenotype.temperature = val;
            }
        }
        
        if let Some(caps) = tokens_re.captures(text) {
            if let Ok(val) = caps[1].parse::<u32>() {
                manifest.phenotype.max_tokens = val;
            }
        }
        
        if let Some(caps) = tone_re.captures(text) {
            manifest.phenotype.tone = caps[1].to_string();
        }
        
        if let Some(caps) = priority_re.captures(text) {
            manifest.phenotype.priority = match caps[1].to_lowercase().as_str() {
                "low" => Priority::Low,
                "high" => Priority::High,
                "critical" => Priority::Critical,
                _ => Priority::Normal,
            };
        }
        
        Ok(manifest)
    }
    
    /// Parse lineage from text
    fn parse_lineage_text(mut manifest: BreedManifest, text: &str) -> Result<BreedManifest> {
        let gen_re = Regex::new(r"(?i)generation[:\s]+(\d+)")?;
        let sire_re = Regex::new(r"(?i)sire[:\s]+([^\n]+)")?;
        let dam_re = Regex::new(r"(?i)dam[:\s]+([^\n]+)")?;
        let bred_re = Regex::new(r"(?i)last\s*bred[:\s]+([^\n]+)")?;
        
        if let Some(caps) = gen_re.captures(text) {
            if let Ok(val) = caps[1].parse::<u32>() {
                manifest.lineage.generation = val;
            }
        }
        
        if let Some(caps) = sire_re.captures(text) {
            manifest.lineage.sire = Some(caps[1].trim().to_string());
        }
        
        if let Some(caps) = dam_re.captures(text) {
            manifest.lineage.dam = Some(caps[1].trim().to_string());
        }
        
        if let Some(caps) = bred_re.captures(text) {
            manifest.lineage.last_bred = Some(caps[1].trim().to_string());
        }
        
        Ok(manifest)
    }
    
    /// Parse table data
    fn parse_table(
        mut manifest: BreedManifest,
        section: Section,
        table_rows: &[Vec<String>],
    ) -> Result<BreedManifest> {
        if table_rows.len() < 2 {
            return Ok(manifest);
        }
        
        match section {
            Section::Phenotype => {
                for row in table_rows.iter().skip(1) {
                    if row.len() >= 2 {
                        let key = row[0].to_lowercase();
                        let value = &row[1];
                        
                        match key.as_str() {
                            "temperature" | "temp" => {
                                if let Ok(val) = value.parse::<f32>() {
                                    manifest.phenotype.temperature = val;
                                }
                            }
                            "max_tokens" | "tokens" => {
                                if let Ok(val) = value.parse::<u32>() {
                                    manifest.phenotype.max_tokens = val;
                                }
                            }
                            "tone" => {
                                manifest.phenotype.tone = value.clone();
                            }
                            "priority" => {
                                manifest.phenotype.priority = match value.to_lowercase().as_str() {
                                    "low" => Priority::Low,
                                    "high" => Priority::High,
                                    "critical" => Priority::Critical,
                                    _ => Priority::Normal,
                                };
                            }
                            _ => {
                                manifest.phenotype.extra.insert(key, value.clone());
                            }
                        }
                    }
                }
            }
            Section::Genetic => {
                for row in table_rows.iter().skip(1) {
                    if row.len() >= 2 {
                        let gene_id = row[0].trim_matches('`').to_string();
                        let weight: f32 = row.get(1)
                            .map(|w| w.trim_matches('`').parse().unwrap_or(1.0))
                            .unwrap_or(1.0);
                        let description = row.get(2).cloned();
                        
                        manifest.genetic_composition.push(GeneWeight {
                            gene_id,
                            weight,
                            description,
                        });
                    }
                }
            }
            Section::ToolAccess => {
                for row in table_rows.iter().skip(1) {
                    if row.len() >= 1 {
                        let tool = row[0].trim_matches('`').to_string();
                        // Check if checkbox is checked
                        let enabled = row.get(1)
                            .map(|v| v.contains('x') || v.contains('X') || v == "true" || v == "yes")
                            .unwrap_or(false);
                        
                        manifest.tool_access.insert(tool, enabled);
                    }
                }
            }
            _ => {}
        }
        
        Ok(manifest)
    }
    
    /// Parse list item for lineage or tool access
    fn parse_list_item(manifest: &mut BreedManifest, section: Section, text: &str) -> Result<()> {
        let text = text.trim();
        
        match section {
            Section::Lineage => {
                let lower = text.to_lowercase();
                if lower.starts_with("generation") {
                    if let Some(val) = text.split(':').nth(1) {
                        if let Ok(gen) = val.trim().parse::<u32>() {
                            manifest.lineage.generation = gen;
                        }
                    }
                } else if lower.starts_with("sire") {
                    manifest.lineage.sire = text.split(':').nth(1).map(|s| s.trim().to_string());
                } else if lower.starts_with("dam") {
                    manifest.lineage.dam = text.split(':').nth(1).map(|s| s.trim().to_string());
                } else if lower.contains("bred") {
                    manifest.lineage.last_bred = Some(text.split(':').nth(1).unwrap_or("").trim().to_string());
                }
            }
            Section::ToolAccess => {
                // Parse checkbox style: - [x] tool_name
                let checked = text.contains("[x]") || text.contains("[X]");
                let tool = text
                    .replace("[x]", "")
                    .replace("[X]", "")
                    .replace("[ ]", "")
                    .trim()
                    .trim_matches('`')
                    .to_string();
                
                if !tool.is_empty() {
                    manifest.tool_access.insert(tool, checked);
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Save the manifest back to breed.md format
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = self.to_markdown();
        std::fs::write(path, content)?;
        info!("Saved breed manifest to {:?}", path);
        Ok(())
    }
    
    /// Convert manifest to Markdown format
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        // Title
        md.push_str(&format!("# {} Breed: {}\n\n", self.species.emoji(), self.name));
        
        // Lineage
        md.push_str("## 🧬 Lineage\n");
        md.push_str(&format!("* **Generation:** `{}`\n", self.lineage.generation));
        if let Some(ref sire) = self.lineage.sire {
            md.push_str(&format!("* **Sire:** {}\n", sire));
        }
        if let Some(ref dam) = self.lineage.dam {
            md.push_str(&format!("* **Dam:** {}\n", dam));
        }
        if let Some(ref last_bred) = self.lineage.last_bred {
            md.push_str(&format!("* **Last Bred:** {}\n", last_bred));
        }
        md.push_str("\n");
        
        // Phenotype
        md.push_str("## ⚙️ Phenotype (Configuration)\n");
        md.push_str("| Gene | Value | Effect |\n");
        md.push_str("| :--- | :--- | :--- |\n");
        md.push_str(&format!("| **Temperature** | `{}` | Controls creativity. |\n", self.phenotype.temperature));
        md.push_str(&format!("| **Max_Tokens** | `{}` | Maximum response length. |\n", self.phenotype.max_tokens));
        md.push_str(&format!("| **Priority** | `{:?}` | Wake priority level. |\n", self.phenotype.priority));
        md.push_str(&format!("| **Tone** | `{}` | Response style. |\n", self.phenotype.tone));
        md.push_str("\n");
        
        // Genetic Composition
        if !self.genetic_composition.is_empty() {
            md.push_str("## 🧬 Genetic Composition (The Recipe)\n");
            md.push_str("| Gene Trait | Weight | Description |\n");
            md.push_str("| :--- | :--- | :--- |\n");
            for gene in &self.genetic_composition {
                let desc = gene.description.as_deref().unwrap_or("");
                md.push_str(&format!("| `{}` | `{}` | {} |\n", gene.gene_id, gene.weight, desc));
            }
            md.push_str("\n");
        }
        
        // Tool Access
        if !self.tool_access.is_empty() {
            md.push_str("## 🛠️ Tool Access\n");
            for (tool, enabled) in &self.tool_access {
                let check = if *enabled { "[x]" } else { "[ ]" };
                md.push_str(&format!("- {} `{}`\n", check, tool));
            }
            md.push_str("\n");
        }
        
        // System Prompt
        md.push_str("## 🧠 Genetic Code (System Prompt)\n");
        md.push_str("```\n");
        md.push_str(&self.system_prompt);
        md.push_str("\n```\n");
        
        md
    }
}

/// Section being parsed
#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    None,
    Title,
    Lineage,
    Phenotype,
    Genetic,
    SystemPrompt,
    ToolAccess,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_breed() {
        let content = r#"
# 🐄 Breed: Test-Cow

## 🧬 Lineage
* **Generation:** 1

## ⚙️ Phenotype
| Gene | Value |
| :--- | :--- |
| **Temperature** | `0.5` |

## 🧬 Genetic Code
```
You are a test assistant.
```
"#;
        
        let manifest = BreedManifest::parse(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(manifest.name, "Test-Cow");
        assert_eq!(manifest.lineage.generation, 1);
        assert_eq!(manifest.phenotype.temperature, 0.5);
        assert_eq!(manifest.system_prompt, "You are a test assistant.");
    }
    
    #[test]
    fn test_parse_genetic_composition() {
        let content = r#"
# 🐄 Breed: Gene-Cow

## 🧬 Genetic Composition
| Gene Trait | Weight |
| :--- | :--- |
| `polite_tone` | `0.8` |
| `json_output` | `0.5` |
"#;
        
        let manifest = BreedManifest::parse(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(manifest.genetic_composition.len(), 2);
        assert_eq!(manifest.genetic_composition[0].gene_id, "polite_tone");
        assert_eq!(manifest.genetic_composition[0].weight, 0.8);
    }
}
