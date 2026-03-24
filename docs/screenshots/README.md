# Screenshots & Visual Assets

This directory contains visual assets for the SuperInstance documentation.

## Required Assets

### 1. TUI Dashboard (`dashboard.png`)
A screenshot of the terminal dashboard showing:
- Species panel with active agents
- Resource usage graphs
- Activity log
- Quick actions menu

### 2. Web Interface (`web-interface.png`)
A screenshot of the web dashboard showing:
- Agent management interface
- Breed editor
- Night School scheduler

### 3. Onboarding Wizard (`onboarding.png`)
A screenshot of the setup wizard showing:
- Feature selection screen
- Channel connector setup

### 4. Night School (`night-school.png`)
A screenshot or GIF of Night School running:
- Evaluation phase
- Breeding progress
- Promotions

## Recording Tips

For best results on Jetson:
```bash
# Record TUI session
asciinema rec dashboard.cast

# Convert to GIF
agg dashboard.cast dashboard.gif

# Or use terminalizer
terminalizer record -k dashboard
terminalizer render dashboard
```

## Current Placeholders

Until real screenshots are available, the README uses ASCII art representations.
