# Reddit Launch Posts — FINAL

> **Version:** v2 (from v1-draft with community-specific polish)
> **Strategy:** Post to each subreddit separately, one per week, in this order:
>   1. r/rust (day 1 — the technical community most likely to contribute)
>   2. r/LocalLLaMA (day 7 — largest audience for local AI)
>   3. r/selfhosted (day 14 — steady long-tail traffic)
>   4. r/MachineLearning (hold until v0.3.0 when Night School has real benchmarks)
> **Rule:** Do not cross-post. Each post is written for its community.

---

## r/rust — FINAL

**Title:** Show r/rust: Self-evolving AI agents in Rust – single binary with LoRA hot-swap, SLERP breeding, 4.2 MB release

**Body:**

Built this over the last few months. Sharing here because I made several choices that
this community would either validate or correctly identify as misguided.

**Unusual constraints I set:**
- Release binary must stay under 5 MB. This is enforced in CI (`stat` the binary, fail
  if >5 MB). Achieved with: `opt-level = "z"`, `lto = true`, `codegen-units = 1`,
  `strip = true`, `panic = "abort"`. Current: 4.2 MB including TUI, web server, SQLite,
  LoRA inference, file watcher, and Discord client.
- No unsafe outside of FFI boundaries. The llama.cpp integration is feature-gated
  and isolated. Core logic is safe Rust throughout.

**The hot-reload design:**
Agent personalities live in `breed.md` files (Markdown). `notify-debouncer-full` watches
the directory. On save, `pulldown-cmark` re-parses the file, `safetensors` loads the
new adapter weights, and the running agent swaps its weights without restart.
Total reload time: ~180ms for a typical 150 MB adapter. The debouncer prevents thrashing
on rapid saves.

**The concurrency model:**
- Ranch state: `Arc<RwLock<_>>` via `parking_lot` for sync accessors
- Channel messages: `tokio::mpsc` channels throughout
- Night School: `tokio::spawn` background task with a computed `sleep` until 02:00
- Web server: `axum` with `Extension<Arc<Ranch>>` for shared state

I went with `parking_lot::RwLock` over `tokio::sync::RwLock` because most state reads
are in synchronous accessor methods. Not sure this was right — would appreciate pushback.

**Questions for Rustaceans:**
1. `parking_lot` vs `tokio::sync` for shared state with mixed sync/async readers?
2. `pulldown-cmark` for parsing structured Markdown tables — is there a lighter option?
3. I'm using `safetensors` for LoRA tensor loading. Is there a crate with better
   memory-mapping support for large tensors on embedded devices?

**What the project does:**
AI agents that evolve overnight. Agent personalities in hot-reloadable Markdown.
Runs on a $499 Jetson Orin Nano. Night School (02:00 nightly scheduler) culls
underperforming agents and breeds new ones via SLERP LoRA weight interpolation.

GitHub: https://github.com/SuperInstance/pasture-ai
License: MIT | Rust 1.85+ | `superinstance --demo` to try without hardware

---

## r/LocalLLaMA — FINAL

**Title:** I built a local AI where agents breed better versions of themselves overnight – SLERP LoRA merging, runs on Jetson

**Body:**

Problem I was trying to solve: I run local LLMs, but they're always the same model with
the same weights doing the same job. I wanted agents that improve at *my* specific tasks
over time — automatically, without me training anything.

**What I built:**

SuperInstance is a local AI agent system with a nightly evolution cycle (Night School).
At 02:00, agents are evaluated on the day's tasks. High-fitness pairs breed — their LoRA
adapter weights are interpolated via SLERP. Offspring run in quarantine alongside parents.
The ones that perform better get promoted. The ones that don't get culled.

After a few weeks, your agents have a genealogy. The current production agent is generation
6 or 8, descended from the best-performing ancestors. It's been naturally selected for
your workflow.

**Hardware and model:**
Target: Jetson Orin Nano 8GB, Phi-3 Mini 4K. ~20 tok/s in MAXN mode.
Fallback: CUDA GPU (auto-detected), CPU via Candle, mock for demo.
The GGUF model download is manual (HuggingFace CLI) — too large for the install script.

**The personality system:**
Each agent has a `breed.md` — a Markdown file with LoRA trait weights and a system prompt.
Edit it, save, it hot-reloads in ~180ms. You can git diff your agent's personality across weeks.
This turned out to be the most practically useful part: fast iteration without a build step.

**Honest v0.2.0 status:**
Night School: ✅ working
breed.md hot-reload: ✅ working
Demo mode (no hardware): ✅ working
TensorRT-LLM: 🔄 in progress (falls back to mock)
Discord connector: 🔄 scaffolded (full integration v0.3.0)

**Questions:**
- SLERP vs TIES for LoRA weight merging: any benchmarks on personality-style adapter blending?
- At 3.8B params, is Phi-3 Mini still the right edge-deploy choice, or has something else emerged?
- Fitness function is currently "human approval rate". What automatic metrics would you use?

Demo (no GPU needed): `superinstance --demo`
GitHub: https://github.com/SuperInstance/pasture-ai

---

## r/selfhosted — FINAL

**Title:** Self-hosted AI that improves at your tasks overnight – single 4.2MB binary, no cloud, no Python, offline-capable

**Body:**

For the self-hosters here: this is a local AI system that gets genuinely better at your
specific workflows over time, with no cloud dependency and a single static binary install.

**The overnight improvement part (Night School):**
Every night at 02:00, a scheduler evaluates how well each agent performed during the day.
Agents below a configurable fitness threshold are deleted. High-performers breed —
their model weights merge via SLERP interpolation — and offspring run in quarantine.
The best offspring promote to production.

After a few weeks your email-triage agent or invoice-processor has been through 6–8
generations of selection. It's learned your patterns. It never phoned home to do it.

**Privacy model:**
- All inference local (no API calls unless you configure cloud fallback in .env)
- breed.md files and stud book (SQLite) are local files you own
- Night School runs entirely on-device
- No telemetry, no heartbeat, no phone-home

**Install (machine with CUDA):**
```bash
# One-liner
curl -sSf https://github.com/SuperInstance/pasture-ai/releases/download/v0.2.0/superinstance \
  -o superinstance && chmod +x superinstance

# Or from source
git clone https://github.com/SuperInstance/pasture-ai && cd pasture-ai && make build
```

No Docker. No pip. No npm. Single binary.

**Hardware:**
- Best: NVIDIA Jetson Orin Nano 8GB (~$499) — designed for this
- Good: Any NVIDIA GPU (CUDA, auto-detected)
- Works: Any Linux x86_64 CPU (Candle backend, ~3 tok/s)
- Not yet: ARM without CUDA (Raspberry Pi 5) — planned for v0.4.0

**Demo without any GPU:**
```
superinstance --demo
```

GitHub: https://github.com/SuperInstance/pasture-ai | MIT | Rust 1.85+
