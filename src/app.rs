use std::io;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::game::{GameState, PrestigeUpgrade, Producer};
use crate::save;
use crate::ui::animation::AnimationState;
use crate::TICKS_PER_SECOND;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Producers,
    Upgrades,
    Stats,
    Visualization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutMode {
    #[default]
    Single,
    TwoColumn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuyAmount {
    One,
    Ten,
    Max,
}

impl BuyAmount {
    pub fn next(self) -> Self {
        match self {
            BuyAmount::One => BuyAmount::Ten,
            BuyAmount::Ten => BuyAmount::Max,
            BuyAmount::Max => BuyAmount::One,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            BuyAmount::One => "1",
            BuyAmount::Ten => "10",
            BuyAmount::Max => "Max",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub game_state: GameState,
    pub last_save: DateTime<Utc>,
}

pub struct App {
    pub game: GameState,
    pub selected_producer: usize,
    pub selected_upgrade: usize,
    pub selected_prestige_upgrade: usize,
    pub selected_achievement: usize,
    pub achievement_tab: usize,
    pub active_panel: Panel,
    pub buy_amount: BuyAmount,
    pub boss_mode: bool,
    pub offline_report: Option<OfflineReport>,
    pub layout_mode: LayoutMode,
    pub show_help: bool,
    pub show_prestige: bool,
    pub show_achievements: bool,
    pub animation: AnimationState,
    pub achievement_notification: Option<(String, String)>, // (name, description)
    pub achievement_notification_timer: u32,
    pub upgrade_hover_timer: u32,
    pub last_selected_upgrade: usize,
    pub show_upgrade_tooltip: bool,
    pub show_producer_detail: bool,
    pub auto_mode: bool,
    pub auto_paused: bool,
}

pub struct OfflineReport {
    pub duration_secs: u64,
    pub energy_earned: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: GameState::new(),
            selected_producer: 0,
            selected_upgrade: 0,
            selected_prestige_upgrade: 0,
            selected_achievement: 0,
            achievement_tab: 0,
            active_panel: Panel::Producers,
            buy_amount: BuyAmount::One,
            boss_mode: false,
            offline_report: None,
            layout_mode: LayoutMode::default(),
            show_help: false,
            show_prestige: false,
            show_achievements: false,
            animation: AnimationState::new(),
            achievement_notification: None,
            achievement_notification_timer: 0,
            upgrade_hover_timer: 0,
            last_selected_upgrade: 0,
            show_upgrade_tooltip: false,
            show_producer_detail: true,
            auto_mode: false,
            auto_paused: false,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_prestige(&mut self) {
        self.show_prestige = !self.show_prestige;
    }

    pub fn toggle_achievements(&mut self) {
        self.show_achievements = !self.show_achievements;
    }

    pub fn toggle_producer_detail(&mut self) {
        self.show_producer_detail = !self.show_producer_detail;
        if self.show_achievements {
            self.selected_achievement = 0;
        }
    }

    pub fn cycle_achievement_tab(&mut self) {
        self.achievement_tab = (self.achievement_tab + 1) % 8;
        self.selected_achievement = 0; // Reset selection when changing tabs
    }

    pub fn tick(&mut self) {
        self.game.tick();
        self.animation.tick();

        // Check for new achievements and show notification
        if let Some(achievement) = self.game.pop_new_achievement() {
            self.achievement_notification = Some((
                achievement.name.to_string(),
                achievement.description.to_string(),
            ));
            self.achievement_notification_timer = 30; // Show for 3 seconds (30 ticks)
        }

        // Decrement achievement notification timer
        if self.achievement_notification_timer > 0 {
            self.achievement_notification_timer -= 1;
            if self.achievement_notification_timer == 0 {
                self.achievement_notification = None;
            }
        }

        // Upgrade tooltip hover timer logic
        if self.selected_upgrade != self.last_selected_upgrade {
            self.upgrade_hover_timer = 0;
            self.show_upgrade_tooltip = false;
            self.last_selected_upgrade = self.selected_upgrade;
        } else if self.active_panel == Panel::Upgrades && !self.show_prestige && !self.show_help {
            if self.upgrade_hover_timer < 30 {
                self.upgrade_hover_timer += 1;
                if self.upgrade_hover_timer >= 30 {
                    self.show_upgrade_tooltip = true;
                }
            }
        }
    }

    pub fn toggle_panel(&mut self, panel: Panel) {
        if self.active_panel == panel {
            self.active_panel = Panel::Producers;
        } else {
            self.active_panel = panel;
        }
        self.upgrade_hover_timer = 0;
        self.show_upgrade_tooltip = false;
    }

    pub fn focus_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
        self.upgrade_hover_timer = 0;
        self.show_upgrade_tooltip = false;
    }

    pub fn cycle_focus_right(&mut self) {
        if self.layout_mode == LayoutMode::TwoColumn {
            self.active_panel = match self.active_panel {
                Panel::Producers => Panel::Visualization,
                Panel::Visualization => Panel::Upgrades,
                Panel::Upgrades => Panel::Producers,
                Panel::Stats => Panel::Producers,
            };
            self.upgrade_hover_timer = 0;
            self.show_upgrade_tooltip = false;
        }
    }

    pub fn cycle_focus_left(&mut self) {
        if self.layout_mode == LayoutMode::TwoColumn {
            self.active_panel = match self.active_panel {
                Panel::Producers => Panel::Upgrades,
                Panel::Visualization => Panel::Producers,
                Panel::Upgrades => Panel::Visualization,
                Panel::Stats => Panel::Producers,
            };
            self.upgrade_hover_timer = 0;
            self.show_upgrade_tooltip = false;
        }
    }

    pub fn cycle_buy_amount(&mut self) {
        self.buy_amount = self.buy_amount.next();
    }

    pub fn move_selection_up(&mut self) {
        if self.show_prestige {
            // Navigate prestige upgrades
            if self.selected_prestige_upgrade > 0 {
                self.selected_prestige_upgrade -= 1;
            }
            return;
        }

        match self.active_panel {
            Panel::Producers => {
                if self.selected_producer > 0 {
                    self.selected_producer -= 1;
                }
            }
            Panel::Upgrades => {
                if self.selected_upgrade > 0 {
                    self.selected_upgrade -= 1;
                    self.upgrade_hover_timer = 0;
                    self.show_upgrade_tooltip = false;
                }
            }
            _ => {}
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.show_prestige {
            // Navigate prestige upgrades
            let max = PrestigeUpgrade::all().len().saturating_sub(1);
            if self.selected_prestige_upgrade < max {
                self.selected_prestige_upgrade += 1;
            }
            return;
        }

        match self.active_panel {
            Panel::Producers => {
                let visible = self.game.visible_producers();
                let max = visible.len().saturating_sub(1);
                if self.selected_producer < max {
                    self.selected_producer += 1;
                }
            }
            Panel::Upgrades => {
                let available = self.game.available_upgrades();
                let max = available.len().saturating_sub(1);
                if self.selected_upgrade < max {
                    self.selected_upgrade += 1;
                    self.upgrade_hover_timer = 0;
                    self.show_upgrade_tooltip = false;
                }
            }
            _ => {}
        }
    }

    pub fn purchase_selected(&mut self) {
        if self.show_prestige {
            // Try to buy prestige upgrade or ascend
            self.purchase_prestige_or_ascend();
            return;
        }

        match self.active_panel {
            Panel::Producers => {
                let visible = self.game.visible_producers();
                if self.selected_producer < visible.len() {
                    let (_, producer) = visible[self.selected_producer];
                    let quantity = self.calculate_buy_quantity(producer);
                    if quantity > 0 {
                        self.game.buy_producer(producer.id, quantity);
                        let _ = self.save(); // Save on purchase
                    }
                }
            }
            Panel::Upgrades => {
                let available = self.game.available_upgrades();
                if self.selected_upgrade < available.len() {
                    let upgrade_id = available[self.selected_upgrade].id;
                    if self.game.buy_upgrade(upgrade_id) {
                        // Adjust selection if now out of bounds
                        let new_available = self.game.available_upgrades();
                        if self.selected_upgrade >= new_available.len() && !new_available.is_empty()
                        {
                            self.selected_upgrade = new_available.len() - 1;
                        }
                        let _ = self.save(); // Save on purchase
                    }
                }
            }
            _ => {}
        }
    }

    fn purchase_prestige_or_ascend(&mut self) {
        let all_upgrades = PrestigeUpgrade::all();

        if self.selected_prestige_upgrade < all_upgrades.len() {
            let upgrade = &all_upgrades[self.selected_prestige_upgrade];

            // Try to buy the upgrade
            if self.game.is_prestige_upgrade_available(upgrade) {
                if self.game.buy_prestige_upgrade(upgrade.id) {
                    let _ = self.save();
                    return;
                }
            }
        }

        // If no upgrade purchased and at the top, try to ascend
        if self.selected_prestige_upgrade == 0 && self.game.can_ascend() {
            self.game.perform_ascension();
            self.selected_producer = 0;
            self.selected_upgrade = 0;
            let _ = self.save();
        }
    }

    fn calculate_buy_quantity(&self, producer: &Producer) -> u64 {
        let owned = self.game.producer_count(producer.id);
        match self.buy_amount {
            BuyAmount::One => {
                if self.game.can_afford_producer(producer, owned, 1) {
                    1
                } else {
                    0
                }
            }
            BuyAmount::Ten => {
                if self.game.can_afford_producer(producer, owned, 10) {
                    10
                } else {
                    0 // Can't afford 10, so can't buy anything
                }
            }
            BuyAmount::Max => self.game.max_affordable(producer, owned, u64::MAX),
        }
    }

    pub fn get_buy_quantity_for_producer(&self, producer: &Producer) -> u64 {
        self.calculate_buy_quantity(producer)
    }

    /// Returns the display quantity for UI cost calculations.
    /// This shows the intended quantity (not what you can afford).
    pub fn get_display_quantity_for_producer(&self, producer: &Producer) -> u64 {
        let owned = self.game.producer_count(producer.id);
        match self.buy_amount {
            BuyAmount::One => 1,
            BuyAmount::Ten => 10,
            BuyAmount::Max => self.game.max_affordable(producer, owned, u64::MAX).max(1),
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let save_data = SaveData {
            game_state: self.game.clone(),
            last_save: Utc::now(),
        };
        save::save_game(&save_data)
    }

    pub fn load(&mut self) -> io::Result<()> {
        if let Some(save_data) = save::load_game()? {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(save_data.last_save);
            let elapsed_secs = elapsed.num_seconds().max(0) as u64;

            // Calculate offline progress (capped at 8 hours)
            let max_offline_secs = 8 * 60 * 60;
            let capped_secs = elapsed_secs.min(max_offline_secs);

            self.game = save_data.game_state;

            if capped_secs > 60 {
                // Only show report if offline for more than a minute
                let energy_per_tick = self.game.total_energy_per_second() / TICKS_PER_SECOND;
                let ticks = capped_secs * TICKS_PER_SECOND as u64;

                // Apply offline bonus from prestige upgrades
                let offline_bonus = self.game.get_offline_bonus_multiplier();
                let energy_earned = energy_per_tick * ticks as f64 * offline_bonus;

                self.game.add_energy(energy_earned);

                self.offline_report = Some(OfflineReport {
                    duration_secs: capped_secs,
                    energy_earned,
                });
            }
        }
        Ok(())
    }

    pub fn clear_offline_report(&mut self) {
        self.offline_report = None;
    }

    pub fn manual_mine(&mut self) {
        self.game.manual_mine();
    }
}
