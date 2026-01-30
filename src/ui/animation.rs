/// Star animation type
#[derive(Clone, Copy, PartialEq)]
pub enum StarKind {
    Static,      // No animation - fixed character
    SlowTwinkle, // ~930ms cycle
    FastTwinkle, // ~230ms cycle
}

/// Simple pseudo-random number generator for deterministic animations
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min + 1))
    }
}

/// A single star in the background
#[derive(Clone)]
pub struct Star {
    pub x: u16,
    pub y: u16,
    pub phase: u8, // 0-5 for twinkle cycle
    pub kind: StarKind,
}

/// Star field background
pub struct StarField {
    pub stars: Vec<Star>,
    last_size: (u16, u16),
}

impl StarField {
    pub fn new() -> Self {
        Self {
            stars: Vec::new(),
            last_size: (0, 0),
        }
    }

    /// Regenerate stars when size changes
    pub fn ensure_size(&mut self, width: u16, height: u16, rng: &mut SimpleRng) {
        if self.last_size == (width, height) {
            return;
        }

        self.stars.clear();
        self.last_size = (width, height);

        // ~2.5% coverage
        let star_count = ((width as u32 * height as u32) * 25 / 1000) as usize;

        for _ in 0..star_count {
            let x = rng.next_range(0, width.saturating_sub(1) as u64) as u16;
            let y = rng.next_range(0, height.saturating_sub(1) as u64) as u16;
            let phase = rng.next_range(0, 5) as u8;
            // 40% static, 35% slow twinkle, 25% fast twinkle
            let kind_roll = rng.next_range(0, 99);
            let kind = if kind_roll < 40 {
                StarKind::Static
            } else if kind_roll < 75 {
                StarKind::SlowTwinkle
            } else {
                StarKind::FastTwinkle
            };
            self.stars.push(Star { x, y, phase, kind });
        }
    }

    /// Get the character for a star based on kind, frame and its phase
    pub fn star_char(kind: StarKind, phase: u8, frame: u64) -> char {
        const STAR_CHARS: &[char] = &['·', '✢', '✳', '∗', '✻', '✽'];
        const STATIC_CHARS: &[char] = &['·', '∗', '✧'];
        const SEQUENCE: &[(usize, u32)] = &[
            (0, 3), // '·' - hold 3 ticks
            (1, 1), // '✢'
            (2, 1), // '✳'
            (3, 1), // '∗'
            (4, 1), // '✻'
            (5, 3), // '✽' - hold 3 ticks
            (4, 1), // '✻'
            (3, 1), // '∗'
            (2, 1), // '✳'
            (1, 1), // '✢'
        ];
        const TOTAL_TICKS: u32 = 14;

        match kind {
            StarKind::Static => {
                // Static stars don't animate - use phase to pick a fixed character
                STATIC_CHARS[(phase as usize) % STATIC_CHARS.len()]
            }
            StarKind::SlowTwinkle | StarKind::FastTwinkle => {
                // SlowTwinkle: divide frame by 4 (~930ms cycle)
                // FastTwinkle: divide frame by 1 (~230ms cycle)
                let frame_divisor = match kind {
                    StarKind::SlowTwinkle => 4,
                    StarKind::FastTwinkle => 1,
                    _ => 1,
                };
                let adjusted_frame = frame / frame_divisor;
                let tick = ((adjusted_frame / 2) + phase as u64 * 2) % TOTAL_TICKS as u64;

                // Find which frame we're on based on tick and weights
                let mut accumulated = 0u32;
                for &(char_idx, weight) in SEQUENCE {
                    accumulated += weight;
                    if (tick as u32) < accumulated {
                        return STAR_CHARS[char_idx];
                    }
                }
                STAR_CHARS[0]
            }
        }
    }
}

impl Default for StarField {
    fn default() -> Self {
        Self::new()
    }
}

/// Main animation state container (simplified)
pub struct AnimationState {
    pub frame_count: u64,
    pub stars: StarField,
    pub rng: SimpleRng,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            stars: StarField::new(),
            rng: SimpleRng::new(42),
        }
    }

    /// Update animations (called each frame at ~60 FPS)
    pub fn tick(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Orbital mechanics constants and helpers
pub mod orbit {
    pub const BASE_RADIUS: f64 = 3.0;
    pub const ORBIT_SPACING: f64 = 2.5;
    pub const BASE_ANGULAR_VELOCITY: f64 = 0.02;

    /// Get the orbital radius for a given tier (0-indexed)
    pub fn radius_for_tier(tier: usize) -> f64 {
        BASE_RADIUS + (tier as f64 * ORBIT_SPACING)
    }

    /// Get the angular velocity for a given tier (inner = faster, Keplerian-style)
    pub fn angular_velocity_for_tier(tier: usize) -> f64 {
        // Outer orbits move slower (inversely proportional to sqrt of radius)
        let radius = radius_for_tier(tier);
        BASE_ANGULAR_VELOCITY / (radius / BASE_RADIUS).sqrt()
    }
}

/// A producer orbiting the central sun
pub struct OrbitalBody {
    #[allow(dead_code)]
    pub producer_id: u32,
    pub icon: &'static str,
    pub tier: usize,
    pub initial_angle: f64,
}

impl OrbitalBody {
    /// Calculate the current position of this orbital body
    pub fn position(&self, frame_count: u64, center_x: u16, center_y: u16) -> (u16, u16) {
        let radius = orbit::radius_for_tier(self.tier);
        let angular_velocity = orbit::angular_velocity_for_tier(self.tier);

        // Calculate current angle
        let angle = self.initial_angle + (frame_count as f64 * angular_velocity);

        // x is stretched 2x to compensate for terminal character aspect ratio
        let x = center_x as f64 + (angle.cos() * radius * 2.0);
        let y = center_y as f64 + (angle.sin() * radius);

        (x.round() as u16, y.round() as u16)
    }
}
