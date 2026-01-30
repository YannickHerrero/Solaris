#[derive(Debug, Clone)]
pub struct Producer {
    pub id: u32,
    pub name: &'static str,
    pub icon: &'static str,
    pub base_energy_per_second: f64,
    pub base_cost: f64,
    #[allow(dead_code)]
    pub description: &'static str,
}

impl Producer {
    pub fn all() -> &'static [Producer] {
        &PRODUCERS
    }
}

// 20 Producers with 5x cost/production scaling between each
static PRODUCERS: [Producer; 20] = [
    // Original 10 producers (with adjusted scaling for consistency)
    Producer {
        id: 1,
        name: "Solar Panel",
        icon: "▤",
        base_energy_per_second: 0.1,
        base_cost: 15.0,
        description: "A simple photovoltaic cell",
    },
    Producer {
        id: 2,
        name: "Mining Drone",
        icon: "⛏",
        base_energy_per_second: 0.5,
        base_cost: 100.0,
        description: "Autonomous resource collector",
    },
    Producer {
        id: 3,
        name: "Asteroid Mine",
        icon: "☄",
        base_energy_per_second: 2.0,
        base_cost: 500.0,
        description: "Harvests energy-rich minerals",
    },
    Producer {
        id: 4,
        name: "Orbital Station",
        icon: "◎",
        base_energy_per_second: 10.0,
        base_cost: 3_000.0,
        description: "Space-based power relay",
    },
    Producer {
        id: 5,
        name: "Lunar Colony",
        icon: "☽",
        base_energy_per_second: 50.0,
        base_cost: 15_000.0,
        description: "Moon-based operations hub",
    },
    Producer {
        id: 6,
        name: "Planetary Harvester",
        icon: "⊕",
        base_energy_per_second: 250.0,
        base_cost: 80_000.0,
        description: "Extracts planetary core energy",
    },
    Producer {
        id: 7,
        name: "Fusion Reactor",
        icon: "✴",
        base_energy_per_second: 1_200.0,
        base_cost: 400_000.0,
        description: "Harnesses stellar fusion",
    },
    Producer {
        id: 8,
        name: "Dyson Swarm",
        icon: "✧",
        base_energy_per_second: 6_000.0,
        base_cost: 2_000_000.0,
        description: "Orbiting solar collectors",
    },
    Producer {
        id: 9,
        name: "Dyson Sphere",
        icon: "⊙",
        base_energy_per_second: 30_000.0,
        base_cost: 10_000_000.0,
        description: "Encapsulates an entire star",
    },
    Producer {
        id: 10,
        name: "Star Forge",
        icon: "★",
        base_energy_per_second: 150_000.0,
        base_cost: 50_000_000.0,
        description: "Creates and harvests stars",
    },
    // New producers (11-20)
    Producer {
        id: 11,
        name: "Neutron Harvester",
        icon: "◉",
        base_energy_per_second: 750_000.0,
        base_cost: 250_000_000.0,
        description: "Harvest neutron star energy",
    },
    Producer {
        id: 12,
        name: "Black Hole Tap",
        icon: "●",
        base_energy_per_second: 3_750_000.0,
        base_cost: 1_250_000_000.0,
        description: "Extract energy from event horizons",
    },
    Producer {
        id: 13,
        name: "Quantum Vacuum",
        icon: "◌",
        base_energy_per_second: 18_750_000.0,
        base_cost: 6_250_000_000.0,
        description: "Zero-point energy extraction",
    },
    Producer {
        id: 14,
        name: "Galactic Core",
        icon: "⊛",
        base_energy_per_second: 93_750_000.0,
        base_cost: 31_250_000_000.0,
        description: "Tap supermassive black holes",
    },
    Producer {
        id: 15,
        name: "Dark Matter Engine",
        icon: "◐",
        base_energy_per_second: 468_750_000.0,
        base_cost: 156_250_000_000.0,
        description: "Convert dark matter to energy",
    },
    Producer {
        id: 16,
        name: "Cosmic String",
        icon: "≋",
        base_energy_per_second: 2_343_750_000.0,
        base_cost: 781_250_000_000.0,
        description: "Vibrating cosmic strings",
    },
    Producer {
        id: 17,
        name: "Multiverse Gate",
        icon: "⧫",
        base_energy_per_second: 11_718_750_000.0,
        base_cost: 3_906_250_000_000.0,
        description: "Energy from parallel universes",
    },
    Producer {
        id: 18,
        name: "Reality Compiler",
        icon: "⌘",
        base_energy_per_second: 58_593_750_000.0,
        base_cost: 19_531_250_000_000.0,
        description: "Rewrite physics for energy",
    },
    Producer {
        id: 19,
        name: "Entropy Reverser",
        icon: "∞",
        base_energy_per_second: 292_968_750_000.0,
        base_cost: 97_656_250_000_000.0,
        description: "Reverse thermodynamic entropy",
    },
    Producer {
        id: 20,
        name: "Big Bang Catalyst",
        icon: "✦",
        base_energy_per_second: 1_464_843_750_000.0,
        base_cost: 488_281_250_000_000.0,
        description: "Create pocket universes",
    },
];
