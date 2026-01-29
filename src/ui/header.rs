use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::format::{format_energy, format_rate};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let energy = format_energy(app.game.energy);
    let rate = format_rate(app.game.total_energy_per_second());

    let text = format!("  Energy: {} ⚛    Rate: {}", energy, rate);

    let block = Block::default()
        .title(" SOLARIS ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}
