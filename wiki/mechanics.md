# Game Mechanics

This page documents all formulas and calculations used in Solaris.

## Game Loop

Solaris runs at **10 ticks per second**. Each tick:
1. Energy is generated based on production rate
2. Energy is added to balance and lifetime total
3. Achievements are checked (once per second)

---

## Cost Formulas

### Single Producer Cost

The cost of the next producer increases by **15%** (1.15x) with each owned:

```
cost(n) = base_cost × 1.15^owned
```

Where:
- `base_cost` = Producer's base cost (see [Producers](producers.md))
- `owned` = Current number of that producer owned

### Bulk Purchase Cost

When buying multiple producers, the total cost uses a geometric series:

```
total_cost = base_cost × 1.15^owned × (1.15^quantity - 1) / (1.15 - 1)
```

Simplified:
```
total_cost = base_cost × 1.15^owned × (1.15^quantity - 1) / 0.15
```

### Example Calculations

**Solar Panel (base_cost = 15):**

| Owned | Next Cost | Cost of 10 |
|-------|-----------|------------|
| 0 | 15.00 | 304.14 |
| 10 | 60.68 | 1,230.35 |
| 50 | 16,085.60 | 326,092.06 |
| 100 | 17,275,092.92 | 350,181,992.75 |

### Upgrade Cost

Upgrade costs scale by **5x** per tier:

```
upgrade_cost = base_upgrade_cost × 5^tier
```

### Upgrade Cost Reduction

Prestige upgrades can reduce upgrade costs:

```
final_cost = upgrade_cost × (1 - reduction)
```

Where `reduction` is capped at **50%** (0.50).

---

## Production Formulas

### Basic Production

Each producer generates energy per second:

```
producer_eps = base_eps × count × producer_multiplier × global_multiplier
```

### Total Energy Per Second

Sum of all producers:

```
total_eps = Σ (base_eps_i × count_i × producer_mult_i × global_mult)
```

### Producer Multiplier

Each producer's multiplier comes from:

1. **Producer Upgrades**: Each tier gives 2x
   ```
   upgrade_mult = 2^(tiers_purchased)
   ```

2. **Synergy Bonus**: From adjacent producer synergies
   ```
   synergy_mult = 1 + (0.05 × source_producer_count)
   ```

Combined:
```
producer_multiplier = upgrade_mult × synergy_mult
```

### Global Multiplier

The global multiplier affects all producers:

```
global_multiplier = upgrade_global_mult × achievement_mult × prestige_mult
```

Where:

1. **Global Upgrades**: Multiplicative stacking of all purchased global upgrades
   ```
   upgrade_global_mult = Π (individual_global_multipliers)
   ```
   Maximum possible: 1.05 × 1.10 × 1.15 × 1.25 × 1.35 × 1.50 × 1.75 × 2.00 × 2.50 × 3.00 = **68.67x**

2. **Achievement Bonus**:
   ```
   achievement_mult = 1.01^(achievement_count)
   ```
   At 260 achievements: **13.20x**

3. **Prestige Multiplier**: From production-boosting prestige upgrades
   ```
   prestige_mult = base_mult × ascension_mult × achievement_prestige_mult
   ```

### Prestige Production Multiplier

```
base_mult = Π (prestige_production_multipliers)
```
- Stellar Foundation: 1.05x
- Stellar Efficiency: 1.25x
- Cosmic Mastery: 2.00x
- Infinite Power: 6.00x
- Galactic Domination: 11.00x
- **Maximum: 173.25x**

```
ascension_mult = 1 + (bonus_per_ascension × total_ascensions)
```
- Cosmic Legacy: +1% per ascension
- Legacy of Legends: +3% per ascension
- Combined: +4% per ascension

```
achievement_prestige_mult = 1 + (0.01 × achievement_count)
```
- Dimensional Echo: +1% per achievement
- At 260 achievements: 3.6x

---

## Manual Mining

### Energy Per Click

```
click_energy = (click_power × manual_multiplier) + (0.05 × total_eps)
```

Where:
- `click_power` = 1 (base)
- `manual_multiplier` = 2^(manual_upgrades_purchased)
- `total_eps` = current total energy per second

### Manual Multiplier Cap

With all 10 manual upgrades: **1,024x** click power

---

## Synergy System

### Synergy Bonus

When a synergy upgrade is purchased:

```
target_bonus = 1.0 + (0.05 × source_count)
```

### Example

With 100 Mining Drones and Drone-Mine Synergy:
- Asteroid Mine bonus = 1.0 + (0.05 × 100) = **6.0x**

This multiplies with the Asteroid Mine's producer multiplier.

---

## Achievement Bonus

### Formula

```
achievement_bonus = 1.01^n
```

Where `n` = number of achievements unlocked.

### Bonus Table

| Achievements | Multiplier |
|--------------|------------|
| 10 | 1.105x |
| 25 | 1.282x |
| 50 | 1.645x |
| 75 | 2.109x |
| 100 | 2.705x |
| 125 | 3.467x |
| 150 | 4.432x |
| 175 | 5.668x |
| 200 | 7.244x |
| 225 | 9.257x |
| 250 | 11.81x |
| 260 | 13.20x |

---

## Prestige System

### Stellar Chips Formula

```
chips = floor(sqrt(total_energy_earned) / 1,000,000)
```

### Energy Required for Chips

| Chips | Total Energy Required |
|-------|----------------------|
| 1 | 10^12 (1 trillion) |
| 2 | 4 × 10^12 |
| 5 | 25 × 10^12 |
| 10 | 10^14 (100 trillion) |
| 100 | 10^16 |
| 1,000 | 10^18 (1 quintillion) |

### Chip Bonus Multiplier

```
chip_multiplier = Π (chip_bonus_upgrades)
```
- Eternal Progression: 1.05x
- Stellar Accumulation: 1.15x
- Combined: **1.2075x**

### Net Chips Per Ascension

```
net_chips = (floor(sqrt(total_energy) / 1,000,000) × chip_multiplier) - previously_earned_chips
```

---

## Offline Progress

### Calculation

When you return to the game after being away:

```
offline_energy = energy_per_tick × ticks_offline × offline_multiplier
```

Where:
- `energy_per_tick` = total_eps / 10
- `ticks_offline` = seconds_offline × 10
- `offline_multiplier` = prestige offline bonus

### Caps and Limits

- **Maximum offline time**: 8 hours (28,800 seconds)
- **Minimum to show report**: 60 seconds

### Offline Multiplier

```
offline_multiplier = Π (offline_bonus_upgrades)
```
- Persistent Memory: 1.10x
- Eternal Offline: 1.50x
- Combined: **1.65x**

---

## Double Energy Chance

### Lucky Stars Effect

With Lucky Stars (+1%) and Enhanced Luck (+5%):

```
double_chance = 0.01 + 0.05 = 0.06 (6%)
```

Each tick has this chance to generate double energy.

### Effective Bonus

Average production increase = double_chance × 100%
- At 6% chance: **+6% average production**

---

## Ascension Mechanics

### Energy Retention

With Transcendent Memory (1%) and Perfect Memory (5%):

```
retained_energy = current_energy × retention_percent
```

Combined retention: 6% (capped at **10%**)

### Starting Energy

From Quick Start prestige upgrade:

```
starting_energy = prestige_starting_bonus + retained_energy
```

- Quick Start: 100 energy

### Producer Unlock Level

```
unlock_level = max(prestige_unlock_upgrades)
```

- Seasoned Explorer: 2
- Accelerated Start: 3
- Universal Knowledge: 5
- Rapid Expansion: 7
- Universal Unlock: 10

---

## Time Calculations

### Time Played

```
time_played_seconds = ticks_played / 10
```

### Time to Afford

Approximate time to afford a producer:

```
time_to_afford = (cost - current_energy) / total_eps
```

(Assumes constant production rate)

---

## Efficiency Calculations

### Producer Efficiency

To compare producers, calculate energy per cost:

```
efficiency = eps_gained / cost
```

### Payback Time

Time for a producer to "pay for itself":

```
payback_time = cost / (base_eps × multipliers)
```

### Rule of Thumb

- If payback time < 10 minutes: Buy it
- If an upgrade doubles production: Buy if cost < 15 minutes of production

---

## Number Formatting

The game uses these suffixes for large numbers:

| Suffix | Value | Full Name |
|--------|-------|-----------|
| K | 10^3 | Thousand |
| M | 10^6 | Million |
| B | 10^9 | Billion |
| T | 10^12 | Trillion |
| Q | 10^15 | Quadrillion |
| Qi | 10^18 | Quintillion |
| Sx | 10^21 | Sextillion |
| Sp | 10^24 | Septillion |

---

## Related Pages

- [Producers](producers.md) - Base stats for all producers
- [Upgrades](upgrades.md) - Upgrade costs and effects
- [Prestige](prestige.md) - Prestige upgrade details
- [Strategy](strategy.md) - Applying these formulas
