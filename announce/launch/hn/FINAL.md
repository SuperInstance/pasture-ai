# HN Launch Post — FINAL

> **Version:** v4 (from v3-story-hook with link placement fix and tighter paragraphs)
> **Target:** Show HN front page
> **Tone:** Honest engineer talking to engineers. No hype. Admit what's alpha.
> **Best time to post:** Tuesday–Thursday, 8–10am PT (peak HN traffic)
>
> **How to handle "why not X?" comments:**
> - Ollama: "Ollama is a model runner. This adds a genetic selection layer on top."
> - LangChain: "Cloud-dependent, no evolution, different target (online SaaS vs. offline edge)."
> - "Is it production ready?": "No. v0.2.0 is alpha. Night School runs. TensorRT-LLM
>   full integration in progress. The demo works and the architecture is sound."
> - "SLERP is just lerp": "SLERP preserves the norm of the interpolated vector, which
>   matters when interpolating in LoRA weight space. Happy to discuss in thread."
> - "Fitness function seems naive": "Agreed. Currently cowboy approval rate. v0.3.0
>   will make it pluggable. What metrics would you want?"

---

## TITLE

```
Show HN: I built an AI that breeds better versions of itself overnight on a $499 Jetson
```

*[Alt titles if that one doesn't feel right on the day:]*
- `Show HN: SuperInstance – local AI agents that evolve via nightly LoRA breeding (Rust, 4.2 MB)`
- `Show HN: Self-evolving AI ranch – agents breed overnight, get better at your tasks (offline, $499 hw)`

---

## BODY

I was spending $240/month on AI APIs that knew nothing specific about my work. Every
month: same price, same generic model, same blank slate.

What I wanted: agents that improve at *my* tasks over time, run on hardware I own, cost
nothing after setup, work offline.

**GitHub:** https://github.com/SuperInstance/pasture-ai
**Demo (no hardware):** `superinstance --demo` or `make run -- --demo` after cloning

---

**The core idea — Night School:**

At 02:00 nightly, a scheduler evaluates every agent's performance for the day.
Underperformers get culled. High-fitness pairs "breed" — their LoRA adapter weights
are interpolated via SLERP (spherical linear interpolation preserves weight norms
better than linear interpolation for this purpose). Offspring run in quarantine
alongside parents. If they outperform, they promote to production. If not, culled
next cycle.

The agents get better at your specific tasks overnight without you touching anything
beyond setting a culling threshold. I called it Night School.

---

**breed.md — agent DNA as a hot-reloadable Markdown file:**

Agent personalities live in `breed.md` — plain Markdown with a trait weight table
and a system prompt. Edit the file, save, the agent reloads in ~180ms via inotify.
No recompile. No restart. You can git-diff your agent's personality evolution across weeks.

---

**The hardware target:**

NVIDIA Jetson Orin Nano 8GB (~$499). About 20 tok/s with Phi-3 Mini 4K in MAXN mode.
Graceful fallback: laptop CPU (Candle backend) → mock (demo mode, instant, no GPU needed).

---

**The binary:**

Everything — TUI dashboard, Axum web UI, inference engine, Night School scheduler,
Discord/Telegram connectors, breed.md watcher — ships as a single 4.2 MB Rust binary.
Probably too obsessive about this constraint, but: `scp` it anywhere and run it.

---

**Honest status (v0.2.0, alpha):**

- Night School: working
- breed.md hot-reload: working
- Demo mode: working
- TensorRT-LLM full integration: in progress (falls back to mock without CUDA)
- Channel connectors: Discord scaffolded, full integration v0.3.0

---

**Things I'd genuinely like feedback on:**

1. The fitness function is currently "cowboy approval rate" (did the human approve
   the output). Plans to make it pluggable in v0.3.0. What would you want to measure?

2. Is Markdown the right format for breed.md? Or would a structured format (TOML,
   JSON Schema, a real DSL) be better? The Markdown choice was intentional (human-readable,
   version-controllable, non-technical users can edit it) but I'm not married to it.

3. Seven species currently: Cattle (reasoning), Sheep (classification), Duck (API calls),
   Goat (navigation/debugging), Hog (hardware/GPIO), Chicken (monitoring), Horse (ETL).
   What's missing for your use case?

Stack: Rust, Tokio, Axum, ratatui, rusqlite, safetensors, serenity (Discord).
License: MIT.
