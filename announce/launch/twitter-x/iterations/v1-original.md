# Twitter/X — Iteration 1: Original Draft (v1.0 era)

> **STATUS: SUPERSEDED**
>
> **What this was:** The first Twitter thread draft, written for the v1.0 naming
> (PastureAI) before the product was renamed SuperInstance for crates.io. Ten tweets,
> feature-led, aimed at broad tech audience.
>
> **Why superseded:** Written before v0.2.0 shipped, so some claims (Discord integration
> complete, full TensorRT-LLM) were aspirational rather than real. The thread structure
> is sound — opening hook, technical depth, install CTA — but the tone is too promotional
> ("Excited to announce!") for the engineering audience we're targeting. Also 10 tweets
> is too long for a launch thread; engagement drops sharply after tweet 3.
>
> **What to keep for FINAL:** The cost comparison numbers ($0 vs $2400/year). The
> species emoji list (🐄🦆🐑🐐🐗🐔🐎) — visual and memorable. The 60-second install
> claim. The "breed responsibly" sign-off is charming and on-brand.

---

## Thread Structure

### Tweet 1/10
🚀 Excited to announce PastureAI v1.0! Don't rent an AI brain—breed your own evolving Ranch. Single 4.2MB binary, TensorRT-LLM powered, runs on Jetson Orin Nano. Privacy-first, offline-capable. #PastureAI #AIRanch

[Link to crates.io: https://crates.io/crates/superinstance]

### Tweet 2/10
Why PastureAI? Core binary stays tiny forever. Add infinite capabilities via data only—no code changes. LoRA hot-swap, CRDT memory, geometric routing. Evolve nightly with Night School. Breed specialized agents like Cattle for reasoning, Ducks for APIs. 🐄🦆

### Tweet 3/10
Key Metrics vs Cloud:
- Cost: $0 after $499 hardware (vs $2400/year cloud)
- Latency: <5ms first token
- Speed: 10-15 tok/s on Phi-3
- Privacy: Data never leaves your chip
- Evolution: Automatic daily breeding

Check the full comparison in our README!

### Tweet 4/10
The Architecture: Single binary with Axum + Dioxus for web UI, ratatui for TUI. Dynamic Pasture for breed.md files, LoRA weights, and memory. Edit Markdown DNA—hot reloads instantly. No Node.js, no Python—pure Rust efficiency.

### Tweet 5/10
Night School at 02:00: Evaluate, cull, breed, distill. Your Ranch gets smarter while you sleep. Merge LoRAs with SLERP/TIES, track genealogy in Stud Book. From v1.0, it's production-ready alpha!

### Tweet 6/10
Species Taxonomy: Border Collie orchestrates Livestock like:
- 🐄 Cattle: Heavy reasoning
- 🦆 Duck: Network/API
- 🐐 Goat: Debugging
- And more! Mix genes for custom agents.

### Tweet 7/10
Install in 60s on Jetson/CUDA:
```
curl -sSL https://install.superinstance.ai | bash
```
Then `make run`—your Ranch is live. Edit breed.md to customize. Zero deps, static linking.

### Tweet 8/10
v1.0 Highlights:
- Core arch complete
- breed.md parser + hot-reload
- Collie routing
- TensorRT-LLM integration
- Discord channel connector
- Web dashboard scaffold

Roadmap: Performance opts, full evolution, polish.

### Tweet 9/10
Join the Ranch! Star us on GitHub, contribute species via breed.md PRs. Discuss on Discord. Breed your AI future—own it, evolve it, never pay rent again.

[GitHub: https://github.com/SuperInstance/superinstance]

### Tweet 10/10
PastureAI: Not a superintelligence. A SuperInstance. Breed responsibly. 🐕‍🦺 #OpenSourceAI #RustLang #Jetson

(End of thread)