use crate::app::{App, BuyAmount, Panel};
use crate::game::{calculate_bulk_cost, Producer, Upgrade};

// ============ Configuration ============

/// Look-ahead: if a high-value purchase has a time-to-save shorter than this
/// multiplier times the best current option's ROI, we wait for it instead.
const SAVE_THRESHOLD_MULTIPLIER: f64 = 0.8;

/// Duration in ticks to pause auto-player when user interacts (5 seconds at 10 Hz)
const PAUSE_TICKS: u32 = 50;

// ============ State Machine ============

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

/// A scored candidate for purchase, used to compare producers and upgrades uniformly.
#[derive(Debug, Clone)]
struct ScoredCandidate {
    target: AutoTarget,
    cost: f64,
    #[allow(dead_code)]
    eps_gain: f64,
    roi_seconds: f64,
}

// ============ Simple RNG ============

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

    fn range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        min + (self.next() % (max - min + 1))
    }
}

// ============ AutoPlayer ============

pub struct AutoPlayer {
    state: AutoState,
    rng: SimpleRng,
    pause_ticks_remaining: u32,
    speed: f64,
}

impl AutoPlayer {
    pub fn new(speed: f64) -> Self {
        let clamped_speed = speed.max(0.1);
        Self {
            state: AutoState::Idle {
                ticks_remaining: Self::scale_delay_with(20, clamped_speed),
            },
            rng: SimpleRng::new(42),
            pause_ticks_remaining: 0,
            speed: clamped_speed,
        }
    }

    /// Generate a random delay in [min, max] and scale it by the speed multiplier.
    /// Always returns at least 1 tick.
    fn scaled_range(&mut self, min: u32, max: u32) -> u32 {
        let raw = self.rng.range(min, max);
        (raw as f64 / self.speed).ceil().max(1.0) as u32
    }

    fn scale_delay_with(ticks: u32, speed: f64) -> u32 {
        (ticks as f64 / speed).ceil().max(1.0) as u32
    }

    /// Returns true if cursor animation should be skipped (instant jump).
    /// At speed >= 5x, moving one step at a time is pointless.
    fn should_skip_cursor_animation(&self) -> bool {
        self.speed >= 5.0
    }

    /// Pause the auto-player for 5 seconds, giving control to the user.
    pub fn pause(&mut self) {
        self.pause_ticks_remaining = PAUSE_TICKS;
        self.state = AutoState::Idle { ticks_remaining: 0 };
    }

    /// Returns true if the auto-player is currently paused.
    pub fn is_paused(&self) -> bool {
        self.pause_ticks_remaining > 0
    }

    /// Called every game tick (10 Hz).
    pub fn tick(&mut self, app: &mut App) {
        if self.pause_ticks_remaining > 0 {
            self.pause_ticks_remaining -= 1;
            return;
        }

        if app.buy_amount != BuyAmount::One {
            app.buy_amount = BuyAmount::One;
        }

        // Close overlays (safety)
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
                    let delay = self.scaled_range(3, 5);
                    app.focus_panel(target.panel());
                    self.state = AutoState::FocusingPanel {
                        target,
                        ticks_remaining: delay,
                    };
                } else {
                    let delay = self.scaled_range(20, 50);
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
                    if self.should_skip_cursor_animation() {
                        // Jump directly to target position
                        self.jump_cursor_to(app, &target);
                        let delay = self.scaled_range(2, 4);
                        self.state = AutoState::Purchasing {
                            ticks_remaining: delay,
                        };
                    } else {
                        let move_delay = self.scaled_range(1, 2);
                        self.state = AutoState::MovingCursor {
                            target,
                            ticks_remaining: move_delay,
                        };
                    }
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
                        let delay = self.scaled_range(2, 4);
                        self.state = AutoState::Purchasing {
                            ticks_remaining: delay,
                        };
                    } else if current_index < target_index {
                        app.move_selection_down();
                        let delay = self.scaled_range(1, 2);
                        self.state = AutoState::MovingCursor {
                            target,
                            ticks_remaining: delay,
                        };
                    } else {
                        app.move_selection_up();
                        let delay = self.scaled_range(1, 2);
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
                    let delay = self.scaled_range(5, 10);
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

    // ============ Decision Engine ============

    /// Decide the globally optimal next action.
    ///
    /// Strategy (in priority order):
    /// 1. Buy first unit of a newly available producer (unlock next tier) if affordable.
    /// 2. Score all affordable producers and upgrades by ROI (cost / E/s gain).
    /// 3. Look-ahead: if a not-yet-affordable option has very high value and the
    ///    time-to-save for it is shorter than the best affordable option's ROI,
    ///    wait (return None) to save for it.
    /// 4. Otherwise, buy the affordable option with the lowest ROI.
    fn decide_next_action(&mut self, app: &App) -> Option<AutoTarget> {
        let current_eps = app.game.total_energy_per_second();

        // Priority 1: Buy first unit of newly unlocked producers to unlock next tier
        if let Some(target) = self.find_unlock_producer(app) {
            return Some(target);
        }

        // Gather all candidates (affordable and not-yet-affordable)
        let mut affordable: Vec<ScoredCandidate> = Vec::new();
        let mut future: Vec<ScoredCandidate> = Vec::new();

        // Score producers
        self.score_producers(app, current_eps, &mut affordable, &mut future);

        // Score upgrades
        self.score_upgrades(app, current_eps, &mut affordable, &mut future);

        // Sort affordable by ROI (lowest = best)
        affordable.sort_by(|a, b| a.roi_seconds.partial_cmp(&b.roi_seconds).unwrap());

        // If nothing affordable, nothing to do
        if affordable.is_empty() {
            return None;
        }

        let best_affordable = &affordable[0];

        // Priority 3: Look-ahead — should we save for a better future purchase?
        if current_eps > 0.0 {
            for candidate in &future {
                let energy_needed = candidate.cost - app.game.energy;
                let time_to_save = energy_needed / current_eps;

                // If we can save for this candidate faster than the best affordable's payback,
                // and the future candidate has a better ROI, wait for it.
                if time_to_save < best_affordable.roi_seconds * SAVE_THRESHOLD_MULTIPLIER
                    && candidate.roi_seconds < best_affordable.roi_seconds
                {
                    // Worth waiting — return None to idle
                    return None;
                }
            }
        }

        // Priority 4: Buy the best affordable option
        Some(best_affordable.target.clone())
    }

    /// Find a newly unlocked producer that we don't own yet — buying one unit
    /// unlocks the next tier.
    fn find_unlock_producer(&self, app: &App) -> Option<AutoTarget> {
        let visible = app.game.visible_producers();

        for (display_idx, (_, producer)) in visible.iter().enumerate() {
            let owned = app.game.producer_count(producer.id);

            // Already own at least one — skip
            if owned > 0 {
                continue;
            }

            // Solar Panel (id 1) is always unlocked and should be auto-bought
            // For other producers: buying the first one unlocks the next producer
            let cost = calculate_bulk_cost(producer.base_cost, owned, 1, producer.id);

            if app.game.energy >= cost {
                return Some(AutoTarget::Producer { index: display_idx });
            }
        }

        None
    }

    /// Score all visible producers by simulated ROI.
    /// Simulation: clone the game state, add 1 producer, measure E/s delta.
    fn score_producers(
        &self,
        app: &App,
        _current_eps: f64,
        affordable: &mut Vec<ScoredCandidate>,
        future: &mut Vec<ScoredCandidate>,
    ) {
        let visible = app.game.visible_producers();

        for (display_idx, (_, producer)) in visible.iter().enumerate() {
            let owned = app.game.producer_count(producer.id);
            let cost = calculate_bulk_cost(producer.base_cost, owned, 1, producer.id);

            // Simulate buying this producer: clone state, add 1, measure E/s
            let eps_gain = self.simulate_producer_purchase(app, producer);

            if eps_gain <= 0.0 {
                continue;
            }

            let roi_seconds = cost / eps_gain;

            let candidate = ScoredCandidate {
                target: AutoTarget::Producer { index: display_idx },
                cost,
                eps_gain,
                roi_seconds,
            };

            if app.game.energy >= cost {
                affordable.push(candidate);
            } else {
                future.push(candidate);
            }
        }
    }

    /// Simulate buying 1 unit of a producer and return the E/s delta.
    /// This captures all cross-producer effects (Thousand Rays, synergies, etc.).
    fn simulate_producer_purchase(&self, app: &App, producer: &Producer) -> f64 {
        let current_eps = app.game.total_energy_per_second();

        // Clone the game state and simulate the purchase
        let mut simulated = app.game.clone();
        *simulated.producers_owned.entry(producer.id).or_insert(0) += 1;

        let new_eps = simulated.total_energy_per_second();
        new_eps - current_eps
    }

    /// Score all available upgrades by simulated ROI.
    /// Simulation: clone the game state, add the upgrade, measure E/s delta.
    fn score_upgrades(
        &self,
        app: &App,
        current_eps: f64,
        affordable: &mut Vec<ScoredCandidate>,
        future: &mut Vec<ScoredCandidate>,
    ) {
        let available = app.game.available_upgrades();

        for (idx, upgrade) in available.iter().enumerate() {
            let cost = app.game.get_upgrade_cost(upgrade);

            let eps_gain = self.simulate_upgrade_purchase(app, upgrade);

            // For upgrades with zero direct E/s impact (ManualMultiplier, ClickEpsPercent),
            // give them a small synthetic value so they're still bought eventually.
            let effective_eps_gain = if eps_gain <= 0.0 {
                // Assign a tiny synthetic value proportional to current EPS
                // so these upgrades get a very high ROI but aren't ignored forever
                current_eps * 0.001
            } else {
                eps_gain
            };

            if effective_eps_gain <= 0.0 {
                continue;
            }

            let roi_seconds = cost / effective_eps_gain;

            let candidate = ScoredCandidate {
                target: AutoTarget::Upgrade { index: idx },
                cost,
                eps_gain: effective_eps_gain,
                roi_seconds,
            };

            if app.game.energy >= cost {
                affordable.push(candidate);
            } else {
                future.push(candidate);
            }
        }
    }

    /// Simulate purchasing an upgrade and return the E/s delta.
    fn simulate_upgrade_purchase(&self, app: &App, upgrade: &Upgrade) -> f64 {
        let current_eps = app.game.total_energy_per_second();

        let mut simulated = app.game.clone();
        simulated.upgrades_purchased.push(upgrade.id);

        let new_eps = simulated.total_energy_per_second();
        new_eps - current_eps
    }

    fn current_selection(&self, app: &App, target: &AutoTarget) -> usize {
        match target {
            AutoTarget::Producer { .. } => app.selected_producer,
            AutoTarget::Upgrade { .. } => app.selected_upgrade,
        }
    }

    /// Jump the cursor directly to the target index (skipping step-by-step animation).
    fn jump_cursor_to(&self, app: &mut App, target: &AutoTarget) {
        let current = self.current_selection(app, target);
        let goal = target.index();
        if current < goal {
            for _ in 0..(goal - current) {
                app.move_selection_down();
            }
        } else if current > goal {
            for _ in 0..(current - goal) {
                app.move_selection_up();
            }
        }
    }
}
