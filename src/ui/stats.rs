use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::format::{format_duration, format_energy, format_rate};
use crate::game::Producer;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let total_earned = format_energy(app.game.total_energy_earned);
    let current = format_energy(app.game.energy);
    let rate = format_rate(app.game.actual_energy_per_second());
    let time_played = format_duration(app.game.time_played_seconds());

    // Find top producer
    let top_producer = Producer::all()
        .iter()
        .map(|p| {
            let count = app.game.producer_count(p.id);
            let mult = app.game.get_producer_multiplier(p.id) * app.game.get_global_multiplier();
            let production = p.base_energy_per_second * count as f64 * mult;
            (p.icon, p.name, production)
        })
        .filter(|(_, _, production)| *production > 0.0)
        .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let top_producer_str = match top_producer {
        Some((icon, name, prod)) => format!("{} {} ({}/s)", icon, name, format_energy(prod)),
        None => "None".to_string(),
    };

    // Count total producers
    let total_producers: u64 = Producer::all()
        .iter()
        .map(|p| app.game.producer_count(p.id))
        .sum();

    // Count upgrades
    let upgrades_purchased = app.game.upgrades_purchased.len();

    let manual_clicks = app.game.total_manual_clicks;
    let manual_power = format_energy(app.game.effective_manual_power());

    let text = format!(
        r#"
  Statistics
  ----------

  Current Energy:     {} ⚛
  Total Earned:       {} ⚛
  Production Rate:    {}

  Time Played:        {}

  Total Producers:    {}
  Upgrades Purchased: {}

  Top Producer:       {}

  Manual Mining
  -------------
  Total Clicks:       {}
  Click Power:        {} ⚛
"#,
        current,
        total_earned,
        rate,
        time_played,
        total_producers,
        upgrades_purchased,
        top_producer_str,
        manual_clicks,
        manual_power
    );

    let border_color = if focused { Color::Blue } else { Color::DarkGray };
    let title = if focused { " Statistics *" } else { " Statistics " };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}
