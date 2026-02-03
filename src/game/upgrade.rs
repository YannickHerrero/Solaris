use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpgradeRequirement {
    ProducerCount {
        producer_id: u32,
        count: u64,
    },
    TotalEnergyPerSecond(f64),
    ManualClicks(u64),
    ProducersPair {
        id_a: u32,
        count_a: u64,
        id_b: u32,
        count_b: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpgradeEffect {
    ProducerMultiplier {
        producer_id: u32,
        multiplier: f64,
    },
    GlobalMultiplier(f64),
    ManualMultiplier(f64),
    Synergy {
        source_id: u32,
        target_id: u32,
        bonus_per_source: f64,
    },
    // Solar Panel special effects (Thousand Rays mechanic)
    CursorBaseMultiplier(f64), // First 3 upgrades: 2x base CPS AND click
    ThousandFingers(f64),      // +X per non-panel building owned (Thousand Rays)
    ThousandFingersMultiplier(f64), // Multiplies Thousand Rays effect
    // Mining Drone special effects (Drone Network)
    GrandmaType {
        building_id: u32,
    }, // Unlocks drone network type, doubles drone CPS
    GrandmaPerBuilding {
        building_id: u32,
        grandmas_per_bonus: u64,
    }, // +1% drone CPS per X drones
    // Flavor upgrades - general bonuses
    CpsPerBuilding {
        producer_id: u32,
        bonus_percent: f64,
    }, // +X% CPS per building of type owned
    CpsPerTotalBuildings(f64), // +X% CPS per total buildings owned
    ClickCpsPercent(f64),      // Manual mining gains +X% of CPS (adds to the 5% base)
    // Milk/Kitten system (renamed to Stellar Essence/Cosmic Cat)
    MilkMultiplier(f64), // Multiplies the effect of stellar essence
    KittenBonus(f64),    // CPS multiplied by (1 + stellar_essence * bonus)
}

#[derive(Debug, Clone)]
pub struct Upgrade {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: f64,
    pub requirement: UpgradeRequirement,
    pub effect: UpgradeEffect,
}

impl Upgrade {
    pub fn all() -> &'static [Upgrade] {
        &UPGRADES
    }
}

// Unlock thresholds for 15 tiers (Cookie Clicker inspired)
const TIER_THRESHOLDS: [u64; 15] = [
    1, 5, 25, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600,
];

// Producer-specific upgrade name themes
const PRODUCER_UPGRADE_NAMES: [[&str; 15]; 20] = [
    // Solar Panel (1) - Thousand Rays progression
    [
        "Enhanced Photovoltaics", // 2x base + click
        "Quantum Absorption",     // 2x base + click
        "Solar Amplification",    // 2x base + click
        "Thousand Rays",          // +0.1 per non-panel building
        "Million Rays",           // 5x Thousand Rays
        "Billion Rays",           // 10x Thousand Rays
        "Trillion Rays",          // 20x Thousand Rays
        "Quadrillion Rays",       // 20x Thousand Rays
        "Quintillion Rays",       // 20x Thousand Rays
        "Sextillion Rays",        // 20x Thousand Rays
        "Septillion Rays",        // 20x Thousand Rays
        "Octillion Rays",         // 20x Thousand Rays
        "Nonillion Rays",         // 20x Thousand Rays
        "Decillion Rays",         // 20x Thousand Rays
        "Undecillion Rays",       // 20x Thousand Rays
    ],
    // Mining Drone (2) - Standard tier progression (drone network types are separate)
    [
        "Reinforced Drill Bits",
        "Autonomous Navigation",
        "Swarm Coordination",
        "Deep Mining Protocols",
        "Quantum Scanners",
        "Adaptive Algorithms",
        "Nano-repair Systems",
        "Hive Mind Link",
        "Temporal Prospecting",
        "Dark Matter Sensors",
        "Reality Mining",
        "Dimensional Excavation",
        "Infinite Fuel Cells",
        "Transcendent Automation",
        "Ultimate Drone Mastery",
    ],
    // Asteroid Mine (3)
    [
        "Deep Core Drilling",
        "Spectral Analysis",
        "Automated Refineries",
        "Quantum Tunneling",
        "Stellar Processing",
        "Galactic Extraction",
        "Cosmic Harvesting",
        "Transcendent Mining",
        "Eternal Excavation",
        "Infinite Resources",
        "Omega Yield",
        "Ascendant Collection",
        "Divine Extraction",
        "Absolute Mining",
        "Ultimate Asteroid Mastery",
    ],
    // Orbital Station (4)
    [
        "Solar Sails",
        "Microwave Transmission",
        "Zero-G Manufacturing",
        "Quantum Relays",
        "Stellar Networks",
        "Galactic Hubs",
        "Cosmic Stations",
        "Transcendent Arrays",
        "Eternal Orbits",
        "Infinite Reach",
        "Omega Transmission",
        "Ascendant Networks",
        "Divine Stations",
        "Absolute Coverage",
        "Ultimate Orbital Mastery",
    ],
    // Lunar Colony (5)
    [
        "Regolith Processing",
        "Helium-3 Extraction",
        "Mass Driver Launch",
        "Quantum Refineries",
        "Stellar Operations",
        "Galactic Bases",
        "Cosmic Colonies",
        "Transcendent Settlements",
        "Eternal Outposts",
        "Infinite Expansion",
        "Omega Colonies",
        "Ascendant Bases",
        "Divine Settlements",
        "Absolute Control",
        "Ultimate Lunar Mastery",
    ],
    // Planetary Harvester (6)
    [
        "Magma Taps",
        "Core Tunneling",
        "Gravitational Siphons",
        "Quantum Drills",
        "Stellar Extraction",
        "Galactic Mining",
        "Cosmic Harvesting",
        "Transcendent Collection",
        "Eternal Yield",
        "Infinite Resources",
        "Omega Extraction",
        "Ascendant Harvesting",
        "Divine Mining",
        "Absolute Power",
        "Ultimate Planetary Mastery",
    ],
    // Fusion Reactor (7)
    [
        "Plasma Containment",
        "Magnetic Bottles",
        "Antimatter Injection",
        "Quantum Fusion",
        "Stellar Cores",
        "Galactic Reactors",
        "Cosmic Fusion",
        "Transcendent Plasma",
        "Eternal Fusion",
        "Infinite Energy",
        "Omega Reactors",
        "Ascendant Fusion",
        "Divine Plasma",
        "Absolute Power",
        "Ultimate Fusion Mastery",
    ],
    // Dyson Swarm (8)
    [
        "Self-Replicating Mirrors",
        "Adaptive Orbits",
        "Quantum Collectors",
        "Neural Coordination",
        "Stellar Swarms",
        "Galactic Arrays",
        "Cosmic Networks",
        "Transcendent Mirrors",
        "Eternal Collectors",
        "Infinite Swarms",
        "Omega Arrays",
        "Ascendant Networks",
        "Divine Collectors",
        "Absolute Coverage",
        "Ultimate Swarm Mastery",
    ],
    // Dyson Sphere (9)
    [
        "Megastructure Maintenance",
        "Stellar Engineering",
        "Dimensional Folding",
        "Quantum Shells",
        "Stellar Enclosure",
        "Galactic Spheres",
        "Cosmic Shells",
        "Transcendent Engineering",
        "Eternal Structures",
        "Infinite Absorption",
        "Omega Spheres",
        "Ascendant Engineering",
        "Divine Structures",
        "Absolute Enclosure",
        "Ultimate Sphere Mastery",
    ],
    // Star Forge (10)
    [
        "Stellar Nurseries",
        "Gravity Wells",
        "Cosmic Genesis",
        "Quantum Creation",
        "Stellar Birth",
        "Galactic Forges",
        "Cosmic Foundries",
        "Transcendent Creation",
        "Eternal Genesis",
        "Infinite Stars",
        "Omega Forges",
        "Ascendant Creation",
        "Divine Genesis",
        "Absolute Power",
        "Ultimate Forge Mastery",
    ],
    // Neutron Harvester (11)
    [
        "Neutron Capture",
        "Density Extraction",
        "Pulsed Energy",
        "Quantum Neutronics",
        "Stellar Density",
        "Galactic Pulsars",
        "Cosmic Neutrons",
        "Transcendent Density",
        "Eternal Pulsars",
        "Infinite Capture",
        "Omega Neutronics",
        "Ascendant Pulsars",
        "Divine Density",
        "Absolute Neutrons",
        "Ultimate Neutron Mastery",
    ],
    // Black Hole Tap (12)
    [
        "Event Horizon Siphon",
        "Hawking Radiation",
        "Singularity Mining",
        "Quantum Tunneling",
        "Stellar Consumption",
        "Galactic Voids",
        "Cosmic Singularities",
        "Transcendent Horizons",
        "Eternal Darkness",
        "Infinite Gravity",
        "Omega Singularities",
        "Ascendant Voids",
        "Divine Horizons",
        "Absolute Consumption",
        "Ultimate Void Mastery",
    ],
    // Quantum Vacuum (13)
    [
        "Zero-Point Extraction",
        "Casimir Collectors",
        "Virtual Particles",
        "Quantum Fluctuations",
        "Stellar Vacuum",
        "Galactic Emptiness",
        "Cosmic Nothing",
        "Transcendent Vacuum",
        "Eternal Zero-Point",
        "Infinite Virtual",
        "Omega Fluctuations",
        "Ascendant Nothing",
        "Divine Emptiness",
        "Absolute Zero",
        "Ultimate Vacuum Mastery",
    ],
    // Galactic Core (14)
    [
        "Core Tapping",
        "Supermassive Siphon",
        "Gravitational Waves",
        "Quantum Core",
        "Stellar Nucleus",
        "Galactic Heart",
        "Cosmic Center",
        "Transcendent Core",
        "Eternal Nucleus",
        "Infinite Gravity",
        "Omega Core",
        "Ascendant Center",
        "Divine Nucleus",
        "Absolute Core",
        "Ultimate Galactic Mastery",
    ],
    // Dark Matter Engine (15)
    [
        "Dark Conversion",
        "Matter Annihilation",
        "Shadow Harvesting",
        "Quantum Darkness",
        "Stellar Shadow",
        "Galactic Dark",
        "Cosmic Shadow",
        "Transcendent Darkness",
        "Eternal Shadow",
        "Infinite Dark",
        "Omega Matter",
        "Ascendant Shadow",
        "Divine Darkness",
        "Absolute Dark",
        "Ultimate Dark Mastery",
    ],
    // Cosmic String (16)
    [
        "String Vibration",
        "Dimensional Resonance",
        "Topological Defects",
        "Quantum Strings",
        "Stellar Vibrations",
        "Galactic Resonance",
        "Cosmic Strings",
        "Transcendent Vibrations",
        "Eternal Resonance",
        "Infinite Strings",
        "Omega Vibrations",
        "Ascendant Resonance",
        "Divine Strings",
        "Absolute Vibration",
        "Ultimate String Mastery",
    ],
    // Multiverse Gate (17)
    [
        "Parallel Siphon",
        "Dimensional Bridge",
        "Reality Tunneling",
        "Quantum Multiverse",
        "Stellar Dimensions",
        "Galactic Parallels",
        "Cosmic Bridges",
        "Transcendent Realities",
        "Eternal Dimensions",
        "Infinite Universes",
        "Omega Gates",
        "Ascendant Bridges",
        "Divine Dimensions",
        "Absolute Reality",
        "Ultimate Gate Mastery",
    ],
    // Reality Compiler (18)
    [
        "Physics Rewrite",
        "Law Manipulation",
        "Constant Editing",
        "Quantum Compilation",
        "Stellar Physics",
        "Galactic Laws",
        "Cosmic Constants",
        "Transcendent Code",
        "Eternal Physics",
        "Infinite Laws",
        "Omega Compilation",
        "Ascendant Code",
        "Divine Physics",
        "Absolute Reality",
        "Ultimate Compiler Mastery",
    ],
    // Entropy Reverser (19)
    [
        "Time Reversal",
        "Order Restoration",
        "Heat Death Denial",
        "Quantum Reversal",
        "Stellar Entropy",
        "Galactic Order",
        "Cosmic Restoration",
        "Transcendent Reversal",
        "Eternal Order",
        "Infinite Time",
        "Omega Entropy",
        "Ascendant Order",
        "Divine Reversal",
        "Absolute Time",
        "Ultimate Entropy Mastery",
    ],
    // Big Bang Catalyst (20)
    [
        "Pocket Creation",
        "Universe Seeding",
        "Cosmic Ignition",
        "Quantum Genesis",
        "Stellar Birth",
        "Galactic Creation",
        "Cosmic Ignition",
        "Transcendent Genesis",
        "Eternal Creation",
        "Infinite Universes",
        "Omega Genesis",
        "Ascendant Creation",
        "Divine Ignition",
        "Absolute Creation",
        "Ultimate Catalyst Mastery",
    ],
];

// Solar Panel upgrade costs - unique scaling
const CURSOR_UPGRADE_COSTS: [f64; 15] = [
    100.0,                                        // Tier 1: Reinforced Index Finger
    500.0,                                        // Tier 2: Carpal Tunnel Prevention
    10_000.0,                                     // Tier 3: Ambidextrous
    100_000.0,                                    // Tier 4: Thousand Fingers
    10_000_000.0,                                 // Tier 5: Million Fingers
    100_000_000.0,                                // Tier 6: Billion Fingers
    1_000_000_000.0,                              // Tier 7: Trillion Fingers
    10_000_000_000.0,                             // Tier 8: Quadrillion Fingers
    10_000_000_000_000.0,                         // Tier 9: Quintillion Fingers
    10_000_000_000_000_000.0,                     // Tier 10: Sextillion Fingers
    10_000_000_000_000_000_000.0,                 // Tier 11: Septillion Fingers
    10_000_000_000_000_000_000_000.0,             // Tier 12: Octillion Fingers
    10_000_000_000_000_000_000_000_000.0,         // Tier 13: Nonillion Fingers
    10_000_000_000_000_000_000_000_000_000.0,     // Tier 14: Decillion Fingers
    10_000_000_000_000_000_000_000_000_000_000.0, // Tier 15: Undecillion Fingers
];

// Solar Panel unlock thresholds
const CURSOR_THRESHOLDS: [u64; 15] = [
    1, 1, 10, 25, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550,
];

// Base costs for first tier of each producer's upgrades
const PRODUCER_BASE_COSTS: [f64; 20] = [
    100.0,                               // Solar Panel - special handling
    1_000.0,                             // Mining Drone
    11_000.0,                            // Asteroid Mine
    120_000.0,                           // Orbital Station
    1_300_000.0,                         // Lunar Colony
    14_000_000.0,                        // Planetary Harvester
    200_000_000.0,                       // Fusion Reactor
    3_300_000_000.0,                     // Dyson Swarm
    51_000_000_000.0,                    // Dyson Sphere
    750_000_000_000.0,                   // Star Forge
    10_000_000_000_000.0,                // Neutron Harvester
    140_000_000_000_000.0,               // Black Hole Tap
    1_700_000_000_000_000.0,             // Quantum Vacuum
    21_000_000_000_000_000.0,            // Galactic Core
    260_000_000_000_000_000.0,           // Dark Matter Engine
    3_100_000_000_000_000_000.0,         // Cosmic String
    710_000_000_000_000_000_000.0,       // Multiverse Gate
    12_000_000_000_000_000_000_000.0,    // Reality Compiler
    190_000_000_000_000_000_000_000.0,   // Entropy Reverser
    5_400_000_000_000_000_000_000_000.0, // Big Bang Catalyst
];

// Producer names for descriptions
const PRODUCER_NAMES: [&str; 20] = [
    "Solar Panels",
    "Mining Drones",
    "Asteroid Mines",
    "Orbital Stations",
    "Lunar Colonies",
    "Planetary Harvesters",
    "Fusion Reactors",
    "Dyson Swarms",
    "Dyson Spheres",
    "Star Forges",
    "Neutron Harvesters",
    "Black Hole Taps",
    "Quantum Vacuums",
    "Galactic Cores",
    "Dark Matter Engines",
    "Cosmic Strings",
    "Multiverse Gates",
    "Reality Compilers",
    "Entropy Reversers",
    "Big Bang Catalysts",
];

// Generate all upgrades programmatically
static UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| {
    let mut upgrades = Vec::new();

    // ============ SOLAR PANEL (Producer 1) - Thousand Rays System ============
    // First 3 upgrades: 2x base CPS AND click power
    for tier in 0..3 {
        let id = 100 + (tier + 1) as u32;
        let name = PRODUCER_UPGRADE_NAMES[0][tier];
        upgrades.push(Upgrade {
            id,
            name,
            description: "Manual mining and Solar Panels are twice as efficient.",
            cost: CURSOR_UPGRADE_COSTS[tier],
            requirement: UpgradeRequirement::ProducerCount {
                producer_id: 1,
                count: CURSOR_THRESHOLDS[tier],
            },
            effect: UpgradeEffect::CursorBaseMultiplier(2.0),
        });
    }

    // Tier 4: Thousand Rays - +0.1 per non-panel building
    upgrades.push(Upgrade {
        id: 104,
        name: PRODUCER_UPGRADE_NAMES[0][3],
        description:
            "Manual mining and Solar Panels gain +0.1 energy for each non-panel building owned.",
        cost: CURSOR_UPGRADE_COSTS[3],
        requirement: UpgradeRequirement::ProducerCount {
            producer_id: 1,
            count: CURSOR_THRESHOLDS[3],
        },
        effect: UpgradeEffect::ThousandFingers(0.1),
    });

    // Tier 5: Million Rays - 5x Thousand Rays
    upgrades.push(Upgrade {
        id: 105,
        name: PRODUCER_UPGRADE_NAMES[0][4],
        description: "Multiplies the gain from Thousand Rays by 5.",
        cost: CURSOR_UPGRADE_COSTS[4],
        requirement: UpgradeRequirement::ProducerCount {
            producer_id: 1,
            count: CURSOR_THRESHOLDS[4],
        },
        effect: UpgradeEffect::ThousandFingersMultiplier(5.0),
    });

    // Tier 6: Billion Rays - 10x Thousand Rays
    upgrades.push(Upgrade {
        id: 106,
        name: PRODUCER_UPGRADE_NAMES[0][5],
        description: "Multiplies the gain from Thousand Rays by 10.",
        cost: CURSOR_UPGRADE_COSTS[5],
        requirement: UpgradeRequirement::ProducerCount {
            producer_id: 1,
            count: CURSOR_THRESHOLDS[5],
        },
        effect: UpgradeEffect::ThousandFingersMultiplier(10.0),
    });

    // Tiers 7-15: 20x Thousand Rays each
    for tier in 6..15 {
        let id = 100 + (tier + 1) as u32;
        let name = PRODUCER_UPGRADE_NAMES[0][tier];
        upgrades.push(Upgrade {
            id,
            name,
            description: "Multiplies the gain from Thousand Rays by 20.",
            cost: CURSOR_UPGRADE_COSTS[tier],
            requirement: UpgradeRequirement::ProducerCount {
                producer_id: 1,
                count: CURSOR_THRESHOLDS[tier],
            },
            effect: UpgradeEffect::ThousandFingersMultiplier(20.0),
        });
    }

    // ============ STANDARD PRODUCER UPGRADES (Producers 2-20) ============
    // All use flat 2x multiplier (Cookie Clicker style)
    for producer_idx in 1..20 {
        let producer_id = (producer_idx + 1) as u32;
        let base_cost = PRODUCER_BASE_COSTS[producer_idx];

        for tier in 0..15 {
            let id = producer_id * 100 + (tier + 1) as u32;
            let name = PRODUCER_UPGRADE_NAMES[producer_idx][tier];
            let producer_name = PRODUCER_NAMES[producer_idx];

            // All tiers give 2x (Cookie Clicker style)
            let multiplier = 2.0;

            let description =
                Box::leak(format!("{} are twice as efficient.", producer_name).into_boxed_str());

            // Cost scales by 10x per tier (Cookie Clicker style)
            let cost = base_cost * 10.0_f64.powi(tier as i32);

            upgrades.push(Upgrade {
                id,
                name,
                description,
                cost,
                requirement: UpgradeRequirement::ProducerCount {
                    producer_id,
                    count: TIER_THRESHOLDS[tier],
                },
                effect: UpgradeEffect::ProducerMultiplier {
                    producer_id,
                    multiplier,
                },
            });
        }
    }

    // ============ DRONE NETWORK UPGRADES ============
    // Each unlocks a "type" of drone network for buildings 3-20 (18 types)
    // Requires: 1 Mining Drone + 15 of the associated building
    // Effect: 2x Drone CPS + 1% per X drones
    let drone_network_types: [(u32, &str, u64); 18] = [
        // (building_id, name, drones_per_bonus)
        (3, "Asteroid Mining Network", 50), // +1% per 50 drones
        (4, "Orbital Drone Network", 50),
        (5, "Lunar Drone Network", 50),
        (6, "Planetary Drone Network", 50),
        (7, "Fusion Drone Network", 50),
        (8, "Swarm Drone Network", 50),
        (9, "Sphere Drone Network", 50),
        (10, "Forge Drone Network", 50),
        (11, "Neutron Drone Network", 50),
        (12, "Void Drone Network", 50),
        (13, "Vacuum Drone Network", 50),
        (14, "Core Drone Network", 50),
        (15, "Dark Drone Network", 50),
        (16, "String Drone Network", 50),
        (17, "Gate Drone Network", 50),
        (18, "Compiler Drone Network", 50),
        (19, "Entropy Drone Network", 50),
        (20, "Catalyst Drone Network", 50),
    ];

    for (idx, (building_id, name, drones_per_bonus)) in drone_network_types.iter().enumerate() {
        let upgrade_id = 7001 + idx as u32;
        // Cost scales with building's base cost
        let building_base_cost = PRODUCER_BASE_COSTS[(*building_id - 1) as usize];
        let cost = building_base_cost * 6.0; // ~6x base building cost

        let building_name = PRODUCER_NAMES[(*building_id - 1) as usize];
        let description = Box::leak(
            format!(
                "Mining Drones are twice as efficient. Mining Drones gain +1% CPS per {} drones. {} gain +1% CPS per {} drones.",
                drones_per_bonus,
                building_name,
                drones_per_bonus
            )
            .into_boxed_str(),
        );

        upgrades.push(Upgrade {
            id: upgrade_id,
            name,
            description,
            cost,
            requirement: UpgradeRequirement::ProducersPair {
                id_a: 2, // Mining Drone
                count_a: 1,
                id_b: *building_id,
                count_b: 15,
            },
            effect: UpgradeEffect::GrandmaType {
                building_id: *building_id,
            },
        });

        // Also add the per-building bonus upgrade (the "+1% per X drones" part)
        let bonus_upgrade_id = 7101 + idx as u32;
        upgrades.push(Upgrade {
            id: bonus_upgrade_id,
            name: Box::leak(format!("{} Workforce", building_name).into_boxed_str()),
            description: Box::leak(
                format!(
                    "{} gain +1% CPS for every {} Mining Drones you own.",
                    building_name, drones_per_bonus
                )
                .into_boxed_str(),
            ),
            cost: cost * 5.0, // More expensive follow-up
            requirement: UpgradeRequirement::ProducersPair {
                id_a: 2,
                count_a: *drones_per_bonus,
                id_b: *building_id,
                count_b: 15,
            },
            effect: UpgradeEffect::GrandmaPerBuilding {
                building_id: *building_id,
                grandmas_per_bonus: *drones_per_bonus,
            },
        });
    }

    // Generate synergy upgrades (38 total - each adjacent pair has 2 synergies, one in each direction)
    // Forward synergies: Building N boosts Building N+1
    // Reverse synergies: Building N+1 boosts Building N
    let synergy_pairs: [(u32, u32, &str, bool); 38] = [
        // Forward synergies (2->3, 3->4, ... 19->20, plus 1->20 wrap-around)
        (2, 3, "Drone-Mine Link", false),
        (3, 4, "Mine-Station Link", false),
        (4, 5, "Station-Lunar Link", false),
        (5, 6, "Lunar-Planetary Link", false),
        (6, 7, "Planetary-Fusion Link", false),
        (7, 8, "Fusion-Swarm Link", false),
        (8, 9, "Swarm-Sphere Link", false),
        (9, 10, "Sphere-Forge Link", false),
        (10, 11, "Forge-Neutron Link", false),
        (11, 12, "Neutron-Void Link", false),
        (12, 13, "Void-Vacuum Link", false),
        (13, 14, "Vacuum-Core Link", false),
        (14, 15, "Core-Dark Link", false),
        (15, 16, "Dark-String Link", false),
        (16, 17, "String-Gate Link", false),
        (17, 18, "Gate-Compiler Link", false),
        (18, 19, "Compiler-Entropy Link", false),
        (19, 20, "Entropy-Catalyst Link", false),
        (1, 20, "Solar-Catalyst Link", false),
        // Reverse synergies (3->2, 4->3, ... 20->19, plus 20->1 wrap-around)
        (3, 2, "Mine-Drone Resonance", true),
        (4, 3, "Station-Mine Resonance", true),
        (5, 4, "Lunar-Station Resonance", true),
        (6, 5, "Planetary-Lunar Resonance", true),
        (7, 6, "Fusion-Planetary Resonance", true),
        (8, 7, "Swarm-Fusion Resonance", true),
        (9, 8, "Sphere-Swarm Resonance", true),
        (10, 9, "Forge-Sphere Resonance", true),
        (11, 10, "Neutron-Forge Resonance", true),
        (12, 11, "Void-Neutron Resonance", true),
        (13, 12, "Vacuum-Void Resonance", true),
        (14, 13, "Core-Vacuum Resonance", true),
        (15, 14, "Dark-Core Resonance", true),
        (16, 15, "String-Dark Resonance", true),
        (17, 16, "Gate-String Resonance", true),
        (18, 17, "Compiler-Gate Resonance", true),
        (19, 18, "Entropy-Compiler Resonance", true),
        (20, 19, "Catalyst-Entropy Resonance", true),
        (20, 1, "Catalyst-Solar Resonance", true),
    ];

    for (idx, (id_a, id_b, name, is_reverse)) in synergy_pairs.iter().enumerate() {
        let synergy_id = 8001 + idx as u32;
        // Cost scales based on which producers are involved
        // Reverse synergies cost more as they unlock later
        let base_cost = PRODUCER_BASE_COSTS[(*id_a - 1) as usize] * 100.0;
        let cost = if *is_reverse {
            base_cost * 10.0
        } else {
            base_cost
        };
        let source_name = PRODUCER_NAMES[(*id_a - 1) as usize];
        let target_name = PRODUCER_NAMES[(*id_b - 1) as usize];
        let description = Box::leak(
            format!("{} gain +2% CPS per {} owned.", target_name, source_name).into_boxed_str(),
        );

        upgrades.push(Upgrade {
            id: synergy_id,
            name,
            description,
            cost,
            requirement: UpgradeRequirement::ProducersPair {
                id_a: *id_a,
                count_a: 15,
                id_b: *id_b,
                count_b: 15,
            },
            effect: UpgradeEffect::Synergy {
                source_id: *id_a,
                target_id: *id_b,
                bonus_per_source: 0.02,
            },
        });
    }

    // Global upgrades (10 total)
    let global_upgrades: [(u32, &str, &str, f64, f64, f64); 10] = [
        (
            9001,
            "Efficient Wiring",
            "All production +5%",
            10.0,
            1_000.0,
            1.05,
        ),
        (
            9002,
            "Central Grid",
            "All production +10%",
            100.0,
            10_000.0,
            1.10,
        ),
        (
            9003,
            "Dark Energy Tap",
            "All production +15%",
            1_000.0,
            100_000.0,
            1.15,
        ),
        (
            9004,
            "Quantum Entanglement",
            "All production +25%",
            10_000.0,
            1_000_000.0,
            1.25,
        ),
        (
            9005,
            "Dimensional Siphon",
            "All production +35%",
            100_000.0,
            10_000_000.0,
            1.35,
        ),
        (
            9006,
            "Universal Conduit",
            "All production +50%",
            1_000_000.0,
            100_000_000.0,
            1.50,
        ),
        (
            9007,
            "Cosmic Harmonics",
            "All production +75%",
            10_000_000.0,
            1_000_000_000.0,
            1.75,
        ),
        (
            9008,
            "Reality Tap",
            "All production +100%",
            100_000_000.0,
            10_000_000_000.0,
            2.00,
        ),
        (
            9009,
            "Multiverse Channel",
            "All production +150%",
            1_000_000_000.0,
            100_000_000_000.0,
            2.50,
        ),
        (
            9010,
            "Absolute Energy",
            "All production +200%",
            10_000_000_000.0,
            1_000_000_000_000.0,
            3.00,
        ),
    ];

    for (id, name, desc, req_eps, cost, mult) in global_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(req_eps),
            effect: UpgradeEffect::GlobalMultiplier(mult),
        });
    }

    // Manual mining upgrades (10 total)
    let manual_upgrades: [(u32, &str, u64, f64); 10] = [
        (9101, "Reinforced Pickaxe", 50, 500.0),
        (9102, "Power Gauntlets", 200, 5_000.0),
        (9103, "Neural Interface", 1_000, 50_000.0),
        (9104, "Quantum Harvester", 5_000, 500_000.0),
        (9105, "Cosmic Grip", 20_000, 5_000_000.0),
        (9106, "Reality Punch", 100_000, 50_000_000.0),
        (9107, "Dimensional Strike", 500_000, 500_000_000.0),
        (9108, "Universal Touch", 2_000_000, 5_000_000_000.0),
        (9109, "Omnipotent Hand", 10_000_000, 50_000_000_000.0),
        (9110, "Infinite Power", 50_000_000, 500_000_000_000.0),
    ];

    for (id, name, clicks, cost) in manual_upgrades {
        let description = "Manual mining is twice as powerful";
        upgrades.push(Upgrade {
            id,
            name,
            description,
            cost,
            requirement: UpgradeRequirement::ManualClicks(clicks),
            effect: UpgradeEffect::ManualMultiplier(2.0),
        });
    }

    // ============ FLAVOR UPGRADES (Cookie Clicker style) ============
    // These provide various bonuses based on building ownership, clicks, etc.

    // Click CPS bonuses - increase the % of CPS gained per click
    let click_cps_upgrades: [(u32, &str, &str, f64, u64, f64); 10] = [
        (
            9201,
            "Energy Tap",
            "Manual mining gains +1% of your CPS",
            50_000.0,
            1_000,
            0.01,
        ),
        (
            9202,
            "Plasma Tap",
            "Manual mining gains +1% of your CPS",
            5_000_000.0,
            10_000,
            0.01,
        ),
        (
            9203,
            "Fusion Tap",
            "Manual mining gains +1% of your CPS",
            500_000_000.0,
            100_000,
            0.01,
        ),
        (
            9204,
            "Stellar Tap",
            "Manual mining gains +1% of your CPS",
            50_000_000_000.0,
            1_000_000,
            0.01,
        ),
        (
            9205,
            "Quantum Tap",
            "Manual mining gains +1% of your CPS",
            5_000_000_000_000.0,
            10_000_000,
            0.01,
        ),
        (
            9206,
            "Neutron Tap",
            "Manual mining gains +1% of your CPS",
            500_000_000_000_000.0,
            50_000_000,
            0.01,
        ),
        (
            9207,
            "Void Tap",
            "Manual mining gains +1% of your CPS",
            500_000_000_000_000_000.0,
            100_000_000,
            0.01,
        ),
        (
            9208,
            "Antimatter Tap",
            "Manual mining gains +1% of your CPS",
            500_000_000_000_000_000_000.0,
            250_000_000,
            0.01,
        ),
        (
            9209,
            "Dark Energy Tap",
            "Manual mining gains +1% of your CPS",
            5e23,
            500_000_000,
            0.01,
        ),
        (
            9210,
            "Singularity Tap",
            "Manual mining gains +1% of your CPS",
            5e27,
            1_000_000_000,
            0.01,
        ),
    ];

    for (id, name, desc, cost, clicks, bonus) in click_cps_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::ManualClicks(clicks),
            effect: UpgradeEffect::ClickCpsPercent(bonus),
        });
    }

    // Per-building CPS bonuses - +1% CPS per building of a type
    // These unlock at high building counts and encourage diverse building strategies
    let per_building_upgrades: [(u32, u32, &str, u64, f64); 20] = [
        (9301, 1, "Solar Network", 300, 1e12),
        (9302, 2, "Grandma Collective", 300, 1e13),
        (9303, 3, "Mining Consortium", 300, 1e14),
        (9304, 4, "Orbital Alliance", 300, 1e15),
        (9305, 5, "Lunar Federation", 300, 1e16),
        (9306, 6, "Planetary Union", 300, 1e17),
        (9307, 7, "Fusion Coalition", 300, 1e18),
        (9308, 8, "Swarm Intelligence", 300, 1e19),
        (9309, 9, "Sphere Collective", 300, 1e20),
        (9310, 10, "Forge Brotherhood", 300, 1e21),
        (9311, 11, "Neutron Order", 300, 1e22),
        (9312, 12, "Void Consortium", 300, 1e23),
        (9313, 13, "Vacuum Assembly", 300, 1e24),
        (9314, 14, "Core Council", 300, 1e25),
        (9315, 15, "Dark Collective", 300, 1e26),
        (9316, 16, "String Harmony", 300, 1e27),
        (9317, 17, "Gate Syndicate", 300, 1e28),
        (9318, 18, "Compiler Guild", 300, 1e29),
        (9319, 19, "Entropy Monks", 300, 1e30),
        (9320, 20, "Catalyst Cabal", 300, 1e31),
    ];

    for (id, producer_id, name, count, cost) in per_building_upgrades {
        let producer_name = PRODUCER_NAMES[(producer_id - 1) as usize];
        let description =
            Box::leak(format!("+1% CPS for every {} you own.", producer_name).into_boxed_str());
        upgrades.push(Upgrade {
            id,
            name,
            description,
            cost,
            requirement: UpgradeRequirement::ProducerCount { producer_id, count },
            effect: UpgradeEffect::CpsPerBuilding {
                producer_id,
                bonus_percent: 0.01,
            },
        });
    }

    // Total buildings CPS bonuses - +X% CPS per total building
    let total_building_upgrades: [(u32, &str, &str, u64, f64, f64); 5] = [
        (
            9401,
            "Empire of Energy",
            "+0.1% CPS per building owned",
            100,
            1e15,
            0.001,
        ),
        (
            9402,
            "Cosmic Dominion",
            "+0.1% CPS per building owned",
            500,
            1e20,
            0.001,
        ),
        (
            9403,
            "Universal Authority",
            "+0.1% CPS per building owned",
            1000,
            1e25,
            0.001,
        ),
        (
            9404,
            "Multiversal Hegemony",
            "+0.1% CPS per building owned",
            2500,
            1e30,
            0.001,
        ),
        (
            9405,
            "Omniversal Empire",
            "+0.1% CPS per building owned",
            5000,
            1e35,
            0.001,
        ),
    ];

    for (id, name, desc, _total_count, cost, bonus) in total_building_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(cost / 100.0), // Unlock at ~1% of cost in EPS
            effect: UpgradeEffect::CpsPerTotalBuildings(bonus),
        });
    }

    // Milestone upgrades - big bonuses for reaching certain totals
    let milestone_upgrades: [(u32, &str, &str, f64, f64); 10] = [
        (
            9501,
            "First Contact",
            "You've made contact with the cosmos. +10% CPS",
            1e6,
            1.10,
        ),
        (
            9502,
            "Stellar Awakening",
            "The stars notice you. +10% CPS",
            1e9,
            1.10,
        ),
        (
            9503,
            "Galactic Presence",
            "You are known throughout the galaxy. +10% CPS",
            1e12,
            1.10,
        ),
        (
            9504,
            "Universal Awareness",
            "The universe acknowledges you. +10% CPS",
            1e15,
            1.10,
        ),
        (
            9505,
            "Cosmic Transcendence",
            "You have transcended. +10% CPS",
            1e18,
            1.10,
        ),
        (
            9506,
            "Dimensional Mastery",
            "Dimensions bow before you. +10% CPS",
            1e21,
            1.10,
        ),
        (
            9507,
            "Reality Dominion",
            "Reality is yours to command. +10% CPS",
            1e24,
            1.10,
        ),
        (
            9508,
            "Existential Authority",
            "Existence itself obeys. +10% CPS",
            1e27,
            1.10,
        ),
        (
            9509,
            "Omnipotent Vision",
            "You see all that is. +10% CPS",
            1e30,
            1.10,
        ),
        (
            9510,
            "Absolute Supremacy",
            "You are everything. +10% CPS",
            1e33,
            1.10,
        ),
    ];

    for (id, name, desc, cost, mult) in milestone_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(cost / 1000.0),
            effect: UpgradeEffect::GlobalMultiplier(mult),
        });
    }

    // Research upgrades - unlock at specific EPS thresholds
    let research_upgrades: [(u32, &str, &str, f64, f64); 15] = [
        (
            9601,
            "Basic Thermodynamics",
            "Understanding heat transfer. +5% CPS",
            1e4,
            1.05,
        ),
        (
            9602,
            "Advanced Materials",
            "Stronger, lighter, better. +5% CPS",
            1e6,
            1.05,
        ),
        (
            9603,
            "Quantum Mechanics",
            "The very small, very strange. +5% CPS",
            1e8,
            1.05,
        ),
        (
            9604,
            "Relativistic Physics",
            "Time and space bend. +5% CPS",
            1e10,
            1.05,
        ),
        (
            9605,
            "Unified Field Theory",
            "All forces as one. +5% CPS",
            1e12,
            1.05,
        ),
        (
            9606,
            "Zero-Point Energy",
            "Energy from nothing. +5% CPS",
            1e14,
            1.05,
        ),
        (
            9607,
            "Exotic Matter",
            "Negative mass exists. +5% CPS",
            1e16,
            1.05,
        ),
        (
            9608,
            "Spacetime Engineering",
            "Warp reality itself. +5% CPS",
            1e18,
            1.05,
        ),
        (
            9609,
            "Dimensional Mathematics",
            "Beyond 3D. +5% CPS",
            1e20,
            1.05,
        ),
        (
            9610,
            "Multiverse Theory",
            "Infinite possibilities. +5% CPS",
            1e22,
            1.05,
        ),
        (
            9611,
            "Entropy Reversal",
            "Undoing disorder. +5% CPS",
            1e24,
            1.05,
        ),
        (
            9612,
            "Causality Loops",
            "Effect before cause. +5% CPS",
            1e26,
            1.05,
        ),
        (
            9613,
            "Reality Compilation",
            "Rewriting existence. +5% CPS",
            1e28,
            1.05,
        ),
        (
            9614,
            "Omniscience Protocols",
            "Knowing everything. +5% CPS",
            1e30,
            1.05,
        ),
        (
            9615,
            "Creation Algorithms",
            "Making universes. +5% CPS",
            1e32,
            1.05,
        ),
    ];

    for (id, name, desc, cost, mult) in research_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(cost / 100.0),
            effect: UpgradeEffect::GlobalMultiplier(mult),
        });
    }

    // ============ COSMIC CAT UPGRADES (Cookie Clicker Kitten style) ============
    // CPS is multiplied by (1 + stellar_essence * bonus) for each kitten upgrade
    // Stellar Essence = 4% per achievement (like Cookie Clicker's milk)
    // These stack multiplicatively!
    let kitten_upgrades: [(u32, &str, &str, f64, f64); 15] = [
        (
            9701,
            "Cosmic Kittens",
            "CPS +5% per 4% Stellar Essence",
            9_000_000.0,
            0.05,
        ),
        (
            9702,
            "Astro Cats",
            "CPS +5% per 4% Stellar Essence",
            9_000_000_000.0,
            0.05,
        ),
        (
            9703,
            "Nebula Felines",
            "CPS +5% per 4% Stellar Essence",
            9_000_000_000_000.0,
            0.05,
        ),
        (
            9704,
            "Pulsar Prowlers",
            "CPS +5% per 4% Stellar Essence",
            9_000_000_000_000_000.0,
            0.05,
        ),
        (
            9705,
            "Quasar Whiskers",
            "CPS +5% per 4% Stellar Essence",
            9e18,
            0.05,
        ),
        (
            9706,
            "Void Tabbies",
            "CPS +5% per 4% Stellar Essence",
            9e21,
            0.05,
        ),
        (
            9707,
            "Singularity Siamese",
            "CPS +5% per 4% Stellar Essence",
            9e24,
            0.05,
        ),
        (
            9708,
            "Dimension Calicos",
            "CPS +5% per 4% Stellar Essence",
            9e27,
            0.05,
        ),
        (
            9709,
            "Reality Persians",
            "CPS +5% per 4% Stellar Essence",
            9e30,
            0.05,
        ),
        (
            9710,
            "Multiverse Maus",
            "CPS +5% per 4% Stellar Essence",
            9e33,
            0.05,
        ),
        (
            9711,
            "Omniverse Ocicats",
            "CPS +5% per 4% Stellar Essence",
            9e36,
            0.05,
        ),
        (
            9712,
            "Eternal Egyptians",
            "CPS +5% per 4% Stellar Essence",
            9e39,
            0.05,
        ),
        (
            9713,
            "Infinite Abyssinians",
            "CPS +5% per 4% Stellar Essence",
            9e42,
            0.05,
        ),
        (
            9714,
            "Transcendent Tabbies",
            "CPS +5% per 4% Stellar Essence",
            9e45,
            0.05,
        ),
        (
            9715,
            "Absolute Angoras",
            "CPS +5% per 4% Stellar Essence",
            9e48,
            0.05,
        ),
    ];

    for (id, name, desc, cost, bonus) in kitten_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(cost / 1000.0),
            effect: UpgradeEffect::KittenBonus(bonus),
        });
    }

    // Stellar Essence multiplier upgrades (increases milk effect)
    let essence_upgrades: [(u32, &str, &str, f64, f64); 5] = [
        (
            9801,
            "Essence Amplifier",
            "Stellar Essence effect +5%",
            1e15,
            1.05,
        ),
        (
            9802,
            "Essence Magnifier",
            "Stellar Essence effect +5%",
            1e21,
            1.05,
        ),
        (
            9803,
            "Essence Concentrator",
            "Stellar Essence effect +5%",
            1e27,
            1.05,
        ),
        (
            9804,
            "Essence Crystallizer",
            "Stellar Essence effect +5%",
            1e33,
            1.05,
        ),
        (
            9805,
            "Essence Perfector",
            "Stellar Essence effect +5%",
            1e39,
            1.05,
        ),
    ];

    for (id, name, desc, cost, mult) in essence_upgrades {
        upgrades.push(Upgrade {
            id,
            name,
            description: desc,
            cost,
            requirement: UpgradeRequirement::TotalEnergyPerSecond(cost / 100.0),
            effect: UpgradeEffect::MilkMultiplier(mult),
        });
    }

    upgrades
});
