# Producers

Solaris features 20 unique energy producers, each representing increasingly advanced cosmic technology. Producers generate energy passively every tick (10 ticks per second).

## Producer Table

| ID | Icon | Name | Base E/s | Base Cost | Description |
|----|------|------|----------|-----------|-------------|
| 1 | ▤ | Solar Panel | 0.1 | 15 | A simple photovoltaic cell |
| 2 | ⛏ | Mining Drone | 0.5 | 100 | Autonomous resource collector |
| 3 | ☄ | Asteroid Mine | 2 | 500 | Harvests energy-rich minerals |
| 4 | ◎ | Orbital Station | 10 | 3,000 | Space-based power relay |
| 5 | ☽ | Lunar Colony | 50 | 15,000 | Moon-based operations hub |
| 6 | ⊕ | Planetary Harvester | 250 | 80,000 | Extracts planetary core energy |
| 7 | ✴ | Fusion Reactor | 1,200 | 400,000 | Harnesses stellar fusion |
| 8 | ✧ | Dyson Swarm | 6,000 | 2,000,000 | Orbiting solar collectors |
| 9 | ⊙ | Dyson Sphere | 30,000 | 10,000,000 | Encapsulates an entire star |
| 10 | ★ | Star Forge | 150,000 | 50,000,000 | Creates and harvests stars |
| 11 | ◉ | Neutron Harvester | 750,000 | 250,000,000 | Harvest neutron star energy |
| 12 | ● | Black Hole Tap | 3,750,000 | 1,250,000,000 | Extract energy from event horizons |
| 13 | ◌ | Quantum Vacuum | 18,750,000 | 6,250,000,000 | Zero-point energy extraction |
| 14 | ⊛ | Galactic Core | 93,750,000 | 31,250,000,000 | Tap supermassive black holes |
| 15 | ◐ | Dark Matter Engine | 468,750,000 | 156,250,000,000 | Convert dark matter to energy |
| 16 | ≋ | Cosmic String | 2,343,750,000 | 781,250,000,000 | Vibrating cosmic strings |
| 17 | ⧫ | Multiverse Gate | 11,718,750,000 | 3,906,250,000,000 | Energy from parallel universes |
| 18 | ⌘ | Reality Compiler | 58,593,750,000 | 19,531,250,000,000 | Rewrite physics for energy |
| 19 | ∞ | Entropy Reverser | 292,968,750,000 | 97,656,250,000,000 | Reverse thermodynamic entropy |
| 20 | ✦ | Big Bang Catalyst | 1,464,843,750,000 | 488,281,250,000,000 | Create pocket universes |

---

## Tier Groupings

### Early Game (Producers 1-4)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Solar Panel | 0.1 | 15 | Starting producer, always available |
| Mining Drone | 0.5 | 100 | First unlock, 5x more efficient |
| Asteroid Mine | 2 | 500 | Good early investment |
| Orbital Station | 10 | 3,000 | First major production jump |

These producers are your foundation. Focus on building a stable base of Solar Panels and Mining Drones before pushing to higher tiers.

### Mid Game (Producers 5-10)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Lunar Colony | 50 | 15,000 | Unlocks synergy chains |
| Planetary Harvester | 250 | 80,000 | Core mid-game producer |
| Fusion Reactor | 1,200 | 400,000 | Major efficiency boost |
| Dyson Swarm | 6,000 | 2M | Transition to mega-structures |
| Dyson Sphere | 30,000 | 10M | Iconic sci-fi technology |
| Star Forge | 150,000 | 50M | Create your own stars |

Mid-game is where synergy upgrades become crucial. Each producer boosts the next in the chain.

### Late Game (Producers 11-15)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Neutron Harvester | 750K | 250M | Exotic stellar remnants |
| Black Hole Tap | 3.75M | 1.25B | Event horizon energy |
| Quantum Vacuum | 18.75M | 6.25B | Zero-point extraction |
| Galactic Core | 93.75M | 31.25B | Supermassive black holes |
| Dark Matter Engine | 468.75M | 156.25B | Convert invisible matter |

Late game producers require significant investment but provide massive returns.

### End Game (Producers 16-20)

| Producer | Base E/s | Base Cost | Notes |
|----------|----------|-----------|-------|
| Cosmic String | 2.34B | 781.25B | Topological defects |
| Multiverse Gate | 11.72B | 3.91T | Parallel universe energy |
| Reality Compiler | 58.59B | 19.53T | Rewrite physics itself |
| Entropy Reverser | 292.97B | 97.66T | Defeat thermodynamics |
| Big Bang Catalyst | 1.46T | 488.28T | Create pocket universes |

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

Each producer's cost increases by **15%** (1.15x) with each purchase:

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

1. **Producer Upgrades**: 15 tiers, each tier gives 2x multiplier
   - Maximum: 2^15 = 32,768x from upgrades alone

2. **Synergy Upgrades**: +5% per source producer owned
   - Example: With 100 Mining Drones and the Drone-Mine Synergy upgrade, Asteroid Mines get +500% (6x) bonus

### Global Multiplier Sources

1. **Global Upgrades**: 10 total, ranging from +5% to +200%
2. **Achievement Bonus**: 1.01^n where n = number of achievements
   - At 100 achievements: ~2.7x multiplier
   - At 260 achievements: ~13.2x multiplier
3. **Prestige Multipliers**: Various production bonuses from prestige upgrades

---

## Producer Upgrades

Each producer has **15 upgrade tiers** that unlock at specific ownership thresholds:

| Tier | Required Count | Effect |
|------|----------------|--------|
| 1 | 1 | 2x production |
| 2 | 5 | 2x production |
| 3 | 25 | 2x production |
| 4 | 50 | 2x production |
| 5 | 100 | 2x production |
| 6 | 150 | 2x production |
| 7 | 200 | 2x production |
| 8 | 250 | 2x production |
| 9 | 300 | 2x production |
| 10 | 350 | 2x production |
| 11 | 400 | 2x production |
| 12 | 450 | 2x production |
| 13 | 500 | 2x production |
| 14 | 550 | 2x production |
| 15 | 600 | 2x production |

See [Upgrades](upgrades.md) for complete upgrade details.

---

## Synergy Chain

Adjacent producers can boost each other with synergy upgrades:

```
Mining Drone → Asteroid Mine → Orbital Station → Lunar Colony → ...
```

The complete chain:
1. Mining Drones boost Asteroid Mines
2. Asteroid Mines boost Orbital Stations
3. Orbital Stations boost Lunar Colonies
4. Lunar Colonies boost Planetary Harvesters
5. Planetary Harvesters boost Fusion Reactors
6. Fusion Reactors boost Dyson Swarms
7. Dyson Swarms boost Dyson Spheres
8. Dyson Spheres boost Star Forges
9. Star Forges boost Neutron Harvesters
10. Neutron Harvesters boost Black Hole Taps
11. Black Hole Taps boost Quantum Vacuums
12. Quantum Vacuums boost Galactic Cores
13. Galactic Cores boost Dark Matter Engines
14. Dark Matter Engines boost Cosmic Strings
15. Cosmic Strings boost Multiverse Gates
16. Multiverse Gates boost Reality Compilers
17. Reality Compilers boost Entropy Reversers
18. Entropy Reversers boost Big Bang Catalysts
19. **Solar Panels boost Big Bang Catalysts** (completing the circle!)

Each synergy provides **+5% per source producer owned** to the target producer.

---

## Related Pages

- [Upgrades](upgrades.md) - All upgrade types and costs
- [Mechanics](mechanics.md) - Detailed production formulas
- [Strategy](strategy.md) - Optimal purchase strategies
