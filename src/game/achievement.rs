use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AchievementRequirement {
    ProducerCount { producer_id: u32, count: u64 },
    TotalEnergyPerSecond(f64),
    TotalEnergyEarned(f64),
    TotalClicks(u64),
    UpgradesPurchased(u32),
    TimePlayed(u64), // seconds
    TotalProducers(u64),
    Ascensions(u64),
}

#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub requirement: AchievementRequirement,
}

impl Achievement {
    pub fn all() -> &'static [Achievement] {
        &ACHIEVEMENTS
    }
}

// Achievement bonus: 0.5% per achievement (multiplicative)
// Reduced from 1% to extend late-game playtime
pub const ACHIEVEMENT_BONUS: f64 = 1.005;

// Generate achievements programmatically
static ACHIEVEMENTS: Lazy<Vec<Achievement>> = Lazy::new(|| {
    let mut achievements = Vec::new();
    let mut id = 1u32;

    // Producer milestones (1, 10, 25, 50, 100, 150, 200, 250, 300) for each producer
    let producer_names = [
        "Solar Panel",
        "Mining Drone",
        "Asteroid Mine",
        "Orbital Station",
        "Lunar Colony",
        "Planetary Harvester",
        "Fusion Reactor",
        "Dyson Swarm",
        "Dyson Sphere",
        "Star Forge",
        "Neutron Harvester",
        "Black Hole Tap",
        "Quantum Vacuum",
        "Galactic Core",
        "Dark Matter Engine",
        "Cosmic String",
        "Multiverse Gate",
        "Reality Compiler",
        "Entropy Reverser",
        "Big Bang Catalyst",
    ];
    let producer_milestones = [1, 10, 25, 50, 100, 150, 200, 250, 300];
    let milestone_names = [
        "Novice",
        "Initiate",
        "Apprentice",
        "Journeyman",
        "Expert",
        "Master",
        "Grandmaster",
        "Legend",
        "Transcendent",
    ];

    for (producer_idx, producer_name) in producer_names.iter().enumerate() {
        let producer_id = (producer_idx + 1) as u32;
        for (milestone_idx, &count) in producer_milestones.iter().enumerate() {
            let name = Box::leak(
                format!("{} {}", milestone_names[milestone_idx], producer_name).into_boxed_str(),
            );
            let description =
                Box::leak(format!("Own {} {}s", count, producer_name).into_boxed_str());
            achievements.push(Achievement {
                id,
                name,
                description,
                requirement: AchievementRequirement::ProducerCount { producer_id, count },
            });
            id += 1;
        }
    }

    // Total production milestones (E/s)
    let production_milestones: [(f64, &str); 15] = [
        (10.0, "Spark of Power"),
        (100.0, "Glimmer of Energy"),
        (1_000.0, "Flowing Current"),
        (10_000.0, "Surge of Power"),
        (100_000.0, "Energy Storm"),
        (1_000_000.0, "Power Cascade"),
        (10_000_000.0, "Energy Tsunami"),
        (100_000_000.0, "Power Singularity"),
        (1_000_000_000.0, "Cosmic Current"),
        (10_000_000_000.0, "Universal Flow"),
        (100_000_000_000.0, "Galactic Surge"),
        (1_000_000_000_000.0, "Reality Bender"),
        (10_000_000_000_000.0, "Dimension Weaver"),
        (100_000_000_000_000.0, "Multiverse Engine"),
        (1_000_000_000_000_000.0, "Infinite Power"),
    ];
    for (rate, name) in production_milestones {
        let description =
            Box::leak(format!("Reach {} E/s", format_large_number(rate)).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::TotalEnergyPerSecond(rate),
        });
        id += 1;
    }

    // Lifetime energy milestones
    let lifetime_milestones: [(f64, &str); 15] = [
        (1_000.0, "First Thousand"),
        (10_000.0, "Ten Thousand Strong"),
        (100_000.0, "Century of Power"),
        (1_000_000.0, "Millionaire"),
        (10_000_000.0, "Energy Baron"),
        (100_000_000.0, "Power Mogul"),
        (1_000_000_000.0, "Billionaire"),
        (10_000_000_000.0, "Energy Tycoon"),
        (100_000_000_000.0, "Power Emperor"),
        (1_000_000_000_000.0, "Trillionaire"),
        (10_000_000_000_000.0, "Cosmic Collector"),
        (100_000_000_000_000.0, "Universal Hoarder"),
        (1_000_000_000_000_000.0, "Quintillionaire"),
        (1e18, "Sextillionaire"),
        (1e21, "Septillionaire"),
    ];
    for (amount, name) in lifetime_milestones {
        let description = Box::leak(
            format!("Earn {} total energy", format_large_number(amount)).into_boxed_str(),
        );
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::TotalEnergyEarned(amount),
        });
        id += 1;
    }

    // Click milestones
    let click_milestones: [(u64, &str); 12] = [
        (100, "Clicker"),
        (500, "Dedicated Clicker"),
        (1_000, "Click Enthusiast"),
        (5_000, "Click Addict"),
        (10_000, "Click Master"),
        (50_000, "Click Legend"),
        (100_000, "Click Deity"),
        (500_000, "Click Overlord"),
        (1_000_000, "Click Immortal"),
        (5_000_000, "Click Transcendent"),
        (10_000_000, "Click Eternal"),
        (50_000_000, "Click Infinite"),
    ];
    for (clicks, name) in click_milestones {
        let description = Box::leak(format!("Click {} times", clicks).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::TotalClicks(clicks),
        });
        id += 1;
    }

    // Upgrade collection milestones
    let upgrade_milestones: [(u32, &str); 10] = [
        (10, "Upgrade Novice"),
        (25, "Upgrade Collector"),
        (50, "Upgrade Enthusiast"),
        (100, "Upgrade Hoarder"),
        (150, "Upgrade Master"),
        (200, "Upgrade Legend"),
        (250, "Upgrade Deity"),
        (300, "Upgrade Overlord"),
        (350, "Upgrade Transcendent"),
        (400, "Upgrade Eternal"),
    ];
    for (count, name) in upgrade_milestones {
        let description = Box::leak(format!("Purchase {} upgrades", count).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::UpgradesPurchased(count),
        });
        id += 1;
    }

    // Time played milestones
    let time_milestones: [(u64, &str); 10] = [
        (60, "One Minute"),
        (300, "Five Minutes"),
        (600, "Ten Minutes"),
        (1800, "Half Hour"),
        (3600, "One Hour"),
        (3600 * 5, "Five Hours"),
        (3600 * 10, "Ten Hours"),
        (3600 * 24, "One Day"),
        (3600 * 24 * 7, "One Week"),
        (3600 * 24 * 30, "One Month"),
    ];
    for (secs, name) in time_milestones {
        let description = Box::leak(format!("Play for {}", format_duration(secs)).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::TimePlayed(secs),
        });
        id += 1;
    }

    // Total producers owned milestones
    let total_producer_milestones: [(u64, &str); 10] = [
        (10, "Small Fleet"),
        (50, "Growing Empire"),
        (100, "Century Collection"),
        (250, "Massive Operation"),
        (500, "Industrial Giant"),
        (1000, "Mega Corporation"),
        (2500, "Galactic Conglomerate"),
        (5000, "Universal Enterprise"),
        (10000, "Cosmic Empire"),
        (25000, "Reality Corporation"),
    ];
    for (count, name) in total_producer_milestones {
        let description = Box::leak(format!("Own {} total producers", count).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::TotalProducers(count),
        });
        id += 1;
    }

    // Ascension milestones
    let ascension_milestones: [(u64, &str); 8] = [
        (1, "First Ascension"),
        (5, "Ascending Star"),
        (10, "Stellar Voyager"),
        (25, "Cosmic Traveler"),
        (50, "Universe Hopper"),
        (100, "Reality Shifter"),
        (250, "Dimension Master"),
        (500, "Eternal Ascendant"),
    ];
    for (count, name) in ascension_milestones {
        let description = Box::leak(format!("Ascend {} times", count).into_boxed_str());
        achievements.push(Achievement {
            id,
            name,
            description,
            requirement: AchievementRequirement::Ascensions(count),
        });
        id += 1;
    }

    achievements
});

fn format_large_number(n: f64) -> String {
    if n >= 1e15 {
        format!("{:.2}Q", n / 1e15)
    } else if n >= 1e12 {
        format!("{:.2}T", n / 1e12)
    } else if n >= 1e9 {
        format!("{:.2}B", n / 1e9)
    } else if n >= 1e6 {
        format!("{:.2}M", n / 1e6)
    } else if n >= 1e3 {
        format!("{:.2}K", n / 1e3)
    } else {
        format!("{:.0}", n)
    }
}

fn format_duration(secs: u64) -> String {
    if secs >= 86400 * 30 {
        format!("{} month(s)", secs / (86400 * 30))
    } else if secs >= 86400 * 7 {
        format!("{} week(s)", secs / (86400 * 7))
    } else if secs >= 86400 {
        format!("{} day(s)", secs / 86400)
    } else if secs >= 3600 {
        format!("{} hour(s)", secs / 3600)
    } else if secs >= 60 {
        format!("{} minute(s)", secs / 60)
    } else {
        format!("{} second(s)", secs)
    }
}
