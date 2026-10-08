pub mod format;
pub mod game;
pub mod save;

pub const TICK_RATE_MS: u64 = 100; // 10 ticks/second for game logic
pub const TICKS_PER_SECOND: f64 = 1000.0 / TICK_RATE_MS as f64;
