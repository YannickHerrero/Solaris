use crate::app::{App, BuyAmount, Panel};
use crate::game::{calculate_bulk_cost, Producer};

/// Maximum ROI in seconds for auto-buying producers (10 minutes)
const MAX_ROI_SECONDS: f64 = 600.0;

/// AutoPlayer state machine states
#[derive(Debug, Clone)]
enum AutoState {
    /// Waiting before making a decision (simulates human thinking)
    Idle { ticks_remaining: u32 },
    /// Evaluating what to buy next
    Deciding,
    /// Switching focus to the target panel
    FocusingPanel {
        target: AutoTarget,
        ticks_remaining: u32,
    },
    /// Moving cursor one step at a time toward the target
    MovingCursor {
        target: AutoTarget,
        ticks_remaining: u32,
    },
    /// Pressing Enter to purchase
    Purchasing { ticks_remaining: u32 },
    /// Brief pause after a purchase
    CooldownAfterPurchase { ticks_remaining: u32 },
    /// Nothing to buy, wait longer before re-checking
    WaitingForFunds { ticks_remaining: u32 },
}

/// What the auto-player wants to buy
#[derive(Debug, Clone)]
enum AutoTarget {
    Producer { index: usize },
    Upgrade { index: usize },
}

impl AutoTarget {
    fn panel(&self) -> Panel {
        match self {
            AutoTarget::Producer { .. } => Panel::Producers,
            AutoTarget::Upgrade { .. } => Panel::Upgrades,
        }
    }

    fn index(&self) -> usize {
        match self {
            AutoTarget::Producer { index } | AutoTarget::Upgrade { index } => *index,
        }
    }
}

/// Simple deterministic pseudo-random number generator (xorshift32)
struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    /// Returns a value in [min, max] inclusive
    fn range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        min + (self.next() % (max - min + 1))
    }
}

/// Duration in ticks to pause auto-player when user interacts (5 seconds at 10 Hz)
const PAUSE_TICKS: u32 = 50;

pub struct AutoPlayer {
    state: AutoState,
    rng: SimpleRng,
    pause_ticks_remaining: u32,
}

impl AutoPlayer {
    pub fn new() -> Self {
        Self {
            state: AutoState::Idle {
                ticks_remaining: 20, // 2 seconds initial delay
            },
            rng: SimpleRng::new(42),
            pause_ticks_remaining: 0,
        }
    }

    /// Pause the auto-player for 5 seconds, giving control to the user.
    /// Resets internal state so the bot re-evaluates from scratch when resuming.
    pub fn pause(&mut self) {
        self.pause_ticks_remaining = PAUSE_TICKS;
        self.state = AutoState::Idle { ticks_remaining: 0 };
    }

    /// Returns true if the auto-player is currently paused (user has control).
    pub fn is_paused(&self) -> bool {
        self.pause_ticks_remaining > 0
    }

    /// Called every game tick (10 Hz). Drives the state machine.
    pub fn tick(&mut self, app: &mut App) {
        // If paused, count down and do nothing
        if self.pause_ticks_remaining > 0 {
            self.pause_ticks_remaining -= 1;
            return;
        }

        // Force buy amount to One for predictable ROI calculations
        if app.buy_amount != BuyAmount::One {
            app.buy_amount = BuyAmount::One;
        }

        // Close any overlays that might have been opened (safety)
        if app.show_prestige {
            app.toggle_prestige();
        }
        if app.show_achievements {
            app.toggle_achievements();
        }
        if app.show_help {
            app.toggle_help();
        }

        match self.state.clone() {
            AutoState::Idle { ticks_remaining } => {
                app.manual_mine();
                if ticks_remaining > 0 {
                    self.state = AutoState::Idle {
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    self.state = AutoState::Deciding;
                }
            }

            AutoState::Deciding => {
                if let Some(target) = self.decide_next_action(app) {
                    let delay = self.rng.range(3, 5);
                    // Focus the panel immediately on entering this state
                    app.focus_panel(target.panel());
                    self.state = AutoState::FocusingPanel {
                        target,
                        ticks_remaining: delay,
                    };
                } else {
                    // Nothing to buy right now, wait and re-check
                    let delay = self.rng.range(20, 50);
                    self.state = AutoState::WaitingForFunds {
                        ticks_remaining: delay,
                    };
                }
            }

            AutoState::FocusingPanel {
                target,
                ticks_remaining,
            } => {
                if ticks_remaining > 0 {
                    self.state = AutoState::FocusingPanel {
                        target,
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    let move_delay = self.rng.range(1, 2);
                    self.state = AutoState::MovingCursor {
                        target,
                        ticks_remaining: move_delay,
                    };
                }
            }

            AutoState::MovingCursor {
                target,
                ticks_remaining,
            } => {
                if ticks_remaining > 0 {
                    self.state = AutoState::MovingCursor {
                        target,
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    let current_index = self.current_selection(app, &target);
                    let target_index = target.index();

                    if current_index == target_index {
                        // We're at the right position, purchase after a brief pause
                        let delay = self.rng.range(2, 4);
                        self.state = AutoState::Purchasing {
                            ticks_remaining: delay,
                        };
                    } else if current_index < target_index {
                        app.move_selection_down();
                        let delay = self.rng.range(1, 2);
                        self.state = AutoState::MovingCursor {
                            target,
                            ticks_remaining: delay,
                        };
                    } else {
                        app.move_selection_up();
                        let delay = self.rng.range(1, 2);
                        self.state = AutoState::MovingCursor {
                            target,
                            ticks_remaining: delay,
                        };
                    }
                }
            }

            AutoState::Purchasing { ticks_remaining } => {
                if ticks_remaining > 0 {
                    self.state = AutoState::Purchasing {
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    app.purchase_selected();
                    let delay = self.rng.range(5, 10);
                    self.state = AutoState::CooldownAfterPurchase {
                        ticks_remaining: delay,
                    };
                }
            }

            AutoState::CooldownAfterPurchase { ticks_remaining } => {
                app.manual_mine();
                if ticks_remaining > 0 {
                    self.state = AutoState::CooldownAfterPurchase {
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    self.state = AutoState::Deciding;
                }
            }

            AutoState::WaitingForFunds { ticks_remaining } => {
                app.manual_mine();
                if ticks_remaining > 0 {
                    self.state = AutoState::WaitingForFunds {
                        ticks_remaining: ticks_remaining - 1,
                    };
                } else {
                    self.state = AutoState::Deciding;
                }
            }
        }
    }

    /// Decide what to buy next. Returns None if nothing is affordable/worthwhile.
    fn decide_next_action(&mut self, app: &App) -> Option<AutoTarget> {
        // Priority 1: Find the best producer with ROI < 10 minutes
        if let Some(target) = self.find_best_producer(app) {
            return Some(target);
        }

        // Priority 2: Buy the cheapest affordable upgrade
        if let Some(target) = self.find_best_upgrade(app) {
            return Some(target);
        }

        None
    }

    /// Find the producer with the lowest ROI that is < 10 minutes and affordable.
    fn find_best_producer(&self, app: &App) -> Option<AutoTarget> {
        let visible = app.game.visible_producers();
        let mut best: Option<(usize, f64)> = None; // (display_index, roi_seconds)

        for (display_idx, (_, producer)) in visible.iter().enumerate() {
            let owned = app.game.producer_count(producer.id);
            let cost = calculate_bulk_cost(producer.base_cost, owned, 1, producer.id);

            // Can we afford it?
            if app.game.energy < cost {
                continue;
            }

            let rate_gain = self.calculate_rate_gain(app, producer, 1);
            if rate_gain <= 0.0 {
                continue;
            }

            let roi_seconds = cost / rate_gain;

            if roi_seconds > MAX_ROI_SECONDS {
                continue;
            }

            match best {
                None => {
                    best = Some((display_idx, roi_seconds));
                }
                Some((_, best_roi)) if roi_seconds < best_roi => {
                    best = Some((display_idx, roi_seconds));
                }
                _ => {}
            }
        }

        best.map(|(index, _)| AutoTarget::Producer { index })
    }

    /// Calculate the E/s gain from buying `quantity` of a producer.
    fn calculate_rate_gain(&self, app: &App, producer: &Producer, quantity: u64) -> f64 {
        if producer.id == 1 {
            // Solar Panel: include Thousand Rays bonus
            let base_rate =
                producer.base_energy_per_second * app.game.get_producer_multiplier(producer.id);
            let tr_bonus = app.game.get_thousand_rays_bonus();
            let non_collector_count = app.game.get_non_collector_building_count();
            let rate_per_panel = base_rate + (tr_bonus * non_collector_count as f64);
            rate_per_panel * app.game.get_global_multiplier() * quantity as f64
        } else {
            let effective_rate = producer.base_energy_per_second
                * app.game.get_producer_multiplier(producer.id)
                * app.game.get_global_multiplier();
            effective_rate * quantity as f64
        }
    }

    /// Find the cheapest affordable upgrade.
    fn find_best_upgrade(&self, app: &App) -> Option<AutoTarget> {
        let available = app.game.available_upgrades();

        let mut best: Option<(usize, f64)> = None; // (index, cost)

        for (idx, upgrade) in available.iter().enumerate() {
            let cost = app.game.get_upgrade_cost(upgrade);

            if app.game.energy < cost {
                continue;
            }

            match best {
                None => {
                    best = Some((idx, cost));
                }
                Some((_, best_cost)) if cost < best_cost => {
                    best = Some((idx, cost));
                }
                _ => {}
            }
        }

        best.map(|(index, _)| AutoTarget::Upgrade { index })
    }

    /// Get the current selection index for the target's panel.
    fn current_selection(&self, app: &App, target: &AutoTarget) -> usize {
        match target {
            AutoTarget::Producer { .. } => app.selected_producer,
            AutoTarget::Upgrade { .. } => app.selected_upgrade,
        }
    }
}
