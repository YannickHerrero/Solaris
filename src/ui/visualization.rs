use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};

use super::animation::{orbit, OrbitalBody, StarField};
use crate::app::App;
use crate::game::Producer;

/// A cell in the render buffer
#[derive(Clone, Copy)]
struct Cell {
    ch: char,
    fg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::White,
        }
    }
}

/// Get color for a producer based on its tier
fn producer_color(tier: usize) -> Color {
    match tier {
        0 => Color::Rgb(200, 200, 100), // Solar Panel - pale yellow
        1 => Color::Rgb(150, 150, 150), // Mining Drone - gray
        2 => Color::Rgb(255, 140, 0),   // Asteroid Mine - orange
        3 => Color::Rgb(100, 200, 255), // Orbital Station - light blue
        4 => Color::Rgb(200, 200, 220), // Lunar Colony - silver
        5 => Color::Rgb(100, 255, 100), // Planetary Harvester - green
        6 => Color::Rgb(255, 100, 255), // Fusion Reactor - magenta
        7 => Color::Rgb(255, 215, 0),   // Dyson Swarm - gold
        8 => Color::Rgb(255, 255, 100), // Dyson Sphere - bright yellow
        9 => Color::Rgb(255, 255, 255), // Star Forge - white
        _ => Color::White,
    }
}

/// Buffer for layered rendering
struct RenderBuffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

impl RenderBuffer {
    fn new(width: u16, height: u16) -> Self {
        Self {
            cells: vec![Cell::default(); (width as usize) * (height as usize)],
            width,
            height,
        }
    }

    fn set(&mut self, x: u16, y: u16, ch: char, fg: Color) {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx] = Cell { ch, fg };
        }
    }

    fn get(&self, x: u16, y: u16) -> Cell {
        if x < self.width && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.cells[idx]
        } else {
            Cell::default()
        }
    }
}

pub fn render(frame: &mut Frame, area: Rect, app: &mut App, focused: bool) {
    let border_color = if focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let title = if focused { " System *" } else { " System " };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 10 || inner.height < 5 {
        return;
    }

    // Reserve space for stellar essence gauge on the right (5 chars: bar + space + "999%")
    let gauge_width: u16 = 5;
    let viz_width = if inner.width > gauge_width + 10 {
        inner.width - gauge_width
    } else {
        inner.width // Too narrow, skip gauge
    };

    // Create render buffer for the visualization area (excluding gauge)
    let mut buffer = RenderBuffer::new(viz_width, inner.height);

    // Calculate center position (centered in viz area, not full inner)
    let center_x = viz_width / 2;
    let center_y = inner.height / 2;

    // Update star field size if needed
    app.animation
        .stars
        .ensure_size(inner.width, inner.height, &mut app.animation.rng);

    // Collect owned producer tiers and create orbital bodies
    let producers = Producer::all();
    let mut owned_tiers: Vec<usize> = Vec::new();
    let mut orbital_bodies: Vec<OrbitalBody> = Vec::new();

    for (tier, producer) in producers.iter().enumerate() {
        if app.game.producer_count(producer.id) > 0 {
            owned_tiers.push(tier);
            // Create orbital body with initial angle based on producer id for variety
            let initial_angle = (producer.id as f64) * 0.7; // Spread out initial positions
            orbital_bodies.push(OrbitalBody {
                producer_id: producer.id,
                icon: producer.icon,
                tier,
                initial_angle,
            });
        }
    }

    // Layer 1: Stars background
    render_stars(&mut buffer, &app.animation.stars, app.animation.frame_count);

    // Layer 2: Orbit paths (subtle dotted circles)
    render_orbit_paths(&mut buffer, center_x, center_y, &owned_tiers);

    // Layer 3: Central sun
    render_sun(&mut buffer, center_x, center_y);

    // Layer 4: Orbiting producer icons
    render_producers(
        &mut buffer,
        center_x,
        center_y,
        app.animation.frame_count,
        &orbital_bodies,
    );

    // Render buffer to frame (only the viz area, not the gauge space)
    let viz_area = Rect {
        x: inner.x,
        y: inner.y,
        width: viz_width,
        height: inner.height,
    };
    render_buffer_to_frame(frame, viz_area, &buffer);

    // Render stellar essence gauge on the right side (if space permits)
    if inner.width > gauge_width + 10 {
        let gauge_area = Rect {
            x: inner.x + viz_width,
            y: inner.y,
            width: gauge_width,
            height: inner.height,
        };
        render_stellar_gauge(frame, gauge_area, app);
    }
}

fn render_stars(buffer: &mut RenderBuffer, stars: &StarField, frame_count: u64) {
    for star in &stars.stars {
        if star.x < buffer.width && star.y < buffer.height {
            let ch = StarField::star_char(star.kind, star.phase, frame_count);
            if ch != ' ' {
                // Brightness based on character (brighter = more intense stars)
                let brightness = match ch {
                    '✽' | '✻' => Color::White,
                    '✳' | '✢' => Color::Gray,
                    '∗' | '✧' => Color::Rgb(180, 180, 180),
                    '·' => Color::DarkGray,
                    _ => Color::DarkGray,
                };
                buffer.set(star.x, star.y, ch, brightness);
            }
        }
    }
}

fn render_sun(buffer: &mut RenderBuffer, center_x: u16, center_y: u16) {
    // Simple sun: single ☀ character
    buffer.set(center_x, center_y, '☀', Color::Yellow);
}

/// Render subtle dotted orbit paths for owned producer tiers
fn render_orbit_paths(
    buffer: &mut RenderBuffer,
    center_x: u16,
    center_y: u16,
    owned_tiers: &[usize],
) {
    for &tier in owned_tiers {
        let radius = orbit::radius_for_tier(tier);
        let color = Color::Rgb(40, 40, 50); // Very subtle dark color

        // Draw dotted ellipse (x stretched 2x for aspect ratio)
        let circumference = (2.0 * std::f64::consts::PI * radius * 1.5) as usize;
        let step_count = circumference.max(16);

        for i in 0..step_count {
            // Only draw every 4th point for dotted effect
            if i % 4 != 0 {
                continue;
            }

            let angle = (i as f64 / step_count as f64) * 2.0 * std::f64::consts::PI;
            let x = center_x as f64 + (angle.cos() * radius * 2.0);
            let y = center_y as f64 + (angle.sin() * radius);

            let px = x.round() as u16;
            let py = y.round() as u16;

            if px < buffer.width && py < buffer.height {
                // Only draw if cell is empty (space)
                if buffer.get(px, py).ch == ' ' {
                    buffer.set(px, py, '·', color);
                }
            }
        }
    }
}

/// Render orbiting producer icons
fn render_producers(
    buffer: &mut RenderBuffer,
    center_x: u16,
    center_y: u16,
    frame_count: u64,
    orbital_bodies: &[OrbitalBody],
) {
    for body in orbital_bodies {
        let (x, y) = body.position(frame_count, center_x, center_y);

        if x < buffer.width && y < buffer.height {
            let color = producer_color(body.tier);
            // Use the first character of the icon
            let ch = body.icon.chars().next().unwrap_or('?');
            buffer.set(x, y, ch, color);
        }
    }
}

fn render_buffer_to_frame(frame: &mut Frame, area: Rect, buffer: &RenderBuffer) {
    // Custom widget to render our buffer
    struct BufferWidget<'a> {
        buffer: &'a RenderBuffer,
    }

    impl Widget for BufferWidget<'_> {
        fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
            for y in 0..area.height.min(self.buffer.height) {
                for x in 0..area.width.min(self.buffer.width) {
                    let cell = self.buffer.get(x, y);
                    let buf_x = area.x + x;
                    let buf_y = area.y + y;
                    if buf_x < area.x + area.width && buf_y < area.y + area.height {
                        buf.set_string(
                            buf_x,
                            buf_y,
                            cell.ch.to_string(),
                            Style::default().fg(cell.fg),
                        );
                    }
                }
            }
        }
    }

    frame.render_widget(BufferWidget { buffer }, area);
}

/// Render vertical Stellar Essence gauge
/// Shows essence level with layered shading for overflow beyond 100%
fn render_stellar_gauge(frame: &mut Frame, area: Rect, app: &App) {
    if area.height < 3 {
        return;
    }

    // Get stellar essence (0.04 per achievement, so 25 achievements = 100%)
    let essence = app.game.get_stellar_essence();
    let essence_percent = essence * 100.0;

    // Reserve bottom 2 rows for labels
    let bar_height = area.height.saturating_sub(2);
    if bar_height == 0 {
        return;
    }

    // Each "layer" represents 100% of essence
    // Layer colors get progressively darker as you overflow
    let layer_chars = ['█', '▓', '▒', '░'];
    let layer_colors = [
        Color::Rgb(150, 220, 255), // Layer 1 (0-100%): Bright cyan
        Color::Rgb(100, 180, 220), // Layer 2 (100-200%): Medium cyan
        Color::Rgb(60, 140, 180),  // Layer 3 (200-300%): Darker cyan
        Color::Rgb(40, 100, 140),  // Layer 4 (300%+): Dark cyan
    ];

    // Determine which layer (100% bracket) we're in
    // Layer 0: 0-100%, Layer 1: 100-200%, etc.
    let current_layer = ((essence_percent / 100.0).floor() as usize).min(layer_chars.len() - 1);

    // Calculate fill within current 100% bracket
    let layer_fill = if essence_percent > 0.0 {
        let fill = (essence_percent % 100.0) / 100.0;
        // If exactly at boundary (e.g., 200%), show full bar
        if fill == 0.0 {
            1.0
        } else {
            fill
        }
    } else {
        0.0
    };

    // Calculate filled rows (from bottom up)
    let filled_rows = ((bar_height as f64) * layer_fill).ceil() as u16;

    // Get character and color for current layer
    let ch = layer_chars[current_layer];
    let color = layer_colors[current_layer];

    // Render the bar from bottom to top
    for row in 0..bar_height {
        let y = area.y + bar_height - 1 - row; // Start from bottom
        let x = area.x;

        if row < filled_rows {
            frame
                .buffer_mut()
                .set_string(x, y, ch.to_string(), Style::default().fg(color));
        }
        // Unfilled rows stay blank
    }

    // Render moon icon at bottom of bar area
    let icon_y = area.y + bar_height;
    frame.buffer_mut().set_string(
        area.x,
        icon_y,
        "☽",
        Style::default().fg(Color::Rgb(200, 200, 255)),
    );

    // Render percentage below the icon
    let label_y = area.y + bar_height + 1;
    if label_y < area.y + area.height {
        let label = format!("{:.0}%", essence_percent);
        let label_color = if essence_percent >= 100.0 {
            Color::Rgb(150, 220, 255) // Bright when over 100%
        } else {
            Color::Gray
        };
        frame
            .buffer_mut()
            .set_string(area.x, label_y, &label, Style::default().fg(label_color));
    }
}
