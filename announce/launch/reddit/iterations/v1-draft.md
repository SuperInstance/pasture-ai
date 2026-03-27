# Reddit Launch Posts — Iteration 1

> **STATUS: DRAFTS — Each subreddit needs its own angle**
>
> **Subreddits identified:**
> - r/rust — technical, the binary/architecture story
> - r/LocalLLaMA — the local AI crowd, Night School is the hook
> - r/selfhosted — privacy, ownership, no cloud
> - r/homeassistant — maker/IoT angle, Hog species for GPIO
> - r/MachineLearning — too academic for v0.2.0 alpha, skip for now
>
> **Key insight:** Each subreddit has a different primary objection.
> - r/rust: "is this idiomatic Rust? what's the unsafe surface?"
> - r/LocalLLaMA: "how does this compare to running llama.cpp directly?"
> - r/selfhosted: "can I run this on a Raspberry Pi? what are the dependencies?"
> - r/homeassistant: "does it work with HA's REST API? can it trigger automations?"
>
> **The rule:** The first sentence of each post must be native to that subreddit.
> Do not copy-paste the same post to every community.

---

## r/rust — DRAFT 1

**Title:** I built a self-evolving AI agent system in Rust — 4.2 MB binary, breed.md hot-reload, SLERP LoRA merging

**Body:**

Sharing this here because I made some unusual architecture choices that Rustaceans might
find interesting (or correctly identify as mistakes):

**The binary size obsession:**
The release profile is: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`,
`panic = "abort"`. I was targeting <5 MB for a single binary that includes TUI (ratatui),
web server (Axum), SQLite (rusqlite bundled), LoRA inference (safetensors + ndarray),
file watcher (notify), and Discord client (serenity).

Current size: 4.2 MB. CI fails the build if it exceeds 5 MB.

**The hot-reload design:**
breed.md files live in a `pasture/` directory. A debounced inotify watcher (`notify-debouncer-full`)
watches the directory. On change, the file is re-parsed with `pulldown-cmark` and the
LoRA adapter weights are swapped via `safetensors`. No unsafe, no runtime reflection —
just re-running the parser and re-loading the tensors.

**The trait system:**
`Species` is a trait. `Cattle`, `Duck`, `Sheep`, etc. implement it via `async-trait`.
The Border Collie holds a `Vec<Box<dyn Species + Send + Sync>>` and routes via a match
on `Intent::kind`.

**What I'd like feedback on:**
- The `parking_lot::RwLock` vs. `tokio::sync::RwLock` choice throughout — I went with
  parking_lot for the synchronous accessors but I'm not sure this was right
- The breed.md format — is pulldown-cmark the right choice or is there a lighter parser
  for structured Markdown?
- Binary size: what am I missing that could push it lower?

GitHub: https://github.com/SuperInstance/pasture-ai (MIT, Rust 1.85+)

---

## r/LocalLLaMA — DRAFT 1

**Title:** Built a self-evolving local AI ranch — agents breed overnight via SLERP, personality in hot-reloadable Markdown

**Body:**

After running various local models for a while, I wanted something that gets better at
my specific tasks over time rather than always starting from a generic base.

Built SuperInstance: a local AI ecosystem where agents "breed" nightly via SLERP LoRA
merging. High-fitness agents reproduce, offspring run in quarantine alongside parents,
get promoted if they outperform.

The personality system (breed.md) is the part that surprised me most in practice —
having agent prompts and trait weights as a plain file you can edit and see reload
in 180ms is genuinely useful for iteration.

Hardware target is Jetson Orin Nano 8GB (~20 tok/s with Phi-3 Mini in MAXN mode) but
it falls back to llama.cpp on any CUDA GPU, or Candle on CPU.

**Current status:** v0.2.0 alpha. Night School runs. breed.md hot-reload works. Demo mode
works without hardware. TensorRT-LLM full integration in progress.

Questions for this community:
- What's your experience with LoRA SLERP merges vs. TIES or linear? I went with SLERP
  for the geometry properties but curious if others have benchmarked this for personality-style merges
- Is Phi-3 Mini the right base model for an edge-deployed general assistant, or is there
  something better at the 3.8B parameter range now?

GitHub: https://github.com/SuperInstance/pasture-ai

---

## r/selfhosted — DRAFT 1

**Title:** Self-hosted AI that evolves overnight — 4.2MB Rust binary, no cloud, no Python, runs on $499 Jetson

**Body:**

For the self-hosters who've been running local LLMs: SuperInstance is a layer on top
that makes your agents get better at your specific tasks over time.

The core promise: privacy-first (never phones home), offline-capable, single binary
(no Docker, no Python virtualenv, no Node.js), runs on edge hardware you own.

**What "evolves" means concretely:**
Every night at 02:00, agents that performed well breed (LoRA weight interpolation),
underperformers get culled. Your invoice-processing agent on Day 30 is a different
(better) beast than on Day 1 — not because you trained it, but because it's been
naturally selected for your tasks.

**Hardware requirements:**
- Ideal: NVIDIA Jetson Orin Nano 8GB (~$499)
- Works: Any CUDA GPU (adapts to available VRAM)
- Works with limitations: Any Linux x86_64 CPU (Candle backend, slower)
- Not yet: ARM without CUDA (Raspberry Pi 5 etc.) — in roadmap

**Single binary:**
```
curl -sSf https://github.com/SuperInstance/pasture-ai/releases/download/v0.2.0/superinstance
chmod +x superinstance && ./superinstance
```

That's the entire install for a machine with CUDA already. No apt, no pip, no npm.

GitHub: https://github.com/SuperInstance/pasture-ai
License: MIT

---

*Note: These are first drafts. Each needs to be posted separately, spaced at least
a week apart to avoid appearing as spam across communities.*
