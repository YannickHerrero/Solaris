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

static PRODUCERS: [Producer; 10] = [
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
        base_cost: 3000.0,
        description: "Space-based power relay",
    },
    Producer {
        id: 5,
        name: "Lunar Colony",
        icon: "☽",
        base_energy_per_second: 50.0,
        base_cost: 15000.0,
        description: "Moon-based operations hub",
    },
    Producer {
        id: 6,
        name: "Planetary Harvester",
        icon: "⊕",
        base_energy_per_second: 250.0,
        base_cost: 80000.0,
        description: "Extracts planetary core energy",
    },
    Producer {
        id: 7,
        name: "Fusion Reactor",
        icon: "✴",
        base_energy_per_second: 1200.0,
        base_cost: 400000.0,
        description: "Harnesses stellar fusion",
    },
    Producer {
        id: 8,
        name: "Dyson Swarm",
        icon: "✧",
        base_energy_per_second: 6000.0,
        base_cost: 2000000.0,
        description: "Orbiting solar collectors",
    },
    Producer {
        id: 9,
        name: "Dyson Sphere",
        icon: "⊙",
        base_energy_per_second: 30000.0,
        base_cost: 10000000.0,
        description: "Encapsulates an entire star",
    },
    Producer {
        id: 10,
        name: "Star Forge",
        icon: "★",
        base_energy_per_second: 150000.0,
        base_cost: 50000000.0,
        description: "Creates and harvests stars",
    },
];
