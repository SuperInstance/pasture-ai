# 60-Second Video — Iteration 2: Demo Focus

> **STATUS: CLOSE BUT COLD**
>
> **What we tried:** Kill the voiceover. Show a real terminal session. Let the software
> speak for itself. `superinstance --demo` cold boot, the TUI appearing, an email
> processed, the result. Pure show-don't-tell.
>
> **What worked:** This felt real. The terminal is beautiful. Watching the TUI boot in
> a real terminal, the species count ticking up, the email triage result appearing —
> this is genuinely impressive to a technical viewer. The silence works.
>
> **What didn't work:** Without any framing, a first-time viewer doesn't know what they're
> watching. "What is that dashboard? What does 'culled: 2, bred: 3' mean?" We're
> assuming context that first-time viewers don't have. The demo is compelling, but it's
> compelling the way watching someone else play a video game is compelling — you
> want to play it but don't know the rules yet.
>
> **Also:** The 60 seconds is too tight for the demo alone. We end up rushing the most
> impressive moment (Night School results) to fit the runtime. This content wants to
> be the *6-minute* video, not the 60-second.
>
> **What to keep:** The cold-boot TUI shot is the hero shot. Opens the final version too.
> The silence-plus-keystrokes audio approach is correct. "Culled: 2 / Bred: 3 / Promoted: 2"
> as on-screen text (no voiceover) is evocative when you understand what it means —
> seed this early so viewers understand by the time they see it.

---

## Script

*[No voiceover throughout. All audio is keyboard sounds, terminal output, system beeps.]*
*[All text is either terminal output or minimal on-screen labels.]*

**[0:00–0:03]**
```
$ superinstance --demo
```
*[Cursor blinks. Half a beat of silence.]*

**[0:03–0:08]**
*[TUI dashboard fills the screen. ASCII art ranch. Species counter climbs.]*
```
🐄 Ranch is open.  Day 1  |  3 species active  |  02:00 Night School
```

**[0:08–0:18]**
*[An email appears in the input panel]*
```
FROM: finance@acme.com
SUBJECT: Q3 invoice approval needed — URGENT

[→ email-cow-v1 routing...]
[→ URGENT: escalate · draft response · flag for human]
[✓ processed in 47ms]
```

**[0:18–0:28]**
*[Split screen: left = terminal editor with breed.md open. Right = TUI dashboard.]*
*[User types a change to the system prompt in breed.md. Saves.]*
```
[hot-reload] email-cow-v1 updated  ← 180ms
```
*[Right side: agent indicator pulses, then steady again.]*

**[0:28–0:40]**
*[Screen goes dark. Text fades in:]*
```
02:00
```
*[Then the Night School log streams:]*
```
🌙 Night School — Day 2
  Evaluated: 17 agents
  Culled:     2  (fitness < 0.4)
  Bred:       3  (SLERP merge)
  Promoted:   2  (to production)
  avg fitness: 0.81 → 0.86
```

**[0:40–0:48]**
*[Morning. TUI back up. New agent appears in the species list.]*
```
+ email-cow-v2  [gen 2]  fitness: 0.91  ← new
```

**[0:48–0:56]**
*[Terminal. Plain text fades in:]*
```
4.2 MB binary.   $499 hardware.   Yours forever.
```

**[0:56–1:00]**
```
github.com/SuperInstance/pasture-ai
```
