# Product Hunt Launch

> **Post when:** Same day as HN post, or the following Tuesday (PH's peak traffic day)
> **Strategy:** PH rewards visual content. Link the 60-second video as the first media
> upload. The tagline must be ≤60 characters.

---

## Tagline (60 char max)

```
AI agents that breed better versions of themselves overnight
```
*(60 chars exactly — do not trim)*

---

## Product Description

SuperInstance is a self-evolving AI ranch that runs entirely on hardware you own.

Every night at 02:00, your agents evolve: weak ones are culled, strong ones breed
via SLERP LoRA weight interpolation. You wake up to a smarter ranch — no training,
no API calls, no subscription.

**The three things that make it different:**

→ **breed.md** — Agent personality is a Markdown file. Edit it, save, the agent
reloads in 180ms. Version control your AI's evolution. Non-technical users can
iterate without code changes.

→ **Night School** — Nightly genetic selection. Day 30's email agent has survived
6+ generations of culling and breeding. It's been naturally selected for your tasks.

→ **4.2 MB single binary** — The entire system (TUI dashboard, web UI, inference
engine, evolution scheduler, Discord/Telegram connectors) ships as one Rust binary.
`scp` it anywhere. No Docker. No Python. No Node.js.

**Hardware:** NVIDIA Jetson Orin Nano 8GB (~$499). Falls back gracefully to any
CUDA GPU, laptop CPU, or demo mode (no hardware needed).

**Try it now:**
```
superinstance --demo
```

---

## First Comment (post immediately after launch)

> Happy to answer any questions about the architecture.
>
> The most common question: why breed.md in Markdown rather than TOML or JSON?
> Answer: we wanted agent personalities to be readable and editable by non-engineers.
> A farmer doesn't write config files. A farmer writes about their livestock.
>
> The second most common: why SLERP for LoRA merging?
> SLERP (spherical linear interpolation) preserves the norm of the interpolated vector,
> which matters when the weights live on a curved manifold. Linear interpolation
> (LERP) shrinks the norm and produces weaker merged adapters in practice.
>
> v0.2.0 is alpha. Night School works. TensorRT-LLM full integration in progress.
> Happy to share the roadmap.

---

## Maker Links
- GitHub: https://github.com/SuperInstance/pasture-ai
- Demo video: [link to 60-second video]
- Full walkthrough: [link to 6-minute video]
