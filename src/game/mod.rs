mod achievement;
mod economy;
mod prestige;
mod producer;
mod tick;
mod upgrade;

pub use achievement::*;
pub use economy::*;
pub use prestige::*;
pub use producer::*;
pub use upgrade::*;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub energy: f64,
    pub total_energy_earned: f64,
    pub producers_owned: HashMap<u32, u64>,
    pub upgrades_purchased: Vec<u32>,
    pub ticks_played: u64,
    #[serde(default = "default_manual_click_power")]
    pub manual_click_power: f64,
    #[serde(default = "default_manual_multiplier")]
    pub manual_multiplier: f64,
    #[serde(default)]
    pub total_manual_clicks: u64,
    #[serde(skip)]
    pub energy_produced_history: VecDeque<f64>,

    // Achievement system
    #[serde(default)]
    pub achievements_unlocked: Vec<u32>,

    // Prestige system
    #[serde(default)]
    pub stellar_chips: u64,
    #[serde(default)]
    pub total_stellar_chips_earned: u64,
    #[serde(default)]
    pub total_ascensions: u64,
    #[serde(default)]
    pub prestige_upgrades: Vec<u32>,

    // Track newly unlocked achievements for notifications
    #[serde(skip)]
    pub new_achievements: Vec<u32>,

    // Per-producer lifetime energy tracking (resets on ascension)
    #[serde(default)]
    pub producer_lifetime_energy: HashMap<u32, f64>,
}

fn default_manual_click_power() -> f64 {
    1.0
}

fn default_manual_multiplier() -> f64 {
    1.0
}

impl GameState {
    pub fn new() -> Self {
        Self {
            energy: 0.0,
            total_energy_earned: 0.0,
            producers_owned: HashMap::new(),
            upgrades_purchased: Vec::new(),
            ticks_played: 0,
            manual_click_power: 1.0,
            manual_multiplier: 1.0,
            total_manual_clicks: 0,
            energy_produced_history: VecDeque::with_capacity(10),
            achievements_unlocked: Vec::new(),
            stellar_chips: 0,
            total_stellar_chips_earned: 0,
            total_ascensions: 0,
            prestige_upgrades: Vec::new(),
            new_achievements: Vec::new(),
            producer_lifetime_energy: HashMap::new(),
        }
    }

    // ============ Producer Visibility System ============

    /// Check if a producer is unlocked
    /// A producer is unlocked if:
    /// - It's the first producer (Solar Panel)
    /// - OR the previous producer has been purchased at least once
    /// - OR a prestige upgrade unlocks it
    pub fn is_producer_unlocked(&self, producer_id: u32) -> bool {
        if producer_id == 1 {
            return true;
        }

        // Check prestige unlock
        let prestige_unlock_level = self.get_prestige_unlock_level();
        if producer_id <= prestige_unlock_level {
            return true;
        }

        // Check if previous producer owned
        self.producer_count(producer_id - 1) >= 1
    }

    /// Get the number of producers unlocked by prestige upgrades
    fn get_prestige_unlock_level(&self) -> u32 {
        let mut max_unlock = 1u32; // Always have Solar Panel

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::UnlockProducers(count) = upgrade.effect {
                    max_unlock = max_unlock.max(count);
                }
            }
        }

        max_unlock
    }

    /// Get all unlocked producers
    pub fn visible_producers(&self) -> Vec<(usize, &Producer)> {
        let all_producers = Producer::all();
        let mut visible = Vec::new();

        for (idx, producer) in all_producers.iter().enumerate() {
            if self.is_producer_unlocked(producer.id) {
                visible.push((idx, producer));
            } else {
                break;
            }
        }

        visible
    }

    // ============ Producer Management ============

    pub fn producer_count(&self, id: u32) -> u64 {
        *self.producers_owned.get(&id).unwrap_or(&0)
    }

    pub fn total_producers_owned(&self) -> u64 {
        self.producers_owned.values().sum()
    }

    /// Get lifetime energy produced by a specific producer
    pub fn producer_lifetime_production(&self, producer_id: u32) -> f64 {
        *self.producer_lifetime_energy.get(&producer_id).unwrap_or(&0.0)
    }

    /// Get total production rate for a specific producer (count * rate * multipliers)
    pub fn producer_total_rate(&self, producer_id: u32) -> f64 {
        let producer = match Producer::all().iter().find(|p| p.id == producer_id) {
            Some(p) => p,
            None => return 0.0,
        };
        let count = self.producer_count(producer_id);
        let producer_mult = self.get_producer_multiplier(producer_id);
        let global_mult = self.get_global_multiplier();
        producer.base_energy_per_second * count as f64 * producer_mult * global_mult
    }

    /// Get percentage of total production from a specific producer
    pub fn producer_production_percentage(&self, producer_id: u32) -> f64 {
        let total_rate = self.total_energy_per_second();
        if total_rate <= 0.0 {
            return 0.0;
        }
        let producer_rate = self.producer_total_rate(producer_id);
        (producer_rate / total_rate) * 100.0
    }

    pub fn add_energy(&mut self, amount: f64) {
        self.energy += amount;
        self.total_energy_earned += amount;
    }

    pub fn buy_producer(&mut self, id: u32, quantity: u64) -> bool {
        // Check if producer is unlocked
        if !self.is_producer_unlocked(id) {
            return false;
        }

        let producer = match Producer::all().iter().find(|p| p.id == id) {
            Some(p) => p,
            None => return false,
        };

        let owned = self.producer_count(id);
        let cost = calculate_bulk_cost(producer.base_cost, owned, quantity, id);

        if self.energy >= cost {
            self.energy -= cost;
            *self.producers_owned.entry(id).or_insert(0) += quantity;
            true
        } else {
            false
        }
    }

    pub fn can_afford_producer(&self, producer: &Producer, owned: u64, quantity: u64) -> bool {
        let cost = calculate_bulk_cost(producer.base_cost, owned, quantity, producer.id);
        self.energy >= cost
    }

    pub fn max_affordable(&self, producer: &Producer, owned: u64, max_quantity: u64) -> u64 {
        calculate_max_affordable(producer.base_cost, owned, self.energy, max_quantity, producer.id)
    }

    // ============ Upgrade Management ============

    pub fn buy_upgrade(&mut self, id: u32) -> bool {
        if self.upgrades_purchased.contains(&id) {
            return false;
        }

        let upgrade = match Upgrade::all().iter().find(|u| u.id == id) {
            Some(u) => u,
            None => return false,
        };

        if !self.is_upgrade_available(upgrade) {
            return false;
        }

        let cost = self.get_upgrade_cost(upgrade);

        if self.energy >= cost {
            self.energy -= cost;
            self.upgrades_purchased.push(id);
            true
        } else {
            false
        }
    }

    /// Get the cost of an upgrade, with prestige discounts applied
    pub fn get_upgrade_cost(&self, upgrade: &Upgrade) -> f64 {
        let discount = self.get_upgrade_cost_reduction();
        upgrade.cost * (1.0 - discount)
    }

    /// Get total upgrade cost reduction from prestige
    fn get_upgrade_cost_reduction(&self) -> f64 {
        let mut reduction = 0.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::UpgradeCostReduction(r) = upgrade.effect {
                    reduction += r;
                }
            }
        }

        reduction.min(0.50) // Cap at 50% reduction
    }

    pub fn is_upgrade_available(&self, upgrade: &Upgrade) -> bool {
        if self.upgrades_purchased.contains(&upgrade.id) {
            return false;
        }

        match upgrade.requirement {
            UpgradeRequirement::ProducerCount { producer_id, count } => {
                self.producer_count(producer_id) >= count
            }
            UpgradeRequirement::TotalEnergyPerSecond(rate) => {
                self.total_energy_per_second() >= rate
            }
            UpgradeRequirement::ManualClicks(clicks) => {
                self.total_manual_clicks >= clicks
            }
            UpgradeRequirement::ProducersPair { id_a, count_a, id_b, count_b } => {
                self.producer_count(id_a) >= count_a && self.producer_count(id_b) >= count_b
            }
        }
    }

    pub fn available_upgrades(&self) -> Vec<&Upgrade> {
        Upgrade::all()
            .iter()
            .filter(|u| self.is_upgrade_available(u))
            .collect()
    }

    // ============ Multiplier Calculations ============

    pub fn get_producer_multiplier(&self, producer_id: u32) -> f64 {
        let mut multiplier = 1.0;

        // Producer-specific upgrades
        for upgrade_id in &self.upgrades_purchased {
            if let Some(upgrade) = Upgrade::all().iter().find(|u| u.id == *upgrade_id) {
                match upgrade.effect {
                    UpgradeEffect::ProducerMultiplier { producer_id: pid, multiplier: m } => {
                        if pid == producer_id {
                            multiplier *= m;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Synergy bonuses
        multiplier *= self.get_synergy_multiplier(producer_id);

        multiplier
    }

    /// Calculate synergy bonus for a producer (capped at 2.5x)
    fn get_synergy_multiplier(&self, target_id: u32) -> f64 {
        let mut bonus = 1.0;

        for upgrade_id in &self.upgrades_purchased {
            if let Some(upgrade) = Upgrade::all().iter().find(|u| u.id == *upgrade_id) {
                if let UpgradeEffect::Synergy { source_id, target_id: tid, bonus_per_source } = upgrade.effect {
                    if tid == target_id {
                        let source_count = self.producer_count(source_id);
                        bonus *= 1.0 + (bonus_per_source * source_count as f64);
                    }
                }
            }
        }

        // Cap synergy bonus at 2.5x to prevent late-game acceleration
        bonus.min(2.5)
    }

    pub fn get_global_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        // Global upgrades
        for upgrade_id in &self.upgrades_purchased {
            if let Some(upgrade) = Upgrade::all().iter().find(|u| u.id == *upgrade_id) {
                if let UpgradeEffect::GlobalMultiplier(m) = upgrade.effect {
                    multiplier *= m;
                }
            }
        }

        // Achievement bonus (1% per achievement)
        multiplier *= self.get_achievement_multiplier();

        // Prestige bonuses
        multiplier *= self.get_prestige_production_multiplier();

        multiplier
    }

    /// Get achievement bonus multiplier (1.01^n)
    pub fn get_achievement_multiplier(&self) -> f64 {
        ACHIEVEMENT_BONUS.powi(self.achievements_unlocked.len() as i32)
    }

    /// Get prestige production multiplier
    fn get_prestige_production_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                match upgrade.effect {
                    PrestigeEffect::ProductionMultiplier(m) => {
                        multiplier *= m;
                    }
                    PrestigeEffect::ProductionPerAscension(bonus) => {
                        // Cap per-ascension bonus at 2.0x (100% bonus) to prevent late-game acceleration
                        let ascension_mult = (1.0 + (bonus * self.total_ascensions as f64)).min(2.0);
                        multiplier *= ascension_mult;
                    }
                    PrestigeEffect::ProductionPerAchievement(bonus) => {
                        multiplier *= 1.0 + (bonus * self.achievements_unlocked.len() as f64);
                    }
                    _ => {}
                }
            }
        }

        multiplier
    }

    pub fn total_energy_per_second(&self) -> f64 {
        let global_mult = self.get_global_multiplier();

        Producer::all()
            .iter()
            .map(|p| {
                let count = self.producer_count(p.id);
                let producer_mult = self.get_producer_multiplier(p.id);
                p.base_energy_per_second * count as f64 * producer_mult * global_mult
            })
            .sum()
    }

    pub fn time_played_seconds(&self) -> u64 {
        self.ticks_played / 10 // 10 ticks per second
    }

    pub fn get_manual_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        for upgrade_id in &self.upgrades_purchased {
            if let Some(upgrade) = Upgrade::all().iter().find(|u| u.id == *upgrade_id) {
                if let UpgradeEffect::ManualMultiplier(m) = upgrade.effect {
                    multiplier *= m;
                }
            }
        }

        multiplier
    }

    pub fn manual_mine(&mut self) -> f64 {
        let manual_mult = self.get_manual_multiplier();
        let eps_bonus = self.total_energy_per_second() * 0.05;
        let energy_gained = (self.manual_click_power * manual_mult) + eps_bonus;

        self.energy += energy_gained;
        self.total_energy_earned += energy_gained;
        self.total_manual_clicks += 1;

        // Add to current tick's tracking for rate display
        if let Some(last) = self.energy_produced_history.back_mut() {
            *last += energy_gained;
        } else {
            self.energy_produced_history.push_back(energy_gained);
        }

        energy_gained
    }

    pub fn actual_energy_per_second(&self) -> f64 {
        self.energy_produced_history.iter().sum()
    }

    pub fn effective_manual_power(&self) -> f64 {
        let manual_mult = self.get_manual_multiplier();
        let eps_bonus = self.total_energy_per_second() * 0.05;
        (self.manual_click_power * manual_mult) + eps_bonus
    }

    // ============ Achievement System ============

    /// Check for new achievements and unlock them
    pub fn check_achievements(&mut self) {
        for achievement in Achievement::all() {
            if self.achievements_unlocked.contains(&achievement.id) {
                continue;
            }

            let earned = match achievement.requirement {
                AchievementRequirement::ProducerCount { producer_id, count } => {
                    self.producer_count(producer_id) >= count
                }
                AchievementRequirement::TotalEnergyPerSecond(rate) => {
                    self.total_energy_per_second() >= rate
                }
                AchievementRequirement::TotalEnergyEarned(amount) => {
                    self.total_energy_earned >= amount
                }
                AchievementRequirement::TotalClicks(clicks) => {
                    self.total_manual_clicks >= clicks
                }
                AchievementRequirement::UpgradesPurchased(count) => {
                    self.upgrades_purchased.len() >= count as usize
                }
                AchievementRequirement::TimePlayed(secs) => {
                    self.time_played_seconds() >= secs
                }
                AchievementRequirement::TotalProducers(count) => {
                    self.total_producers_owned() >= count
                }
                AchievementRequirement::Ascensions(count) => {
                    self.total_ascensions >= count
                }
            };

            if earned {
                self.achievements_unlocked.push(achievement.id);
                self.new_achievements.push(achievement.id);
            }
        }
    }

    /// Pop a new achievement for notification display
    pub fn pop_new_achievement(&mut self) -> Option<&'static Achievement> {
        let id = self.new_achievements.pop()?;
        Achievement::all().iter().find(|a| a.id == id)
    }

    // ============ Prestige System ============

    /// Calculate stellar chips that would be earned on ascension
    pub fn calculate_potential_stellar_chips(&self) -> u64 {
        let base_chips = calculate_stellar_chips(self.total_energy_earned);

        // Apply chip bonus from prestige upgrades
        let chip_multiplier = self.get_chip_multiplier();
        let total_chips = (base_chips as f64 * chip_multiplier) as u64;

        total_chips.saturating_sub(self.total_stellar_chips_earned)
    }

    /// Get the chip earning multiplier from prestige upgrades
    fn get_chip_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::ChipBonus(bonus) = upgrade.effect {
                    multiplier *= bonus;
                }
            }
        }

        multiplier
    }

    /// Check if player can ascend (would earn at least 1 chip)
    pub fn can_ascend(&self) -> bool {
        self.calculate_potential_stellar_chips() >= 1
    }

    /// Perform ascension - reset game state but keep prestige progress
    pub fn perform_ascension(&mut self) {
        let chips_earned = self.calculate_potential_stellar_chips();

        if chips_earned == 0 {
            return;
        }

        // Calculate energy to keep (if prestige upgrade purchased)
        let keep_percent = self.get_energy_keep_percent();
        let kept_energy = self.energy * keep_percent;

        // Calculate starting energy from prestige upgrades
        let starting_energy = self.get_starting_energy();

        // Grant chips
        self.stellar_chips += chips_earned;
        self.total_stellar_chips_earned += chips_earned;
        self.total_ascensions += 1;

        // Reset game state
        self.energy = starting_energy + kept_energy;
        self.total_energy_earned = 0.0;
        self.producers_owned.clear();
        self.upgrades_purchased.clear();
        self.ticks_played = 0;
        self.total_manual_clicks = 0;
        self.energy_produced_history.clear();
        self.producer_lifetime_energy.clear();
        // Keep: achievements_unlocked, stellar_chips, prestige_upgrades, total_ascensions

        // Note: Achievements are kept across ascensions!
    }

    /// Get percentage of energy to keep after ascension
    fn get_energy_keep_percent(&self) -> f64 {
        let mut percent = 0.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::KeepEnergyPercent(p) = upgrade.effect {
                    percent += p;
                }
            }
        }

        percent.min(0.10) // Cap at 10%
    }

    /// Get starting energy from prestige upgrades
    fn get_starting_energy(&self) -> f64 {
        let mut energy = 0.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::StartingEnergy(e) = upgrade.effect {
                    energy += e;
                }
            }
        }

        energy
    }

    /// Get offline bonus multiplier from prestige
    pub fn get_offline_bonus_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        for upgrade_id in &self.prestige_upgrades {
            if let Some(upgrade) = PrestigeUpgrade::by_id(*upgrade_id) {
                if let PrestigeEffect::OfflineBonus(bonus) = upgrade.effect {
                    multiplier *= bonus;
                }
            }
        }

        multiplier
    }

    /// Check if a prestige upgrade is available
    pub fn is_prestige_upgrade_available(&self, upgrade: &PrestigeUpgrade) -> bool {
        if self.prestige_upgrades.contains(&upgrade.id) {
            return false;
        }

        if self.stellar_chips < upgrade.cost {
            return false;
        }

        match upgrade.requirement {
            None => true,
            Some(PrestigeRequirement::Ascensions(count)) => {
                self.total_ascensions >= count
            }
            Some(PrestigeRequirement::TotalChips(count)) => {
                self.total_stellar_chips_earned >= count
            }
            Some(PrestigeRequirement::PrestigeUpgrade(required_id)) => {
                self.prestige_upgrades.contains(&required_id)
            }
        }
    }

    /// Buy a prestige upgrade
    pub fn buy_prestige_upgrade(&mut self, id: u32) -> bool {
        let upgrade = match PrestigeUpgrade::by_id(id) {
            Some(u) => u,
            None => return false,
        };

        if !self.is_prestige_upgrade_available(upgrade) {
            return false;
        }

        self.stellar_chips -= upgrade.cost;
        self.prestige_upgrades.push(id);
        true
    }

}
