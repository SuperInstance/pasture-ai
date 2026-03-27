# 6-Minute Video — FINAL SCRIPT

> **Version:** v4 (from v3-show-and-tell with pacing cuts and "Beyond the Ranch" refined)
> **Runtime:** 6:05–6:20 target
> **Format:** Terminal recording + narration. No music except optional ambient
>   at very low volume during Night School sequence.
> **Chapters:** 6 clearly titled, viewer can seek to any chapter
>
> **Chapter titles for YouTube:**
> - 0:00 — What you're looking at
> - 1:00 — Your first breed (editing DNA)
> - 2:30 — Making your own from scratch
> - 3:30 — Night School: how evolution works
> - 4:45 — Going further than the makers did
> - 5:30 — Your ranch at Day 30

---

## CHAPTER 1: WHAT YOU'RE LOOKING AT (0:00–1:00)

*[TUI boots. Voice enters after 8 seconds of silent boot — same clip as 60s video.]*

> "This is a ranch.
>
> Not a metaphor. That's the name of the system — the Ranch. You're the cowboy.
> Those are your livestock. They work, they compete, the weak get culled, the strong
> breed. Every night at 02:00.
>
> The border collie in the middle is the foreman. She decides which animal handles
> which task. You don't manage routing.
>
> The whole thing — this dashboard, the inference engine, the evolution scheduler,
> channel connectors for Discord and Slack — is a 4.2 megabyte binary. One file.
> No Node.js. No Python. No Docker.
>
> Let me show you how to use it. And then how to take it further than we planned."

---

## CHAPTER 2: YOUR FIRST BREED (1:00–2:30)

*[Terminal: `ls pasture/cattle/` — shows email-cow-v1 directory.]*
*[Opens breed.md in Neovim.]*

> "Every agent's personality lives in this file.
> This is its DNA. We call it a breed.
>
> Three things matter:"

*[Highlights each:]*

> "**Genetic composition** — trait names and weights. Each trait is a pre-trained LoRA
> adapter on disk. You're not training. You're blending.
>
> **Genetic code** — the system prompt. Plain English. Tell it who it is, how it talks,
> what it cares about.
>
> **Lineage** — where it came from. Right now this says 'generation 1'. After Night School
> this will have parents. After a few weeks it will have a genealogy."

*[Changes: raises 'professional_tone' weight from 0.6 to 0.9. Saves.]*

> "Watch the dashboard."

*[Agent pulses. 'hot-reload email-cow-v1 ← 158ms']*

> "158 milliseconds. It read the change, swapped the weights, and it's different now.
> No recompile. No restart.
>
> You wrote a Markdown file."

*[1.5s pause.]*

---

## CHAPTER 3: MAKING YOUR OWN (2:30–3:15)

*[Terminal: `make breed`]*
*[Onboarding wizard. Fast: name, species type, two or three traits. Exit.]*
*[Dashboard: new agent appears immediately.]*

> "`make breed` writes the breed file for you interactively. Name it, pick a species
> type — Duck for API work, Cattle for reasoning, Sheep for classification — and set
> initial trait weights.
>
> 40 seconds and your new agent is in production.
>
> Now just use it. Every task it handles is a data point. Night School will use that
> data tonight."

---

## CHAPTER 4: NIGHT SCHOOL (3:15–4:30)

*[Slow fade to black. '02:00' appears centre screen. Hold 2s.]*

*[Night School log streams — unhurried:]*

```
🌙  Night School — Day 4

    Evaluated   23 agents  (4 species)
    Culled        3  (fitness < 0.40)
    Bred          5  (SLERP merge)
    Quarantined   5  (awaiting production trial)
    Promoted      3  (passed trial — into production)

    avg fitness   0.79  →  0.85
```

> "Three agents deleted. They scored below 40% on the cowboy's approval rate.
>
> Five new agents bred. SLERP — spherical linear interpolation — takes the weight
> tensors of two high-fitness parents and finds a point between them. The offspring
> inherits both parents' traits, weighted by their fitness scores.
>
> Five new offspring go into quarantine. They run in parallel with production agents.
> Their outputs are compared. If they score higher, they get promoted. If not, they're
> culled in the next cycle.
>
> You configured one number: the culling threshold. 0.4. Everything else was automatic."

*[TUI returns — Day 4. New agents visible. Fitness scores slightly higher.]*

> "This is what compound improvement looks like.
> Day 1 average fitness: 0.71.
> Day 4: 0.85.
> Day 30 — I'll show you that in a moment."

---

## CHAPTER 5: BEYOND THE RANCH (4:30–5:20)

*[Split screen: left = species/mod.rs (the SpeciesType enum), right = new breed.md]*

> "We built 7 species. They cover most cases.
>
> But let's say you're building agricultural monitoring. You need an agent that reads
> soil sensors and triggers irrigation webhooks. None of the 7 fits perfectly.
>
> Add a new variant to the SpeciesType enum." *[types: `Sparrow,`]* "20 characters.
>
> Write a breed.md. Define what traits it uses. Add it to the pasture directory.
>
> The border collie will route to it automatically — her routing logic works on
> species type, and she's never met a Sparrow before, but she'll figure it out.
> Night School will evolve it the same as any other species.
>
> We designed 7. The architecture supports as many as you need.
> The most interesting uses of this system are ones we haven't thought of."

*[The new Sparrow agent appears in the dashboard species list.]*

---

## CHAPTER 6: DAY 30 (5:20–6:10)

*[Dashboard: Day 30. 4 species. Agents at gen 5–9. Fitness 0.85–0.94.]*

> "Day 30.
>
> The invoice-handling agent is on generation 8. It's been culled and reborn twice —
> the lineage field in its breed.md has a full genealogy now. It processes invoices
> in 19 milliseconds. It's better at your invoices than you are.
>
> 18 edits to breed files over 30 days. Zero changes to the binary.
> Zero API calls to any external service. Zero dollars after setup.
>
> The hardware cost $499. You own it. It's running in the room with you.
> It learned your patterns and nobody else has access to what it learned.
>
> That's what a ranch is."

*[Fade to black. Cursor blinks.]*

```
github.com/SuperInstance/pasture-ai
star  ·  clone  ·  breed
```

---

## PRODUCTION NOTES

- **Chapter titles** should appear as full-screen title cards for 1.5 seconds before
  each chapter begins. This makes the video navigable on YouTube and signals structure.
- **The Night School sequence** in Chapter 4 is the most technically dense section.
  Practise the narration to make sure the pace matches the log streaming. Do not rush.
  The viewer needs to read each line.
- **Chapter 5 should feel like an invitation**, not a technical tutorial. The enum edit
  should be on screen for 3 seconds max. The point is: "this is possible and it's easy",
  not "here's how to do it in detail."
- **The Day 30 dashboard**: populate with realistic data using the CLI or a seed script
  before recording. Real numbers, real genealogy in breed files. Do not use fabricated
  terminal output.
- **Do not end with a "like and subscribe" ask.** The GitHub URL is sufficient. Let the
  work speak. If viewers want to support the project, they'll star it.
