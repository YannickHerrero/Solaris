use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrestigeEffect {
    ProductionMultiplier(f64),      // Permanent production bonus
    StartingEnergy(f64),            // Energy after ascension
    OfflineBonus(f64),              // Offline earnings multiplier
    UnlockProducers(u32),           // Start with N producers unlocked
    UpgradeCostReduction(f64),      // Upgrades cost less
    ChipBonus(f64),                 // More stellar chips earned
    ProductionPerAscension(f64),    // +X% per ascension
    ProductionPerAchievement(f64),  // +X% per achievement
    KeepEnergyPercent(f64),         // Keep X% of energy after ascension
    DoubleEnergyChance(f64),        // Chance for double energy per tick
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrestigeRequirement {
    Ascensions(u64),
    TotalChips(u64),
    PrestigeUpgrade(u32),
}

#[derive(Debug, Clone)]
pub struct PrestigeUpgrade {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub cost: u64, // stellar chips
    pub effect: PrestigeEffect,
    pub requirement: Option<PrestigeRequirement>,
}

impl PrestigeUpgrade {
    pub fn all() -> &'static [PrestigeUpgrade] {
        &PRESTIGE_UPGRADES
    }

    pub fn by_id(id: u32) -> Option<&'static PrestigeUpgrade> {
        PRESTIGE_UPGRADES.iter().find(|p| p.id == id)
    }
}

// Calculate stellar chips from total energy earned
// Formula: floor(total_energy_earned^0.5 / 1_000_000)
pub fn calculate_stellar_chips(total_energy_earned: f64) -> u64 {
    (total_energy_earned.sqrt() / 1_000_000.0).floor() as u64
}

static PRESTIGE_UPGRADES: Lazy<Vec<PrestigeUpgrade>> = Lazy::new(|| {
    vec![
        // Tier 1 (1-10 chips)
        PrestigeUpgrade {
            id: 1,
            name: "Stellar Foundation",
            description: "+5% all production permanently",
            cost: 1,
            effect: PrestigeEffect::ProductionMultiplier(1.05),
            requirement: None,
        },
        PrestigeUpgrade {
            id: 2,
            name: "Quick Start",
            description: "Start with 100 energy after ascension",
            cost: 3,
            effect: PrestigeEffect::StartingEnergy(100.0),
            requirement: None,
        },
        PrestigeUpgrade {
            id: 3,
            name: "Persistent Memory",
            description: "+10% offline earnings",
            cost: 5,
            effect: PrestigeEffect::OfflineBonus(1.10),
            requirement: None,
        },
        PrestigeUpgrade {
            id: 4,
            name: "Lucky Stars",
            description: "+1% chance for double energy per tick",
            cost: 7,
            effect: PrestigeEffect::DoubleEnergyChance(0.01),
            requirement: None,
        },
        PrestigeUpgrade {
            id: 5,
            name: "Seasoned Explorer",
            description: "Start with Mining Drone unlocked",
            cost: 10,
            effect: PrestigeEffect::UnlockProducers(2),
            requirement: None,
        },

        // Tier 2 (15-50 chips)
        PrestigeUpgrade {
            id: 6,
            name: "Cosmic Legacy",
            description: "+1% production per ascension",
            cost: 15,
            effect: PrestigeEffect::ProductionPerAscension(0.01),
            requirement: Some(PrestigeRequirement::TotalChips(10)),
        },
        PrestigeUpgrade {
            id: 7,
            name: "Accelerated Start",
            description: "Start with first 3 producers unlocked",
            cost: 20,
            effect: PrestigeEffect::UnlockProducers(3),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(5)),
        },
        PrestigeUpgrade {
            id: 8,
            name: "Stellar Efficiency",
            description: "+25% all production permanently",
            cost: 30,
            effect: PrestigeEffect::ProductionMultiplier(1.25),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(1)),
        },
        PrestigeUpgrade {
            id: 9,
            name: "Bargain Hunter",
            description: "Upgrades cost 10% less",
            cost: 40,
            effect: PrestigeEffect::UpgradeCostReduction(0.10),
            requirement: None,
        },
        PrestigeUpgrade {
            id: 10,
            name: "Universal Knowledge",
            description: "Start with first 5 producers unlocked",
            cost: 50,
            effect: PrestigeEffect::UnlockProducers(5),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(7)),
        },

        // Tier 3 (75-200 chips)
        PrestigeUpgrade {
            id: 11,
            name: "Dimensional Echo",
            description: "+1% production per achievement",
            cost: 75,
            effect: PrestigeEffect::ProductionPerAchievement(0.01),
            requirement: Some(PrestigeRequirement::TotalChips(50)),
        },
        PrestigeUpgrade {
            id: 12,
            name: "Transcendent Memory",
            description: "Keep 1% of energy after ascension",
            cost: 100,
            effect: PrestigeEffect::KeepEnergyPercent(0.01),
            requirement: Some(PrestigeRequirement::Ascensions(5)),
        },
        PrestigeUpgrade {
            id: 13,
            name: "Cosmic Mastery",
            description: "+100% all production permanently",
            cost: 150,
            effect: PrestigeEffect::ProductionMultiplier(2.0),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(8)),
        },
        PrestigeUpgrade {
            id: 14,
            name: "Eternal Progression",
            description: "+5% Stellar Chips earned",
            cost: 200,
            effect: PrestigeEffect::ChipBonus(1.05),
            requirement: Some(PrestigeRequirement::Ascensions(10)),
        },

        // Tier 4 (300-500 chips)
        PrestigeUpgrade {
            id: 15,
            name: "Rapid Expansion",
            description: "Start with first 7 producers unlocked",
            cost: 300,
            effect: PrestigeEffect::UnlockProducers(7),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(10)),
        },
        PrestigeUpgrade {
            id: 16,
            name: "Enhanced Luck",
            description: "+5% chance for double energy per tick",
            cost: 400,
            effect: PrestigeEffect::DoubleEnergyChance(0.05),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(4)),
        },
        PrestigeUpgrade {
            id: 17,
            name: "Stellar Accumulation",
            description: "+15% Stellar Chips earned",
            cost: 500,
            effect: PrestigeEffect::ChipBonus(1.15),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(14)),
        },

        // Tier 5 (750-1500 chips) - End game
        PrestigeUpgrade {
            id: 18,
            name: "Universal Unlock",
            description: "Start with first 10 producers unlocked",
            cost: 750,
            effect: PrestigeEffect::UnlockProducers(10),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(15)),
        },
        PrestigeUpgrade {
            id: 19,
            name: "Perfect Memory",
            description: "Keep 5% of energy after ascension",
            cost: 1000,
            effect: PrestigeEffect::KeepEnergyPercent(0.05),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(12)),
        },
        PrestigeUpgrade {
            id: 20,
            name: "Infinite Power",
            description: "+500% all production permanently",
            cost: 1500,
            effect: PrestigeEffect::ProductionMultiplier(6.0),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(13)),
        },

        // Tier 6 (2000+ chips) - True end game
        PrestigeUpgrade {
            id: 21,
            name: "Legacy of Legends",
            description: "+3% production per ascension",
            cost: 2000,
            effect: PrestigeEffect::ProductionPerAscension(0.03),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(6)),
        },
        PrestigeUpgrade {
            id: 22,
            name: "Discount Master",
            description: "Upgrades cost 25% less",
            cost: 3000,
            effect: PrestigeEffect::UpgradeCostReduction(0.25),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(9)),
        },
        PrestigeUpgrade {
            id: 23,
            name: "Eternal Offline",
            description: "+50% offline earnings",
            cost: 4000,
            effect: PrestigeEffect::OfflineBonus(1.50),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(3)),
        },
        PrestigeUpgrade {
            id: 24,
            name: "Galactic Domination",
            description: "+1000% all production permanently",
            cost: 5000,
            effect: PrestigeEffect::ProductionMultiplier(11.0),
            requirement: Some(PrestigeRequirement::PrestigeUpgrade(20)),
        },
    ]
});
