# 🌾 pasture/ — The Living Ecosystem

```
                    THE PASTURE
                    ═════════════
                    
         ┌──────────────────────────────────────┐
         │                                      │
         │    🌅 Where the Livestock Graze 🌅   │
         │                                      │
         │      🐄 🐑 🦆 🐐 🐗 🐔 🐎            │
         │                                      │
         │    "The filesystem IS the database"  │
         │                                      │
         └──────────────────────────────────────┘
```

## 🌱 Philosophy

The Pasture is where your agents live. Unlike traditional AI systems that store configuration in opaque databases, SuperInstance uses the filesystem itself as the source of truth. This means:

- **Git-trackable**: Every breed is version-controlled
- **Human-readable**: Edit Markdown, not JSON
- **Hot-reloadable**: Save file → instant evolution
- **Portable**: Copy folder → clone agent

---

## 📁 Directory Structure

```
pasture/
├── 📁 cattle/                    # Heavy LLM breeds
│   ├── 📁 email-cow-v1/
│   │   ├── breed.md              # 👈 THE SOURCE OF TRUTH
│   │   ├── adapter.safetensors   # (auto-generated)
│   │   └── meta.json             # (auto-generated)
│   ├── 📁 code-cow-v1/
│   └── 📁 summary-cow-v1/
│
├── 📁 sheep/                     # Classifier breeds
│   ├── 📁 email-sheep-v1/
│   └── 📁 sentiment-sheep-v1/
│
├── 📁 ducks/                     # Network agent breeds
│   ├── 📁 calendar-duck-v1/
│   └── 📁 fetch-duck-v1/
│
├── 📁 goats/                     # Navigator breeds
│   ├── 📁 filesystem-goat-v1/
│   └── 📁 debug-goat-v1/
│
├── 📁 hogs/                      # Hardware agent breeds
│   └── 📁 sensor-hog-v1/
│
├── 📁 chickens/                  # Monitor breeds
│   └── 📁 watchdog-chicken-v1/
│
├── 📁 horses/                    # Pipeline breeds
│   └── 📁 etl-horse-v1/
│
└── 📁 base_models/               # Foundation models
    ├── 📁 phi-3-mini/
    └── 📁 mamba/
```

---

## 🐄 Species Taxonomy Map

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SPECIES TAXONOMY                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                           ┌──────────────┐                                  │
│                           │   BASE MODEL │                                  │
│                           │  (Phi-3/Mamba)│                                  │
│                           └──────┬───────┘                                  │
│                                  │                                          │
│            ┌─────────────────────┼─────────────────────┐                    │
│            │                     │                     │                    │
│            ▼                     ▼                     ▼                    │
│     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐             │
│     │   🐄 CATTLE  │     │   🦆  DUCK   │     │   🐐  GOAT   │             │
│     │              │     │              │     │              │             │
│     │ • Reasoning  │     │ • API calls  │     │ • Navigate   │             │
│     │ • Code gen   │     │ • Fetch      │     │ • Debug      │             │
│     │ • Analysis   │     │ • Calendar   │     │ • Explore    │             │
│     │              │     │              │     │              │             │
│     │ VRAM: 500MB  │     │ VRAM: 100MB  │     │ VRAM: 150MB  │             │
│     └──────────────┘     └──────────────┘     └──────────────┘             │
│            │                     │                     │                    │
│            │                     │                     │                    │
│     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐             │
│     │   🐑  SHEEP  │     │   🐔CHICKEN  │     │   🐗  HOG    │             │
│     │              │     │              │     │              │             │
│     │ • Classify   │     │ • Monitor    │     │ • GPIO       │             │
│     │ • Vote       │     │ • Alert      │     │ • Sensors    │             │
│     │ • Filter     │     │ • Watchdog   │     │ • Realtime   │             │
│     │              │     │              │     │              │             │
│     │ VRAM: 50MB   │     │ VRAM: 5MB    │     │ VRAM: 10MB   │             │
│     └──────────────┘     └──────────────┘     └──────────────┘             │
│            │                                                               │
│            │                                                               │
│     ┌──────────────┐                                                       │
│     │   🐎 HORSE   │                                                       │
│     │              │                                                       │
│     │ • Pipeline   │                                                       │
│     │ • ETL        │                                                       │
│     │ • Transform  │                                                       │
│     │              │                                                       │
│     │ VRAM: 200MB  │                                                       │
│     └──────────────┘                                                       │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📝 The breed.md Standard

Every agent is defined by a `breed.md` Markdown file. This is the **DNA** of your agent.

### Example: Email-Cow-v1

```markdown
# 🐄 Breed: Email-Cow-v1

## 🧬 Lineage
* **Generation:** 1
* **Sire:** Base-Phi-3
* **Last Bred:** 2024-01-15

## ⚙️ Phenotype (Configuration)
| Gene | Value | Effect |
| :--- | :--- | :--- |
| **Temperature** | `0.3` | Low creativity, high consistency |
| **Max_Tokens** | `500` | Maximum response length |
| **Priority** | `High` | Wake immediately on intent |
| **Tone** | `Professional` | Response style |

## 🧬 Genetic Composition (The Recipe)
| Gene Trait | Weight | Description |
| :--- | :--- | :--- |
| `polite_tone` | `0.8` | Strong formal style |
| `json_output` | `0.1` | Light structure |
| `concise_style` | `0.5` | Brevity |

## 🛠️ Tool Access
- [x] `gmail_api`
- [x] `calendar_api`
- [ ] `filesystem`

## 🧠 Genetic Code (System Prompt)
```
You are an Email Triage Specialist.
Be concise. Prioritize boss emails.
```
```

### Anatomy of a breed.md

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      breed.md ANATOMY                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ # 🐄 Breed: Name                                                    │   │
│  │ ─────────────────────                                               │   │
│  │ Species emoji + breed name                                          │   │
│  │ Determines which species folder this belongs to                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ## 🧬 Lineage                                                       │   │
│  │ ─────────────────                                                   │   │
│  │ • Generation: Incremental version                                   │   │
│  │ • Sire: Father's breed ID (or "Base-Phi-3")                         │   │
│  │ • Dam: Mother's breed ID (from distillation)                        │   │
│  │ • Last Bred: Date of last Night School                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ## ⚙️ Phenotype                                                     │   │
│  │ ────────────────────                                                │   │
│  │ Table of runtime settings:                                          │   │
│  │ • Temperature: 0.0-2.0 (creativity)                                 │   │
│  │ • Max_Tokens: Response length limit                                 │   │
│  │ • Priority: Low/Normal/High/Critical                                │   │
│  │ • Tone: Any custom style                                            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ## 🧬 Genetic Composition                                           │   │
│  │ ─────────────────────────                                           │   │
│  │ Table mapping gene traits to weights:                               │   │
│  │ • Gene Trait: ID from genetics/traits/                              │   │
│  │ • Weight: 0.0-1.0+ (influence strength)                              │   │
│  │ • Description: Human-readable note                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ## 🛠️ Tool Access                                                   │   │
│  │ ────────────────────                                                │   │
│  │ Checklist of enabled tools:                                         │   │
│  │ - [x] enabled_tool                                                  │   │
│  │ - [ ] disabled_tool                                                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ ## 🧠 Genetic Code                                                  │   │
│  │ ───────────────────                                                 │   │
│  │ Code block containing the system prompt                             │   │
│  │ This is the "soul" of the agent                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔥 Hot-Reload

When you save a `breed.md` file, the Border Collie immediately:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      HOT-RELOAD FLOW                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. 👀 File Watcher detects change                                          │
│              │                                                               │
│              ▼                                                               │
│  2. 📖 Parser reads new breed.md                                            │
│              │                                                               │
│              ▼                                                               │
│  3. 🧬 Composer calculates new weights                                      │
│              │                                                               │
│              ▼                                                               │
│  4. 💾 LoRA Manager swaps adapter                                           │
│              │                                                               │
│              ▼                                                               │
│  5. ✅ Dashboard shows "Genetics Updated"                                   │
│                                                                              │
│  Total time: <100ms (no restart!)                                           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🌙 Night School Evolution

Every night at 02:00, the Pasture evolves:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    EVOLUTION EXAMPLE                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Day 1:                                                                     │
│  ───────                                                                    │
│  pasture/cattle/                                                            │
│  └── email-cow-v1/  (fitness: 0.75)                                        │
│                                                                              │
│  Day 7: (After Night School breeding)                                       │
│  ───────                                                                    │
│  pasture/cattle/                                                            │
│  ├── email-cow-v1/  (fitness: 0.68, marked for culling)                    │
│  ├── email-cow-v2/  (fitness: 0.82, bred from v1 + polite_tone)            │
│  └── email-cow-v3/  (fitness: 0.79, bred from v1 + concise_style)          │
│                                                                              │
│  Day 14:                                                                    │
│  ───────                                                                    │
│  pasture/cattle/                                                            │
│  ├── email-cow-v2/  (fitness: 0.85, promoted)                              │
│  └── email-cow-v4/  (fitness: 0.91, v2 + v3 merge)                         │
│                                                                              │
│  Your Ranch got smarter.                                                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🎨 Creating a New Breed

### Option 1: Use the Wizard

```bash
make breed
# or
cargo run --release --bin superinstance-onboard
```

### Option 2: Copy and Edit

```bash
# Copy template
cp -r pasture/cattle/email-cow-v1 pasture/cattle/my-cow-v1

# Edit the DNA
nano pasture/cattle/my-cow-v1/breed.md

# Save → instantly live!
```

### Option 3: Create from Scratch

```bash
mkdir -p pasture/cattle/my-cow-v1
cat > pasture/cattle/my-cow-v1/breed.md << 'EOF'
# 🐄 Breed: My-Cow-v1

## ⚙️ Phenotype
| Gene | Value |
| :--- | :--- |
| **Temperature** | `0.7` |

## 🧬 Genetic Code
```
You are a helpful assistant.
```
EOF
```

---

> *"A breed.md is not a config file. It's a love letter to your agent."*
