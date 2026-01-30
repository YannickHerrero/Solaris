# Prestige System (Stellar Ascension)

The prestige system in Solaris allows you to reset your progress in exchange for **Stellar Chips**, a permanent currency used to purchase powerful prestige upgrades.

## Overview

Press `a` to open the Ascension panel where you can:
- View your current Stellar Chips
- See potential chips from ascending
- Browse and purchase prestige upgrades
- Perform an ascension

---

## Stellar Chips

### Earning Formula

Stellar Chips are calculated from your total energy earned:

```
chips = floor(sqrt(total_energy_earned) / 1,000,000)
```

This means:
- 1 trillion (10^12) energy → 1 chip
- 4 trillion energy → 2 chips
- 100 trillion energy → 10 chips
- 1 quadrillion (10^15) energy → 31 chips
- 1 quintillion (10^18) energy → 1,000 chips

### Chip Bonuses

Prestige upgrades can increase chip earnings:
- **Eternal Progression** (200 chips): +5% more chips
- **Stellar Accumulation** (500 chips): +15% more chips

These bonuses stack multiplicatively.

### Chips on Ascension

When you ascend, you earn:
```
New Chips = (Total Chips from formula × bonus multiplier) - Previously earned chips
```

Only the net new chips are gained each ascension.

---

## Ascension Mechanics

### What Resets

When you ascend, you lose:
- Current energy balance
- All producers owned
- All regular upgrades purchased
- Total manual clicks (for this run)
- Ticks played (for this run)

### What Persists

The following carries over between ascensions:
- **Achievements** (and their +1% bonus each!)
- **Stellar Chips** (both balance and total earned)
- **Prestige upgrades purchased**
- **Total ascension count**

### Minimum Requirements

You can only ascend if you would earn at least **1 Stellar Chip**.

---

## Prestige Upgrades (24 total)

### Tier 1 (1-10 chips) - Starting Bonuses

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 1 | Stellar Foundation | 1 | +5% all production permanently | None |
| 2 | Quick Start | 3 | Start with 100 energy after ascension | None |
| 3 | Persistent Memory | 5 | +10% offline earnings | None |
| 4 | Lucky Stars | 7 | +1% chance for double energy per tick | None |
| 5 | Seasoned Explorer | 10 | Start with Mining Drone unlocked | None |

### Tier 2 (15-50 chips) - Early Progression

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 6 | Cosmic Legacy | 15 | +0.5% production per ascension | 10 total chips earned |
| 7 | Accelerated Start | 20 | Start with first 3 producers unlocked | Seasoned Explorer |
| 8 | Stellar Efficiency | 30 | +15% all production permanently | Stellar Foundation |
| 9 | Bargain Hunter | 40 | Upgrades cost 10% less | None |
| 10 | Universal Knowledge | 50 | Start with first 5 producers unlocked | Accelerated Start |

### Tier 3 (75-200 chips) - Mid-Game Power

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 11 | Dimensional Echo | 75 | +0.25% production per achievement | 50 total chips earned |
| 12 | Transcendent Memory | 100 | Keep 1% of energy after ascension | 5 ascensions |
| 13 | Cosmic Mastery | 150 | +50% all production permanently | Stellar Efficiency |
| 14 | Eternal Progression | 200 | +5% Stellar Chips earned | 10 ascensions |

### Tier 4 (300-500 chips) - Advanced Bonuses

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 15 | Rapid Expansion | 300 | Start with first 7 producers unlocked | Universal Knowledge |
| 16 | Enhanced Luck | 400 | +5% chance for double energy per tick | Lucky Stars |
| 17 | Stellar Accumulation | 500 | +15% Stellar Chips earned | Eternal Progression |

### Tier 5 (750-1,500 chips) - Late-Game Power

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 18 | Universal Unlock | 750 | Start with first 10 producers unlocked | Rapid Expansion |
| 19 | Perfect Memory | 1,000 | Keep 5% of energy after ascension | Transcendent Memory |
| 20 | Infinite Power | 1,500 | +100% all production permanently | Cosmic Mastery |

### Tier 6 (2,000-5,000 chips) - End-Game

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 21 | Legacy of Legends | 2,000 | +1% production per ascension | Cosmic Legacy |
| 22 | Discount Master | 3,000 | Upgrades cost 25% less | Bargain Hunter |
| 23 | Eternal Offline | 4,000 | +50% offline earnings | Persistent Memory |
| 24 | Galactic Domination | 5,000 | +200% all production permanently | Infinite Power |

---

## Upgrade Dependencies Tree

```
Stellar Foundation (1) ─────► Stellar Efficiency (8) ─────► Cosmic Mastery (13) ─────► Infinite Power (20) ─────► Galactic Domination (24)

Quick Start (2)

Persistent Memory (3) ─────────────────────────────────────────────────────────────────────────────────────────► Eternal Offline (23)

Lucky Stars (4) ─────────────────────────────────────────────► Enhanced Luck (16)

Seasoned Explorer (5) ─────► Accelerated Start (7) ─────► Universal Knowledge (10) ─────► Rapid Expansion (15) ─────► Universal Unlock (18)

Cosmic Legacy (6) [Requires 10 total chips] ─────────────────────────────────────────────────────────────────────► Legacy of Legends (21)

Bargain Hunter (9) ─────────────────────────────────────────────────────────────────────────────────────────────► Discount Master (22)

Dimensional Echo (11) [Requires 50 total chips]

Transcendent Memory (12) [Requires 5 ascensions] ─────────────────────────────────────────────────────────────► Perfect Memory (19)

Eternal Progression (14) [Requires 10 ascensions] ─────► Stellar Accumulation (17)
```

---

## Effect Stacking

### Production Multipliers

Production multipliers from prestige upgrades stack **multiplicatively**:

| Upgrade | Multiplier |
|---------|------------|
| Stellar Foundation | 1.05x |
| Stellar Efficiency | 1.15x |
| Cosmic Mastery | 1.50x |
| Infinite Power | 2.00x |
| Galactic Domination | 3.00x |
| **Combined** | **10.87x** |

### Per-Ascension Bonus

With both Cosmic Legacy (+0.5%) and Legacy of Legends (+1%):
- Total: +1.5% per ascension
- **Capped at 2.0x maximum**
- At 67+ ascensions: reaches 2.0x cap

### Per-Achievement Bonus

Dimensional Echo grants +0.25% per achievement on top of the base achievement bonus:
- Base: 1.005^n
- With Dimensional Echo: 1.005^n × (1 + 0.0025 × n)
- At 260 achievements: 3.67x × 1.65x = **6.06x multiplier**

### Double Energy Chance

Lucky Stars (+1%) and Enhanced Luck (+5%) combine:
- Total: 6% chance per tick for double energy
- Effective bonus: +6% average production

### Upgrade Cost Reduction

Bargain Hunter (10%) and Discount Master (25%) combine:
- Total: 35% reduction
- **Maximum cap: 50%**

### Offline Bonus

Persistent Memory (+10%) and Eternal Offline (+50%) stack **multiplicatively**:
- Combined: 1.10 × 1.50 = **1.65x offline earnings**

### Energy Retention

Transcendent Memory (1%) and Perfect Memory (5%) combine:
- Total: 6% of energy kept after ascension
- **Maximum cap: 10%**

---

## Recommended Purchase Order

### First Ascension (1-10 chips)

1. **Stellar Foundation** (1) - Immediate production boost
2. **Quick Start** (3) - Speeds up early game
3. **Seasoned Explorer** (10) - Skip Solar Panel grind

### Early Game (10-50 chips)

4. **Accelerated Start** (20) - Start with 3 producers
5. **Stellar Efficiency** (30) - More production
6. **Universal Knowledge** (50) - Start with 5 producers

### Mid Game (50-300 chips)

7. **Cosmic Mastery** (150) - Double production
8. **Eternal Progression** (200) - More chips per run
9. **Rapid Expansion** (300) - Start with 7 producers

### Late Game (300-1500 chips)

10. **Enhanced Luck** (400) - Free production boost
11. **Stellar Accumulation** (500) - Even more chips
12. **Universal Unlock** (750) - Start with 10 producers
13. **Perfect Memory** (1000) - Keep energy percentage
14. **Infinite Power** (1500) - Massive multiplier

### End Game (2000+ chips)

15. **Legacy of Legends** (2000) - Per-ascension scaling
16. **Discount Master** (3000) - Cheaper upgrades
17. **Galactic Domination** (5000) - Ultimate multiplier

---

## Tips for Prestige

1. **Don't ascend too early** - Build up achievements first, they persist!
2. **Aim for 5-10 chips** on first ascension
3. **Producer unlocks are powerful** - Less clicking, faster starts
4. **Production multipliers compound** - Prioritize the multiplier chain
5. **Chip bonuses pay off long-term** - More chips = faster upgrades
6. **Offline bonuses help casual play** - Great for overnight progress

---

## Related Pages

- [Mechanics](mechanics.md) - Detailed formulas
- [Achievements](achievements.md) - Achievement bonuses
- [Strategy](strategy.md) - Optimal ascension timing
