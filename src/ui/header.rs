use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::format::{format_energy, format_rate};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let energy = format_energy(app.game.energy);
    let rate = format_rate(app.game.total_energy_per_second());

    let speed_label = if app.auto_speed != 1.0 {
        if app.auto_speed == app.auto_speed.floor() {
            format!(" {}x", app.auto_speed as u32)
        } else {
            format!(" {:.1}x", app.auto_speed)
        }
    } else {
        String::new()
    };

    let auto_indicator = if app.auto_mode {
        if app.auto_paused {
            format!("  [AUTO{} PAUSED]", speed_label)
        } else {
            format!("  [AUTO{}]", speed_label)
        }
    } else {
        String::new()
    };
    let text = format!("  Energy: {} ⚛    Rate: {}{}", energy, rate, auto_indicator);

    let title = if app.auto_mode {
        if app.auto_paused {
            format!(" SOLARIS [AUTO{} PAUSED] ", speed_label)
        } else {
            format!(" SOLARIS [AUTO{}] ", speed_label)
        }
    } else {
        String::from(" SOLARIS ")
    };

    let border_color = if app.auto_mode {
        if app.auto_paused {
            Color::Yellow
        } else {
            Color::Magenta
        }
    } else {
        Color::Cyan
    };

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}
