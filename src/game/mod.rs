mod economy;
mod producer;
mod tick;
mod upgrade;

pub use economy::*;
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
        }
    }

    pub fn producer_count(&self, id: u32) -> u64 {
        *self.producers_owned.get(&id).unwrap_or(&0)
    }

    pub fn add_energy(&mut self, amount: f64) {
        self.energy += amount;
        self.total_energy_earned += amount;
    }

    pub fn buy_producer(&mut self, id: u32, quantity: u64) -> bool {
        let producer = match Producer::all().iter().find(|p| p.id == id) {
            Some(p) => p,
            None => return false,
        };

        let owned = self.producer_count(id);
        let cost = calculate_bulk_cost(producer.base_cost, owned, quantity);

        if self.energy >= cost {
            self.energy -= cost;
            *self.producers_owned.entry(id).or_insert(0) += quantity;
            true
        } else {
            false
        }
    }

    pub fn can_afford_producer(&self, producer: &Producer, owned: u64, quantity: u64) -> bool {
        let cost = calculate_bulk_cost(producer.base_cost, owned, quantity);
        self.energy >= cost
    }

    pub fn max_affordable(&self, producer: &Producer, owned: u64, max_quantity: u64) -> u64 {
        calculate_max_affordable(producer.base_cost, owned, self.energy, max_quantity)
    }

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

        if self.energy >= upgrade.cost {
            self.energy -= upgrade.cost;
            self.upgrades_purchased.push(id);
            true
        } else {
            false
        }
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
        }
    }

    pub fn available_upgrades(&self) -> Vec<&Upgrade> {
        Upgrade::all()
            .iter()
            .filter(|u| self.is_upgrade_available(u))
            .collect()
    }

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
                    UpgradeEffect::GlobalMultiplier(_) | UpgradeEffect::ManualMultiplier(_) => {}
                }
            }
        }

        multiplier
    }

    pub fn get_global_multiplier(&self) -> f64 {
        let mut multiplier = 1.0;

        for upgrade_id in &self.upgrades_purchased {
            if let Some(upgrade) = Upgrade::all().iter().find(|u| u.id == *upgrade_id) {
                if let UpgradeEffect::GlobalMultiplier(m) = upgrade.effect {
                    multiplier *= m;
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
}
