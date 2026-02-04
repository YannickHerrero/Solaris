# Producers

Solaris features 20 unique energy producers, each representing increasingly advanced cosmic technology. Producers generate energy passively every tick (10 ticks per second).

## Producer Table

All producers use incremental-style cost and E/s ratios with progressively longer payback times.

| ID | Icon | Name | Base E/s | Base Cost | Description |
|----|------|------|----------|-----------|-------------|
| 1 | ▤ | Solar Panel | 0.1 | 15 | A simple photovoltaic cell |
| 2 | ⛏ | Mining Drone | 1 | 100 | Autonomous resource collector |
| 3 | ☄ | Asteroid Mine | 8 | 1,100 | Harvests energy-rich minerals |
| 4 | ◎ | Orbital Station | 47 | 12,000 | Space-based power relay |
| 5 | ☽ | Lunar Colony | 260 | 130,000 | Moon-based operations hub |
| 6 | ⊕ | Planetary Harvester | 1,400 | 1,400,000 | Extracts planetary core energy |
| 7 | ✴ | Fusion Reactor | 7,800 | 20,000,000 | Harnesses stellar fusion |
| 8 | ✧ | Dyson Swarm | 44,000 | 330,000,000 | Orbiting solar collectors |
| 9 | ⊙ | Dyson Sphere | 260,000 | 5,100,000,000 | Encapsulates an entire star |
| 10 | ★ | Star Forge | 1,600,000 | 75,000,000,000 | Creates and harvests stars |
| 11 | ◉ | Neutron Harvester | 10,000,000 | 1,000,000,000,000 | Harvests neutron star energy |
| 12 | ● | Black Hole Tap | 65,000,000 | 14,000,000,000,000 | Extracts energy from event horizons |
| 13 | ◌ | Quantum Vacuum | 430,000,000 | 170,000,000,000,000 | Zero-point energy extraction |
| 14 | ⊛ | Galactic Core | 2,900,000,000 | 2,100,000,000,000,000 | Taps supermassive black holes |
| 15 | ◐ | Dark Matter Engine | 21,000,000,000 | 26,000,000,000,000,000 | Converts dark matter to energy |
| 16 | ≋ | Cosmic String | 150,000,000,000 | 310,000,000,000,000,000 | Vibrating cosmic strings |
| 17 | ⧫ | Multiverse Gate | 1,100,000,000,000 | 71,000,000,000,000,000,000 | Energy from parallel universes |
| 18 | ⌘ | Reality Compiler | 8,300,000,000,000 | 12,000,000,000,000,000,000,000 | Rewrites physics for energy |
| 19 | ∞ | Entropy Reverser | 64,000,000,000,000 | 1,900,000,000,000,000,000,000,000 | Reverses thermodynamic entropy |
| 20 | ✦ | Big Bang Catalyst | 510,000,000,000,000 | 540,000,000,000,000,000,000,000,000 | Creates pocket universes |

---

## Tier Groupings

### Early Game (Producers 1-4)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Solar Panel | 0.1 | 15 | Uses Thousand Rays mechanic |
| Mining Drone | 1 | 100 | Uses Drone Network mechanic |
| Asteroid Mine | 8 | 1.1K | Good early investment |
| Orbital Station | 47 | 12K | First major production jump |

Solar Panels and Mining Drones have special mechanics that make them valuable throughout the game. See the [Special Mechanics](#special-mechanics) section below.

### Mid Game (Producers 5-10)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Lunar Colony | 260 | 130K | Unlocks synergy chains |
| Planetary Harvester | 1,400 | 1.4M | Core mid-game producer |
| Fusion Reactor | 7,800 | 20M | Major efficiency boost |
| Dyson Swarm | 44,000 | 330M | Transition to mega-structures |
| Dyson Sphere | 260,000 | 5.1B | Iconic sci-fi technology |
| Star Forge | 1.6M | 75B | Create your own stars |

Mid-game is where synergy upgrades become crucial. Each producer can boost adjacent producers in both directions.

### Late Game (Producers 11-15)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Neutron Harvester | 10M | 1T | Exotic stellar remnants |
| Black Hole Tap | 65M | 14T | Event horizon energy |
| Quantum Vacuum | 430M | 170T | Zero-point extraction |
| Galactic Core | 2.9B | 2.1Q | Supermassive black holes |
| Dark Matter Engine | 21B | 26Q | Convert invisible matter |

Late game producers require significant investment but provide massive returns.

### End Game (Producers 16-20)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Cosmic String | 150B | 310Q | Topological defects |
| Multiverse Gate | 1.1T | 71Qi | Parallel universe energy |
| Reality Compiler | 8.3T | 12Sx | Rewrite physics itself |
| Entropy Reverser | 64T | 1.9Sp | Defeat thermodynamics |
| Big Bang Catalyst | 510T | 540Sp | Create pocket universes |

These cosmic-scale producers represent the pinnacle of energy technology.

---

## Unlock Conditions

Producers unlock sequentially:
1. **Solar Panel**: Always available
2. **Producer N**: Unlocked when you own at least 1 of Producer N-1

Additionally, prestige upgrades can unlock multiple producers at the start:
- **Seasoned Explorer** (10 chips): Start with Mining Drone (Producer 2) unlocked
- **Accelerated Start** (20 chips): Start with first 3 producers unlocked
- **Universal Knowledge** (50 chips): Start with first 5 producers unlocked
- **Rapid Expansion** (300 chips): Start with first 7 producers unlocked
- **Universal Unlock** (750 chips): Start with first 10 producers unlocked

---

## Cost Scaling

All producer costs scale by a flat **1.15x multiplier** per purchase:

```
Cost(n) = base_cost × 1.15^owned
```

### Bulk Purchase Formula

When buying multiple producers:
```
Total Cost = base_cost × 1.15^owned × (1.15^quantity - 1) / 0.15
```

### Example: Solar Panel Costs

| Owned | Next Cost | Total Spent |
|-------|-----------|-------------|
| 0 | 15.00 | 0 |
| 1 | 17.25 | 15.00 |
| 5 | 30.17 | 101.14 |
| 10 | 60.68 | 304.14 |
| 25 | 491.37 | 2,975.82 |
| 50 | 16,085.60 | 105,185.33 |
| 100 | 17,275,092.92 | 112,993,937.78 |

---

## Effective Production

A producer's actual output is calculated as:

```
Effective E/s = base_eps × count × producer_multiplier × global_multiplier
```

Where:
- **base_eps**: The producer's base energy per second
- **count**: Number of that producer owned
- **producer_multiplier**: Combined multiplier from producer upgrades and synergies
- **global_multiplier**: Combined multiplier from global upgrades, achievements, and prestige

### Producer Multiplier Sources

1. **Producer Upgrades**: 15 tiers, each giving **2x** multiplier
   - All tiers give flat 2.0x multiplier
   - Maximum: **32,768x** from all 15 upgrades (2^15)

2. **Synergy Upgrades**: +2% per source producer owned (capped at 2.5x)
   - 38 total synergies (bidirectional between adjacent producers)
   - Example: With 75+ Mining Drones and Drone-Mine Link, Asteroid Mines get 2.5x bonus (capped)

3. **Drone Network Bonus** (Mining Drones only): +1% per 50 drones for each network type

### Global Multiplier Sources

1. **Global Upgrades**: 10 total, ranging from +5% to +200%
2. **Achievement Bonus**: 1.005^n where n = number of achievements
   - At 100 achievements: ~1.65x multiplier
   - At 260 achievements: ~3.67x multiplier
3. **Stellar Essence & Cosmic Cats**: +5% E/s per 4% Stellar Essence per Cosmic Cat upgrade
4. **Prestige Multipliers**: Various production bonuses from 100 prestige upgrades

---

## Producer Upgrades

Each producer has **15 upgrade tiers** that unlock at specific ownership thresholds. All tiers give a flat **2x multiplier**:

| Tier | Required Count | Effect | Cumulative |
|------|----------------|--------|------------|
| 1 | 1 | 2.0x | 2x |
| 2 | 5 | 2.0x | 4x |
| 3 | 25 | 2.0x | 8x |
| 4 | 50 | 2.0x | 16x |
| 5 | 100 | 2.0x | 32x |
| 6 | 150 | 2.0x | 64x |
| 7 | 200 | 2.0x | 128x |
| 8 | 250 | 2.0x | 256x |
| 9 | 300 | 2.0x | 512x |
| 10 | 350 | 2.0x | 1,024x |
| 11 | 400 | 2.0x | 2,048x |
| 12 | 450 | 2.0x | 4,096x |
| 13 | 500 | 2.0x | 8,192x |
| 14 | 550 | 2.0x | 16,384x |
| 15 | 600 | 2.0x | 32,768x |

**Note**: Solar Panel uses a special upgrade system called Thousand Rays. See [Special Mechanics](#special-mechanics) below.

See [Upgrades](upgrades.md) for complete upgrade details.

---

## Special Mechanics

### Thousand Rays (Solar Panel)

Solar Panels use a unique upgrade system called Thousand Rays. Instead of standard 2x multipliers, Solar Panel upgrades provide scaling bonuses based on your total buildings:

**First 3 Upgrades (Tiers 1-3):**
- Each upgrade provides **2x to both Solar Panel E/s AND manual mining power**
- These stack multiplicatively: all 3 = 8x base Solar Panel E/s and 8x click power

**Tier 4: Thousand Rays**
- Grants **+0.1 energy per non-panel building owned** to both Solar Panels and manual mining
- With 100 Mining Drones, 50 Asteroid Mines, etc., this adds up quickly

**Tiers 5-15: Rays Multipliers**
- Tier 5 (Million Rays): **5x** the Thousand Rays bonus
- Tier 6 (Billion Rays): **10x** the Thousand Rays bonus
- Tiers 7-15: Each gives **20x** the Thousand Rays bonus

**Example Calculation:**
```
Non-panel buildings: 500 total
Thousand Rays base: 500 × 0.1 = 50 energy
With Million Rays (5x): 250 energy
With Billion Rays (10x): 2,500 energy
With all tier 7-15 (20x each, 9 upgrades): 2,500 × 20^9 = massive bonus!
```

### Drone Network (Mining Drone)

Mining Drones have a unique synergy system where they gain bonuses from ALL other building types:

**Drone Network Upgrades (18 total):**
Each building type (3-20) has a corresponding Drone Network upgrade:
- **Requirement**: 1 Mining Drone + 15 of the associated building
- **Effect**: Mining Drones are **2x** more efficient
- **Bonus**: Mining Drones gain **+1% E/s per 50 drones** for that network type

**Workforce Upgrades (18 total):**
Each building also has a "Workforce" upgrade:
- **Requirement**: 50+ Mining Drones + 15 of the building
- **Effect**: That building gains **+1% E/s per 50 Mining Drones owned**

**Example:**
With Asteroid Mining Network and 200 Mining Drones:
- Mining Drones get 2x multiplier (from network unlock)
- Mining Drones get +4% E/s (200 ÷ 50 = 4)
- With Asteroid Mine Workforce: Asteroid Mines also get +4% E/s

This creates a powerful feedback loop where investing in Mining Drones boosts all your other buildings!

---

## Synergy Chain

Adjacent producers can boost each other with synergy upgrades. There are now **38 bidirectional synergies**:

### Forward Synergies (19 total)

Forward synergies follow the producer chain:

1. Mining Drones → Asteroid Mines
2. Asteroid Mines → Orbital Stations
3. Orbital Stations → Lunar Colonies
4. Lunar Colonies → Planetary Harvesters
5. Planetary Harvesters → Fusion Reactors
6. Fusion Reactors → Dyson Swarms
7. Dyson Swarms → Dyson Spheres
8. Dyson Spheres → Star Forges
9. Star Forges → Neutron Harvesters
10. Neutron Harvesters → Black Hole Taps
11. Black Hole Taps → Quantum Vacuums
12. Quantum Vacuums → Galactic Cores
13. Galactic Cores → Dark Matter Engines
14. Dark Matter Engines → Cosmic Strings
15. Cosmic Strings → Multiverse Gates
16. Multiverse Gates → Reality Compilers
17. Reality Compilers → Entropy Reversers
18. Entropy Reversers → Big Bang Catalysts
19. **Solar Panels → Big Bang Catalysts** (completing the circle!)

### Reverse Synergies (19 total)

Reverse synergies go the opposite direction, allowing later producers to boost earlier ones:

1. Asteroid Mines → Mining Drones
2. Orbital Stations → Asteroid Mines
3. Lunar Colonies → Orbital Stations
4. Planetary Harvesters → Lunar Colonies
5. Fusion Reactors → Planetary Harvesters
6. Dyson Swarms → Fusion Reactors
7. Dyson Spheres → Dyson Swarms
8. Star Forges → Dyson Spheres
9. Neutron Harvesters → Star Forges
10. Black Hole Taps → Neutron Harvesters
11. Quantum Vacuums → Black Hole Taps
12. Galactic Cores → Quantum Vacuums
13. Dark Matter Engines → Galactic Cores
14. Cosmic Strings → Dark Matter Engines
15. Multiverse Gates → Cosmic Strings
16. Reality Compilers → Multiverse Gates
17. Entropy Reversers → Reality Compilers
18. Big Bang Catalysts → Entropy Reversers
19. **Big Bang Catalysts → Solar Panels** (reverse wrap-around!)

### Synergy Formula

Each synergy provides **+2% per source producer owned** to the target producer:

```
Synergy Bonus = min(2.5, 1.0 + (0.02 × source_count))
```

- With 75+ source producers: **2.5x maximum bonus** (capped)
- Bidirectional synergies mean investing in any producer benefits its neighbors in both directions!

---

## Related Pages

- [Upgrades](upgrades.md) - All upgrade types and costs
- [Mechanics](mechanics.md) - Detailed production formulas
- [Strategy](strategy.md) - Optimal purchase strategies
