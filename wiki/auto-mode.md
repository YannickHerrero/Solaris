# Auto Mode

Solaris includes a built-in auto-play bot that can play the game for you. It mimics human behavior by navigating the UI naturally — focusing panels, moving the cursor step by step, and purchasing items just like a player would.

## Launching Auto Mode

```
solaris --auto
```

The game launches normally with the bot active. You'll see `[AUTO]` in the header bar and a magenta border to indicate auto mode is running.

---

## How the Bot Works

### Human-Like Behavior

The bot doesn't bypass the UI. It interacts with the game the same way a human would:

1. **Thinks** before acting (1-3 second pause)
2. **Focuses** the correct panel (Producers or Upgrades)
3. **Moves the cursor** one step at a time (j/k) toward the target item
4. **Presses Enter** to purchase
5. **Rests** briefly after buying (0.5-1 second)
6. **Repeats** with a fresh evaluation

All delays are slightly randomized to look natural.

### Manual Mining

During idle phases (thinking, resting, waiting for funds), the bot spams manual mine at 10 clicks per second. This is especially impactful in the early game when click power is a significant source of income.

The bot does **not** mine during active phases (panel switching, cursor movement, purchasing) to maintain realistic behavior.

---

## Decision Engine

The bot uses an optimized decision engine that evaluates every possible action on a unified ROI scale. It picks the single best action available at any given moment.

### Priority Order

1. **Unlock new producers** — Always buy the first unit of a newly available producer to unlock the next tier. This opens up better producers and their upgrades.

2. **Best ROI action** — Score every affordable producer and upgrade by simulated ROI, then pick the one with the lowest ROI (fastest payback).

3. **Look-ahead** — Before buying, check if saving for a not-yet-affordable option would be more efficient than spending now.

4. **Wait for funds** — If nothing is affordable, idle and mine while energy accumulates.

### ROI Calculation

The bot computes ROI as:

```
ROI = cost / E/s_gain
```

Where `E/s_gain` is the actual production increase from the purchase. This is calculated by **simulating** the purchase on a cloned game state and measuring the difference in `total_energy_per_second()` before and after.

This simulation approach is what makes the bot optimal — it automatically accounts for all complex mechanics:

| Mechanic | How it's captured |
|----------|-------------------|
| Thousand Rays | Buying a non-panel producer increases Solar Panel output — the simulation sees this |
| Drone Network | Buying Mining Drones boosts other buildings — captured by the E/s delta |
| Synergies | Adjacent producer bonuses are reflected in the simulated total E/s |
| Global multipliers | Upgrade effects like +5% all production are measured directly |
| Producer multipliers | A 2x upgrade on your top producer shows as a massive E/s gain |
| EpsPerBuilding / EpsPerTotalBuildings | Building count bonuses included in the simulation |
| Cosmic Cat / Stellar Essence | Upgrades that multiply via essence are captured in the E/s delta |

### Unified Scoring

Producers and upgrades are scored on the **same ROI scale**. The bot doesn't arbitrarily prefer one category over the other — it picks whichever single option has the lowest ROI globally.

For example, if a 2x producer upgrade costs 10,000 energy and would double your top producer's output (adding 500 E/s), its ROI is 20 seconds. If the best producer costs 5,000 energy and adds 50 E/s, its ROI is 100 seconds. The bot buys the upgrade.

### Upgrades with No Direct E/s Impact

Some upgrades don't affect E/s directly:
- **ManualMultiplier** — doubles click power
- **ClickEpsPercent** — adds % of E/s to clicks

These receive a small synthetic score (0.1% of current E/s) so they're eventually purchased but never prioritized over real E/s gains.

### Look-Ahead (Saving for Better Purchases)

After identifying the best affordable option, the bot scans not-yet-affordable candidates. If it finds one where:

- The **time to save** for it (at current E/s) is less than **80%** of the best affordable option's ROI
- AND the future candidate has a **better ROI**

...then the bot waits instead of spending. This prevents wasting energy on a mediocre producer when a game-changing upgrade is almost affordable.

### Producer Unlocking

The bot always buys the first unit of any visible producer that it doesn't yet own, as long as it can afford it. This is highest priority because:

- Unlocking a new producer reveals the next one in the chain
- New producers come with their own upgrade tree
- The upgrades often have better ROI than buying more of existing producers

---

## Interacting During Auto Mode

You can still interact with the game while the bot is running. The user always has priority.

### Pause on Input

Any keypress **pauses the bot for 5 seconds**. During the pause:

- The header shows `[AUTO PAUSED]` with a yellow border
- You have full control to navigate, buy, check stats, etc.
- The bot does nothing — no cursor movement, no purchases, no mining
- After 5 seconds of inactivity, the bot resumes with a fresh evaluation

This means you can:
- Open the prestige panel (`a`) to check chips without the bot closing it
- Browse achievements (`x`) freely
- Manually buy something the bot hasn't prioritized
- Look at stats (`s`) to see progress

### Quitting

Quit normally with `q` or `Ctrl+C`. The game saves on exit as usual.

### What the Bot Never Does

- **Never ascends** — The bot will never open the prestige panel or trigger ascension. Ascension is always a manual decision.
- **Never buys prestige upgrades** — Only regular producers and upgrades are purchased.
- **Never opens overlays** — It actively closes any overlays (prestige, achievements, help) if they're somehow open when the bot resumes.

---

## Bot Behavior by Game Phase

### Early Game (0 - 1,000 E/s)

- Spams manual mine heavily (10 clicks/sec) — this is the main income source
- Buys Solar Panels and Mining Drones as soon as affordable
- Snaps up early upgrades like Enhanced Photovoltaics (2x panels + clicks)
- Quickly unlocks Asteroid Mines, Orbital Stations, etc.

### Mid Game (1,000 - 1M E/s)

- Prioritizes high-impact upgrades (2x multipliers on top producers)
- Buys the first unit of each new producer as it becomes affordable
- Synergy and Drone Network upgrades get high scores due to their E/s impact
- Manual mining becomes less significant but still contributes

### Late Game (1M+ E/s)

- Global upgrades and Cosmic Cat upgrades become top picks
- Producer purchases slow down as costs grow exponentially
- Look-ahead logic becomes important — the bot saves for expensive game-changers
- Milestone and research upgrades are bought as E/s thresholds are met

---

## Technical Details

### State Machine

The bot operates as a state machine with 7 states:

```
Idle ──> Deciding ──> FocusingPanel ──> MovingCursor ──> Purchasing ──> CooldownAfterPurchase
  ^                                                                           |
  |                                                                           v
  └──────────────────────────── WaitingForFunds <─────────────────────────────┘
```

| State | Duration | Mines? | Description |
|-------|----------|--------|-------------|
| Idle | 1-3s | Yes | Initial thinking pause |
| Deciding | Instant | No | Evaluates all options, picks best |
| FocusingPanel | 300-500ms | No | Switches to Producers or Upgrades panel |
| MovingCursor | 100-200ms per step | No | Moves selection j/k toward target |
| Purchasing | 200-400ms | No | Presses Enter |
| CooldownAfterPurchase | 500ms-1s | Yes | Brief rest after buying |
| WaitingForFunds | 2-5s | Yes | Nothing affordable, wait and retry |

### Performance

The decision engine clones the `GameState` to simulate each candidate purchase. With ~20 producers and ~474 upgrades (of which only a subset are available at any time), this is lightweight — typically evaluating 10-30 candidates per decision cycle, running at most once every few seconds.

### Configuration Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `SAVE_THRESHOLD_MULTIPLIER` | 0.8 | Save for future purchase if time-to-save < 80% of best ROI |
| `PAUSE_TICKS` | 50 | Pause duration on user input (5 seconds) |

---

## Tips

- **Let it run overnight** — Combined with offline progress, the bot makes steady progress even during sleep
- **Check in periodically** — Press any key to pause and review the bot's progress
- **Ascend manually** — When the bot stalls (nothing worth buying), consider ascending for prestige bonuses
- **Combine with prestige** — After manually ascending and buying prestige upgrades, restart with `--auto` for a boosted run

---

## Related Pages

- [Controls](controls.md) - Keybinding reference
- [Strategy Guide](strategy.md) - Manual optimization tips
- [Mechanics](mechanics.md) - Game formulas and math
- [Getting Started](getting-started.md) - First steps for new players
