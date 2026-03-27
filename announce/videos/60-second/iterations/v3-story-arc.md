# 60-Second Video — Iteration 3: Story Arc

> **STATUS: NEARLY FINAL — Used as basis for FINAL.md**
>
> **What we tried:** Give the demo 10 seconds of framing that names the enemy and the
> prize before any code appears. "You're paying $X/month to rent a brain. We built
> something you own." Then the demo. Then the overnight moment. Then the price.
> Emotional arc: frustration → surprise → desire → relief.
>
> **What worked:** The enemy framing ("paying rent for a brain") immediately distinguishes
> this from every other local AI project, which lead with technical specs. Viewers
> who have ever looked at their OpenAI bill felt it. The demo lands harder because now
> the viewer is emotionally primed. The "02:00" night school sequence hits differently
> when the viewer already understands what they're watching.
>
> **What to refine for FINAL:**
> - The opening framing line is 2 seconds too long. Trim to one punchy sentence.
> - The email demo is the weakest part — abstract enough that some viewers won't
>   clock what happened. Either speed it up to 3 seconds (so it's about the *speed*,
>   not the content) or replace with something more visceral (Slack message? code review?).
>   Keeping email for now since Cattle/email is the shipped demo feature.
> - The "yours forever" ending is correct but the transition needs a half-beat of black
>   between the hardware shot and the text — it's currently rushed.
> - Night School timing: the "02:00" fade-to-black + log sequence should be 8 seconds
>   not 6. That moment is the most magical. Give it room.
>
> **Decision:** This structure becomes FINAL.md with those refinements applied.

---

## Script

*[Audio: keyboard sounds, terminal output. No music. No voiceover.]*

**[0:00–0:05] OPENING FRAME**
*[Black screen. White text appears, one word at a time:]*
```
You're renting
someone else's
brain.
```

**[0:05–0:08] PIVOT**
*[Text disappears. Terminal appears. Cursor blinks.]*

**[0:08–0:13] COLD BOOT**
```
$ superinstance --demo
```
*[Half a beat. Then the TUI erupts onto screen — ASCII ranch, live agent stats.]*

*[On-screen label fades in then out:]*
```
4.2 MB  ·  zero dependencies  ·  your hardware
```

**[0:13–0:25] THE DEMO**
*[Email arrives in the input pane. Border Collie routes it. Cattle processes it.]*
```
📨 invoice approval — URGENT
→ email-cow-v1  [routing: 2ms]
→ URGENT  ·  draft prepared  ·  flag: human review
✓ 47ms
```

*[Cursor moves to breed.md. A line of the system prompt changes. Save.]*
```
[hot-reload] email-cow-v1  ← 180ms
```

**[0:25–0:35] NIGHT SCHOOL**
*[Screen fades to dark. Soft clock sound.]*
```
02:00
```
*[Night School log streams, unhurried:]*
```
🌙  Evaluated 17  ·  Culled 2  ·  Bred 3  ·  Promoted 2
    avg fitness  0.81 → 0.86
```
*[Tiny pause.]*
```
Your ranch evolved while you slept.
```

**[0:35–0:45] MORNING**
*[TUI back. New agent in the species list — gen 2, fitness 0.91.]*
*[Dashboard shows: Day 2. All green.]*

**[0:45–0:52] THE PRICE**
*[Terminal. Plain text:]*
```
$499 hardware.
Once.
```

**[0:52–1:00] CTA**
*[GitHub page. Star count. One line:]*
```
github.com/SuperInstance/pasture-ai
```
*[Terminal cursor blinks once. Cut to black.]*
