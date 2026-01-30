use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpgradeRequirement {
    ProducerCount { producer_id: u32, count: u64 },
    TotalEnergyPerSecond(f64),
    ManualClicks(u64),
    ProducersPair { id_a: u32, count_a: u64, id_b: u32, count_b: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpgradeEffect {
    ProducerMultiplier { producer_id: u32, multiplier: f64 },
    GlobalMultiplier(f64),
    ManualMultiplier(f64),
    Synergy { source_id: u32, target_id: u32, bonus_per_source: f64 },
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
const TIER_THRESHOLDS: [u64; 15] = [1, 5, 25, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600];

// Producer-specific upgrade name themes
const PRODUCER_UPGRADE_NAMES: [[&str; 15]; 20] = [
    // Solar Panel (1)
    ["Better Silicon", "Anti-Reflective Coating", "Quantum Dots", "Perovskite Cells", "Stellar Collectors",
     "Galactic Arrays", "Cosmic Absorption", "Transcendent Photons", "Eternal Light Capture", "Infinite Spectrum",
     "Omega Radiance", "Ascendant Luminosity", "Divine Brilliance", "Absolute Efficiency", "Ultimate Solar Mastery"],
    // Mining Drone (2)
    ["Improved Thrusters", "AI Navigation", "Swarm Intelligence", "Quantum Pathfinding", "Stellar Mining",
     "Galactic Excavation", "Cosmic Drilling", "Transcendent Extraction", "Eternal Harvest", "Infinite Yield",
     "Omega Mining", "Ascendant Gathering", "Divine Collection", "Absolute Efficiency", "Ultimate Drone Mastery"],
    // Asteroid Mine (3)
    ["Deep Core Drilling", "Spectral Analysis", "Automated Refineries", "Quantum Tunneling", "Stellar Processing",
     "Galactic Extraction", "Cosmic Harvesting", "Transcendent Mining", "Eternal Excavation", "Infinite Resources",
     "Omega Yield", "Ascendant Collection", "Divine Extraction", "Absolute Mining", "Ultimate Asteroid Mastery"],
    // Orbital Station (4)
    ["Solar Sails", "Microwave Transmission", "Zero-G Manufacturing", "Quantum Relays", "Stellar Networks",
     "Galactic Hubs", "Cosmic Stations", "Transcendent Arrays", "Eternal Orbits", "Infinite Reach",
     "Omega Transmission", "Ascendant Networks", "Divine Stations", "Absolute Coverage", "Ultimate Orbital Mastery"],
    // Lunar Colony (5)
    ["Regolith Processing", "Helium-3 Extraction", "Mass Driver Launch", "Quantum Refineries", "Stellar Operations",
     "Galactic Bases", "Cosmic Colonies", "Transcendent Settlements", "Eternal Outposts", "Infinite Expansion",
     "Omega Colonies", "Ascendant Bases", "Divine Settlements", "Absolute Control", "Ultimate Lunar Mastery"],
    // Planetary Harvester (6)
    ["Magma Taps", "Core Tunneling", "Gravitational Siphons", "Quantum Drills", "Stellar Extraction",
     "Galactic Mining", "Cosmic Harvesting", "Transcendent Collection", "Eternal Yield", "Infinite Resources",
     "Omega Extraction", "Ascendant Harvesting", "Divine Mining", "Absolute Power", "Ultimate Planetary Mastery"],
    // Fusion Reactor (7)
    ["Plasma Containment", "Magnetic Bottles", "Antimatter Injection", "Quantum Fusion", "Stellar Cores",
     "Galactic Reactors", "Cosmic Fusion", "Transcendent Plasma", "Eternal Fusion", "Infinite Energy",
     "Omega Reactors", "Ascendant Fusion", "Divine Plasma", "Absolute Power", "Ultimate Fusion Mastery"],
    // Dyson Swarm (8)
    ["Self-Replicating Mirrors", "Adaptive Orbits", "Quantum Collectors", "Neural Coordination", "Stellar Swarms",
     "Galactic Arrays", "Cosmic Networks", "Transcendent Mirrors", "Eternal Collectors", "Infinite Swarms",
     "Omega Arrays", "Ascendant Networks", "Divine Collectors", "Absolute Coverage", "Ultimate Swarm Mastery"],
    // Dyson Sphere (9)
    ["Megastructure Maintenance", "Stellar Engineering", "Dimensional Folding", "Quantum Shells", "Stellar Enclosure",
     "Galactic Spheres", "Cosmic Shells", "Transcendent Engineering", "Eternal Structures", "Infinite Absorption",
     "Omega Spheres", "Ascendant Engineering", "Divine Structures", "Absolute Enclosure", "Ultimate Sphere Mastery"],
    // Star Forge (10)
    ["Stellar Nurseries", "Gravity Wells", "Cosmic Genesis", "Quantum Creation", "Stellar Birth",
     "Galactic Forges", "Cosmic Foundries", "Transcendent Creation", "Eternal Genesis", "Infinite Stars",
     "Omega Forges", "Ascendant Creation", "Divine Genesis", "Absolute Power", "Ultimate Forge Mastery"],
    // Neutron Harvester (11)
    ["Neutron Capture", "Density Extraction", "Pulsed Energy", "Quantum Neutronics", "Stellar Density",
     "Galactic Pulsars", "Cosmic Neutrons", "Transcendent Density", "Eternal Pulsars", "Infinite Capture",
     "Omega Neutronics", "Ascendant Pulsars", "Divine Density", "Absolute Neutrons", "Ultimate Neutron Mastery"],
    // Black Hole Tap (12)
    ["Event Horizon Siphon", "Hawking Radiation", "Singularity Mining", "Quantum Tunneling", "Stellar Consumption",
     "Galactic Voids", "Cosmic Singularities", "Transcendent Horizons", "Eternal Darkness", "Infinite Gravity",
     "Omega Singularities", "Ascendant Voids", "Divine Horizons", "Absolute Consumption", "Ultimate Void Mastery"],
    // Quantum Vacuum (13)
    ["Zero-Point Extraction", "Casimir Collectors", "Virtual Particles", "Quantum Fluctuations", "Stellar Vacuum",
     "Galactic Emptiness", "Cosmic Nothing", "Transcendent Vacuum", "Eternal Zero-Point", "Infinite Virtual",
     "Omega Fluctuations", "Ascendant Nothing", "Divine Emptiness", "Absolute Zero", "Ultimate Vacuum Mastery"],
    // Galactic Core (14)
    ["Core Tapping", "Supermassive Siphon", "Gravitational Waves", "Quantum Core", "Stellar Nucleus",
     "Galactic Heart", "Cosmic Center", "Transcendent Core", "Eternal Nucleus", "Infinite Gravity",
     "Omega Core", "Ascendant Center", "Divine Nucleus", "Absolute Core", "Ultimate Galactic Mastery"],
    // Dark Matter Engine (15)
    ["Dark Conversion", "Matter Annihilation", "Shadow Harvesting", "Quantum Darkness", "Stellar Shadow",
     "Galactic Dark", "Cosmic Shadow", "Transcendent Darkness", "Eternal Shadow", "Infinite Dark",
     "Omega Matter", "Ascendant Shadow", "Divine Darkness", "Absolute Dark", "Ultimate Dark Mastery"],
    // Cosmic String (16)
    ["String Vibration", "Dimensional Resonance", "Topological Defects", "Quantum Strings", "Stellar Vibrations",
     "Galactic Resonance", "Cosmic Strings", "Transcendent Vibrations", "Eternal Resonance", "Infinite Strings",
     "Omega Vibrations", "Ascendant Resonance", "Divine Strings", "Absolute Vibration", "Ultimate String Mastery"],
    // Multiverse Gate (17)
    ["Parallel Siphon", "Dimensional Bridge", "Reality Tunneling", "Quantum Multiverse", "Stellar Dimensions",
     "Galactic Parallels", "Cosmic Bridges", "Transcendent Realities", "Eternal Dimensions", "Infinite Universes",
     "Omega Gates", "Ascendant Bridges", "Divine Dimensions", "Absolute Reality", "Ultimate Gate Mastery"],
    // Reality Compiler (18)
    ["Physics Rewrite", "Law Manipulation", "Constant Editing", "Quantum Compilation", "Stellar Physics",
     "Galactic Laws", "Cosmic Constants", "Transcendent Code", "Eternal Physics", "Infinite Laws",
     "Omega Compilation", "Ascendant Code", "Divine Physics", "Absolute Reality", "Ultimate Compiler Mastery"],
    // Entropy Reverser (19)
    ["Time Reversal", "Order Restoration", "Heat Death Denial", "Quantum Reversal", "Stellar Entropy",
     "Galactic Order", "Cosmic Restoration", "Transcendent Reversal", "Eternal Order", "Infinite Time",
     "Omega Entropy", "Ascendant Order", "Divine Reversal", "Absolute Time", "Ultimate Entropy Mastery"],
    // Big Bang Catalyst (20)
    ["Pocket Creation", "Universe Seeding", "Cosmic Ignition", "Quantum Genesis", "Stellar Birth",
     "Galactic Creation", "Cosmic Ignition", "Transcendent Genesis", "Eternal Creation", "Infinite Universes",
     "Omega Genesis", "Ascendant Creation", "Divine Ignition", "Absolute Creation", "Ultimate Catalyst Mastery"],
];

// Base costs for first tier of each producer's upgrades
const PRODUCER_BASE_COSTS: [f64; 20] = [
    100.0,          // Solar Panel
    500.0,          // Mining Drone
    2_500.0,        // Asteroid Mine
    15_000.0,       // Orbital Station
    75_000.0,       // Lunar Colony
    400_000.0,      // Planetary Harvester
    2_000_000.0,    // Fusion Reactor
    10_000_000.0,   // Dyson Swarm
    50_000_000.0,   // Dyson Sphere
    250_000_000.0,  // Star Forge
    1_250_000_000.0,       // Neutron Harvester
    6_250_000_000.0,       // Black Hole Tap
    31_250_000_000.0,      // Quantum Vacuum
    156_250_000_000.0,     // Galactic Core
    781_250_000_000.0,     // Dark Matter Engine
    3_906_250_000_000.0,   // Cosmic String
    19_531_250_000_000.0,  // Multiverse Gate
    97_656_250_000_000.0,  // Reality Compiler
    488_281_250_000_000.0, // Entropy Reverser
    2_441_406_250_000_000.0, // Big Bang Catalyst
];

// Producer names for descriptions
const PRODUCER_NAMES: [&str; 20] = [
    "Solar Panels", "Mining Drones", "Asteroid Mines", "Orbital Stations",
    "Lunar Colonies", "Planetary Harvesters", "Fusion Reactors", "Dyson Swarms",
    "Dyson Spheres", "Star Forges", "Neutron Harvesters", "Black Hole Taps",
    "Quantum Vacuums", "Galactic Cores", "Dark Matter Engines", "Cosmic Strings",
    "Multiverse Gates", "Reality Compilers", "Entropy Reversers", "Big Bang Catalysts",
];

// Generate all upgrades programmatically
static UPGRADES: Lazy<Vec<Upgrade>> = Lazy::new(|| {
    let mut upgrades = Vec::new();

    // Generate producer upgrades (20 producers × 15 tiers = 300 upgrades)
    for producer_idx in 0..20 {
        let producer_id = (producer_idx + 1) as u32;
        let base_cost = PRODUCER_BASE_COSTS[producer_idx];

        for tier in 0..15 {
            let id = producer_id * 100 + (tier + 1) as u32;
            let name = PRODUCER_UPGRADE_NAMES[producer_idx][tier];
            let producer_name = PRODUCER_NAMES[producer_idx];

            // Tiered multipliers: early game unchanged, mid/late game slower
            // Tiers 1-5 (0-4): 2.0x, Tiers 6-10 (5-9): 1.7x, Tiers 11-15 (10-14): 1.4x
            let multiplier = if tier < 5 {
                2.0
            } else if tier < 10 {
                1.7
            } else {
                1.4
            };

            let description = Box::leak(format!("{} are {:.0}% more efficient", producer_name, (multiplier - 1.0) * 100.0).into_boxed_str());

            // Cost scales by 5x per tier
            let cost = base_cost * 5.0_f64.powi(tier as i32);

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

    // Generate synergy upgrades (19 pairs)
    let synergy_pairs: [(u32, u32, &str, &str); 19] = [
        (2, 3, "Drone-Mine Synergy", "Mining Drones boost Asteroid Mines"),
        (3, 4, "Mine-Station Synergy", "Asteroid Mines boost Orbital Stations"),
        (4, 5, "Station-Lunar Synergy", "Orbital Stations boost Lunar Colonies"),
        (5, 6, "Lunar-Planetary Synergy", "Lunar Colonies boost Planetary Harvesters"),
        (6, 7, "Planetary-Fusion Synergy", "Planetary Harvesters boost Fusion Reactors"),
        (7, 8, "Fusion-Swarm Synergy", "Fusion Reactors boost Dyson Swarms"),
        (8, 9, "Swarm-Sphere Synergy", "Dyson Swarms boost Dyson Spheres"),
        (9, 10, "Sphere-Forge Synergy", "Dyson Spheres boost Star Forges"),
        (10, 11, "Forge-Neutron Synergy", "Star Forges boost Neutron Harvesters"),
        (11, 12, "Neutron-Void Synergy", "Neutron Harvesters boost Black Hole Taps"),
        (12, 13, "Void-Vacuum Synergy", "Black Hole Taps boost Quantum Vacuums"),
        (13, 14, "Vacuum-Core Synergy", "Quantum Vacuums boost Galactic Cores"),
        (14, 15, "Core-Dark Synergy", "Galactic Cores boost Dark Matter Engines"),
        (15, 16, "Dark-String Synergy", "Dark Matter Engines boost Cosmic Strings"),
        (16, 17, "String-Gate Synergy", "Cosmic Strings boost Multiverse Gates"),
        (17, 18, "Gate-Compiler Synergy", "Multiverse Gates boost Reality Compilers"),
        (18, 19, "Compiler-Entropy Synergy", "Reality Compilers boost Entropy Reversers"),
        (19, 20, "Entropy-Catalyst Synergy", "Entropy Reversers boost Big Bang Catalysts"),
        (1, 20, "Solar-Catalyst Synergy", "Solar Panels boost Big Bang Catalysts"), // Full circle!
    ];

    for (idx, (id_a, id_b, name, desc)) in synergy_pairs.iter().enumerate() {
        let synergy_id = 8001 + idx as u32;
        // Cost scales based on which producers are involved
        let cost = PRODUCER_BASE_COSTS[(*id_a - 1) as usize] * 100.0;
        let description = Box::leak(format!("{}: +2% per {} to {}", desc, PRODUCER_NAMES[(*id_a - 1) as usize], PRODUCER_NAMES[(*id_b - 1) as usize]).into_boxed_str());

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
        (9001, "Efficient Wiring", "All production +5%", 10.0, 1_000.0, 1.05),
        (9002, "Central Grid", "All production +10%", 100.0, 10_000.0, 1.10),
        (9003, "Dark Energy Tap", "All production +15%", 1_000.0, 100_000.0, 1.15),
        (9004, "Quantum Entanglement", "All production +25%", 10_000.0, 1_000_000.0, 1.25),
        (9005, "Dimensional Siphon", "All production +35%", 100_000.0, 10_000_000.0, 1.35),
        (9006, "Universal Conduit", "All production +50%", 1_000_000.0, 100_000_000.0, 1.50),
        (9007, "Cosmic Harmonics", "All production +75%", 10_000_000.0, 1_000_000_000.0, 1.75),
        (9008, "Reality Tap", "All production +100%", 100_000_000.0, 10_000_000_000.0, 2.00),
        (9009, "Multiverse Channel", "All production +150%", 1_000_000_000.0, 100_000_000_000.0, 2.50),
        (9010, "Absolute Energy", "All production +200%", 10_000_000_000.0, 1_000_000_000_000.0, 3.00),
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

    upgrades
});
