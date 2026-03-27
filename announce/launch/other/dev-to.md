# dev.to Article

> **Post:** 1–2 weeks after HN to ride the long-tail traffic
> **Format:** Technical deep-dive. dev.to readers want to understand the *how*, not
> just the *what*. This is the place to explain SLERP, the breed.md parser, the
> binary size approach. 800–1200 words.
> **Tags:** #rust #ai #machinelearning #opensource

---

## Title

**I built an AI that breeds better versions of itself every night — here's how the genetics work**

---

## Article

*[Cover image: TUI dashboard screenshot with Night School log visible]*

Six weeks ago I was annoyed that my local AI setup always started from scratch. Same
model, same weights, same generic behaviour. I wanted something that learned from
how I actually used it.

So I built Night School.

### What Night School Is

Night School is a nightly scheduler (02:00 by default) that runs a genetic selection
loop over a set of AI agents. Here's what it actually does:

**Evaluation:**
Each agent accumulates a fitness score throughout the day based on task outcomes.
Currently this is "did the human approve the output" — simple but effective as a
starting point.

**Culling:**
Agents below a configurable threshold (default: 0.40) are deleted from the registry
and stud book. No mercy.

**Breeding:**
The remaining high-fitness agents are paired via tournament selection. For each pair,
a new "offspring" agent is created by interpolating their LoRA adapter weights using SLERP.

**Quarantine:**
New offspring don't go directly into production. They run alongside their parents for
24 hours. If their fitness score exceeds the parents', they promote. If not, they're
culled in the next cycle.

**The result:**
After a few weeks, your agents have a genuine genealogy. The invoice-processing agent
in production might be generation 8, descended from six generations of selection.

### Why SLERP (Not LERP)

When merging LoRA adapters, the naive approach is linear interpolation:

```
offspring = α × parent_A + (1-α) × parent_B
```

This works but has a problem: LERP shrinks the norm of the result when α ≠ 0 or 1.
For a unit vector, LERP at α=0.5 produces a vector with norm ~0.71, not 1.0.

SLERP (spherical linear interpolation) travels along the surface of the unit sphere
instead, preserving norm:

```
offspring = sin((1-α)θ)/sin(θ) × parent_A + sin(αθ)/sin(θ) × parent_B
```

where θ is the angle between the two weight vectors.

In practice with LoRA adapters: SLERP-merged adapters behave more like "halfway between
both parents" rather than "weaker than either parent." The effect is subtle but
measurable on longer inference runs.

```rust
// The actual implementation in superinstance/src/evolution/breeding.rs
fn slerp(a: &[f32], b: &[f32], t: f32) -> Vec<f32> {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>().clamp(-1.0, 1.0);
    let theta = dot.acos();

    if theta.abs() < 1e-6 {
        // Nearly identical vectors — fall back to lerp
        return a.iter().zip(b.iter()).map(|(x, y)| x + t * (y - x)).collect();
    }

    let sin_theta = theta.sin();
    let scale_a = ((1.0 - t) * theta).sin() / sin_theta;
    let scale_b = (t * theta).sin() / sin_theta;

    a.iter().zip(b.iter()).map(|(x, y)| scale_a * x + scale_b * y).collect()
}
```

### The breed.md Hot-Reload

Every agent's personality lives in a Markdown file (`breed.md`). When you edit it,
the system reloads the agent in ~180ms without any restart.

The watcher uses `notify-debouncer-full` with a 50ms debounce window:

```rust
let (tx, rx) = std::sync::mpsc::channel();
let mut debouncer = new_debouncer(Duration::from_millis(50), None, tx)?;
debouncer.watcher().watch(&pasture_path, RecursiveMode::Recursive)?;
```

On a debounced event: re-parse the Markdown, extract the trait weight table via
`pulldown-cmark`, reload the tensors via `safetensors`. The agent's `Arc<RwLock<BreedManifest>>`
is swapped atomically so in-flight requests finish on the old manifest.

The trait weight table in a breed.md looks like this:

```markdown
## Genetic Composition

| Gene Trait          | Weight | Description        |
| :------------------ | :----- | :----------------- |
| `professional_tone` | `0.9`  | Formal, precise    |
| `brief_responses`   | `0.7`  | Concise outputs    |
| `email_expertise`   | `0.85` | Email conventions  |
```

Each trait name maps to a pre-trained LoRA adapter file on disk. The weights become
the blend ratios for the SLERP merge.

### The 4.2 MB Binary Constraint

I set a hard limit of 5 MB for the release binary, enforced in CI:

```yaml
- name: Check binary size
  run: |
    SIZE=$(stat -c%s superinstance/target/release/superinstance)
    if [ $SIZE -gt 5242880 ]; then
      echo "Binary size $SIZE exceeds 5MB limit"
      exit 1
    fi
```

The Cargo.toml release profile:
```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = "z"
```

The biggest wins were: `opt-level = "z"` (size over speed), `panic = "abort"` (removes
unwinding code), and `strip = true` (removes debug symbols from the final binary).

The most expensive dependency by binary contribution is `serenity` (Discord) at ~800 KB.
This is feature-gated and excluded from the default build.

### What's Next

v0.2.0 is alpha. Night School works. breed.md hot-reload works. Demo mode works.

v0.3.0 will focus on:
- Full TensorRT-LLM integration for Jetson (the backend is scaffolded)
- Pluggable fitness functions (currently just "human approval rate")
- Discord channel connector (scaffolded, needs the real bot integration)
- Real-time breeding animation in the TUI

If any of this sounds interesting: https://github.com/SuperInstance/pasture-ai

The codebase is Rust 1.85+, MIT licensed, and contributions are welcome — especially
from people who want to add new species types or fitness measurement approaches.

---

*Tags: #rust #ai #locallm #machinelearning #opensource*
