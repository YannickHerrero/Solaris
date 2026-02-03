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

// 20 Producers matching Cookie Clicker's cost/E/s ratios
// Each producer has progressively longer payback times (cost/E/s)
// This creates the characteristic CC slow-down as you progress
static PRODUCERS: [Producer; 20] = [
    // Producer 1: Solar Panel
    // Special: Uses Thousand Rays mechanic (gains +0.1 per other building)
    Producer {
        id: 1,
        name: "Solar Panel",
        icon: "▤",
        base_energy_per_second: 0.1,
        base_cost: 15.0,
        description: "A simple photovoltaic cell",
    },
    // Producer 2: Mining Drone
    // Special: Boosted by Drone Network from other buildings
    Producer {
        id: 2,
        name: "Mining Drone",
        icon: "⛏",
        base_energy_per_second: 1.0,
        base_cost: 100.0,
        description: "Autonomous resource collector",
    },
    // Producer 3: Asteroid Mine
    Producer {
        id: 3,
        name: "Asteroid Mine",
        icon: "☄",
        base_energy_per_second: 8.0,
        base_cost: 1_100.0,
        description: "Harvests energy-rich minerals",
    },
    // Producer 4: Orbital Station
    Producer {
        id: 4,
        name: "Orbital Station",
        icon: "◎",
        base_energy_per_second: 47.0,
        base_cost: 12_000.0,
        description: "Space-based power relay",
    },
    // Producer 5: Lunar Colony
    Producer {
        id: 5,
        name: "Lunar Colony",
        icon: "☽",
        base_energy_per_second: 260.0,
        base_cost: 130_000.0,
        description: "Moon-based operations hub",
    },
    // Producer 6: Planetary Harvester
    Producer {
        id: 6,
        name: "Planetary Harvester",
        icon: "⊕",
        base_energy_per_second: 1_400.0,
        base_cost: 1_400_000.0,
        description: "Extracts planetary core energy",
    },
    // Producer 7: Fusion Reactor
    Producer {
        id: 7,
        name: "Fusion Reactor",
        icon: "✴",
        base_energy_per_second: 7_800.0,
        base_cost: 20_000_000.0,
        description: "Harnesses stellar fusion",
    },
    // Producer 8: Dyson Swarm
    Producer {
        id: 8,
        name: "Dyson Swarm",
        icon: "✧",
        base_energy_per_second: 44_000.0,
        base_cost: 330_000_000.0,
        description: "Orbiting solar collectors",
    },
    // Producer 9: Dyson Sphere
    Producer {
        id: 9,
        name: "Dyson Sphere",
        icon: "⊙",
        base_energy_per_second: 260_000.0,
        base_cost: 5_100_000_000.0,
        description: "Encapsulates an entire star",
    },
    // Producer 10: Star Forge
    Producer {
        id: 10,
        name: "Star Forge",
        icon: "★",
        base_energy_per_second: 1_600_000.0,
        base_cost: 75_000_000_000.0,
        description: "Creates and harvests stars",
    },
    // Producer 11: Neutron Harvester
    Producer {
        id: 11,
        name: "Neutron Harvester",
        icon: "◉",
        base_energy_per_second: 10_000_000.0,
        base_cost: 1_000_000_000_000.0,
        description: "Harvests neutron star energy",
    },
    // Producer 12: Black Hole Tap
    Producer {
        id: 12,
        name: "Black Hole Tap",
        icon: "●",
        base_energy_per_second: 65_000_000.0,
        base_cost: 14_000_000_000_000.0,
        description: "Extracts energy from event horizons",
    },
    // Producer 13: Quantum Vacuum
    Producer {
        id: 13,
        name: "Quantum Vacuum",
        icon: "◌",
        base_energy_per_second: 430_000_000.0,
        base_cost: 170_000_000_000_000.0,
        description: "Zero-point energy extraction",
    },
    // Producer 14: Galactic Core
    Producer {
        id: 14,
        name: "Galactic Core",
        icon: "⊛",
        base_energy_per_second: 2_900_000_000.0,
        base_cost: 2_100_000_000_000_000.0,
        description: "Taps supermassive black holes",
    },
    // Producer 15: Dark Matter Engine
    Producer {
        id: 15,
        name: "Dark Matter Engine",
        icon: "◐",
        base_energy_per_second: 21_000_000_000.0,
        base_cost: 26_000_000_000_000_000.0,
        description: "Converts dark matter to energy",
    },
    // Producer 16: Cosmic String
    Producer {
        id: 16,
        name: "Cosmic String",
        icon: "≋",
        base_energy_per_second: 150_000_000_000.0,
        base_cost: 310_000_000_000_000_000.0,
        description: "Vibrating cosmic strings",
    },
    // Producer 17: Multiverse Gate
    Producer {
        id: 17,
        name: "Multiverse Gate",
        icon: "⧫",
        base_energy_per_second: 1_100_000_000_000.0,
        base_cost: 71_000_000_000_000_000_000.0,
        description: "Energy from parallel universes",
    },
    // Producer 18: Reality Compiler
    Producer {
        id: 18,
        name: "Reality Compiler",
        icon: "⌘",
        base_energy_per_second: 8_300_000_000_000.0,
        base_cost: 12_000_000_000_000_000_000_000.0,
        description: "Rewrites physics for energy",
    },
    // Producer 19: Entropy Reverser
    Producer {
        id: 19,
        name: "Entropy Reverser",
        icon: "∞",
        base_energy_per_second: 64_000_000_000_000.0,
        base_cost: 1_900_000_000_000_000_000_000_000.0,
        description: "Reverses thermodynamic entropy",
    },
    // Producer 20: Big Bang Catalyst
    Producer {
        id: 20,
        name: "Big Bang Catalyst",
        icon: "✦",
        base_energy_per_second: 510_000_000_000_000.0,
        base_cost: 540_000_000_000_000_000_000_000_000.0,
        description: "Creates pocket universes",
    },
];
