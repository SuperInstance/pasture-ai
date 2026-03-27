# HN Launch Post — Iteration 2: Technical Hook

> **STATUS: BETTER — But the hook is too narrow**
>
> **What we tried:** Lead with the most technically surprising thing: that you can define
> an AI agent's personality as a Markdown file that hot-reloads in 180ms. The hook is
> aimed at engineers who will immediately appreciate what that implies (no recompile,
> no Docker, data-driven personality, version-controllable).
>
> **What worked:** The reactions in internal testing: "wait, that's actually interesting."
> The breed.md concept is the most defensible technical novelty — there are other local
> AI projects but none with this exact abstraction. It gives HN readers something to
> argue about constructively (is Markdown the right DSL? could you use TOML? what about
> structured prompting libraries?). That's good. HN threads where people argue
> constructively about design decisions are the best outcomes.
>
> **What didn't work:** Leads with the mechanism (hot-reload), not the *reason you'd want
> the mechanism* (you want to iterate on your agent without infrastructure overhead).
> The emotional appeal is missing. Who is this for? Why would someone's life be better
> with this?
>
> **Also:** HN readers will ask "why not LangChain/LlamaIndex/OpenAgents?" and the
> post doesn't pre-empt this. The key answer: those require internet, those don't evolve,
> those cost money per call, those don't run on edge hardware. This answer needs to be
> in the post, briefly, so the thread doesn't start with "why not just use X?"
>
> **What to keep for v3:** The breed.md explanation. The SLERP evolution description.
> The benchmark numbers. The "why this exists" paragraph (the off-grid workshop use case).

---

## Draft

**Title:** Show HN: AI agent personalities as hot-reloadable Markdown files (Rust, runs offline)

**Body:**

I got frustrated with AI agent frameworks that require constant redeployment to change
agent behaviour, and cloud APIs that need internet access and charge per token forever.

So I built something different: SuperInstance.

The core idea is that agent personalities live in `breed.md` files — plain Markdown.
Change the file, save it, and the agent reloads within 200ms. No recompile. No restart.
No container rebuild. The watcher uses inotify and swaps the LoRA adapter weights live.

**Why this matters for iteration speed:**
Most agent frameworks treat the prompt/personality as code: change it, rebuild, redeploy,
test. breed.md makes personality a data file. You can git diff your agent's evolution.
You can roll back to yesterday's personality with `git checkout`. Non-technical users
can edit it.

**The evolution part:**
At 02:00 nightly, a scheduler (Night School) evaluates agent fitness, culls underperformers,
and breeds new agents from champions via SLERP LoRA weight interpolation. The metaphor
is intentional — it's a ranch. I found the farm framing made the system more intuitive
to explain and reason about.

**The hardware target:**
It's optimised for a $499 NVIDIA Jetson Orin Nano 8GB. ~20 tok/s with Phi-3 Mini in
MAXN mode. Falls back to Candle on laptop CPU. The whole thing — TUI, web dashboard,
inference, evolution, Discord/Telegram connectors — is a 4.2 MB Rust binary.

**Why not LangChain/OpenAgents/etc.:**
Those require cloud calls or significant Python infrastructure. This runs offline, on
edge hardware, with zero API costs after setup. Different target.

GitHub: https://github.com/SuperInstance/pasture-ai
Demo (no hardware needed): `superinstance --demo`

Happy to answer questions about the SLERP implementation, the breed.md parser, or
the binary size constraints.

---

> **Post-mortem:** This is a competent HN post. It will get upvotes. It won't go
> to the front page. The title is too abstract ("hot-reloadable Markdown files")
> for someone scrolling to stop on. The "why this exists" story is buried in paragraph 1.
> The benchmark number ("$499 hardware, 20 tok/s") is buried in paragraph 5.
> Those are the two most arresting facts. They need to be in the first sentence or the title.
