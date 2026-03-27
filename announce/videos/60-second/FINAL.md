# 60-Second Video — FINAL SCRIPT

> **Version:** v4 (from v3-story-arc with timing and pacing refinements)
> **Runtime:** 60 seconds
> **Format:** Terminal recording + text overlays. No voiceover. No music.
> **Audio:** Keyboard sounds, terminal output only.
>
> **To record:** Use `asciinema` or OBS with a clean terminal (Fira Code, 14pt,
> dark background). Record at 1920×1080. The TUI dashboard should be in a full-screen
> tmux pane.

---

## SHOT SEQUENCE

### [0:00–0:06] THE HOOK

*[Pure black screen. White monospace text appears word by word, each with a 0.4s pause:]*

```
You're paying rent
for someone else's AI.
```

*[Hold 0.5s. Then cut — hard — to terminal.]*

---

### [0:06–0:11] COLD BOOT

*[Terminal. Prompt visible.]*

```
$ superinstance --demo
```

*[0.8s of cursor blinking. Silence. Then:]*

*[The full TUI dashboard explodes onto screen — ASCII ranch header, live species
counter, VRAM bar, day counter. It feels alive.]*

*[Small text overlay fades in at bottom, 1.5s, then out:]*
```
4.2 MB binary  ·  no cloud  ·  your hardware
```

---

### [0:11–0:26] THE RANCH IN ACTION

*[Left 60% of screen: TUI dashboard, agents ticking.]*
*[Right 40%: a terminal pane. Email arrives:]*

```
📨  From: finance@acme.com
    "Q3 invoice — needs approval URGENT"

→ Border Collie routing...
→ email-cow-v1  [2ms]
→ Category: URGENT
  Draft: "Hi Sarah, confirming receipt..."
  Action: flag for human review
✓  47ms total
```

*[Brief pause. Then: cursor moves to breed.md in the editor pane. A line changes.
The user hits save.]*

```
[hot-reload]  email-cow-v1  ←  180ms
```

*[The agent indicator in the TUI pulses once, goes steady. The change is live.]*

---

### [0:26–0:38] NIGHT SCHOOL

*[Screen fades slowly to black over 1.5s. One word appears in the centre:]*

```
02:00
```

*[Hold 1s. Then the Night School log streams in, unhurried — each line appearing
with a 0.3s gap:]*

```
🌙  Night School  —  Day 2

    Evaluated   17 agents
    Culled        2   (fitness < 0.40)
    Bred          3   (SLERP merge)
    Promoted      2   (to production)

    avg fitness   0.81  →  0.86
```

*[Everything holds for 1.5s.]*

*[Then, quietly, centred below the log:]*
```
Your ranch got smarter while you slept.
```

---

### [0:38–0:48] MORNING

*[TUI returns. It's Day 2. The species list has a new entry:]*

```
  email-cow-v2   gen 2   fitness 0.91   ← new overnight
```

*[Dashboard: VRAM healthy. All agents active. A quiet steady-state.]*

---

### [0:48–0:54] THE PRICE

*[Fade to black. Terminal font. Two lines appear:]*

```
$499 hardware.
Once.
```

*[Hold 2s.]*

---

### [0:54–1:00] CTA

*[GitHub repository page — clean, star count visible.]*

*[One line in terminal font overlaid at bottom:]*
```
github.com/SuperInstance/pasture-ai
```

*[Terminal cursor blinks once.]*
*[Cut to black.]*

---

## PRODUCTION NOTES

- **Pace:** Resist the urge to speed up the Night School sequence. That 8-second pause
  at 02:00 is the most important moment in the video. Let it breathe.
- **The hot-reload:** This is the second most important moment. Make sure the 180ms
  timestamp is clearly legible. It should feel impossibly fast.
- **Colour:** Terminal should be dark (not pure black — something like #1a1b26 or
  Catppuccin Mocha). Text in #cdd6f4. The species counter and fitness numbers should
  pop in a warm colour (amber or soft orange).
- **The morning return:** After the Night School log, the TUI returning should feel
  like waking up. A slow fade-in, not a cut. The new agent in the list should have
  a subtle highlight or pulse.
- **Do not add a logo or watermark** during the video. Let the terminal be the brand.
  The only branding is the GitHub URL at the end.
