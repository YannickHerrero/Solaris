use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::game::Producer;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let producers = Producer::all();

    // Build visualization based on owned producers
    let mut lines = Vec::new();

    lines.push(String::new());
    lines.push("                          Solar System Empire".to_string());
    lines.push(String::new());

    // Sun in the center
    lines.push("                                  *".to_string());
    lines.push("                                 ***".to_string());
    lines.push("                                *****    STAR".to_string());
    lines.push("                                 ***".to_string());
    lines.push("                                  *".to_string());
    lines.push(String::new());

    // Show orbital rings with producer counts
    for producer in producers.iter() {
        let count = app.game.producer_count(producer.id);
        if count > 0 {
            let indicator = match producer.id {
                1 => format!("  [{}] Solar Panels orbiting", count),
                2 => format!("    [{}] Mining Drones active", count),
                3 => format!("      [{}] Asteroid Mines operating", count),
                4 => format!("        [{}] Orbital Stations online", count),
                5 => format!("          [{}] Lunar Colonies established", count),
                6 => format!("            [{}] Planetary Harvesters deployed", count),
                7 => format!("              [{}] Fusion Reactors running", count),
                8 => format!("                [{}] Dyson Swarms collecting", count),
                9 => format!("                  [{}] Dyson Spheres encapsulating", count),
                10 => format!("                    [{}] Star Forges creating", count),
                _ => String::new(),
            };
            lines.push(indicator);
        }
    }

    if lines.len() <= 9 {
        lines.push(String::new());
        lines.push("  No producers yet. Purchase Solar Panels to begin!".to_string());
    }

    let text = lines.join("\n");

    let border_color = if focused { Color::Cyan } else { Color::DarkGray };
    let title = if focused { " Visualization *" } else { " Visualization " };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(paragraph, area);
}
