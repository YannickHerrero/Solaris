use super::{GameState, Producer};
use crate::TICKS_PER_SECOND;

impl GameState {
    /// Process one game tick (called 10 times per second)
    pub fn tick(&mut self) {
        self.ticks_played += 1;

        // Calculate and track per-producer energy production
        let global_mult = self.get_global_multiplier();
        let mut total_energy_per_tick = 0.0;

        for producer in Producer::all() {
            let count = self.producer_count(producer.id);
            if count == 0 {
                continue;
            }
            let producer_mult = self.get_producer_multiplier(producer.id);
            let producer_energy_per_tick =
                producer.base_energy_per_second * count as f64 * producer_mult * global_mult
                    / TICKS_PER_SECOND;

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
}
