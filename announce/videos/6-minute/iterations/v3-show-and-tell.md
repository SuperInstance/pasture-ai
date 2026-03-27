# 6-Minute Video — Iteration 3: Show & Tell (Near Final)

> **STATUS: BASIS FOR FINAL — Needs timing polish and one cut**
>
> **What we built:** The narrative structure from v2, stripped of the character
> abstraction, combined with the live demo rigour from v2-demo-focus of the 60s video.
> We follow the *ranch* — not Alex, not "you" — from Day 1 through Day 30. The viewer
> is implicitly the cowboy. We show every command. Nothing is described without being shown.
>
> **What works:** This version has the right emotional arc. The "you wrote a Markdown file"
> beat lands harder here because we've earned it with the live edit. The Day 30 finish is
> genuinely moving — watching fitness scores improve over time, seeing generation numbers
> climb, is a form of satisfaction that no other local AI project can offer right now.
>
> **What to cut for FINAL:**
> - The onboarding wizard section (1:30–2:15) is good but too slow. Cut to 30 seconds.
>   The wizard is available; we don't need to show every screen.
> - The channel connectors section (5:15–5:45) should be cut entirely. It's feature
>   enumeration and breaks the arc. Replace with one line: "When you're ready, connect
>   it to Discord, Slack, or your email. That's in .env." Done.
> - The "going further than the makers thought possible" promise in the brief needs
>   a concrete moment. Currently missing. Add: editing the SpeciesType to create a
>   totally novel 8th species that doesn't exist in the codebase. A 20-second shot
>   of someone adding a new variant to the enum and writing a breed.md for it. That's
>   the moment the video earns its "go further" claim.
>
> **Runtime after cuts:** 6:10. Within target.
>
> **The new section to add:** "Beyond the Ranch" (from cut channel section slot).
> Show: adding a new species type. The point: the makers built 7 species but the
> system supports arbitrarily many. Your domain might need a species that doesn't
> exist yet. Here's how to create one. 30 seconds. This delivers the "take it further
> than the makers thought" promise in the brief.

---

## Full Script (with inline production notes)

*[No voiceover for first 20 seconds — terminal only, same opening as 60s video but
the boot-up gets more time to breathe.]*

---

### CHAPTER 1: WHAT YOU'RE LOOKING AT (0:00–1:00)

*[TUI dashboard, freshly booted. Day 1.]*

*[Voice — calm, deliberate, not a hype voice:]*

> "This is a ranch.
>
> That's not a metaphor for something else — that's what the system calls itself.
> You're the cowboy. Those " *[gesture to species list]* " are your livestock.
> Not models. Not agents in the AI-product sense. Livestock. They work, they compete,
> the weak ones get culled, and every night the strong ones breed.
>
> The border collie " *[highlight Collie section in TUI]* " is the foreman. She
> decides which animal handles which task. You don't manage routing. She does.
>
> And the whole thing — TUI, web dashboard, inference engine, evolution scheduler,
> channel connectors — lives in a 4.2 megabyte binary.
>
> Let me show you how to use it."

*[Production: slow pan across the TUI. Let it be present. This is the landscape
the viewer will spend time in. Don't rush it.]*

---

### CHAPTER 2: YOUR FIRST BREED (1:00–2:30)

*[Terminal: `ls pasture/cattle/`]*
*[Opens email-cow-v1/breed.md in editor (Neovim preferred for aesthetics)]*

> "Everything your agent knows about how to behave lives in this file.
> This is its DNA. We call it a breed.
>
> Three sections."

*[Highlight each in turn:]*

> "**Lineage** — where it came from. Day 1 this is empty. By week 4 this will have
> a genealogy.
>
> **Genetic composition** — " *[highlight the weight table]* " — trait names and
> weights. Each trait is a pre-trained LoRA adapter. You're not training anything.
> You're blending personalities that already exist.
>
> **Genetic code** — the system prompt. This is where you talk to it. Plain English.
> Tell it who it is."

*[Demonstrates: changes one weight in the composition table from 0.8 to 0.6. Saves.]*

> "Watch the dashboard."

*[Dashboard: agent pulses. 'hot-reload email-cow-v1 ← 173ms']*

> "173 milliseconds. It read the change, reloaded the adapter weights, and it's
> different now. No recompile. No restart. No Docker container rebuild.
>
> You wrote a Markdown file."

*[Pause 1.5s. Let that land.]*

---

### CHAPTER 3: MAKE YOUR OWN (2:30–3:30)

*[Terminal: `make breed`]*
*[Interactive onboarding wizard appears — name, species type, first trait weights]*

> "You don't have to start from the template. `make breed` opens the onboarding wizard.
> Name it. Pick a species type. The wizard writes the initial breed.md.
>
> Let's make one for processing Slack mentions — a Duck, since Duck is the species
> for API and network tasks."

*[Fast: name it 'slack-duck-v1'. Pick Duck. Set a few weights. Wizard generates the file.]*

> "Done. 45 seconds. It hot-loaded already — " *[points to dashboard, new agent appears]*
> " — and it's in production."

---

### CHAPTER 4: NIGHT SCHOOL (3:30–4:45)

*[Fade to black. "02:00" appears. Same as 60s video but given more time.]*

> "At 02:00, Night School runs.
>
> Here's what happens:"

*[Night School log streams:]*
```
🌙 Night School — Day 2

  Evaluated:  19 agents across 4 species
  Culled:      3  (fitness < 0.40)
  Bred:        4  (SLERP merge — champion pairs)
  Quarantine:  2  (new offspring, not yet in production)
  Promoted:    2  (passed quarantine threshold)

  avg fitness  0.78 → 0.83
```

> "Culled means: deleted. Three agents whose fitness dropped below 40% are gone.
>
> Bred means: two high-fitness parents were identified, their LoRA adapter weights
> were interpolated together using SLERP — spherical linear interpolation. The offspring
> inherits characteristics from both parents.
>
> Quarantine means: we don't put untested offspring directly into production. They
> run alongside production agents, their outputs are compared, and if they score higher
> they get promoted.
>
> You didn't configure any of this beyond setting a threshold in .env.
> You just had to go to sleep."

*[TUI returns — Day 2. New agents visible. Slightly higher fitness scores across the board.]*

---

### CHAPTER 5: BEYOND THE RANCH (4:45–5:30)

*[Split screen: left = species/mod.rs open in editor, right = a breed.md for a new species]*

> "The makers built 7 species. But the system supports however many you need.
>
> Let's say you're building home automation. None of the 7 species fits perfectly.
> You want something between a Hog — which handles hardware — and a Duck — which
> handles network calls. Let's call it a Starling.
>
> Add it to the SpeciesType enum. Write a breed.md. Define what traits it has access to.
> That's it. The border collie will route to it. Night School will evolve it.
>
> The makers gave you 7. The system is built to grow. Where you take it is up to you."

*[Breed.md for 'home-starling-v1' visible. A few sensor-reading and webhook traits.]*

---

### CHAPTER 6: 30 DAYS FROM NOW (5:30–6:10)

*[Dashboard: Day 30. Multiple agents, all gen 4–8. Fitness 0.85–0.94.]*

> "This is a ranch at Day 30.
>
> The highest-fitness agent has been culled and reborn six times. It processes your
> tasks in 23 milliseconds. It understands your patterns. It has never phoned home.
>
> You made 17 edits to breed files. You never touched the binary.
>
> The hardware cost $499. You own it. It's not a subscription.
> It doesn't know what ChatGPT is.
>
> It's yours."

*[Fade to black. Terminal cursor blinks.]*

```
github.com/SuperInstance/pasture-ai
```

*[Cut to black.]*

---

## PRODUCTION NOTES

- **Chapter 4 is the heart of the video.** Do not rush the Night School sequence.
  Let the log stream at real speed — the viewer needs time to read and understand each line.
- **The "you wrote a Markdown file" beat** in Chapter 2 should have a half-second
  of silence before the next sentence. It's the thesis of the entire product.
- **Chapter 5 (new species)** should feel achievable, not technical. We're not asking
  the viewer to understand LoRA. We're showing them they can add their own species in
  20 lines of Rust + one Markdown file. Keep the code on screen briefly — just long
  enough to show it exists and is short.
- **The Day 30 sequence** should be filmed with a script that sets `day_counter` to 30
  and populates the stud book with generated data so the dashboard looks real.
  Do not fake the terminal output with static text.
- **Runtime target:** 6:00–6:30. Not 7 minutes. Not 5:30. The 6-minute promise in
  the title card should be accurate.
