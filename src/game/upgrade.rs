#[derive(Debug, Clone, Copy)]
pub enum UpgradeRequirement {
    ProducerCount { producer_id: u32, count: u64 },
    TotalEnergyPerSecond(f64),
    ManualClicks(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum UpgradeEffect {
    ProducerMultiplier { producer_id: u32, multiplier: f64 },
    GlobalMultiplier(f64),
    ManualMultiplier(f64),
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

static UPGRADES: [Upgrade; 48] = [
    // Solar Panel upgrades (producer_id: 1)
    Upgrade {
        id: 101,
        name: "Better Silicon",
        description: "Solar Panels are twice as efficient",
        cost: 150.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 1, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 1, multiplier: 2.0 },
    },
    Upgrade {
        id: 102,
        name: "Anti-Reflective Coating",
        description: "Solar Panels are twice as efficient",
        cost: 750.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 1, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 1, multiplier: 2.0 },
    },
    Upgrade {
        id: 103,
        name: "Quantum Dots",
        description: "Solar Panels are twice as efficient",
        cost: 7500.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 1, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 1, multiplier: 2.0 },
    },
    Upgrade {
        id: 104,
        name: "Perovskite Cells",
        description: "Solar Panels are twice as efficient",
        cost: 75000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 1, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 1, multiplier: 2.0 },
    },

    // Mining Drone upgrades (producer_id: 2)
    Upgrade {
        id: 201,
        name: "Improved Thrusters",
        description: "Mining Drones are twice as efficient",
        cost: 1000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 2, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 2, multiplier: 2.0 },
    },
    Upgrade {
        id: 202,
        name: "AI Navigation",
        description: "Mining Drones are twice as efficient",
        cost: 5000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 2, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 2, multiplier: 2.0 },
    },
    Upgrade {
        id: 203,
        name: "Swarm Intelligence",
        description: "Mining Drones are twice as efficient",
        cost: 50000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 2, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 2, multiplier: 2.0 },
    },
    Upgrade {
        id: 204,
        name: "Nanite Repair",
        description: "Mining Drones are twice as efficient",
        cost: 500000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 2, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 2, multiplier: 2.0 },
    },

    // Asteroid Mine upgrades (producer_id: 3)
    Upgrade {
        id: 301,
        name: "Deep Core Drilling",
        description: "Asteroid Mines are twice as efficient",
        cost: 5000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 3, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 3, multiplier: 2.0 },
    },
    Upgrade {
        id: 302,
        name: "Spectral Analysis",
        description: "Asteroid Mines are twice as efficient",
        cost: 25000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 3, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 3, multiplier: 2.0 },
    },
    Upgrade {
        id: 303,
        name: "Automated Refineries",
        description: "Asteroid Mines are twice as efficient",
        cost: 250000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 3, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 3, multiplier: 2.0 },
    },
    Upgrade {
        id: 304,
        name: "Gravitational Tractors",
        description: "Asteroid Mines are twice as efficient",
        cost: 2500000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 3, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 3, multiplier: 2.0 },
    },

    // Orbital Station upgrades (producer_id: 4)
    Upgrade {
        id: 401,
        name: "Solar Sails",
        description: "Orbital Stations are twice as efficient",
        cost: 30000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 4, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 4, multiplier: 2.0 },
    },
    Upgrade {
        id: 402,
        name: "Microwave Transmission",
        description: "Orbital Stations are twice as efficient",
        cost: 150000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 4, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 4, multiplier: 2.0 },
    },
    Upgrade {
        id: 403,
        name: "Zero-G Manufacturing",
        description: "Orbital Stations are twice as efficient",
        cost: 1500000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 4, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 4, multiplier: 2.0 },
    },
    Upgrade {
        id: 404,
        name: "Lagrange Networks",
        description: "Orbital Stations are twice as efficient",
        cost: 15000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 4, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 4, multiplier: 2.0 },
    },

    // Lunar Colony upgrades (producer_id: 5)
    Upgrade {
        id: 501,
        name: "Regolith Processing",
        description: "Lunar Colonies are twice as efficient",
        cost: 150000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 5, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 5, multiplier: 2.0 },
    },
    Upgrade {
        id: 502,
        name: "Helium-3 Extraction",
        description: "Lunar Colonies are twice as efficient",
        cost: 750000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 5, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 5, multiplier: 2.0 },
    },
    Upgrade {
        id: 503,
        name: "Mass Driver Launch",
        description: "Lunar Colonies are twice as efficient",
        cost: 7500000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 5, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 5, multiplier: 2.0 },
    },
    Upgrade {
        id: 504,
        name: "Lunar Elevator",
        description: "Lunar Colonies are twice as efficient",
        cost: 75000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 5, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 5, multiplier: 2.0 },
    },

    // Planetary Harvester upgrades (producer_id: 6)
    Upgrade {
        id: 601,
        name: "Magma Taps",
        description: "Planetary Harvesters are twice as efficient",
        cost: 800000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 6, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 6, multiplier: 2.0 },
    },
    Upgrade {
        id: 602,
        name: "Core Tunneling",
        description: "Planetary Harvesters are twice as efficient",
        cost: 4000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 6, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 6, multiplier: 2.0 },
    },
    Upgrade {
        id: 603,
        name: "Gravitational Siphons",
        description: "Planetary Harvesters are twice as efficient",
        cost: 40000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 6, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 6, multiplier: 2.0 },
    },
    Upgrade {
        id: 604,
        name: "Planet Crackers",
        description: "Planetary Harvesters are twice as efficient",
        cost: 400000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 6, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 6, multiplier: 2.0 },
    },

    // Fusion Reactor upgrades (producer_id: 7)
    Upgrade {
        id: 701,
        name: "Plasma Containment",
        description: "Fusion Reactors are twice as efficient",
        cost: 4000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 7, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 7, multiplier: 2.0 },
    },
    Upgrade {
        id: 702,
        name: "Magnetic Bottles",
        description: "Fusion Reactors are twice as efficient",
        cost: 20000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 7, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 7, multiplier: 2.0 },
    },
    Upgrade {
        id: 703,
        name: "Antimatter Injection",
        description: "Fusion Reactors are twice as efficient",
        cost: 200000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 7, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 7, multiplier: 2.0 },
    },
    Upgrade {
        id: 704,
        name: "Pocket Suns",
        description: "Fusion Reactors are twice as efficient",
        cost: 2000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 7, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 7, multiplier: 2.0 },
    },

    // Dyson Swarm upgrades (producer_id: 8)
    Upgrade {
        id: 801,
        name: "Self-Replicating Mirrors",
        description: "Dyson Swarms are twice as efficient",
        cost: 20000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 8, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 8, multiplier: 2.0 },
    },
    Upgrade {
        id: 802,
        name: "Adaptive Orbits",
        description: "Dyson Swarms are twice as efficient",
        cost: 100000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 8, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 8, multiplier: 2.0 },
    },
    Upgrade {
        id: 803,
        name: "Quantum Collectors",
        description: "Dyson Swarms are twice as efficient",
        cost: 1000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 8, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 8, multiplier: 2.0 },
    },
    Upgrade {
        id: 804,
        name: "Neural Coordination",
        description: "Dyson Swarms are twice as efficient",
        cost: 10000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 8, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 8, multiplier: 2.0 },
    },

    // Dyson Sphere upgrades (producer_id: 9)
    Upgrade {
        id: 901,
        name: "Megastructure Maintenance",
        description: "Dyson Spheres are twice as efficient",
        cost: 100000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 9, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 9, multiplier: 2.0 },
    },
    Upgrade {
        id: 902,
        name: "Stellar Engineering",
        description: "Dyson Spheres are twice as efficient",
        cost: 500000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 9, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 9, multiplier: 2.0 },
    },
    Upgrade {
        id: 903,
        name: "Dimensional Folding",
        description: "Dyson Spheres are twice as efficient",
        cost: 5000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 9, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 9, multiplier: 2.0 },
    },
    Upgrade {
        id: 904,
        name: "Total Absorption",
        description: "Dyson Spheres are twice as efficient",
        cost: 50000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 9, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 9, multiplier: 2.0 },
    },

    // Star Forge upgrades (producer_id: 10)
    Upgrade {
        id: 1001,
        name: "Stellar Nurseries",
        description: "Star Forges are twice as efficient",
        cost: 500000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 10, count: 10 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 10, multiplier: 2.0 },
    },
    Upgrade {
        id: 1002,
        name: "Gravity Wells",
        description: "Star Forges are twice as efficient",
        cost: 2500000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 10, count: 25 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 10, multiplier: 2.0 },
    },
    Upgrade {
        id: 1003,
        name: "Cosmic Genesis",
        description: "Star Forges are twice as efficient",
        cost: 25000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 10, count: 50 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 10, multiplier: 2.0 },
    },
    Upgrade {
        id: 1004,
        name: "Universal Harvest",
        description: "Star Forges are twice as efficient",
        cost: 250000000000.0,
        requirement: UpgradeRequirement::ProducerCount { producer_id: 10, count: 100 },
        effect: UpgradeEffect::ProducerMultiplier { producer_id: 10, multiplier: 2.0 },
    },

    // Global upgrades based on E/s
    Upgrade {
        id: 9001,
        name: "Efficient Wiring",
        description: "All production +5%",
        cost: 1000.0,
        requirement: UpgradeRequirement::TotalEnergyPerSecond(10.0),
        effect: UpgradeEffect::GlobalMultiplier(1.05),
    },
    Upgrade {
        id: 9002,
        name: "Central Grid",
        description: "All production +10%",
        cost: 10000.0,
        requirement: UpgradeRequirement::TotalEnergyPerSecond(100.0),
        effect: UpgradeEffect::GlobalMultiplier(1.10),
    },
    Upgrade {
        id: 9003,
        name: "Dark Energy Tap",
        description: "All production +15%",
        cost: 100000.0,
        requirement: UpgradeRequirement::TotalEnergyPerSecond(1000.0),
        effect: UpgradeEffect::GlobalMultiplier(1.15),
    },
    Upgrade {
        id: 9004,
        name: "Quantum Entanglement Grid",
        description: "All production +25%",
        cost: 1000000.0,
        requirement: UpgradeRequirement::TotalEnergyPerSecond(10000.0),
        effect: UpgradeEffect::GlobalMultiplier(1.25),
    },

    // Manual mining upgrades
    Upgrade {
        id: 9101,
        name: "Reinforced Pickaxe",
        description: "Manual mining is twice as powerful",
        cost: 500.0,
        requirement: UpgradeRequirement::ManualClicks(50),
        effect: UpgradeEffect::ManualMultiplier(2.0),
    },
    Upgrade {
        id: 9102,
        name: "Power Gauntlets",
        description: "Manual mining is twice as powerful",
        cost: 5000.0,
        requirement: UpgradeRequirement::ManualClicks(200),
        effect: UpgradeEffect::ManualMultiplier(2.0),
    },
    Upgrade {
        id: 9103,
        name: "Neural Interface",
        description: "Manual mining is twice as powerful",
        cost: 50000.0,
        requirement: UpgradeRequirement::ManualClicks(1000),
        effect: UpgradeEffect::ManualMultiplier(2.0),
    },
    Upgrade {
        id: 9104,
        name: "Quantum Harvester",
        description: "Manual mining is twice as powerful",
        cost: 500000.0,
        requirement: UpgradeRequirement::ManualClicks(5000),
        effect: UpgradeEffect::ManualMultiplier(2.0),
    },
];
