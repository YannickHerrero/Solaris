use super::{GameState, Producer};
use crate::TICKS_PER_SECOND;

const MAX_OFFLINE_SECS: u64 = 8 * 60 * 60;

impl GameState {
    /// Process one game tick (called 10 times per second)
    pub fn tick(&mut self) {
        self.ticks_played += 1;
        self.all_time_ticks_played += 1;

        // Calculate and track per-producer energy production
        let mut total_energy_per_tick = 0.0;

        for producer in Producer::all() {
            if self.producer_count(producer.id) == 0 {
                continue;
            }
            let producer_energy_per_tick = self.producer_total_rate(producer.id) / TICKS_PER_SECOND;

            total_energy_per_tick += producer_energy_per_tick;

            // Track lifetime energy for this producer
            *self
                .producer_lifetime_energy
                .entry(producer.id)
                .or_insert(0.0) += producer_energy_per_tick;
        }

        self.add_energy(total_energy_per_tick);

        // Keep this for backward compatibility with total_energy_per_second calculation
        let energy_per_tick = total_energy_per_tick;

        // Track actual production for rate display
        self.energy_produced_history.push_back(energy_per_tick);
        if self.energy_produced_history.len() > 10 {
            self.energy_produced_history.pop_front();
        }

        // Check for new achievements every second (every 10 ticks)
        if self.ticks_played % 10 == 0 {
            self.check_achievements();
        }
    }

    /// Migrate old saves: seed all-time counters from current per-ascension values
    pub fn migrate_all_time_counters(&mut self) {
        if self.all_time_ticks_played == 0 && self.ticks_played > 0 {
            self.all_time_ticks_played = self.ticks_played;
        }
        if self.all_time_manual_clicks == 0 && self.total_manual_clicks > 0 {
            self.all_time_manual_clicks = self.total_manual_clicks;
        }
    }

    /// Credit production for time away, capped at 8 hours. Returns the
    /// seconds counted and the energy earned, or None for a minute or less.
    pub fn apply_offline_progress(&mut self, elapsed_secs: u64) -> Option<(u64, f64)> {
        let capped_secs = elapsed_secs.min(MAX_OFFLINE_SECS);
        if capped_secs <= 60 {
            return None;
        }

        let energy_per_tick = self.total_energy_per_second() / TICKS_PER_SECOND;
        let ticks = capped_secs * TICKS_PER_SECOND as u64;

        // Apply offline bonus from prestige upgrades
        let offline_bonus = self.get_offline_bonus_multiplier();
        let energy_earned = energy_per_tick * ticks as f64 * offline_bonus;

        self.add_energy(energy_earned);
        Some((capped_secs, energy_earned))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_produce_the_displayed_rate_including_thousand_rays() {
        let mut game = GameState::new();
        game.producers_owned.insert(1, 10);
        game.producers_owned.insert(2, 5);
        game.upgrades_purchased.push(104); // Thousand Rays
        assert!(game.get_thousand_rays_bonus() > 0.0);
        let rate = game.total_energy_per_second();

        for _ in 0..TICKS_PER_SECOND as u64 {
            game.tick();
        }

        assert!((game.energy - rate).abs() < 1e-9 * rate);
    }
}
