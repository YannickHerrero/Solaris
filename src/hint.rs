use crate::app::App;
use crate::auto::{decide_best_action, AutoTarget, DecisionResult};
use crate::format::{format_energy, format_rate};

/// The result of evaluating the best hint for the player.
pub struct HintResult {
    pub lines: Vec<String>,
}

/// Generate a purchase hint using the same decision logic as the bot.
pub fn generate_hint(app: &App) -> HintResult {
    let decision = decide_best_action(app);
    let current_eps = app.game.total_energy_per_second();

    match decision {
        DecisionResult::UnlockProducer(target) => {
            let (name, cost) = resolve_target_info(app, &target);
            HintResult {
                lines: vec![
                    "Unlock new producer!".to_string(),
                    format!("  Buy: {}", name),
                    format!("  Cost: {} E", format_energy(cost)),
                    String::new(),
                    "Unlocking new tiers is the".to_string(),
                    "highest priority for progress.".to_string(),
                ],
            }
        }
        DecisionResult::Buy(target) => {
            let (name, cost) = resolve_target_info(app, &target);
            // Recompute the EPS gain for display
            let eps_gain = compute_eps_gain(app, &target);
            let roi = if eps_gain > 0.0 {
                cost / eps_gain
            } else {
                f64::INFINITY
            };

            let mut lines = vec![
                "Best purchase right now:".to_string(),
                format!("  {}", name),
                format!("  Cost: {} E", format_energy(cost)),
            ];

            if eps_gain > 0.0 {
                lines.push(format!("  EPS gain: +{}", format_rate(eps_gain)));
                if roi.is_finite() {
                    lines.push(format!("  Pays for itself in: {}", format_seconds(roi)));
                }
            }

            HintResult { lines }
        }
        DecisionResult::Wait {
            save_for,
            best_affordable,
        } => {
            let (save_name, _) = resolve_target_info(app, &save_for.target);
            let (affordable_name, _) = resolve_target_info(app, &best_affordable.target);
            let energy_needed = save_for.cost - app.game.energy;
            let time_to_save = if current_eps > 0.0 {
                energy_needed / current_eps
            } else {
                f64::INFINITY
            };

            let mut lines = vec![
                "Wait! Save for:".to_string(),
                format!("  {}", save_name),
                format!("  Cost: {} E", format_energy(save_for.cost)),
            ];

            if time_to_save.is_finite() {
                lines.push(format!(
                    "  Need {} E more (~{})",
                    format_energy(energy_needed),
                    format_seconds(time_to_save)
                ));
            }

            lines.push(String::new());
            lines.push(format!("  Better ROI than: {}", affordable_name));

            HintResult { lines }
        }
        DecisionResult::Nothing => HintResult {
            lines: vec![
                "No recommendation right now.".to_string(),
                String::new(),
                "Keep mining manually with Space!".to_string(),
            ],
        },
    }
}

/// Resolve the name and cost of a target (producer or upgrade).
fn resolve_target_info(app: &App, target: &AutoTarget) -> (String, f64) {
    match target {
        AutoTarget::Producer { index } => {
            let visible = app.game.visible_producers();
            if let Some((_, producer)) = visible.get(*index) {
                let owned = app.game.producer_count(producer.id);
                let cost =
                    crate::game::calculate_bulk_cost(producer.base_cost, owned, 1, producer.id);
                (format!("{} (producer)", producer.name), cost)
            } else {
                ("Unknown producer".to_string(), 0.0)
            }
        }
        AutoTarget::Upgrade { index } => {
            let available = app.game.available_upgrades();
            if let Some(upgrade) = available.get(*index) {
                let cost = app.game.get_upgrade_cost(upgrade);
                (format!("{} (upgrade)", upgrade.name), cost)
            } else {
                ("Unknown upgrade".to_string(), 0.0)
            }
        }
    }
}

/// Compute the EPS gain for a target by simulation.
fn compute_eps_gain(app: &App, target: &AutoTarget) -> f64 {
    match target {
        AutoTarget::Producer { index } => {
            let visible = app.game.visible_producers();
            if let Some((_, producer)) = visible.get(*index) {
                crate::auto::simulate_producer_purchase(app, producer)
            } else {
                0.0
            }
        }
        AutoTarget::Upgrade { index } => {
            let available = app.game.available_upgrades();
            if let Some(upgrade) = available.get(*index) {
                crate::auto::simulate_upgrade_purchase(app, upgrade)
            } else {
                0.0
            }
        }
    }
}

/// Format seconds into a human-readable string.
fn format_seconds(secs: f64) -> String {
    if secs < 1.0 {
        "< 1s".to_string()
    } else if secs < 60.0 {
        format!("{:.0}s", secs)
    } else if secs < 3600.0 {
        let mins = (secs / 60.0).floor() as u64;
        let s = (secs % 60.0).floor() as u64;
        if s > 0 {
            format!("{}m {}s", mins, s)
        } else {
            format!("{}m", mins)
        }
    } else {
        let hours = (secs / 3600.0).floor() as u64;
        let mins = ((secs % 3600.0) / 60.0).floor() as u64;
        if mins > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}h", hours)
        }
    }
}
