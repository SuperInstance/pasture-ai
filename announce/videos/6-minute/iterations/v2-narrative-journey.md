# 6-Minute Video — Iteration 2: Narrative Journey

> **STATUS: STRONG BUT ABSTRACT — Inform final, do not use as-is**
>
> **What we tried:** Lead with a character: "Meet Alex. Alex runs a small consulting firm.
> Alex was spending $240/month on AI APIs that knew nothing about Alex's clients, nothing
> about Alex's communication style, nothing about the way Alex's business works. Then Alex
> found SuperInstance." Story-driven. Follow Alex through setup, first breed, first Night
> School, one month later.
>
> **What worked surprisingly well:** The narrative structure makes every technical decision
> feel consequential. When "Alex" edits the breed.md to add client context to the system
> prompt, it's not a tutorial step — it's Alex *teaching* their AI about their business.
> When Night School runs and the fitness score improves, it's a milestone. The farmer
> metaphor lands naturally inside a story: Alex is the cowboy, the agents are livestock,
> Night School is literally tending the herd before dawn.
>
> **What didn't work:** "Alex" creates an abstraction layer between the viewer and the
> software. The person watching should *be* Alex, not watch Alex. Every moment of
> "Alex does X" pulls the viewer slightly out. We tried to fix this mid-draft by
> switching to second person ("You run a consulting firm") which was better but then
> felt presumptuous — not everyone watching runs a consulting firm.
>
> **The real insight from this version:** The narrative structure is right but the
> character is wrong. The character should be *the ranch itself*. Open with the ranch
> on Day 1. Follow it through Day 1 → Night 1 → Day 2 → ... Day 30. Show what changes.
> The viewer is the cowboy implied by the ranch's existence. This reframe becomes the
> basis for v3.
>
> **Specific lines to salvage for FINAL:**
> - "Your AI learned something about your business while you were in a meeting." (Day 2)
> - "The agent that processes invoices has processed 340 invoices. It is better at
>   invoices than you are." (Day 30 beat)
> - "You didn't write any code to make this happen. You wrote a Markdown file." (breed.md reveal)
> - The "livestock graze on tasks, not grass" metaphor explanation

---

## Script (selected scenes — full version was 8 min, too long)

### SCENE: THE BRIEF (0:00–0:45)

*[Warm terminal. The TUI dashboard. Day 1.]*

*[Narration — unhurried, no hype:]*

> "This is a ranch. Not metaphorically. That's what the software calls it. You're the
> cowboy. These — " *[species list lights up]* " — are your livestock.
>
> They don't eat. They don't sleep. But they *work*. And at 02:00 every night, while
> you're asleep, the weak ones get culled and the strong ones breed.
>
> Your job is to give them good DNA to start from. That's it.
>
> Let me show you how."

---

### SCENE: WRITING DNA (1:30–3:00)

*[Editor opens: `pasture/cattle/email-cow-v1/breed.md`]*

> "This is a breed file. It's a Markdown file. It describes everything your Cattle
> agent knows about how to behave.
>
> The genetic composition section — " *[highlights table]* " — this is where you
> assign weights to LoRA traits. Think of each trait as a personality dial. Turn up
> 'professional_tone' and turn down 'verbose_responses'.
>
> The system prompt at the bottom is where you talk to your agent directly. In plain
> English. Tell it who it is, what it cares about, how it talks.
>
> You didn't write any code to do this. You wrote a Markdown file.
>
> Save it." *[Ctrl+S]* "Watch the dashboard." *[agent pulses, 'hot-reload 180ms']*
>
> "180 milliseconds. It read your DNA change, reloaded, and it's different now."

---

### SCENE: THE MORNING AFTER (4:15–5:00)

*[Time-lapse style: TUI shows Day 1 → arrow → Day 2. New agent in the list.]*

> "Overnight, Night School ran.
>
> Your email-cow-v1 had been processing invoices all day. Its fitness score — which
> is just: did the cowboy approve its output? — was 0.78.
>
> One agent scored 0.43. Below the culling threshold. It's gone.
>
> The other two bred. Here's their offspring: email-cow-v2, generation 2, fitness 0.91.
>
> You didn't configure this. You set a threshold in .env and went to sleep.
>
> Your AI learned something about your business while you were in a meeting."

---

### SCENE: DAY 30 (5:15–5:55)

*[Dashboard: Day 30. Multiple generations. Fitness scores all above 0.85.]*

> "Day 30. You've made 12 edits to breed files. You've never touched the binary.
> You've never recompiled.
>
> The agent that handles your invoice emails has processed 340 of them. It's been
> culled twice and bred six offspring. The current production agent is generation 8.
>
> It is better at your invoices than you are. It understands your clients' names. It
> knows which ones need human escalation. It drafts in your voice.
>
> You taught it that. In a Markdown file. While it got better overnight on its own."

---

*Note: Full version clocked at 8:20. Too long. The consultancy use case was too specific.
The Day 30 beat should move earlier. Refactored in v3.*
