use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::format::format_cost;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let available = app.game.available_upgrades();

    let border_color = if focused { Color::Magenta } else { Color::DarkGray };
    let title = if focused { " Upgrades *" } else { " Upgrades " };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    if available.is_empty() {
        let text = "\n  No upgrades available yet.\n\n  Build more producers to unlock upgrades!";
        let paragraph = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(paragraph, area);
        return;
    }

    // Calculate dynamic column widths
    let name_width = available
        .iter()
        .map(|u| u.name.len())
        .max()
        .unwrap_or(0)
        .max("Upgrade".len())
        + 1;

    let cost_width = available
        .iter()
        .map(|u| format_cost(app.game.get_upgrade_cost(u)).len())
        .max()
        .unwrap_or(0)
        .max("Cost".len())
        + 1;

    let items: Vec<ListItem> = available
        .iter()
        .enumerate()
        .map(|(i, upgrade)| {
            let cost = app.game.get_upgrade_cost(upgrade);
            let can_afford = app.game.energy >= cost;

            // Truncate description if needed
            let desc = if upgrade.description.len() > 35 {
                format!("{}...", &upgrade.description[..32])
            } else {
                upgrade.description.to_string()
            };

            let line = format!(
                "{:<name_width$} {:>cost_width$}  {}",
                upgrade.name,
                format_cost(cost),
                desc,
                name_width = name_width,
                cost_width = cost_width
            );

            let style = if i == app.selected_upgrade {
                if can_afford {
                    Style::default().fg(Color::Black).bg(Color::Green)
                } else {
                    Style::default().fg(Color::Black).bg(Color::Red)
                }
            } else if can_afford {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Render header
    let header = format!(
        "{:<name_width$} {:>cost_width$}  {}",
        "Upgrade", "Cost", "Effect",
        name_width = name_width,
        cost_width = cost_width
    );
    let header_widget = Paragraph::new(header)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(header_widget, chunks[0]);

    // Render list
    let list = List::new(items);
    let mut state = ListState::default();
    state.select(Some(app.selected_upgrade.min(available.len().saturating_sub(1))));
    frame.render_stateful_widget(list, chunks[1], &mut state);

    // Render tooltip if hover timer has reached threshold
    if app.show_upgrade_tooltip && app.selected_upgrade < available.len() {
        render_tooltip(frame, area, &available[app.selected_upgrade]);
    }
}

fn render_tooltip(frame: &mut Frame, parent_area: Rect, upgrade: &crate::game::Upgrade) {
    // Calculate tooltip dimensions
    let tooltip_height = 4;
    let tooltip_width = parent_area.width.min(60);

    // Position at bottom of the upgrades panel area
    let x = parent_area.x + (parent_area.width.saturating_sub(tooltip_width)) / 2;
    let y = parent_area.y + parent_area.height.saturating_sub(tooltip_height).saturating_sub(1);

    let tooltip_area = Rect::new(x, y, tooltip_width, tooltip_height);

    // Clear background
    frame.render_widget(Clear, tooltip_area);

    let block = Block::default()
        .title(format!(" {} ", upgrade.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let text = upgrade.description.to_string();
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, tooltip_area);
}
