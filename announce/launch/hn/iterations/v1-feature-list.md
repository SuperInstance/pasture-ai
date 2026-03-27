# HN Launch Post — Iteration 1: Feature List

> **STATUS: REJECTED — Never submit this**
>
> **What we tried:** Lead with the feature list and the binary size. Technical credibility
> first. "4.2 MB Rust binary. TensorRT-LLM. SLERP. breed.md. Night School." Let the
> features speak for themselves to a technical audience.
>
> **Why HN will destroy this:** HN readers in 2024–2026 have developed a finely-tuned
> radar for AI project hype. The moment they see a list of buzzwords without a
> clear "here is what problem this solves" statement, the top comment will be
> "can't it just run Ollama?" and the thread dies.
>
> **Also:** "Show HN" posts that lead with "I built X with Y and Z" without explaining
> *why* X matters get fewer upvotes than posts that lead with the insight or the
> observation that motivated X. HN readers reward intellectual honesty about problems,
> not just solutions.
>
> **What to keep:** The benchmarks are real and interesting (20 tok/s on $499 hardware).
> The binary size is genuinely remarkable. But these should be *supporting evidence*
> for the main argument, not the lede.

---

## Draft (do not use)

**Title:** Show HN: SuperInstance – Self-evolving AI ranch in a 4.2 MB Rust binary

**Body:**
I built SuperInstance, a self-evolving AI ecosystem that runs on a $499 NVIDIA Jetson
Orin Nano.

Key features:
- 4.2 MB single Rust binary (TUI + web dashboard + inference + evolution)
- breed.md: define agent personalities in Markdown, hot-reloads in <200ms
- Night School: nightly SLERP LoRA merging at 02:00 — agents evolve while you sleep
- 7 species types: Cattle (reasoning), Sheep (classification), Duck (API), Goat
  (debugging), Hog (hardware), Chicken (monitoring), Horse (ETL)
- Border Collie orchestrator handles routing automatically
- TensorRT-LLM on Jetson, Candle on laptop, Mock for demo
- ~20 tok/s on Phi-3 Mini in MAXN mode

GitHub: https://github.com/SuperInstance/pasture-ai

Tech stack: Rust, Tokio, Axum, ratatui, rusqlite, safetensors, serenity (Discord)

---

*This was the first draft. It describes what the project is but not why it matters.*
*A HN reader's response: "sounds interesting, is there a demo?" — and then they leave.*
