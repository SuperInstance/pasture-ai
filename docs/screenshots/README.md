# SuperInstance Screenshots & Visual Assets

This directory contains high-quality visual assets for the SuperInstance Ranch documentation.

## Files

| File | Description | Usage |
|------|-------------|-------|
| `tui-dashboard.svg` | Real-time TUI dashboard with live activity | Shows the terminal interface |
| `dioxus-onboarding.svg` | Dioxus web onboarding wizard | Shows the web setup flow |
| `geometric-breeding.svg` | Night School SLERP/TIES breeding | Visualizes the evolution pipeline |
| `memory-pasture-sync.svg` | Multi-Jetson CRDT sync | Shows distributed memory architecture |

## Static vs Animated

All files are **animated SVGs** that work in:
- GitHub README (animated)
- Browsers (animated)
- Print/export (degrades gracefully)

## Adding to README

```markdown
![TUI Dashboard](docs/screenshots/tui-dashboard.svg)
![Onboarding](docs/screenshots/dioxus-onboarding.svg)
![Breeding](docs/screenshots/geometric-breeding.svg)
![Memory Sync](docs/screenshots/memory-pasture-sync.svg)
```

## Creating New Assets

When adding new screenshots:
1. Use 800x500 canvas size for consistency
2. Use the project color palette:
   - Primary: `#06b6d4` (cyan)
   - Accent: `#4ade80` (green)
   - Warning: `#f59e0b` (amber)
   - Background: `#0f172a` to `#1e293b` gradient
3. Include animation where appropriate
4. Add caption at bottom

## Legacy Files

- `dashboard.txt`, `breeding.txt`, etc. are ASCII art placeholders
- `dashboard.svg`, `breeding.svg`, etc. are earlier versions
- New files (`tui-dashboard.svg`, etc.) are production-ready
