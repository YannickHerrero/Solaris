use super::GameState;
use crate::TICKS_PER_SECOND;

impl GameState {
    /// Process one game tick (called 10 times per second)
    pub fn tick(&mut self) {
        self.ticks_played += 1;

        // Calculate energy production for this tick
        let energy_per_second = self.total_energy_per_second();
        let energy_per_tick = energy_per_second / TICKS_PER_SECOND;

        self.add_energy(energy_per_tick);

        // Track actual production for rate display
        self.energy_produced_history.push_back(energy_per_tick);
        if self.energy_produced_history.len() > 10 {
            self.energy_produced_history.pop_front();
        }
    }
}
