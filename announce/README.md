# SuperInstance v0.2.0 — Launch Materials

This directory contains all workshopped launch content for the v0.2.0 release.
Each piece went through multiple iterations with annotations explaining what we
tried, what worked, what didn't, and why we made the final decisions.

---

## Videos

| File | Status | Runtime | Purpose |
|---|---|---|---|
| `videos/CREATIVE-BRIEF.md` | Reference | — | Shared direction, tone, shots |
| `videos/60-second/FINAL.md` | **Ready to record** | ~60s | Hook — gets viewers to click the 6-minute |
| `videos/6-minute/FINAL.md` | **Ready to record** | ~6:10 | Deep-dive — turns interested viewers into cloners |

**Iteration history:**
- `videos/60-second/iterations/v1-feature-dump.md` — Rejected: too much, no soul
- `videos/60-second/iterations/v2-demo-focus.md` — Close but cold: great demo, no framing
- `videos/60-second/iterations/v3-story-arc.md` — Near-final: became basis for FINAL
- `videos/6-minute/iterations/v1-tutorial-format.md` — Rejected: documentation, not inspiration
- `videos/6-minute/iterations/v2-narrative-journey.md` — Strong: narrative right, character wrong
- `videos/6-minute/iterations/v3-show-and-tell.md` — Near-final: basis for FINAL

---

## Launch Posts

| Platform | File | Status |
|---|---|---|
| **Hacker News** | `launch/hn/FINAL.md` | **Ready** |
| **Twitter/X** | `launch/twitter-x/FINAL.md` | **Ready** |
| **Reddit r/rust** | `launch/reddit/FINAL.md` | **Ready** (post day 1) |
| **Reddit r/LocalLLaMA** | `launch/reddit/FINAL.md` | **Ready** (post day 7) |
| **Reddit r/selfhosted** | `launch/reddit/FINAL.md` | **Ready** (post day 14) |
| **Product Hunt** | `launch/other/product-hunt.md` | **Ready** |
| **dev.to** | `launch/other/dev-to.md` | **Ready** (post week 2) |

**Iteration history:**
- `launch/hn/iterations/v1-feature-list.md` — Rejected: feature enumeration, no "why"
- `launch/hn/iterations/v2-technical-hook.md` — Better: wrong lede, right content
- `launch/hn/iterations/v3-story-hook.md` — Near-final: basis for FINAL
- `launch/twitter-x/iterations/v1-original.md` — Original v1.0-era draft, superseded
- `launch/twitter-x/iterations/v2-short-punchy.md` — Near-final, missing breed.md tweet
- `launch/reddit/iterations/v1-draft.md` — First drafts for each subreddit

---

## Launch Sequence

**Day 0 (launch day — Tuesday–Thursday, post 9am ET):**
1. Push videos to YouTube (60s first, then 6-minute)
2. Post HN (`launch/hn/FINAL.md`)
3. Post Twitter/X thread (`launch/twitter-x/FINAL.md`)
4. Post Product Hunt (`launch/other/product-hunt.md`)

**Day 7:**
5. Post r/LocalLLaMA (`launch/reddit/FINAL.md` — LocalLLaMA section)
6. Post r/rust (`launch/reddit/FINAL.md` — rust section)

**Day 14:**
7. Post r/selfhosted (`launch/reddit/FINAL.md` — selfhosted section)
8. Publish dev.to article (`launch/other/dev-to.md`)

---

## Key Decisions Log

**Why "ranch" metaphor throughout:** Made the system more intuitive to explain.
"Livestock" maps cleanly to agents. "Culling" and "breeding" are exact descriptions
of what Night School does. Non-technical users understood it faster than technical
terminology.

**Why no music in 60-second video:** Silence + terminal sounds makes it feel real,
not like a product ad. Music signals "we're trying to sell you something."

**Why the Night School log is the centrepiece of every video and post:** It's the
most concrete, legible proof that the system works and is different. A raw terminal
log is more trustworthy than any claim.

**Why HN post asks three questions at the end:** HN threads where the maker invites
technical discussion perform better than announcement-only posts. The three questions
(fitness function, breed.md format, missing species) are all genuinely open — we
don't have the final answers to any of them.

**Why dev.to article explains SLERP in detail:** The dev.to audience rewards depth.
The SLERP explanation with actual code is the kind of content that gets bookmarked
and shared weeks after the launch.
