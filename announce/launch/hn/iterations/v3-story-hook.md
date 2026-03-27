# HN Launch Post — Iteration 3: Story Hook (Near Final)

> **STATUS: STRONG — Used as basis for FINAL.md with one structural change**
>
> **What we tried:** Lead with the problem in the first person, specific and concrete,
> before naming the solution. The reader should feel the problem before they see the
> product. Then: the "aha" fact (nightly breeding), then the "how?" explanation,
> then the benchmark, then the code.
>
> **What worked:** Internal testing showed this version held attention longer. The
> opening — "$240/month, knew nothing about my clients" — is specific enough to be
> credible (not "I was frustrated with AI") and relatable enough that many HN readers
> have thought the same thing. The "what if the agents could get better at your
> specific tasks overnight" question is genuinely novel and makes people want to know
> the answer.
>
> **The one structural change for FINAL:** Move the GitHub link + demo command to
> immediately after paragraph 1 (the hook), before any technical explanation.
> The title should drive the click. Once they're reading, give them the link before
> you explain anything — some readers will just clone it without reading the rest,
> and that's fine. Then continue with the explanation for those who keep reading.
>
> **What HN will say (prepared answers):**
> - "Why not just use Ollama?" — Ollama doesn't evolve. It's a model runner. This is
>   an agent ecosystem with a genetic selection layer.
> - "SLERP is just linear interpolation in disguise" — actually no, SLERP preserves
>   norm during interpolation which matters for LoRA weight spaces. But this is a
>   good thread to have.
> - "Is this production ready?" — No, v0.2.0 is alpha. Night School runs. Demo works.
>   TensorRT full integration still in progress. Be honest.
> - "What's the fitness function?" — Currently: cowboy approval rate (did the human
>   approve the output). Plans for automatic feedback loops in v0.3.0.

---

## Draft

**Title:** Show HN: I built an AI that breeds better versions of itself overnight on a $499 Jetson

**Body:**

I was spending $240/month on AI APIs that knew nothing specific about my work.
Every month, same price, same generic model, same blank slate.

What I wanted: AI agents that get better at *my* specific tasks over time, run on hardware
I own, cost nothing after setup, and work without internet.

So I built SuperInstance. Here's the part that took longest to get right:

At 02:00 every night, a scheduler evaluates every agent's performance for the day.
Underperformers get culled. High-fitness agents "breed" — their LoRA adapter weights
are merged via SLERP interpolation to produce offspring. The offspring run in quarantine
alongside the parents. If they outperform, they get promoted. If not, they're culled
in the next cycle.

I called it Night School. The agents get smarter at your tasks while you're asleep,
without you touching anything beyond setting a threshold.

**The other thing I'm proud of:** agent personalities live in `breed.md` files —
plain Markdown. Change the system prompt or trait weights, save the file, and the agent
reloads in under 200ms via an inotify watcher. No recompile. You can git-diff your
agent's personality across weeks.

**Hardware target:**
NVIDIA Jetson Orin Nano 8GB ($499). About 20 tok/s with Phi-3 Mini 4K in MAXN mode.
Falls back gracefully to laptop CPU (Candle backend) or mock (demo mode — no hardware needed).

**Binary:**
The whole system — TUI dashboard, web UI, inference engine, Night School scheduler,
Discord/Telegram connectors — is a single 4.2 MB Rust binary. I was probably too
obsessive about this constraint but I like that you can `scp` it to any machine and run it.

**Demo (no hardware needed):**
```
git clone https://github.com/SuperInstance/pasture-ai
cd pasture-ai && make run -- --demo
```

**What it is right now (v0.2.0, alpha):**
Night School works. breed.md hot-reload works. Demo mode works. TensorRT-LLM full
integration is still in progress — falls back to mock on hardware without CUDA.
Honest about where we are.

**What I'd love feedback on:**
- The fitness function is currently "did the human approve the output". Plans to
  make this automatic and pluggable in v0.3.0. What metrics would you want?
- The breed.md format — is Markdown the right DSL for this, or would a structured
  format (TOML? JSON Schema?) be better?
- Species taxonomy: 7 types currently (Cattle, Sheep, Duck, Goat, Hog, Chicken, Horse).
  What species is missing for your use case?

GitHub: https://github.com/SuperInstance/pasture-ai
