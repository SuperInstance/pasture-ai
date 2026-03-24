# 🧬 genetics/ — The Gene Pool

```
                    THE GENE POOL
                    ═══════════════
                    
         ┌──────────────────────────────────────┐
         │                                      │
         │     🧬 Periodic Table of LoRAs 🧬    │
         │                                      │
         │   Mix traits like LEGO blocks        │
         │   Breed agents like Pokemon          │
         │                                      │
         └──────────────────────────────────────┘
```

## 🧪 What are Genes?

Genes are **atomic LoRA adapters** - small, single-purpose weight modifications that can be stacked together to create specialized agents.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        GENE CONCEPT                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Traditional Approach:                                                       │
│  ═════════════════════                                                      │
│  ┌─────────────────────────────────────────┐                                │
│  │         MONOLITHIC MODEL                │                                │
│  │  (2GB - all behaviors hardcoded)        │                                │
│  │                                         │                                │
│  │  • Coding                               │                                │
│  │  • Writing                              │                                │
│  │  • Math                                 │                                │
│  │  • Chat                                 │                                │
│  │  • ...                                  │                                │
│  └─────────────────────────────────────────┘                                │
│  Problem: Can't customize. Can't evolve.                                    │
│                                                                              │
│  Gene Pool Approach:                                                         │
│  ═════════════════════                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                          │
│  │ Base Model  │  │ Gene: Code  │  │ Gene: Tone  │                          │
│  │   (2.5GB)   │  │   (50MB)    │  │   (50MB)    │                          │
│  └─────────────┘  └─────────────┘  └─────────────┘                          │
│         │                │                 │                                │
│         └────────────────┼─────────────────┘                                │
│                          ▼                                                  │
│                  ┌─────────────┐                                            │
│                  │ Composed    │                                            │
│                  │ Agent       │                                            │
│                  │ (2.6GB)     │                                            │
│                  └─────────────┘                                            │
│  Benefit: Mix and match. Evolve. Share.                                     │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📊 The Periodic Table of LoRA Traits

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    PERIODIC TABLE OF GENES                                   │
├──────────────┬──────────────┬──────────────┬──────────────┬────────────────┤
│   STYLE      │   FUNCTION   │   DOMAIN     │   OUTPUT     │   TOOLS        │
├──────────────┼──────────────┼──────────────┼──────────────┼────────────────┤
│              │              │              │              │                │
│  polite_tone │ json_output  │ rust_coder   │ concise      │ gmail_api      │
│  ─────────── │ ─────────── │ ─────────── │ ─────────── │ ───────────    │
│  Formal,     │ Structured   │ Expert Rust  │ Brief,       │ Gmail access   │
│  courteous   │ JSON format  │ code gen     │ to the point │                │
│              │              │              │              │                │
├──────────────┼──────────────┼──────────────┼──────────────┼────────────────┤
│ casual_tone  │ yaml_output  │ python_coder │ verbose      │ calendar_api   │
│  ─────────── │ ─────────── │ ─────────── │ ─────────── │ ───────────    │
│  Relaxed,    │ YAML format  │ Python       │ Detailed     │ Calendar       │
│  friendly    │              │ expertise    │ explanations │ access         │
│              │              │              │              │                │
├──────────────┼──────────────┼──────────────┼──────────────┼────────────────┤
│ pirate_slang │ xml_output   │ sql_expert   │ structured   │ filesystem     │
│  ─────────── │ ─────────── │ ─────────── │ ─────────── │ ───────────    │
│  Arr, matey! │ XML format   │ Database     │ Organized    │ File access    │
│              │              │ queries      │ sections     │                │
│              │              │              │              │                │
├──────────────┼──────────────┼──────────────┼──────────────┼────────────────┤
│ formal_tone  │ markdown_out │ web_dev      │ bullet_pt    │ web_search     │
│  ─────────── │ ─────────── │ ─────────── │ ─────────── │ ───────────    │
│  Business    │ Markdown     │ HTML/CSS/JS  │ Bullet       │ Search the     │
│  appropriate │ formatted    │ development  │ points       │ web            │
│              │              │              │              │                │
├──────────────┼──────────────┼──────────────┼──────────────┼────────────────┤
│ witty_style  │ code_block   │ data_science │ emoji_rich   │ slack_api      │
│  ─────────── │ ─────────── │ ─────────── │ ─────────── │ ───────────    │
│  Humor,      │ Code        │ Pandas,      │ Emoji        │ Slack          │
│  clever      │ formatted   │ NumPy        │ sprinkled    │ integration    │
│              │              │              │              │                │
└──────────────┴──────────────┴──────────────┴──────────────┴────────────────┘
```

---

## 🧪 Gene Composition Recipes

### Recipe 1: Professional Emailer

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 PROFESSIONAL EMAILER                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Ingredients:                                                               │
│  ───────────                                                                │
│  • polite_tone (0.8)      → Formal, courteous                              │
│  • concise_style (0.5)    → Brief but complete                             │
│  • json_output (0.1)      → Light structure                                │
│                                                                              │
│  Result:                                                                    │
│  ───────                                                                    │
│  "Dear Sir/Madam,                                                           │
│                                                                              │
│   I am writing to confirm our meeting scheduled for tomorrow at 2 PM.       │
│   Please find the agenda attached.                                          │
│                                                                              │
│   Best regards"                                                             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Recipe 2: Fun Chatbot

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      FUN CHATBOT                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Ingredients:                                                               │
│  ───────────                                                                │
│  • casual_tone (0.9)      → Relaxed, friendly                              │
│  • emoji_rich (0.7)       → Emojis sprinkled                               │
│  • witty_style (0.5)      → Humor added                                    │
│                                                                              │
│  Result:                                                                    │
│  ───────                                                                    │
│  "Hey there! 👋 So great to hear from you!                                  │
│                                                                              │
│   What can I help you with today? I'm all ears! 🐄"                        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Recipe 3: Code Documentation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                  CODE DOCUMENTATION                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Ingredients:                                                               │
│  ───────────                                                                │
│  • rust_coder (1.0)       → Expert Rust knowledge                          │
│  • markdown_out (0.8)     → Markdown formatted                             │
│  • concise_style (0.6)    → Brief explanations                             │
│  • code_block (0.9)       → Code formatted                                 │
│                                                                              │
│  Result:                                                                    │
│  ───────                                                                    │
│  "## Function: `process_email`                                              │
│                                                                              │
│   ```rust                                                                   │
│   fn process_email(email: Email) -> Result<EmailAction> {                   │
│       // Classify and route email                                           │
│   }                                                                         │
│   ```                                                                       │
│                                                                              │
│   **Purpose:** Classifies incoming emails and determines routing."         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔬 Gene Anatomy

Each gene is stored in its own directory:

```
genetics/traits/
├── polite_tone/
│   ├── adapter.safetensors    # LoRA weights (~50MB)
│   ├── meta.json              # Metadata
│   └── README.md              # Documentation
│
├── rust_coder/
│   ├── adapter.safetensors
│   ├── meta.json
│   └── README.md
│
└── ...
```

### meta.json Structure

```json
{
  "id": "polite_tone",
  "name": "Professional Tone",
  "description": "Makes responses formal and courteous",
  "size_bytes": 52428800,
  "compatible_species": ["Cattle", "Sheep"],
  "tags": ["style", "formal", "professional"],
  "layer_info": {
    "attention_layers": 32,
    "affected_heads": "all",
    "primary_function": "style_transfer"
  },
  "performance": {
    "quality_score": 0.89,
    "coherence_impact": 0.02,
    "hallucination_risk": "none"
  },
  "lineage": {
    "generation": 1,
    "source": "distillation",
    "parent_models": ["claude-3-sonnet"]
  }
}
```

---

## 🧬 Creating a New Gene

### Step 1: Prepare Training Data

```bash
# Create training examples
mkdir -p genetics/traits/my_gene/training
cat > genetics/traits/my_gene/training/data.jsonl << 'EOF'
{"input": "Write a greeting", "output": "Greetings, esteemed colleague!"}
{"input": "Say hello", "output": "Hello there, good sir!"}
EOF
```

### Step 2: Train or Distill

```bash
# Option A: Distill from cloud API
cargo run --release --bin distill -- \
  --from claude-3-sonnet \
  --output genetics/traits/my_gene

# Option B: Fine-tune locally
python scripts/train_lora.py \
  --base phi-3-mini \
  --data genetics/traits/my_gene/training/data.jsonl \
  --output genetics/traits/my_gene
```

### Step 3: Register in Gene Pool

```bash
# The gene is automatically detected by the watcher
# Or manually:
cargo run --release --bin gene-pool -- register genetics/traits/my_gene
```

---

## 🌙 Breeding Mechanics

### SLERP (Spherical Linear Interpolation)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SLERP BREEDING                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Parent A (Sire)                    Parent B (Dam)                          │
│  ┌───────────────────────┐         ┌───────────────────────┐               │
│  │ polite_tone (0.8)    │         │ concise_style (0.5)   │               │
│  │ fitness: 0.85        │         │ fitness: 0.78         │               │
│  └───────────────────────┘         └───────────────────────┘               │
│              │                               │                               │
│              └───────────────┬───────────────┘                               │
│                              │                                               │
│                              ▼                                               │
│                    ┌───────────────────────┐                                │
│                    │ SLERP INTERPOLATION   │                                │
│                    │ θ = arccos(A·B)       │                                │
│                    │ W = sin((1-t)θ)/sin(θ)│                                │
│                    │     × A + sin(tθ)/    │                                │
│                    │     sin(θ) × B        │                                │
│                    └───────────────────────┘                                │
│                              │                                               │
│                              ▼                                               │
│                    ┌───────────────────────┐                                │
│                    │ Offspring             │                                │
│                    │ polite_tone (0.6)     │                                │
│                    │ concise_style (0.4)   │                                │
│                    │ est. fitness: 0.82    │                                │
│                    └───────────────────────┘                                │
│                                                                              │
│  Result: Best of both parents with smooth interpolation                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### TIES (Trim, Elect, Merge)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          TIES BREEDING                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Multiple Parents:                                                          │
│  ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐  ┌─────┐                               │
│  │ A   │  │ B   │  │ C   │  │ D   │  │ E   │                               │
│  └─────┘  └─────┘  └─────┘  └─────┘  └─────┘                               │
│     │        │        │        │        │                                   │
│     ▼        ▼        ▼        ▼        ▼                                   │
│  ┌─────────────────────────────────────────────────┐                        │
│  │ STEP 1: TRIM                                    │                        │
│  │ Keep only top 20% magnitude weights            │                        │
│  └─────────────────────────────────────────────────┘                        │
│                        │                                                    │
│                        ▼                                                    │
│  ┌─────────────────────────────────────────────────┐                        │
│  │ STEP 2: ELECT                                   │                        │
│  │ Choose sign direction by majority vote          │                        │
│  └─────────────────────────────────────────────────┘                        │
│                        │                                                    │
│                        ▼                                                    │
│  ┌─────────────────────────────────────────────────┐                        │
│  │ STEP 3: MERGE                                   │                        │
│  │ Average weights that agree with elected sign    │                        │
│  └─────────────────────────────────────────────────┘                        │
│                        │                                                    │
│                        ▼                                                    │
│              ┌─────────────────┐                                            │
│              │ Merged Offspring │                                            │
│              └─────────────────┘                                            │
│                                                                              │
│  Result: Conflict-free merge of many genes                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📊 Gene Performance Tracking

Every gene's performance is tracked in the Stud Book:

```sql
SELECT 
    gene_id,
    AVG(fitness_delta) as avg_improvement,
    COUNT(*) as usage_count,
    SUM(CASE WHEN success THEN 1 ELSE 0 END) * 1.0 / COUNT(*) as success_rate
FROM gene_performance
GROUP BY gene_id
ORDER BY avg_improvement DESC;
```

---

> *"A gene is not code. A gene is potential waiting to be composed."*
