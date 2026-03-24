# 🔧 src/ — The Engine Room

```
                    THE ENGINE ROOM
                    ═════════════════
                    
         ┌──────────────────────────────────────┐
         │                                      │
         │      ███████╗██████╗ ███████╗        │
         │      ██╔════╝██╔══██╗██╔════╝        │
         │      ███████╗██████╔╝█████╗          │
         │      ╚════██║██╔══██╗██╔══╝          │
         │      ███████║██║  ██║███████╗        │
         │      ╚══════╝╚═╝  ╚═╝╚══════╝        │
         │                                      │
         │        R U S T   R U N T I M E       │
         └──────────────────────────────────────┘
```

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SUPERINSTANCE RUNTIME                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         FLOW DIAGRAM                                 │   │
│  │                                                                      │   │
│  │   Cowboy                Collie                  Species              │   │
│  │   ┌─────┐              ┌─────────┐            ┌───────────┐         │   │
│  │   │ 👤  │──Intent────►│ 🐕      │──Route────►│ 🐄🦆🐐🐔🐗 │         │   │
│  │   │User │              │Collie   │            │ Livestock │         │   │
│  │   └─────┘              │         │            └───────────┘         │   │
│  │        │               │ ┌─────┐ │                  │              │   │
│  │        │               │ │Reflex│ │                  │              │   │
│  │        │               │ │<1ms  │ │                  ▼              │   │
│  │        │               │ └─────┘ │            ┌───────────┐         │   │
│  │        │               │ ┌─────┐ │            │ Pasture   │         │   │
│  │        │               │ │Antic.│ │            │ (Model +  │         │   │
│  │        │               │ │~10ms │ │◄───────────│ LoRAs)    │         │   │
│  │        │               │ └─────┘ │            └───────────┘         │   │
│  │        │               │ ┌─────┐ │                                  │   │
│  │        │               │ │Cogn. │ │                                  │   │
│  │        │               │ │<50ms │ │                                  │   │
│  │        │               │ └─────┘ │                                  │   │
│  │        │               └─────────┘                                  │   │
│  │        │                    │                                       │   │
│  │        │                    ▼                                       │   │
│  │        │              ┌───────────┐                                 │   │
│  │        └──────────────│ Response  │                                 │   │
│  │                       └───────────┘                                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Structure

```
src/
├── main.rs                 # Entry point - initializes Ranch
├── ranch.rs                # The Container - manages all components
│
├── 📁 collie/              # THE BORDER COLLIE (Orchestrator)
│   ├── mod.rs              # Main collie logic
│   ├── shepherd.rs         # Species-specific herding strategies
│   ├── anticipation.rs     # Shadow Pup (speculative decoding)
│   └── reflex.rs           # CUDA Graph cache (<1ms responses)
│
├── 📁 genetics/            # OPEN GENOMICS ENGINE
│   ├── mod.rs              # Gene pool management
│   ├── manifest.rs         # breed.md Markdown parser
│   ├── composer.rs         # LoRA weight composition
│   ├── watcher.rs          # Hot-reload file monitoring
│   └── anatomy.rs          # Weight metadata inspection
│
├── 📁 species/             # AGENT IMPLEMENTATIONS
│   ├── mod.rs              # Species trait & registry
│   ├── cattle.rs           # 🐄 Heavy LLM (reasoning)
│   ├── sheep.rs            # 🐑 Ensemble voting
│   ├── duck.rs             # 🦆 Network/API
│   ├── goat.rs             # 🐐 Navigation/Debugging
│   ├── hog.rs              # 🐗 Hardware/GPIO
│   ├── chicken.rs          # 🐔 Monitoring
│   └── horse.rs            # 🐎 Pipeline/ETL
│
├── 📁 pasture/             # RESOURCE MANAGEMENT
│   ├── mod.rs              # Pasture container
│   ├── lora_manager.rs     # Hot-swap LoRA adapters
│   └── model_pool.rs       # Base model + KV cache
│
├── 📁 evolution/           # NIGHT SCHOOL
│   ├── mod.rs              # Evolution module
│   ├── stud_book.rs        # SQLite genealogy
│   ├── breeding.rs         # LoRA merge algorithms
│   └── night_school.rs     # Breeding cycle
│
├── 📁 channels/            # COMMUNICATION
│   ├── mod.rs              # Channel manager
│   ├── discord.rs          # Discord integration
│   ├── telegram.rs         # Telegram integration
│   ├── webhook.rs          # Generic webhook
│   └── types.rs            # Shared channel types
│
├── 📁 onboarding/          # SETUP WIZARD
│   ├── mod.rs              # Onboarding state
│   ├── wizard.rs           # Interactive CLI
│   ├── config.rs           # Configuration
│   └── connectors.rs       # Agent system connectors
│
└── 📁 dashboard.rs         # TUI Dashboard
```

---

## 🐕 The Border Collie

### Speed Layers

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        BORDER COLLIE SPEED LAYERS                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   LAYER 1: REFLEX (Muscle Memory)                                           │
│   ══════════════════════════════                                            │
│   • CUDA Graph compiled for patterns used >3x                               │
│   • Execution time: <1ms                                                    │
│   • Zero cognition - pure cached response                                   │
│   • Use case: Common commands, greetings, status checks                     │
│                                                                              │
│   LAYER 2: ANTICIPATION (Instinct)                                          │
│   ══════════════════════════════                                            │
│   • Shadow Pup model predicts user intent                                   │
│   • Execution time: ~10ms                                                   │
│   • Pre-acknowledges before heavy work                                      │
│   • Use case: Pre-warming adapters, instant feedback                        │
│                                                                              │
│   LAYER 3: COGNITION (Thought)                                              │
│   ══════════════════════════════                                            │
│   • LoRA hot-swap to specific species                                       │
│   • Execution time: <50ms                                                   │
│   • Full inference with composed weights                                    │
│   • Use case: Real work                                                     │
│                                                                              │
│   LAYER 4: DELIBERATION (Escalation)                                        │
│   ══════════════════════════════                                            │
│   • Cloud API fallback                                                      │
│   • Execution time: seconds                                                 │
│   • Distills result into local LoRA that night                              │
│   • Use case: Out-of-scope tasks, learning                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Herding Strategies

| Species | Strategy | Description |
|:--------|:---------|:------------|
| 🐄 Cattle | Strong Eye | Lock GPU, block, steady pressure |
| 🐑 Sheep | The Wear | Flock ensemble, consensus voting |
| 🦆 Duck | Whistle Stop | Async, fire-and-recall |
| 🐐 Goat | Balance | Monitor depth, high agility |
| 🐗 Hog | The Drive | Real-time priority, low latency |
| 🐔 Chicken | Free Range | Constant pecking, watch for silence |
| 🐎 Horse | Saddle Up | Linear throughput, pipeline stages |

---

## 🧬 Open Genomics Engine

### breed.md Parser

```rust
// Input: breed.md Markdown file
// Output: BreedManifest struct

pub struct BreedManifest {
    pub name: String,              // From H1 heading
    pub species: SpeciesType,      // Inferred from name
    pub lineage: Lineage,          // Parsed from ## Lineage section
    pub phenotype: Phenotype,      // Runtime settings from table
    pub genetic_composition: Vec<GeneWeight>,  // LoRA recipe
    pub system_prompt: String,     // Code block content
    pub tool_access: HashMap<String, bool>,    // Checklist items
}
```

### LoRA Composition Methods

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      COMPOSITION METHODS                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  LINEAR: W = Σ(w_i × A_i)                                                   │
│  ─────────────────────────────                                              │
│  Simple weighted sum of adapter weights. Fast but can cause conflicts.      │
│                                                                              │
│  WEIGHTED: W = Σ(w_i/Σw × A_i)                                              │
│  ─────────────────────────────                                              │
│  Normalized interpolation. Smooth blending of traits.                       │
│                                                                              │
│  SLERP: W = sin((1-t)θ)/sin(θ) × A₁ + sin(tθ)/sin(θ) × A₂                   │
│  ─────────────────────────────                                              │
│  Spherical Linear Interpolation. Preserves geometric structure.             │
│  Best for breeding two high-quality parents.                                │
│                                                                              │
│  TIES: Trim → Elect → Merge                                                 │
│  ─────────────────────────────                                              │
│  1. Trim: Remove low-magnitude weights (20% threshold)                      │
│  2. Elect: Choose sign direction by majority vote                           │
│  3. Merge: Average weights that agree with elected sign                     │
│  Best for merging many adapters without conflicts.                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🐄 Species Implementations

### Memory Footprint

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SPECIES VRAM USAGE                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  🐄 Cattle (Heavy LLM)                                                      │
│  ████████████████████████████████████████████████████████████ 500MB         │
│  Deep reasoning, code generation, complex analysis                          │
│                                                                              │
│  🐑 Sheep (Classifier)                                                      │
│  ██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 50MB          │
│  Ensemble voting, spam detection, categorization                            │
│                                                                              │
│  🦆 Duck (Network)                                                          │
│  ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 100MB          │
│  API calls, web scraping, calendar integration                              │
│                                                                              │
│  🐐 Goat (Navigator)                                                        │
│  ██████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 150MB          │
│  File navigation, DOM traversal, debugging                                  │
│                                                                              │
│  🐗 Hog (Hardware)                                                          │
│  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 10MB           │
│  GPIO control, sensor reading, real-time signals                            │
│                                                                              │
│  🐔 Chicken (Monitor)                                                       │
│  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 5MB            │
│  Health checks, alerts, watchdog                                            │
│                                                                              │
│  🐎 Horse (Pipeline)                                                        │
│  █████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 200MB          │
│  ETL, encoding, batch processing                                            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## ⚡ Performance Characteristics

| Operation | Latency | Notes |
|:----------|:--------|:------|
| Reflex cache hit | <1ms | CUDA Graph |
| Anticipation prediction | ~10ms | Shadow Pup model |
| LoRA hot-swap | <50ms | Memory mapping |
| Full inference (Cattle) | 100-500ms | Phi-3 on Orin Nano |
| Night School cycle | 30-120min | Depends on gene pool size |

---

## 🔧 Building

```bash
# Debug build (fast compile, slow run)
cargo build

# Release build (slow compile, fast run)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --release
```

---

## 📊 Memory Safety

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    WHY RUST?                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  C++ (Traditional AI Runtime)                                               │
│  ══════════════════════════════                                             │
│  • Segfaults in production                                                  │
│  • Memory leaks in inference loops                                          │
│  • Data races in async code                                                 │
│  • Undefined behavior at scale                                              │
│                                                                              │
│  Rust (SuperInstance)                                                       │
│  ══════════════════════════════                                             │
│  ✓ Compile-time memory safety                                               │
│  ✓ No null pointer dereferences                                             │
│  ✓ Thread-safe by default                                                   │
│  ✓ Zero-cost abstractions                                                   │
│  ✓ Fearless concurrency                                                     │
│                                                                              │
│  Result: Ranch runs for months without restart                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

> *"Rust doesn't just prevent bugs. It prevents entire categories of bugs."*
