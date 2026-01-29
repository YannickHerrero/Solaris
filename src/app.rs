use std::io;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::game::{GameState, Producer};
use crate::save;
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
    pub active_panel: Panel,
    pub buy_amount: BuyAmount,
    pub boss_mode: bool,
    pub offline_report: Option<OfflineReport>,
    pub layout_mode: LayoutMode,
    pub show_help: bool,
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
            active_panel: Panel::Producers,
            buy_amount: BuyAmount::One,
            boss_mode: false,
            offline_report: None,
            layout_mode: LayoutMode::default(),
            show_help: false,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn tick(&mut self) {
        self.game.tick();
    }

    pub fn toggle_panel(&mut self, panel: Panel) {
        if self.active_panel == panel {
            self.active_panel = Panel::Producers;
        } else {
            self.active_panel = panel;
        }
    }

    pub fn focus_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
    }

    pub fn cycle_focus_right(&mut self) {
        if self.layout_mode == LayoutMode::TwoColumn {
            self.active_panel = match self.active_panel {
                Panel::Producers => Panel::Visualization,
                Panel::Visualization => Panel::Upgrades,
                Panel::Upgrades => Panel::Producers,
                Panel::Stats => Panel::Producers,
            };
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
        }
    }

    pub fn cycle_buy_amount(&mut self) {
        self.buy_amount = self.buy_amount.next();
    }

    pub fn move_selection_up(&mut self) {
        match self.active_panel {
            Panel::Producers => {
                if self.selected_producer > 0 {
                    self.selected_producer -= 1;
                }
            }
            Panel::Upgrades => {
                if self.selected_upgrade > 0 {
                    self.selected_upgrade -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.active_panel {
            Panel::Producers => {
                let max = Producer::all().len().saturating_sub(1);
                if self.selected_producer < max {
                    self.selected_producer += 1;
                }
            }
            Panel::Upgrades => {
                let available = self.game.available_upgrades();
                let max = available.len().saturating_sub(1);
                if self.selected_upgrade < max {
                    self.selected_upgrade += 1;
                }
            }
            _ => {}
        }
    }

    pub fn purchase_selected(&mut self) {
        match self.active_panel {
            Panel::Producers => {
                let producers = Producer::all();
                if self.selected_producer < producers.len() {
                    let producer = &producers[self.selected_producer];
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
                        let _ = self.save(); // Save on purchase
                    }
                }
            }
            _ => {}
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
                    // Buy as many as possible up to 10
                    self.game.max_affordable(producer, owned, 10)
                }
            }
            BuyAmount::Max => self.game.max_affordable(producer, owned, u64::MAX),
        }
    }

    pub fn get_buy_quantity_for_producer(&self, producer: &Producer) -> u64 {
        self.calculate_buy_quantity(producer)
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
                let energy_earned = energy_per_tick * ticks as f64;

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
