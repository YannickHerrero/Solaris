# Upgrades

Solaris features **474 upgrades** across 13 categories. Upgrades provide permanent bonuses and are essential for progression.

## Upgrade Summary

| Category | Count | Effect |
|----------|-------|--------|
| Producer Upgrades | 300 | 2x specific producer output (15 per producer) |
| Synergy Upgrades | 38 | Source producers boost target (+2% each) |
| Global Upgrades | 10 | Boost all production |
| Manual Upgrades | 10 | 2x manual mining power |
| Click E/s Upgrades | 10 | +1% of E/s per click each |
| Per-Building E/s | 20 | +1% E/s per building of type |
| Total Building E/s | 5 | +0.1% E/s per total buildings |
| Milestone Upgrades | 10 | +10% E/s at thresholds |
| Research Upgrades | 15 | +5% E/s each |
| Cosmic Cat Upgrades | 15 | E/s × (1 + stellar_essence × 0.05) |
| Stellar Essence Upgrades | 5 | +5% Stellar Essence effect |
| Drone Network Upgrades | 36 | Mining Drone synergies |
| **Total** | **474** | |

---

## Producer Upgrades (300 total)

Each of the 20 producers has **15 upgrade tiers**. All tiers give a flat **2x multiplier** (Cookie Clicker style).

### Unlock Thresholds

| Tier | Required Count | Multiplier | Cumulative |
|------|----------------|------------|------------|
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
| 15 | 600 | 2.0x | **32,768x** |

### Cost Formula

Upgrade costs scale by **10x** per tier (Cookie Clicker style):

```
Upgrade Cost = base_upgrade_cost × 10^tier
```

### Solar Panel Upgrades (Thousand Rays System)

Solar Panel upgrades work differently from other producers:

| Tier | Name | Required | Cost | Effect |
|------|------|----------|------|--------|
| 1 | Enhanced Photovoltaics | 1 | 100 | 2x Solar Panels AND manual mining |
| 2 | Quantum Absorption | 1 | 500 | 2x Solar Panels AND manual mining |
| 3 | Solar Amplification | 10 | 10K | 2x Solar Panels AND manual mining |
| 4 | Thousand Rays | 25 | 100K | +0.1 energy per non-panel building |
| 5 | Million Rays | 50 | 10M | 5x Thousand Rays bonus |
| 6 | Billion Rays | 100 | 100M | 10x Thousand Rays bonus |
| 7 | Trillion Rays | 150 | 1B | 20x Thousand Rays bonus |
| 8 | Quadrillion Rays | 200 | 10B | 20x Thousand Rays bonus |
| 9 | Quintillion Rays | 250 | 10T | 20x Thousand Rays bonus |
| 10 | Sextillion Rays | 300 | 10Q | 20x Thousand Rays bonus |
| 11 | Septillion Rays | 350 | 10Qi | 20x Thousand Rays bonus |
| 12 | Octillion Rays | 400 | 10Sx | 20x Thousand Rays bonus |
| 13 | Nonillion Rays | 450 | 10Sp | 20x Thousand Rays bonus |
| 14 | Decillion Rays | 500 | 10Oc | 20x Thousand Rays bonus |
| 15 | Undecillion Rays | 550 | 10No | 20x Thousand Rays bonus |

**Key mechanic**: The first 3 upgrades boost both Solar Panel E/s AND manual mining power. Tier 4+ adds energy based on your total non-panel buildings, then subsequent tiers multiply that bonus.

### Mining Drone Upgrades

Mining Drones use standard 2x tier upgrades:

| Tier | Name | Required | Cost | Effect |
|------|------|----------|------|--------|
| 1 | Reinforced Drill Bits | 1 | 1K | 2x Mining Drones |
| 2 | Autonomous Navigation | 5 | 10K | 2x Mining Drones |
| 3 | Swarm Coordination | 25 | 100K | 2x Mining Drones |
| 4 | Deep Mining Protocols | 50 | 1M | 2x Mining Drones |
| 5 | Quantum Scanners | 100 | 10M | 2x Mining Drones |
| 6 | Adaptive Algorithms | 150 | 100M | 2x Mining Drones |
| 7 | Nano-repair Systems | 200 | 1B | 2x Mining Drones |
| 8 | Hive Mind Link | 250 | 10B | 2x Mining Drones |
| 9 | Temporal Prospecting | 300 | 100B | 2x Mining Drones |
| 10 | Dark Matter Sensors | 350 | 1T | 2x Mining Drones |
| 11 | Reality Mining | 400 | 10T | 2x Mining Drones |
| 12 | Dimensional Excavation | 450 | 100T | 2x Mining Drones |
| 13 | Infinite Fuel Cells | 500 | 1Q | 2x Mining Drones |
| 14 | Transcendent Automation | 550 | 10Q | 2x Mining Drones |
| 15 | Ultimate Drone Mastery | 600 | 100Q | 2x Mining Drones |

### Base Costs by Producer

| Producer | Base Upgrade Cost | Tier 15 Cost |
|----------|-------------------|--------------|
| Solar Panel | 100 | 10No |
| Mining Drone | 1K | 100Q |
| Asteroid Mine | 11K | 1.1Qi |
| Orbital Station | 120K | 12Qi |
| Lunar Colony | 1.3M | 130Qi |
| Planetary Harvester | 14M | 1.4Sx |
| Fusion Reactor | 200M | 20Sx |
| Dyson Swarm | 3.3B | 330Sx |
| Dyson Sphere | 51B | 5.1Sp |
| Star Forge | 750B | 75Sp |
| Neutron Harvester | 10T | 1Oc |
| Black Hole Tap | 140T | 14Oc |
| Quantum Vacuum | 1.7Q | 170Oc |
| Galactic Core | 21Q | 2.1No |
| Dark Matter Engine | 260Q | 26No |
| Cosmic String | 3.1Qi | 310No |
| Multiverse Gate | 710Qi | 71Dc |
| Reality Compiler | 12Sx | 1.2Un |
| Entropy Reverser | 190Sx | 19Un |
| Big Bang Catalyst | 5.4Sp | 540Un |

---

## Drone Network Upgrades (36 total)

Mining Drones gain special synergies with ALL other building types through the Drone Network system.

### Network Upgrades (18)

Each building (3-20) has a corresponding network upgrade:

| ID | Name | Requirement | Cost | Effect |
|----|------|-------------|------|--------|
| 7001 | Asteroid Mining Network | 1 Drone + 15 Mines | 6.6K | 2x Drones, +1%/50 drones |
| 7002 | Orbital Drone Network | 1 Drone + 15 Stations | 72K | 2x Drones, +1%/50 drones |
| 7003 | Lunar Drone Network | 1 Drone + 15 Colonies | 780K | 2x Drones, +1%/50 drones |
| 7004 | Planetary Drone Network | 1 Drone + 15 Harvesters | 8.4M | 2x Drones, +1%/50 drones |
| 7005 | Fusion Drone Network | 1 Drone + 15 Reactors | 120M | 2x Drones, +1%/50 drones |
| 7006 | Swarm Drone Network | 1 Drone + 15 Swarms | 2B | 2x Drones, +1%/50 drones |
| 7007 | Sphere Drone Network | 1 Drone + 15 Spheres | 30.6B | 2x Drones, +1%/50 drones |
| 7008 | Forge Drone Network | 1 Drone + 15 Forges | 450B | 2x Drones, +1%/50 drones |
| 7009 | Neutron Drone Network | 1 Drone + 15 Harvesters | 6T | 2x Drones, +1%/50 drones |
| 7010 | Void Drone Network | 1 Drone + 15 Taps | 84T | 2x Drones, +1%/50 drones |
| 7011 | Vacuum Drone Network | 1 Drone + 15 Vacuums | 1Q | 2x Drones, +1%/50 drones |
| 7012 | Core Drone Network | 1 Drone + 15 Cores | 12.6Q | 2x Drones, +1%/50 drones |
| 7013 | Dark Drone Network | 1 Drone + 15 Engines | 156Q | 2x Drones, +1%/50 drones |
| 7014 | String Drone Network | 1 Drone + 15 Strings | 1.86Qi | 2x Drones, +1%/50 drones |
| 7015 | Gate Drone Network | 1 Drone + 15 Gates | 426Qi | 2x Drones, +1%/50 drones |
| 7016 | Compiler Drone Network | 1 Drone + 15 Compilers | 72Sx | 2x Drones, +1%/50 drones |
| 7017 | Entropy Drone Network | 1 Drone + 15 Reversers | 11.4Sp | 2x Drones, +1%/50 drones |
| 7018 | Catalyst Drone Network | 1 Drone + 15 Catalysts | 3.24Oc | 2x Drones, +1%/50 drones |

### Workforce Upgrades (18)

Each network also has a Workforce upgrade that gives the target building +1% E/s per 50 drones:

| ID | Name | Requirement | Cost | Effect |
|----|------|-------------|------|--------|
| 7101 | Asteroid Mine Workforce | 50 Drones + 15 Mines | 33K | +1% Mine E/s per 50 drones |
| 7102 | Orbital Station Workforce | 50 Drones + 15 Stations | 360K | +1% Station E/s per 50 drones |
| ... | ... | ... | ... | ... |
| 7118 | Big Bang Catalyst Workforce | 50 Drones + 15 Catalysts | 16.2Oc | +1% Catalyst E/s per 50 drones |

**Strategy**: With 200 Mining Drones and all Workforce upgrades, every building gets +4% E/s bonus!

---

## Synergy Upgrades (38 total)

Synergy upgrades create production chains where one producer type boosts another. There are **19 forward synergies** and **19 reverse synergies**.

### Effect

Each synergy upgrade grants **+2% bonus per source producer** to the target producer (capped at 2.5x maximum).

### Formula
```
Target Multiplier = min(2.5, 1.0 + (0.02 × source_count))
```

Example: With 100 Mining Drones and Drone-Mine Link:
- Asteroid Mine bonus = min(2.5, 1.0 + (0.02 × 100)) = 2.5x multiplier (capped)

### Forward Synergies (19)

| ID | Name | Requirement | Cost | Effect |
|----|------|-------------|------|--------|
| 8001 | Drone-Mine Link | 15 Drones + 15 Mines | 100K | Drones boost Mines |
| 8002 | Mine-Station Link | 15 Mines + 15 Stations | 1.1M | Mines boost Stations |
| 8003 | Station-Lunar Link | 15 Stations + 15 Colonies | 12M | Stations boost Colonies |
| 8004 | Lunar-Planetary Link | 15 Colonies + 15 Harvesters | 130M | Colonies boost Harvesters |
| 8005 | Planetary-Fusion Link | 15 Harvesters + 15 Reactors | 1.4B | Harvesters boost Reactors |
| 8006 | Fusion-Swarm Link | 15 Reactors + 15 Swarms | 20B | Reactors boost Swarms |
| 8007 | Swarm-Sphere Link | 15 Swarms + 15 Spheres | 330B | Swarms boost Spheres |
| 8008 | Sphere-Forge Link | 15 Spheres + 15 Forges | 5.1T | Spheres boost Forges |
| 8009 | Forge-Neutron Link | 15 Forges + 15 Harvesters | 75T | Forges boost Neutron Harvesters |
| 8010 | Neutron-Void Link | 15 N.Harvesters + 15 Taps | 1Q | N.Harvesters boost Taps |
| 8011 | Void-Vacuum Link | 15 Taps + 15 Vacuums | 14Q | Taps boost Vacuums |
| 8012 | Vacuum-Core Link | 15 Vacuums + 15 Cores | 170Q | Vacuums boost Cores |
| 8013 | Core-Dark Link | 15 Cores + 15 Engines | 2.1Qi | Cores boost Dark Matter |
| 8014 | Dark-String Link | 15 Engines + 15 Strings | 26Qi | Dark Matter boosts Strings |
| 8015 | String-Gate Link | 15 Strings + 15 Gates | 310Qi | Strings boost Gates |
| 8016 | Gate-Compiler Link | 15 Gates + 15 Compilers | 71Sx | Gates boost Compilers |
| 8017 | Compiler-Entropy Link | 15 Compilers + 15 Reversers | 12Sp | Compilers boost Reversers |
| 8018 | Entropy-Catalyst Link | 15 Reversers + 15 Catalysts | 1.9Oc | Reversers boost Catalysts |
| 8019 | Solar-Catalyst Link | 15 Panels + 15 Catalysts | 10K | Panels boost Catalysts |

### Reverse Synergies (19)

| ID | Name | Requirement | Cost | Effect |
|----|------|-------------|------|--------|
| 8020 | Mine-Drone Resonance | 15 Mines + 15 Drones | 1M | Mines boost Drones |
| 8021 | Station-Mine Resonance | 15 Stations + 15 Mines | 11M | Stations boost Mines |
| ... | ... | ... | ... | ... |
| 8037 | Catalyst-Entropy Resonance | 15 Catalysts + 15 Reversers | 19Oc | Catalysts boost Reversers |
| 8038 | Catalyst-Solar Resonance | 15 Catalysts + 15 Panels | 54Oc | Catalysts boost Panels |

Reverse synergies cost 10x their forward counterpart but allow later buildings to boost earlier ones!

---

## Global Upgrades (10 total)

Global upgrades boost **all production** and unlock based on your total energy per second.

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9001 | Efficient Wiring | 10 | 1K | +5% all production |
| 9002 | Central Grid | 100 | 10K | +10% all production |
| 9003 | Dark Energy Tap | 1K | 100K | +15% all production |
| 9004 | Quantum Entanglement | 10K | 1M | +25% all production |
| 9005 | Dimensional Siphon | 100K | 10M | +35% all production |
| 9006 | Universal Conduit | 1M | 100M | +50% all production |
| 9007 | Cosmic Harmonics | 10M | 1B | +75% all production |
| 9008 | Reality Tap | 100M | 10B | +100% all production |
| 9009 | Multiverse Channel | 1B | 100B | +150% all production |
| 9010 | Absolute Energy | 10B | 1T | +200% all production |

### Cumulative Global Multiplier

If you purchase all global upgrades:
```
1.05 × 1.10 × 1.15 × 1.25 × 1.35 × 1.50 × 1.75 × 2.00 × 2.50 × 3.00 = 68.67x
```

---

## Manual/Click Upgrades (10 total)

Manual upgrades double your manual mining (Space bar) power. They unlock based on total clicks.

| ID | Name | Required Clicks | Cost | Effect |
|----|------|-----------------|------|--------|
| 9101 | Reinforced Pickaxe | 50 | 500 | 2x manual mining |
| 9102 | Power Gauntlets | 200 | 5K | 2x manual mining |
| 9103 | Neural Interface | 1K | 50K | 2x manual mining |
| 9104 | Quantum Harvester | 5K | 500K | 2x manual mining |
| 9105 | Cosmic Grip | 20K | 5M | 2x manual mining |
| 9106 | Reality Punch | 100K | 50M | 2x manual mining |
| 9107 | Dimensional Strike | 500K | 500M | 2x manual mining |
| 9108 | Universal Touch | 2M | 5B | 2x manual mining |
| 9109 | Omnipotent Hand | 10M | 50B | 2x manual mining |
| 9110 | Infinite Power | 50M | 500B | 2x manual mining |

### Cumulative Manual Multiplier

With all manual upgrades:
```
2^10 = 1,024x manual mining power
```

---

## Click E/s Upgrades (10 total)

These upgrades add a percentage of your E/s to each manual click, stacking with the base 5% bonus.

| ID | Name | Required Clicks | Cost | Effect |
|----|------|-----------------|------|--------|
| 9201 | Energy Tap | 1K | 50K | +1% of E/s per click |
| 9202 | Plasma Tap | 10K | 5M | +1% of E/s per click |
| 9203 | Fusion Tap | 100K | 500M | +1% of E/s per click |
| 9204 | Stellar Tap | 1M | 50B | +1% of E/s per click |
| 9205 | Quantum Tap | 10M | 5T | +1% of E/s per click |
| 9206 | Neutron Tap | 50M | 500T | +1% of E/s per click |
| 9207 | Void Tap | 100M | 500Q | +1% of E/s per click |
| 9208 | Antimatter Tap | 250M | 500Qi | +1% of E/s per click |
| 9209 | Dark Energy Tap | 500M | 5Sp | +1% of E/s per click |
| 9210 | Singularity Tap | 1B | 5No | +1% of E/s per click |

### Click Energy Formula

With all click E/s upgrades:
```
Click Energy = (click_power × manual_multiplier) + (0.15 × total_eps)
```

The base 5% becomes 15% with all 10 upgrades!

---

## Per-Building E/s Upgrades (20 total)

These upgrades grant +1% E/s for each building of a specific type you own.

| ID | Name | Required Count | Cost | Effect |
|----|------|----------------|------|--------|
| 9301 | Solar Network | 300 Panels | 1T | +1% E/s per Panel |
| 9302 | Grandma Collective | 300 Drones | 10T | +1% E/s per Drone |
| 9303 | Mining Consortium | 300 Mines | 100T | +1% E/s per Mine |
| 9304 | Orbital Alliance | 300 Stations | 1Q | +1% E/s per Station |
| 9305 | Lunar Federation | 300 Colonies | 10Q | +1% E/s per Colony |
| 9306 | Planetary Union | 300 Harvesters | 100Q | +1% E/s per Harvester |
| 9307 | Fusion Coalition | 300 Reactors | 1Qi | +1% E/s per Reactor |
| 9308 | Swarm Intelligence | 300 Swarms | 10Qi | +1% E/s per Swarm |
| 9309 | Sphere Collective | 300 Spheres | 100Qi | +1% E/s per Sphere |
| 9310 | Forge Brotherhood | 300 Forges | 1Sx | +1% E/s per Forge |
| 9311 | Neutron Order | 300 N.Harvesters | 10Sx | +1% E/s per N.Harvester |
| 9312 | Void Consortium | 300 Taps | 100Sx | +1% E/s per Tap |
| 9313 | Vacuum Assembly | 300 Vacuums | 1Sp | +1% E/s per Vacuum |
| 9314 | Core Council | 300 Cores | 10Sp | +1% E/s per Core |
| 9315 | Dark Collective | 300 Engines | 100Sp | +1% E/s per Engine |
| 9316 | String Harmony | 300 Strings | 1Oc | +1% E/s per String |
| 9317 | Gate Syndicate | 300 Gates | 10Oc | +1% E/s per Gate |
| 9318 | Compiler Guild | 300 Compilers | 100Oc | +1% E/s per Compiler |
| 9319 | Entropy Monks | 300 Reversers | 1No | +1% E/s per Reverser |
| 9320 | Catalyst Cabal | 300 Catalysts | 10No | +1% E/s per Catalyst |

With 400 of each building type: +400% E/s per upgrade = +8,000% total!

---

## Total Building E/s Upgrades (5 total)

These upgrades grant a small E/s bonus for your total building count across all types.

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9401 | Empire of Energy | 10T | 1Q | +0.1% E/s per building |
| 9402 | Cosmic Dominion | 100Q | 100Qi | +0.1% E/s per building |
| 9403 | Universal Authority | 100Sx | 100Sp | +0.1% E/s per building |
| 9404 | Multiversal Hegemony | 100Oc | 100No | +0.1% E/s per building |
| 9405 | Omniversal Empire | 100Dc | 100Dc | +0.1% E/s per building |

With 5,000 total buildings and all 5 upgrades: +2,500% E/s!

---

## Milestone Upgrades (10 total)

Milestone upgrades provide +10% E/s each when you reach certain energy production thresholds.

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9501 | First Contact | 1K | 1M | +10% E/s |
| 9502 | Stellar Awakening | 1M | 1B | +10% E/s |
| 9503 | Galactic Presence | 1B | 1T | +10% E/s |
| 9504 | Universal Awareness | 1T | 1Q | +10% E/s |
| 9505 | Cosmic Transcendence | 1Q | 1Qi | +10% E/s |
| 9506 | Dimensional Mastery | 1Qi | 1Sx | +10% E/s |
| 9507 | Reality Dominion | 1Sx | 1Sp | +10% E/s |
| 9508 | Existential Authority | 1Sp | 1Oc | +10% E/s |
| 9509 | Omnipotent Vision | 1Oc | 1No | +10% E/s |
| 9510 | Absolute Supremacy | 1No | 1Dc | +10% E/s |

All 10 milestones: 1.10^10 = **2.59x** cumulative multiplier

---

## Research Upgrades (15 total)

Research upgrades provide +5% E/s each and unlock at specific E/s thresholds.

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9601 | Basic Thermodynamics | 100 | 10K | +5% E/s |
| 9602 | Advanced Materials | 10K | 1M | +5% E/s |
| 9603 | Quantum Mechanics | 1M | 100M | +5% E/s |
| 9604 | Relativistic Physics | 100M | 10B | +5% E/s |
| 9605 | Unified Field Theory | 10B | 1T | +5% E/s |
| 9606 | Zero-Point Energy | 1T | 100T | +5% E/s |
| 9607 | Exotic Matter | 100T | 10Q | +5% E/s |
| 9608 | Spacetime Engineering | 10Q | 1Qi | +5% E/s |
| 9609 | Dimensional Mathematics | 1Qi | 100Qi | +5% E/s |
| 9610 | Multiverse Theory | 100Qi | 10Sx | +5% E/s |
| 9611 | Entropy Reversal | 10Sx | 1Sp | +5% E/s |
| 9612 | Causality Loops | 1Sp | 100Sp | +5% E/s |
| 9613 | Reality Compilation | 100Sp | 10Oc | +5% E/s |
| 9614 | Omniscience Protocols | 10Oc | 1No | +5% E/s |
| 9615 | Creation Algorithms | 1No | 100No | +5% E/s |

All 15 research upgrades: 1.05^15 = **2.08x** cumulative multiplier

---

## Cosmic Cat Upgrades (15 total)

Cosmic Cats are the Solaris equivalent of Cookie Clicker's "Kitten" upgrades. They multiply your E/s based on your **Stellar Essence** (which is 4% per achievement).

### Stellar Essence Formula
```
Stellar Essence = 0.04 × achievement_count
```

At 260 achievements: 10.4 Stellar Essence (1,040%)

### Cosmic Cat Effect
```
E/s Multiplier = 1 + (stellar_essence × bonus)
```

Each Cosmic Cat upgrade adds 0.05 (5%) per 4% Stellar Essence.

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9701 | Cosmic Kittens | 9K | 9M | +5% E/s per 4% Essence |
| 9702 | Astro Cats | 9M | 9B | +5% E/s per 4% Essence |
| 9703 | Nebula Felines | 9B | 9T | +5% E/s per 4% Essence |
| 9704 | Pulsar Prowlers | 9T | 9Q | +5% E/s per 4% Essence |
| 9705 | Quasar Whiskers | 9Q | 9Qi | +5% E/s per 4% Essence |
| 9706 | Void Tabbies | 9Qi | 9Sx | +5% E/s per 4% Essence |
| 9707 | Singularity Siamese | 9Sx | 9Sp | +5% E/s per 4% Essence |
| 9708 | Dimension Calicos | 9Sp | 9Oc | +5% E/s per 4% Essence |
| 9709 | Reality Persians | 9Oc | 9No | +5% E/s per 4% Essence |
| 9710 | Multiverse Maus | 9No | 9Dc | +5% E/s per 4% Essence |
| 9711 | Omniverse Ocicats | 9Dc | 9Un | +5% E/s per 4% Essence |
| 9712 | Eternal Egyptians | 9Un | 9Du | +5% E/s per 4% Essence |
| 9713 | Infinite Abyssinians | 9Du | 9Tr | +5% E/s per 4% Essence |
| 9714 | Transcendent Tabbies | 9Tr | 9Qa | +5% E/s per 4% Essence |
| 9715 | Absolute Angoras | 9Qa | 9Qi | +5% E/s per 4% Essence |

### Example Calculation

With 260 achievements (10.4 Stellar Essence) and all 15 Cosmic Cat upgrades:
```
Per cat: 1 + (10.4 × 0.05) = 1.52x
All 15 cats: 1.52^15 = 695x multiplier!
```

---

## Stellar Essence Upgrades (5 total)

These upgrades increase the effectiveness of your Stellar Essence (making Cosmic Cats even more powerful).

| ID | Name | Required E/s | Cost | Effect |
|----|------|--------------|------|--------|
| 9801 | Essence Amplifier | 10T | 1Q | +5% Stellar Essence effect |
| 9802 | Essence Magnifier | 10Qi | 100Qi | +5% Stellar Essence effect |
| 9803 | Essence Concentrator | 10Sp | 100Sp | +5% Stellar Essence effect |
| 9804 | Essence Crystallizer | 10No | 100No | +5% Stellar Essence effect |
| 9805 | Essence Perfector | 10Dc | 100Dc | +5% Stellar Essence effect |

These multiply the effectiveness of Stellar Essence: 1.05^5 = **1.276x**

---

## Upgrade Cost Reduction

Prestige upgrades can reduce upgrade costs:
- **Bargain Hunter** (40 chips): 10% reduction
- **Thrifty I** (90 chips): 15% reduction
- **Thrifty II** (250 chips): 20% reduction
- **Discount Master** (3,000 chips): 25% reduction
- **Discount Champion** (40,000 chips): 40% reduction
- **Free Shopping** (300,000 chips): 50% reduction

Maximum reduction: **50%** (capped)

---

## Related Pages

- [Producers](producers.md) - Producer stats and tiers
- [Mechanics](mechanics.md) - Cost and production formulas
- [Strategy](strategy.md) - When to buy upgrades
