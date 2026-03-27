# 6-Minute Video — Iteration 1: Tutorial Format

> **STATUS: REJECTED**
>
> **What we tried:** Traditional developer tutorial video. Introduction → installation
> → first run → writing a breed → running tests → outro. Step-by-step. Screenshot every
> command. Describe what each file does.
>
> **Why it failed:** This is documentation, not inspiration. Someone who just watched
> the 60-second video and felt excited is now watching a screen-recording of `cargo build`
> for 40 seconds. The emotional temperature drops from 80°F to 40°F in the first minute.
>
> **The deeper problem:** Tutorial format puts the *tool* at the centre of the story.
> We want the *viewer* at the centre. "Here's what YOU will be able to do. Here's what
> YOUR ranch will look like at 02:01 on day 30." Tutorial says "here's how the software
> works." We need to say "here's what your life looks like when you have this."
>
> **What to keep:** The `make breed` interactive onboarding wizard is genuinely great
> and should appear in the final video. The explanation of SLERP as "interpolating
> between two personalities" rather than "spherical linear interpolation of adapter
> weight tensors" is the right level of abstraction. Keep that.
>
> **What this version got completely wrong:** Spending 90 seconds on installation.
> If you got here from the 60-second video you already know you want to install it.
> Either skip installation or do it in 15 seconds with `curl | bash` and `make run`.

---

## Script (abbreviated — this version wasn't taken to full polish)

**[0:00–0:15] Intro**
"Hey, welcome back. If you just watched the short video and you're ready to dig in,
this is the video for you. I'm going to show you how to install SuperInstance, write
your first breed, and understand how Night School actually works under the hood."

**[0:15–1:45] Installation**
*[Screen share: clone repo, cd into it, `make install`]*
*[Watches progress. Explains each step: Rust toolchain, cargo build, binary size check.]*
*[Runs `superinstance --demo`. TUI appears.]*
"Okay, so we're running. You can see the dashboard here..."
*[Spends 45 seconds describing TUI elements]*

**[1:45–3:00] Your First Breed**
*[Opens pasture/cattle/template/breed.md. Walks through each section.]*
"The genetic composition section is where you specify which LoRA traits this agent
has access to and their weights. Think of it like a recipe..."
*[Copies template, renames folder, edits system prompt]*
*[Runs `make run`, watches hot-reload]*

**[3:00–4:30] Night School Explained**
*[Shows night_school.rs in editor]*
"At the code level, Night School is a Tokio cron task that fires at 02:00..."
*[Explains SLERP, breeding config, culling threshold]*
"The culling threshold is 0.4 by default, meaning agents with fitness below 40% are
removed. You can change this in .env..."

**[4:30–5:30] Channel Connectors**
*[Shows .env.example Discord section]*
"If you want your ranch to connect to Discord, you just paste in your bot token..."
*[Navigates to channels/ source directory]*

**[5:30–6:00] Outro**
"And that's the basics! Check the README for more, star the repo if you found this
useful, and I'll see you in the next one."

---

*Note: This version was not recorded. Rejected at script stage.*
