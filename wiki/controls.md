# Controls

Complete keybinding reference and UI guide for Solaris.

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `h` / `←` | Focus previous panel (two-column mode) |
| `l` / `→` | Focus next panel (two-column mode) |

### Actions

| Key | Action |
|-----|--------|
| `Enter` | Purchase selected item |
| `Tab` | Cycle buy amount (1x → 10x → Max → 1x) |
| `Space` | Manual mine |

### Panel Navigation

| Key | Action |
|-----|--------|
| `p` | Producers panel |
| `u` | Upgrades panel |
| `s` | Stats panel (single-column mode only) |
| `v` | Visualization panel |
| `a` | Ascension/Prestige panel |
| `x` | Achievements panel |

### General

| Key | Action |
|-----|--------|
| `?` | Toggle help screen |
| `` ` `` (backtick) | Toggle boss mode |
| `q` | Quit game |
| `Ctrl+C` | Quit game (alternative) |
| `Esc` | Close current overlay/panel |

---

## Layout Modes

Solaris automatically adapts to your terminal size.

### Single-Column Mode (< 130 columns wide)

In narrow terminals, panels are displayed one at a time:

```
┌─────────────────────────────────┐
│ Energy: 1,234 | +45.67/s        │
├─────────────────────────────────┤
│                                 │
│     [Current Panel Content]    │
│                                 │
│                                 │
└─────────────────────────────────┘
```

- Use `p`, `u`, `s`, `v` to switch between panels
- Only one panel visible at a time
- Stats panel (`s`) is available

### Two-Column Mode (≥ 130 columns wide)

In wider terminals, multiple panels are shown simultaneously:

```
┌────────────────────┬────────────────────┐
│ Energy: 1,234      │  +45.67/s          │
├────────────────────┼────────────────────┤
│                    │   [Visualization]  │
│   [Producers]      │                    │
│                    ├────────────────────┤
│                    │                    │
│                    │   [Upgrades]       │
│                    │                    │
└────────────────────┴────────────────────┘
```

- Producers panel on the left
- Visualization panel (top-right)
- Upgrades panel (bottom-right)
- Use `h`/`l` or `←`/`→` to switch focus between panels
- Press `p`, `u`, `v` to focus specific panels
- Stats panel not visible (shown in header instead)

---

## UI Panels

### Header Bar

Always visible at the top, showing key stats:

```
Energy: 1,234,567.89 | +12,345.67/s | Producers: 42 | Achievements: 85/260
```

- **Energy**: Current energy balance
- **E/s**: Production rate (energy per second)
- **Producers**: Total producers owned
- **Achievements**: Unlocked / Total

### Producers Panel

Lists all available producers:

```
┌─ Producers (p) ──────────────────────┐
│ ▤ Solar Panel (25)    2.50 E/s   150│
│►⛏ Mining Drone (10)   5.00 E/s   287│
│ ☄ Asteroid Mine (3)   6.00 E/s   742│
└──────────────────────────────────────┘
```

- **Icon**: Producer's unique symbol
- **Name (count)**: Producer name and owned quantity
- **E/s**: Current production from this type
- **Cost**: Price for next purchase
- **►**: Indicates currently selected item

**Colors**:
- Green: Can afford
- Yellow: Selected but can't afford

### Upgrades Panel

Lists available upgrades (those meeting requirements):

```
┌─ Upgrades (u) ────────────────────────┐
│►Better Silicon          100│
│  Solar Panels are twice as efficient │
│                                       │
│ Improved Thrusters      500│
│  Mining Drones are twice as efficient│
└───────────────────────────────────────┘
```

- Shows upgrade name and cost
- Description appears below selected upgrade
- Only shows upgrades you can currently unlock

### Visualization Panel

An animated solar system showing your cosmic empire:

```
┌─ Visualization (v) ───────────────────┐
│                    .                  │
│          *    .  *                    │
│       .     ☀     .    *             │
│          *    .  *                    │
│                    .                  │
└───────────────────────────────────────┘
```

- Central sun with orbiting elements
- Visual representation of your progress
- Purely decorative but satisfying!

### Stats Panel (Single-Column Only)

Detailed statistics about your game:

```
┌─ Stats (s) ────────────────────────────┐
│ Time Played: 2h 34m                    │
│ Total Clicks: 1,234                    │
│ Total Energy Earned: 5,678,901         │
│ Upgrades Purchased: 45                 │
│ Production Breakdown:                  │
│   Solar Panel: 25.00 E/s (20%)        │
│   Mining Drone: 100.00 E/s (80%)      │
└────────────────────────────────────────┘
```

### Ascension Panel (Overlay)

Press `a` to open:

```
┌─ Stellar Ascension ───────────────────┐
│                                       │
│ Stellar Chips: 42                     │
│ Chips on Ascension: +8                │
│                                       │
│ ─────────────────────────────────────│
│                                       │
│►Stellar Foundation (1 chip)           │
│  +5% all production permanently       │
│                                       │
│ Quick Start (3 chips)                 │
│  Start with 100 energy after ascension│
│                                       │
│ [Press Enter to purchase/ascend]      │
│ [Press 'a' or Esc to close]           │
└───────────────────────────────────────┘
```

- View chip balance and potential gains
- Browse prestige upgrades
- Purchase upgrades or perform ascension

### Achievements Panel (Overlay)

Press `x` to open:

```
┌─ Achievements ────────────────────────┐
│ [All] [Producers] [Progress] [Locked] │
│                                       │
│ ✓ Novice Solar Panel                  │
│   Own 1 Solar Panel                   │
│                                       │
│ ✓ Clicker                             │
│   Click 100 times                     │
│                                       │
│ ○ Click Enthusiast                    │
│   Click 1,000 times (234/1000)        │
│                                       │
│ [Press Tab to change filter]          │
│ [Press 'x' or Esc to close]           │
└───────────────────────────────────────┘
```

- Browse all achievements by category
- See progress toward locked achievements
- Filter by status (All/Unlocked/Locked)

### Help Screen (Overlay)

Press `?` to open:

```
┌─ Help ────────────────────────────────┐
│                                       │
│ Navigation:                           │
│   j/↓  Move down                      │
│   k/↑  Move up                        │
│   h/←  Focus left panel               │
│   l/→  Focus right panel              │
│                                       │
│ Actions:                              │
│   Enter   Purchase                    │
│   Tab     Cycle buy amount            │
│   Space   Manual mine                 │
│                                       │
│ Panels: p u s v a x                   │
│                                       │
│ [Press any key to close]              │
└───────────────────────────────────────┘
```

---

## Buy Amount

Press `Tab` to cycle through purchase amounts:

| Mode | Behavior |
|------|----------|
| **1x** | Buy one at a time |
| **10x** | Buy 10 at once |
| **Max** | Buy as many as affordable |

The current mode is shown in the UI. Max mode is especially useful for:
- Buying many of a cheap producer quickly
- Spending excess energy efficiently

---

## Boss Mode

Press `` ` `` (backtick) to toggle boss mode.

Boss mode disguises the game as a spreadsheet:

```
┌─────────────────────────────────────────────┐
│ Q4 Revenue Projections - CONFIDENTIAL       │
├──────────────────┬──────────────────────────┤
│ Revenue Stream   │ Projected    │ Actual    │
├──────────────────┼──────────────┼───────────┤
│ Solar Division   │ $1,234,567   │ $1,198... │
│ Mining Division  │ $5,678,901   │ $5,432... │
│ Orbital Ops      │ $9,012,345   │ $8,901... │
└──────────────────┴──────────────┴───────────┘
```

- Game continues running in the background
- Only backtick (`` ` ``) works to exit boss mode
- All other keys are ignored

---

## Context-Specific Controls

### In Ascension Panel

| Key | Action |
|-----|--------|
| `j` / `↓` | Navigate upgrades |
| `k` / `↑` | Navigate upgrades |
| `Enter` | Purchase selected upgrade |
| `a` / `Esc` | Close panel |

### In Achievements Panel

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll achievements |
| `k` / `↑` | Scroll achievements |
| `Tab` | Change filter category |
| `x` / `Esc` | Close panel |

### In Help Screen

| Key | Action |
|-----|--------|
| Any key | Close help |

### During Offline Report

| Key | Action |
|-----|--------|
| Any key | Dismiss report |

---

## Tips

1. **Use vim keys** (`h`, `j`, `k`, `l`) for efficient navigation
2. **Tab for bulk buying** - Switch to Max mode when you have excess energy
3. **Check upgrades often** - Press `u` to see available upgrades
4. **Boss mode is instant** - Backtick toggles immediately
5. **Resize your terminal** - Wider terminals show more information

---

## Related Pages

- [Getting Started](getting-started.md) - Basic tutorial
- [Mechanics](mechanics.md) - How the game works
