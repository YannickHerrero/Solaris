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

All producers use a flat **1.15x cost multiplier**:

```
cost(n) = base_cost × 1.15^owned
```

Where:
- `base_cost` = Producer's base cost (see [Producers](producers.md))
- `owned` = Current number of that producer owned

### Bulk Purchase Cost

When buying multiple producers, the total cost uses a geometric series:

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

Upgrade costs scale by **10x** per tier:

```
upgrade_cost = base_upgrade_cost × 10^tier
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

1. **Producer Upgrades**: All tiers give 2x multiplier
   ```
   upgrade_mult = 2^(upgrades_purchased)
   ```
   Maximum: **32,768x** from all 15 tiers

2. **Synergy Bonus**: From adjacent producer synergies (capped at 2.5x)
   ```
   synergy_mult = min(2.5, 1 + (0.02 × source_producer_count))
   ```

3. **Drone Network Bonus** (Mining Drone only): +1% per 50 drones per network type
   ```
   drone_network_mult = 1 + (0.01 × (drone_count / 50) × network_types_owned)
   ```

Combined:
```
producer_multiplier = upgrade_mult × synergy_mult × special_bonuses
```

### Global Multiplier

The global multiplier affects all producers:

```
global_multiplier = upgrade_global_mult × achievement_mult × prestige_mult × cosmic_cat_mult × misc_mult
```

Where:

1. **Global Upgrades**: Multiplicative stacking of all purchased global upgrades
   ```
   upgrade_global_mult = Π (individual_global_multipliers)
   ```
   Maximum possible: 1.05 × 1.10 × 1.15 × 1.25 × 1.35 × 1.50 × 1.75 × 2.00 × 2.50 × 3.00 = **68.67x**

2. **Achievement Bonus**:
   ```
   achievement_mult = 1.005^(achievement_count)
   ```
   At 260 achievements: **3.67x**

3. **Cosmic Cat Multiplier**: Based on Stellar Essence
   ```
   cosmic_cat_mult = (1 + stellar_essence × 0.05)^(cosmic_cat_upgrades_owned)
   ```

4. **Miscellaneous Multipliers**: Milestone, Research, Per-Building E/s, etc.

---

## Special Mechanics

### Thousand Rays (Solar Panel)

Solar Panels have a unique upgrade system that scales with your total non-panel buildings:

**Base Calculation:**
```
thousand_rays_base = 0.1 × non_panel_building_count
```

**With Multipliers:**
```
thousand_rays_bonus = thousand_rays_base × multiplier_chain
```

Multiplier chain:
- Million Rays: 5x
- Billion Rays: 10x
- Tiers 7-15: 20x each (9 upgrades)

**Total with all upgrades:**
```
multiplier_chain = 5 × 10 × 20^9 = 2.56 × 10^13
```

This bonus is added to both Solar Panel E/s and manual mining per click.

### Drone Network (Mining Drone)

Mining Drones gain synergies from all building types through the Drone Network system:

**Network Types (18 total):**
Each network upgrade provides:
1. 2x multiplier to Mining Drone E/s
2. +1% E/s per 50 drones for that network type

**Workforce Upgrades (18 total):**
Each workforce upgrade gives the target building +1% E/s per 50 Mining Drones.

**Example with 200 Drones and 10 network types:**
```
drone_network_bonus = 2^10 × (1 + 0.01 × 200/50 × 10) = 1024 × 1.40 = 1,433.6x
```

### Stellar Essence & Cosmic Cats

**Stellar Essence** is a resource earned through achievements:
```
stellar_essence = 0.04 × achievement_count
```

At 260 achievements: 10.4 Stellar Essence

**Cosmic Cat Bonus:**
```
single_cat_mult = 1 + (stellar_essence × 0.05 × essence_multiplier)
total_cat_mult = single_cat_mult^(cosmic_cat_upgrades_owned)
```

**Example with 260 achievements, all 15 cats, and all 5 essence upgrades:**
```
essence_multiplier = 1.05^5 = 1.276
single_cat_mult = 1 + (10.4 × 0.05 × 1.276) = 1.663
total_cat_mult = 1.663^15 = 5,847x
```

---

## Manual Mining

### Energy Per Click

```
click_energy = (click_power × manual_multiplier × panel_multiplier) + (click_eps_percent × total_eps) + thousand_rays_bonus
```

Where:
- `click_power` = 1 (base)
- `manual_multiplier` = 2^(manual_upgrades_purchased)
- `panel_multiplier` = 2^(first_3_solar_upgrades) (from Thousand Rays tiers 1-3)
- `click_eps_percent` = 0.05 + (0.01 × click_eps_upgrades) (base 5% + 1% per upgrade)
- `total_eps` = current total energy per second
- `thousand_rays_bonus` = calculated Thousand Rays bonus (if tier 4+ owned)

### Manual Multiplier Cap

With all 10 manual upgrades: **1,024x** click power
With first 3 Solar Panel upgrades: additional **8x** click power

Combined: **8,192x** base click power

---

## Synergy System

### Synergy Bonus

When a synergy upgrade is purchased:

```
target_bonus = min(2.5, 1.0 + (0.02 × source_count))
```

### Bidirectional Synergies

There are 38 total synergies (19 forward + 19 reverse). Each adjacent producer pair has:
- Forward: Producer N boosts Producer N+1
- Reverse: Producer N+1 boosts Producer N

### Example

With 100 Mining Drones and both synergies affecting Asteroid Mines:
- From Drone-Mine Link: min(2.5, 1.0 + (0.02 × 100)) = **2.5x** (capped)
- Total synergy bonus to Asteroid Mines: 2.5x

This multiplies with the Asteroid Mine's producer multiplier.

---

## Achievement Bonus

### Formula

```
achievement_bonus = 1.005^n
```

Where `n` = number of achievements unlocked.

### Bonus Table

| Achievements | Multiplier |
|--------------|------------|
| 10 | 1.051x |
| 25 | 1.133x |
| 50 | 1.284x |
| 75 | 1.454x |
| 100 | 1.649x |
| 125 | 1.868x |
| 150 | 2.118x |
| 175 | 2.400x |
| 200 | 2.715x |
| 225 | 3.077x |
| 250 | 3.487x |
| 260 | 3.67x |

---

## Prestige System

### Stellar Chips Formula (Cubic)

Solaris uses a **cubic formula** for slower prestige progression:

```
chips = floor(cbrt(total_energy_earned / 1,000,000,000,000))
```

This is equivalent to:
```
chips = floor((total_energy_earned)^(1/3) / 10,000)
```

### Energy Required for Chips

| Chips | Total Energy Required |
|-------|----------------------|
| 1 | 1 trillion (10^12) |
| 8 | 512 trillion (5.12 × 10^14) |
| 27 | 19.7 quadrillion (1.97 × 10^16) |
| 100 | 1 quintillion (10^18) |
| 1,000 | 1 sextillion (10^21) |
| 10,000 | 1 octillion (10^27) |

### Chip Bonus Multiplier

Prestige upgrades can increase chip earnings:
```
chip_multiplier = Π (chip_bonus_upgrades)
```

Maximum possible with all chip bonus upgrades: **~26x**

### Net Chips Per Ascension

```
net_chips = (floor(cbrt(total_energy / 1e12)) × chip_multiplier) - previously_earned_chips
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

Maximum possible with all offline prestige upgrades: **~26x**

---

## Double Energy Chance

### Lucky Stars Effect

With Lucky Stars and all luck prestige upgrades:

```
double_chance = sum(all_double_energy_chance_upgrades)
```

Maximum possible: up to **~95%** with late-game prestige upgrades

Each tick has this chance to generate double energy.

### Effective Bonus

Average production increase = double_chance × 100%
- At 6% chance: **+6% average production**
- At 50% chance: **+50% average production**

---

## Ascension Mechanics

### Energy Retention

With energy retention prestige upgrades:

```
retained_energy = current_energy × retention_percent
```

Maximum retention: **10%** (capped)

### Starting Energy

From Quick Start prestige upgrades:

```
starting_energy = sum(prestige_starting_bonuses) + retained_energy
```

Maximum starting energy: **100,000,000** (with Quick Start VII)

### Producer Unlock Level

```
unlock_level = max(prestige_unlock_upgrades)
```

- Seasoned Explorer: 2
- Accelerated Start: 3
- Galaxy Starter: 4
- Universal Knowledge: 5
- Stellar Starter: 6
- Rapid Expansion: 7
- Cosmic Starter: 8
- Universal Unlock: 10
- Dimensional Mastery: 12
- Universal Unlock II: 15
- Complete Mastery: 20

---

## Per-Ascension Bonus

With production per-ascension prestige upgrades:

```
ascension_bonus = min(cap, 1 + (bonus_per_ascension × total_ascensions))
```

Maximum bonus per ascension: **~87.5%** (with all per-ascension upgrades)
Cap: Implementation-dependent but effectively very high

---

## Per-Achievement Bonus

With Dimensional Echo and related prestige upgrades:

```
achievement_prestige_mult = 1 + (bonus_per_achievement × achievement_count)
```

Maximum bonus per achievement: **~18.75%** (with all per-achievement upgrades)
At 260 achievements with all upgrades: **~49.75x** multiplier

---

## Number Formatting

The game uses named suffixes for large numbers, covering the full short-scale naming system up to **centillion** (10^303). Beyond centillion, numbers fall back to scientific notation.

| Value | Full Name |
|-------|-----------|
| 10^3 | Thousand |
| 10^6 | Million |
| 10^9 | Billion |
| 10^12 | Trillion |
| 10^15 | Quadrillion |
| 10^18 | Quintillion |
| 10^21 | Sextillion |
| 10^24 | Septillion |
| 10^27 | Octillion |
| 10^30 | Nonillion |
| 10^33 | Decillion |
| 10^36 | Undecillion |
| 10^39 | Duodecillion |
| 10^42 | Tredecillion |
| 10^45 | Quattuordecillion |
| 10^48 | Quindecillion |
| 10^51 | Sexdecillion |
| 10^54 | Septendecillion |
| 10^57 | Octodecillion |
| 10^60 | Novemdecillion |
| 10^63 | Vigintillion |
| 10^93 | Trigintillion |
| 10^123 | Quadragintillion |
| 10^153 | Quinquagintillion |
| 10^183 | Sexagintillion |
| 10^213 | Septuagintillion |
| 10^243 | Octogintillion |
| 10^273 | Nonagintillion |
| 10^303 | Centillion |

All intermediate names (e.g., unvigintillion, duovigintillion, etc.) are also supported between each major milestone listed above.

---

## Related Pages

- [Producers](producers.md) - Base stats for all producers
- [Upgrades](upgrades.md) - Upgrade costs and effects
- [Prestige](prestige.md) - Prestige upgrade details
- [Strategy](strategy.md) - Applying these formulas
