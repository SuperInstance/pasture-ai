# 🌱 Quick Start: Hello Ranch in 5 Minutes

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                    🌱 HELLO RANCH - 5 MINUTE SETUP                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Prerequisites

- Jetson Orin Nano 8GB (or CUDA Linux box)
- Ubuntu 22.04+
- 10GB free disk space

## Step 1: Install (2 minutes)

```bash
curl -sSL https://install.superinstance.ai | bash
```

**What this does:**
- ✅ Detects hardware (Jetson vs CUDA Linux)
- ✅ Installs Rust + Bun + dependencies
- ✅ Downloads Phi-3 Mini base model (2.5GB)
- ✅ Builds optimized binaries
- ✅ Creates 16GB swap (critical for Jetson)

## Step 2: Start the Ranch (1 minute)

```bash
cd superinstance
make run
```

**You'll see:**

```
╔═══════════════════════════════════════════════════════════╗
║        SUPERINSTANCE RANCH - Day 1                        ║
╠═══════════════════════════════════════════════════════════╣
║                                                           ║
║  SPECIES PANEL              │  RESOURCE USAGE             ║
║  ┌─────────────────────┐    │  ┌─────────────────────┐    ║
║  │ 🐄 Cattle    x2     │    │  │ GPU Memory: 42%    │    ║
║  │ 🐑 Sheep     x5     │    │  │ 2.7GB / 6.5GB      │    ║
║  │ 🦆 Duck      x3     │    │  │ CPU: 28%            │    ║
║  │ 🐐 Goat      x2     │    │  └─────────────────────┘    ║
║  │ 🐗 Hog       x1     │    │                             ║
║  │ 🐔 Chicken   x3     │    │  💰 SAVINGS: $0.00         ║
║  │ 🐎 Horse     x1     │    │                             ║
║  └─────────────────────┘    │                             ║
║                              │                             ║
║  [D] Morning Routine         │  [Q] Quit                  ║
╚═══════════════════════════════════════════════════════════╝
```

## Step 3: Run the Morning Routine (2 minutes)

Press `D` in the dashboard, or run:

```bash
make morning-routine
```

**Watch the magic:**

```
🌅 THE MORNING ROUTINE - Demo

🐔 Step 1: Chicken detects motion event...
🐑 Step 2: Sheep flock triages emails...
  Flock consensus: 12 spam, 35 legitimate
🐐 Step 3: Goat navigates complex log file...
  Found 3 anomalies
🦆 Step 4: Duck flies to fetch calendar data...
  Calendar events: 5
🐄 Step 5: Cattle synthesizes the morning briefing...
  Briefing length: 450 chars

═══════════════════════════════════════════════════════════
  ✅ MORNING ROUTINE COMPLETE
  Time: 3.2s | Cloud calls: 0 | RAM: 5.4GB
  💰 Saved: $0.50 (vs cloud API)
═══════════════════════════════════════════════════════════
```

## 🎉 You Have a Ranch!

Your SuperInstance is now:

- ✅ Running locally on your Jetson
- ✅ Using <6.5GB VRAM
- ✅ Making zero cloud calls
- ✅ Saving money with every task
- ✅ Evolving every night at 02:00

## Next Steps

| Step | Command | Description |
|:-----|:--------|:------------|
| Edit an agent | `nano pasture/cattle/email-cow-v1/breed.md` | Customize behavior |
| Create new breed | `make breed` | Interactive wizard |
| Connect Discord | Edit `.env` | Add `DISCORD_BOT_TOKEN` |
| View logs | `make logs` | See activity |
| Run Night School | `make night-school` | Force evolution |

---

> *"5 minutes. Zero cloud. Infinite possibilities."*
