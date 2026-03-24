# 🌅 The Morning Routine: Your Daily AI Workflow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                    🌅 THE MORNING ROUTINE                                    │
│                   Your AI-Powered Start to the Day                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Overview

The Morning Routine is SuperInstance's signature demo - a fully automated morning briefing that runs locally in under 5 seconds with zero cloud calls.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MORNING ROUTINE FLOW                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   06:00 ──► 🐔 Chicken detects motion                                      │
│                 │                                                           │
│                 ▼                                                           │
│   06:01 ──► 🐑 Sheep flock triages emails                                  │
│                 │                                                           │
│                 ▼                                                           │
│   06:02 ──► 🐐 Goat navigates log files                                    │
│                 │                                                           │
│                 ▼                                                           │
│   06:03 ──► 🦆 Duck fetches calendar data                                  │
│                 │                                                           │
│                 ▼                                                           │
│   06:04 ──► 🐄 Cattle synthesizes briefing                                 │
│                 │                                                           │
│                 ▼                                                           │
│   06:05 ──► 📋 Your morning briefing is ready!                             │
│                                                                              │
│   Total: 3-5 seconds | Cloud: 0 calls | Cost: $0                           │
│   Equivalent cloud cost: $0.50                                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Step-by-Step Breakdown

### Step 1: Chicken Detects Motion (Event Trigger)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🐔 WATCHDOG CHICKEN                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Type: Chicken (Monitor)                                                    │
│  VRAM: 5MB                                                                  │
│  Latency: 50ms                                                              │
│                                                                              │
│  What it does:                                                              │
│  • Watches for motion events (camera, sensor, or keyboard)                  │
│  • Triggers the Morning Routine at first detection                          │
│  • Logs activity to pasture/activity.log                                    │
│                                                                              │
│  Implementation:                                                            │
│  pasture/chickens/watchdog-chicken-v1/breed.md                             │
│                                                                              │
│  📤 Output: "Motion detected at 06:00. Starting Morning Routine..."        │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Step 2: Sheep Triage Emails (Ensemble Voting)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🐑 EMAIL SHEEP FLOCK                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Type: Sheep (Classifier)                                                  │
│  Count: 5 agents                                                           │
│  VRAM: 50MB each (250MB total)                                             │
│  Latency: 500ms for 50 emails                                              │
│                                                                              │
│  What they do:                                                              │
│  • Each sheep classifies emails independently                               │
│  • Categories: URGENT, IMPORTANT, NEWSLETTER, SPAM, LOW                    │
│  • Consensus reached when 3+ sheep agree                                    │
│                                                                              │
│  Herding Strategy: "The Wear"                                               │
│  ─────────────────────────────                                              │
│  The Collie applies gentle pressure, letting the flock                     │
│  reach natural consensus. No single point of failure.                      │
│                                                                              │
│  📤 Output:                                                                 │
│  • URGENT: 2 (from: boss@company.com, investor@vc.com)                    │
│  • IMPORTANT: 12                                                            │
│  • NEWSLETTER: 8                                                            │
│  • SPAM: 15 (auto-archived)                                                │
│  • LOW: 13                                                                  │
│                                                                              │
│  💰 Saved: $0.10 vs cloud classification                                   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Step 3: Goat Navigates Log Files

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🐐 DEBUG GOAT                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Type: Goat (Navigator)                                                    │
│  VRAM: 150MB                                                               │
│  Latency: 200ms                                                            │
│                                                                              │
│  What it does:                                                              │
│  • Climbs into /var/log/ directory                                         │
│  • Identifies anomalies in syslog, auth.log, kern.log                      │
│  • Extracts relevant error patterns                                        │
│                                                                              │
│  Herding Strategy: "Balance"                                                │
│  ────────────────────────────                                               │
│  Monitor depth, high agility. The Goat can navigate                        │
│  complex hierarchies without getting lost.                                 │
│                                                                              │
│  📤 Output:                                                                 │
│  • 3 failed SSH attempts from 192.168.1.100                               │
│  • 1 disk I/O warning on /dev/sda1                                        │
│  • All other systems nominal                                               │
│                                                                              │
│  💰 Saved: $0.05 vs cloud log analysis                                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Step 4: Duck Fetches Calendar Data

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🦆 CALENDAR DUCK                                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Type: Duck (Network)                                                      │
│  VRAM: 100MB                                                               │
│  Latency: 800ms (network dependent)                                        │
│                                                                              │
│  What it does:                                                              │
│  • Connects to Google Calendar API                                         │
│  • Fetches today's events                                                  │
│  • Notes conflicts and important meetings                                  │
│                                                                              │
│  Herding Strategy: "Whistle Stop"                                           │
│  ───────────────────────────────                                            │
│  Fire-and-recall. The Duck flies out, grabs data, returns.                │
│  No persistent connection needed.                                          │
│                                                                              │
│  📤 Output:                                                                 │
│  • 09:00 - Team Standup                                                    │
│  • 11:00 - 1:1 with Manager                                                │
│  • 14:00 - Product Review                                                  │
│  • 3 available slots for deep work                                         │
│                                                                              │
│  💰 Saved: $0.10 vs cloud API orchestration                               │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Step 5: Cattle Synthesizes Briefing

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🐄 BRIEFING CATTLE                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Type: Cattle (Heavy LLM)                                                  │
│  VRAM: 500MB                                                               │
│  Latency: 1-2s for synthesis                                              │
│                                                                              │
│  What it does:                                                              │
│  • Consumes outputs from all previous agents                               │
│  • Synthesizes into coherent morning briefing                              │
│  • Prioritizes action items                                                │
│                                                                              │
│  Herding Strategy: "Strong Eye"                                             │
│  ────────────────────────────                                               │
│  Lock GPU, block, apply steady pressure. The Cattle works                 │
│  deeply on synthesis while the Collie waits patiently.                    │
│                                                                              │
│  📤 Output:                                                                 │
│                                                                              │
│  ══════════════════════════════════════════════════════════════════       │
│  🌅 MORNING BRIEFING - January 15, 2025                                    │
│  ══════════════════════════════════════════════════════════════════       │
│                                                                              │
│  📧 EMAILS REQUIRING ATTENTION (2 URGENT)                                  │
│  • Boss: "Q4 numbers attached" - respond by 10 AM                          │
│  • Investor: "Follow-up from yesterday" - schedule call                    │
│                                                                              │
│  📅 TODAY'S SCHEDULE                                                        │
│  • 09:00 - Standup (15 min)                                                │
│  • 11:00 - 1:1 with Manager (prep: Q4 numbers)                            │
│  • 14:00 - Product Review                                                  │
│                                                                              │
│  🔍 SYSTEM NOTES                                                            │
│  • 3 failed SSH attempts detected - check security                         │
│  • Deep work windows: 10:00-11:00, 15:00-17:00                            │
│                                                                              │
│  ⚡ TOP 3 ACTIONS                                                            │
│  1. Respond to boss email with Q4 analysis                                │
│  2. Schedule investor call                                                 │
│  3. Review security logs                                                   │
│  ══════════════════════════════════════════════════════════════════       │
│                                                                              │
│  💰 Saved: $0.25 vs cloud synthesis                                       │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Total Savings Per Morning

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    DAILY COST COMPARISON                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  SERVICE              CLOUD COST      SUPERINSTANCE                        │
│  ─────────────────────────────────────────────────────────────────────     │
│  Email Classification   $0.10          $0.00                               │
│  Log Analysis           $0.05          $0.00                               │
│  Calendar API           $0.10          $0.00                               │
│  Synthesis LLM          $0.25          $0.00                               │
│  ─────────────────────────────────────────────────────────────────────     │
│  TOTAL                  $0.50          $0.00                               │
│                                                                              │
│  MONTHLY SAVINGS: $15.00                                                    │
│  YEARLY SAVINGS: $182.50                                                    │
│                                                                              │
│  Note: This is ONE routine. Your Ranch does dozens of these daily.          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Customizing Your Morning Routine

Edit the breeding files to customize:

```bash
# Customize email classification
nano pasture/sheep/email-sheep-v1/breed.md

# Add new calendar sources
nano pasture/ducks/calendar-duck-v1/breed.md

# Change briefing style
nano pasture/cattle/briefing-cow-v1/breed.md
```

### Example: Add Slack Integration

```markdown
# 🦆 Breed: Slack-Duck-v1

## 🧬 Genetic Code
```
You are a Slack message fetcher.
Fetch messages from channels:
- #general
- #engineering
- #alerts

Summarize unread messages by priority.
```

## 🛠️ Tool Access
- [x] `slack_api`
```

Then add to Morning Routine:

```bash
# Edit the routine
nano pasture/routines/morning-routine.md

# Add Slack Duck before Cattle
```

---

## Running the Routine

### Automatic (Daily at 06:00)

The routine runs automatically if your Ranch is running.

### Manual Trigger

```bash
# Via CLI
make morning-routine

# Via Dashboard
Press 'D' key

# Via API
curl -X POST http://localhost:3000/api/routine/morning
```

---

> *"Every morning, your Ranch saves you 30 minutes and $0.50. That compounds."*
