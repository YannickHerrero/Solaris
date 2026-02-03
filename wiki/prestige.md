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

### Earning Formula (Cubic)

Stellar Chips are calculated using a **cubic formula** for slower progression (matching Cookie Clicker's feel):

```
chips = floor(cbrt(total_energy_earned / 1,000,000,000,000))
```

This means:
- 1 trillion (10^12) energy = 1 chip
- 8 trillion energy = 2 chips
- 27 trillion energy = 3 chips
- 1 quadrillion (10^15) energy = 10 chips
- 1 quintillion (10^18) energy = 100 chips
- 1 sextillion (10^21) energy = 1,000 chips

### Chip Bonuses

Prestige upgrades can increase chip earnings:
- **Eternal Progression** (200 chips): +5% more chips
- **Stellar Accumulation** (500 chips): +15% more chips
- **Chip Collector I** (120 chips): +10% more chips
- **Chip Collector II** (600 chips): +20% more chips
- **Chip Magnet** (12,500 chips): +25% more chips
- **Stellar Magnate** (75,000 chips): +50% more chips
- **Chip Tycoon** (500,000 chips): +100% more chips
- **Chip Overlord** (3,000,000 chips): +200% more chips
- **Chip Deity** (50,000,000 chips): +400% more chips
- **Chip Absolute** (2,000,000,000 chips): +1000% more chips
- **Chip God** (100,000,000,000 chips): +2500% more chips

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
- **Achievements** (and their +0.5% bonus each!)
- **Stellar Chips** (both balance and total earned)
- **Prestige upgrades purchased**
- **Total ascension count**

### Minimum Requirements

You can only ascend if you would earn at least **1 Stellar Chip**.

---

## Prestige Upgrades (100 total)

### Tier 1 (1-10 chips) - Starting Bonuses

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 1 | Stellar Foundation | 1 | +5% all production | None |
| 2 | Quick Start | 3 | Start with 100 energy | None |
| 3 | Persistent Memory | 5 | +10% offline earnings | None |
| 4 | Lucky Stars | 7 | +1% double energy chance | None |
| 5 | Seasoned Explorer | 10 | Start with 2 producers | None |

### Tier 2 (15-50 chips) - Early Progression

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 6 | Cosmic Legacy | 15 | +0.5% production per ascension | 10 total chips |
| 7 | Accelerated Start | 20 | Start with 3 producers | Seasoned Explorer |
| 72 | Quick Start II | 25 | Start with 1,000 energy | Quick Start |
| 8 | Stellar Efficiency | 30 | +15% all production | Stellar Foundation |
| 82 | Galaxy Starter | 35 | Start with 4 producers | Accelerated Start |
| 9 | Bargain Hunter | 40 | Upgrades cost 10% less | None |
| 87 | Ascension Power I | 45 | +0.75% production per ascension | Cosmic Legacy |
| 10 | Universal Knowledge | 50 | Start with 5 producers | Accelerated Start |

### Tier 3 (55-120 chips) - Building Power

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 86 | Fortune II | 55 | +3% double energy chance | Fortune I |
| 78 | Offline Boost I | 60 | +25% offline earnings | Persistent Memory |
| 11 | Dimensional Echo | 75 | +0.25% production per achievement | 50 total chips |
| 83 | Stellar Starter | 80 | Start with 6 producers | Universal Knowledge |
| 91 | Thrifty I | 90 | Upgrades cost 15% less | Bargain Hunter |
| 12 | Transcendent Memory | 100 | Keep 1% of energy | 5 ascensions |
| 73 | Quick Start III | 100 | Start with 10,000 energy | Quick Start II |
| 88 | Ascension Power II | 120 | +1.25% production per ascension | Ascension Power I |
| 80 | Chip Collector I | 120 | +10% Stellar Chips | Eternal Progression |

### Tier 4 (150-300 chips) - Mid-Game Power

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 13 | Cosmic Mastery | 150 | +50% all production | Stellar Efficiency |
| 84 | Cosmic Starter | 175 | Start with 8 producers | Rapid Expansion |
| 89 | Achievement Echo I | 180 | +0.35% production per achievement | Dimensional Echo |
| 14 | Eternal Progression | 200 | +5% Stellar Chips | 10 ascensions |
| 93 | Memory Fragment I | 250 | Keep 2% of energy | Transcendent Memory |
| 92 | Thrifty II | 250 | Upgrades cost 20% less | Thrifty I |
| 79 | Offline Boost II | 300 | +35% offline earnings | Offline Boost I |
| 15 | Rapid Expansion | 300 | Start with 7 producers | Universal Knowledge |

### Tier 5 (400-750 chips) - Advanced Bonuses

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 16 | Enhanced Luck | 400 | +5% double energy chance | Lucky Stars |
| 90 | Achievement Echo II | 450 | +0.4% production per achievement | Achievement Echo I |
| 17 | Stellar Accumulation | 500 | +15% Stellar Chips | Eternal Progression |
| 94 | Memory Fragment II | 500 | Keep 3% of energy | Memory Fragment I |
| 74 | Quick Start IV | 500 | Start with 100,000 energy | Quick Start III |
| 81 | Chip Collector II | 600 | +20% Stellar Chips | Chip Collector I |
| 18 | Universal Unlock | 750 | Start with 10 producers | Rapid Expansion |

### Tier 6 (1,000-5,000 chips) - Late-Game

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 19 | Perfect Memory | 1,000 | Keep 5% of energy | Transcendent Memory |
| 20 | Infinite Power | 1,500 | +100% all production | Cosmic Mastery |
| 21 | Legacy of Legends | 2,000 | +1% production per ascension | Cosmic Legacy |
| 75 | Quick Start V | 2,500 | Start with 1M energy | Quick Start IV |
| 22 | Discount Master | 3,000 | Upgrades cost 25% less | Bargain Hunter |
| 23 | Eternal Offline | 4,000 | +50% offline earnings | Persistent Memory |
| 24 | Galactic Domination | 5,000 | +200% all production | Infinite Power |

### Tier 7 (7,500-15,000 chips) - Advanced End-Game

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 25 | Dimensional Mastery | 7,500 | Start with 12 producers | Universal Unlock |
| 67 | Cosmic Surge I | 8,000 | +50% all production | 5,000 total chips |
| 26 | Eternal Fortune | 10,000 | +10% double energy chance | Enhanced Luck |
| 27 | Chip Magnet | 12,500 | +25% Stellar Chips | Stellar Accumulation |
| 76 | Quick Start VI | 12,500 | Start with 10M energy | Quick Start V |
| 28 | Perfect Retention | 15,000 | Keep 10% of energy | Perfect Memory |

### Tier 8 (18,000-45,000 chips) - Deep End-Game

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 68 | Cosmic Surge II | 18,000 | +75% all production | Cosmic Surge I |
| 29 | Stellar Supremacy | 20,000 | +300% all production | Galactic Domination |
| 30 | Universal Unlock II | 25,000 | Start with 15 producers | Dimensional Mastery |
| 31 | Achievement Synergy | 30,000 | +0.5% production per achievement | Dimensional Echo |
| 32 | Ascension Mastery | 35,000 | +2% production per ascension | Legacy of Legends |
| 33 | Discount Champion | 40,000 | Upgrades cost 40% less | Discount Master |
| 69 | Cosmic Surge III | 45,000 | +100% all production | Cosmic Surge II |

### Tier 9 (50,000-100,000 chips) - Master Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 34 | Offline Empire | 50,000 | +100% offline earnings | Eternal Offline |
| 35 | Lucky Universe | 60,000 | +15% double energy chance | Eternal Fortune |
| 77 | Quick Start VII | 62,500 | Start with 100M energy | Quick Start VI |
| 36 | Stellar Magnate | 75,000 | +50% Stellar Chips | Chip Magnet |
| 70 | Cosmic Surge IV | 90,000 | +150% all production | Cosmic Surge III |
| 37 | Cosmic Emperor | 100,000 | +500% all production | Stellar Supremacy |

### Tier 10 (150,000-350,000 chips) - Legendary Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 38 | Complete Mastery | 150,000 | Start with all 20 producers | Universal Unlock II |
| 71 | Cosmic Surge V | 180,000 | +200% all production | Cosmic Surge IV |
| 39 | Achievement Legend | 200,000 | +1% production per achievement | Achievement Synergy |
| 40 | Eternal Ascender | 250,000 | +5% production per ascension | Ascension Mastery |
| 41 | Free Shopping | 300,000 | Upgrades cost 50% less | Discount Champion |
| 95 | Stellar Dynasty | 350,000 | +500% all production | Cosmic Surge V |

### Tier 11 (400,000-750,000 chips) - Mythic Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 42 | Transcendent Luck | 400,000 | +20% double energy chance | Lucky Universe |
| 43 | Chip Tycoon | 500,000 | +100% Stellar Chips | Stellar Magnate |
| 44 | Offline Dominion | 600,000 | +200% offline earnings | Offline Empire |
| 96 | Cosmic Dynasty | 700,000 | +750% all production | Stellar Dynasty |
| 45 | Universal Power | 750,000 | +1000% all production | Cosmic Emperor |

### Tier 12 (1M-5M chips) - Godlike Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 46 | Beyond Achievement | 1,000,000 | +2% production per achievement | Achievement Legend |
| 97 | Universal Dynasty | 1,400,000 | +1500% all production | Cosmic Dynasty |
| 47 | Infinite Ascension | 1,500,000 | +10% production per ascension | Eternal Ascender |
| 48 | Quantum Luck | 2,000,000 | +25% double energy chance | Transcendent Luck |
| 98 | Dimensional Dynasty | 2,800,000 | +3000% all production | Universal Dynasty |
| 49 | Chip Overlord | 3,000,000 | +200% Stellar Chips | Chip Tycoon |
| 50 | Absolute Dominance | 5,000,000 | +2500% all production | Universal Power |
| 99 | Reality Dynasty | 5,600,000 | +6000% all production | Dimensional Dynasty |

### Tier 13+ (10M+ chips) - Transcendent Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 51 | Eternal Offline II | 10,000,000 | +500% offline earnings | Offline Dominion |
| 100 | Multiversal Dynasty | 11,200,000 | +15000% all production | Reality Dynasty |
| 52 | Achievement Godhood | 15,000,000 | +5% production per achievement | Beyond Achievement |
| 53 | Ascension Mastery II | 20,000,000 | +20% production per ascension | Infinite Ascension |
| 54 | Reality Luck | 30,000,000 | +30% double energy chance | Quantum Luck |
| 55 | Chip Deity | 50,000,000 | +400% Stellar Chips | Chip Overlord |
| 56 | Omnipotent Production | 100,000,000 | +5000% all production | Absolute Dominance |

### Tier 14+ (500M+ chips) - Ultimate Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 57 | Eternal Presence | 500,000,000 | +1000% offline earnings | Eternal Offline II |
| 58 | Achievement Transcendence | 750,000,000 | +10% production per achievement | Achievement Godhood |
| 59 | Ultimate Luck | 1,000,000,000 | +35% double energy chance | Reality Luck |
| 60 | Chip Absolute | 2,000,000,000 | +1000% Stellar Chips | Chip Deity |
| 61 | Ascension Pinnacle | 3,000,000,000 | +50% production per ascension | Ascension Mastery II |
| 62 | Infinite Production | 5,000,000,000 | +10000% all production | Omnipotent Production |

### Tier 15+ (10B+ chips) - Omega Tier

| ID | Name | Cost | Effect | Requirement |
|----|------|------|--------|-------------|
| 63 | Eternal Vigilance | 10,000,000,000 | +2500% offline earnings | Eternal Presence |
| 64 | Divine Luck | 20,000,000,000 | +40% double energy chance | Ultimate Luck |
| 65 | Ultimate Production | 50,000,000,000 | +25000% all production | Infinite Production |
| 66 | Chip God | 100,000,000,000 | +2500% Stellar Chips | Chip Absolute |

---

## Upgrade Categories Summary

### Production Multipliers
All production multipliers stack **multiplicatively**:

| Upgrades | Combined Multiplier |
|----------|---------------------|
| Tier 1-6 base (IDs 1,8,13,20,24) | ~10.87x |
| Cosmic Surge series (67-71, 95-100) | ~15,000x+ |
| Dynasty series | ~150,000x+ |
| **Total possible** | **Astronomical** |

### Per-Ascension Bonus
Total bonus per ascension with all upgrades:

| Upgrade | Bonus |
|---------|-------|
| Cosmic Legacy | +0.5% |
| Ascension Power I | +0.75% |
| Ascension Power II | +1.25% |
| Legacy of Legends | +1% |
| Ascension Mastery | +2% |
| Eternal Ascender | +5% |
| Infinite Ascension | +10% |
| Ascension Mastery II | +20% |
| Ascension Pinnacle | +50% |
| **Total** | **+90.5% per ascension** |

### Per-Achievement Bonus
Total bonus per achievement with all upgrades:

| Upgrade | Bonus |
|---------|-------|
| Dimensional Echo | +0.25% |
| Achievement Echo I | +0.35% |
| Achievement Echo II | +0.4% |
| Achievement Synergy | +0.5% |
| Achievement Legend | +1% |
| Beyond Achievement | +2% |
| Achievement Godhood | +5% |
| Achievement Transcendence | +10% |
| **Total** | **+19.5% per achievement** |

At 260 achievements: **+5,070%** multiplier (51.7x)

### Double Energy Chance
Total double energy chance with all upgrades:

| Upgrade | Chance |
|---------|--------|
| Lucky Stars | +1% |
| Fortune I | +2% |
| Fortune II | +3% |
| Enhanced Luck | +5% |
| Eternal Fortune | +10% |
| Lucky Universe | +15% |
| Transcendent Luck | +20% |
| Quantum Luck | +25% |
| Reality Luck | +30% |
| Ultimate Luck | +35% |
| Divine Luck | +40% |
| **Total** | **186%** (capped implementation) |

### Upgrade Cost Reduction
Maximum reduction with all upgrades:

| Upgrade | Reduction |
|---------|-----------|
| Bargain Hunter | 10% |
| Thrifty I | 15% |
| Thrifty II | 20% |
| Discount Master | 25% |
| Discount Champion | 40% |
| Free Shopping | 50% |
| **Maximum** | **50%** (capped) |

---

## Recommended Purchase Order

### First Ascension (1-10 chips)

1. **Stellar Foundation** (1 chip) - Immediate production boost
2. **Quick Start** (3 chips) - Speeds up early game
3. **Seasoned Explorer** (10 chips) - Skip Solar Panel grind

### Early Game (10-50 chips)

4. **Accelerated Start** (20 chips) - Start with 3 producers
5. **Stellar Efficiency** (30 chips) - More production
6. **Universal Knowledge** (50 chips) - Start with 5 producers

### Mid Game (50-500 chips)

7. **Dimensional Echo** (75 chips) - Per-achievement scaling
8. **Cosmic Mastery** (150 chips) - +50% production
9. **Eternal Progression** (200 chips) - More chips per run
10. **Rapid Expansion** (300 chips) - Start with 7 producers

### Late Game (500-5000 chips)

11. **Universal Unlock** (750 chips) - Start with 10 producers
12. **Infinite Power** (1500 chips) - +100% production
13. **Galactic Domination** (5000 chips) - +200% production

### End Game (5000+ chips)

14. **Production multiplier chains** - Cosmic Surge series
15. **Dynasty series** - Ultimate production multipliers
16. **Chip bonus upgrades** - Long-term value
17. **Luck upgrades** - Free production boost

---

## Tips for Prestige

1. **Don't ascend too early** - Build up achievements first, they persist!
2. **Aim for 5-10 chips** on first ascension
3. **Producer unlocks are powerful** - Less grinding, faster starts
4. **Production multipliers compound** - Prioritize the multiplier chains
5. **Chip bonuses pay off long-term** - More chips = faster upgrades
6. **Per-achievement upgrades scale infinitely** - Very powerful late-game
7. **Cubic formula means patience** - You need 8x the energy for just 2x the chips

---

## Related Pages

- [Mechanics](mechanics.md) - Detailed formulas
- [Achievements](achievements.md) - Achievement bonuses
- [Strategy](strategy.md) - Optimal ascension timing
