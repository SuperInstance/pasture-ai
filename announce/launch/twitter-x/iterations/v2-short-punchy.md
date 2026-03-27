# Twitter/X — Iteration 2: Short & Punchy

> **STATUS: CLOSE — Tone right, thread too short, needs one more technical tweet**
>
> **What we tried:** Cut to 5 tweets. Lead tweet is a single striking claim. No
> "excited to announce." Every tweet earns the scroll to the next.
>
> **What worked:** The lead tweet ("Your AI should get smarter every night. For free.
> On hardware you own. 🧵") gets attention without hype. Tweet 3 (the Night School log)
> is the best single piece of content in any of the drafts — a raw terminal dump that
> shows what happened overnight, with no explanation needed. It's the demo-in-a-tweet.
>
> **What to fix for FINAL:** Add one more tweet between the Night School log and the
> CTA — the breed.md concept needs its own tweet. It's the second most novel thing
> (after Night School) and it disappears in this version. Also: the CTA tweet should
> lead with the demo command (`superinstance --demo`), not the GitHub link. The demo
> is the lowest-friction path to the wow moment.
>
> **Runtime test:** Thread read-time ~45 seconds. Target is 30–60 seconds. Good.

---

## Thread

**Tweet 1/5 [hook]**
Your AI should get smarter at your tasks every night.
For free.
On hardware you own.
Here's how. 🧵

**Tweet 2/5 [what it is]**
SuperInstance is a self-evolving AI ranch.

Seven types of agents (Cattle 🐄 for reasoning, Duck 🦆 for API calls, Sheep 🐑
for classification, four more) managed by a Border Collie orchestrator.

4.2 MB Rust binary. Runs on a $499 Jetson. No cloud.

**Tweet 3/5 [Night School — the demo tweet]**
Every night at 02:00, this runs:

```
🌙  Night School — Day 14

    Culled:    3  (fitness < 0.40)
    Bred:      4  (SLERP merge)
    Promoted:  3

    avg fitness  0.79 → 0.87
```

Your agents got better at your tasks while you slept.
You changed nothing.

**Tweet 4/5 [breed.md]**
Agent personalities live in a Markdown file.

Edit the system prompt or trait weights. Save.
The agent reloads in 180ms — no recompile, no restart.

You can git-diff your AI's evolution across weeks.

**Tweet 5/5 [CTA]**
Try it now (no hardware needed):

```
superinstance --demo
```

Full ranch: https://github.com/SuperInstance/pasture-ai

$499 hardware. Once. Yours forever.
Breed responsibly. 🐕‍🦺
