use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::game::{PrestigeRequirement, PrestigeUpgrade};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Create a centered popup
    let popup_area = centered_rect(70, 80, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Stellar Ascension [A to close, Enter to buy/ascend] ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightCyan))
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Split into header info and upgrade list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Info section
            Constraint::Min(1),    // Upgrade list
        ])
        .split(inner);

    // Render prestige info
    render_prestige_info(frame, chunks[0], app);

    // Render prestige upgrades
    render_prestige_upgrades(frame, chunks[1], app);
}

fn render_prestige_info(frame: &mut Frame, area: Rect, app: &App) {
    let stellar_chips = app.game.stellar_chips;
    let potential_chips = app.game.calculate_potential_stellar_chips();
    let ascensions = app.game.total_ascensions;
    let can_ascend = app.game.can_ascend();

    let ascend_status = if can_ascend {
        format!(
            "Press ENTER to ascend and gain {} Stellar Chips!",
            potential_chips
        )
    } else {
        "Need ~1 trillion total energy to earn first chip".to_string()
    };

    let info_text = format!(
        r#"
  Stellar Chips: {}     |     Total Ascensions: {}

  Potential Chips on Ascension: {}

  {}

  Ascension resets your progress but grants permanent bonuses!
"#,
        stellar_chips, ascensions, potential_chips, ascend_status
    );

    let style = if can_ascend {
        Style::default().fg(Color::LightGreen)
    } else {
        Style::default().fg(Color::White)
    };

    let paragraph = Paragraph::new(info_text).style(style);
    frame.render_widget(paragraph, area);
}

fn render_prestige_upgrades(frame: &mut Frame, area: Rect, app: &App) {
    let purchased = &app.game.prestige_upgrades;

    // Show all prestige upgrades with their status
    let all_upgrades = PrestigeUpgrade::all();

    let items: Vec<ListItem> = all_upgrades
        .iter()
        .enumerate()
        .map(|(i, upgrade)| {
            let is_purchased = purchased.contains(&upgrade.id);
            let is_available = !is_purchased && app.game.is_prestige_upgrade_available(upgrade);
            let can_afford = app.game.stellar_chips >= upgrade.cost;

            let (status, description) = if is_purchased {
                ("[OWNED]".to_string(), upgrade.description.to_string())
            } else if is_available && can_afford {
                ("[BUY]".to_string(), upgrade.description.to_string())
            } else if is_available {
                ("[NEED $]".to_string(), upgrade.description.to_string())
            } else {
                // Show what requirement is missing
                let req_str = get_requirement_string(upgrade, app);
                (format!("[REQ]"), req_str)
            };

            let cost_str = if is_purchased {
                String::new()
            } else {
                format!("({} chips)", upgrade.cost)
            };

            let line = format!(
                "{:<8} {:<22} {:>12}  {}",
                status, upgrade.name, cost_str, description
            );

            let style = if i == app.selected_prestige_upgrade {
                if is_purchased {
                    Style::default().fg(Color::Black).bg(Color::DarkGray)
                } else if is_available && can_afford {
                    Style::default().fg(Color::Black).bg(Color::Green)
                } else if is_available {
                    Style::default().fg(Color::Black).bg(Color::Red)
                } else {
                    Style::default().fg(Color::Black).bg(Color::DarkGray)
                }
            } else if is_purchased {
                Style::default().fg(Color::DarkGray)
            } else if is_available && can_afford {
                Style::default().fg(Color::Green)
            } else if is_available {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let header = format!(
        "{:<8} {:<22} {:>12}  {}",
        "Status", "Upgrade", "Cost", "Effect"
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    let header_widget = Paragraph::new(header).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(header_widget, chunks[0]);

    let list = List::new(items);
    let mut state = ListState::default();
    state.select(Some(
        app.selected_prestige_upgrade
            .min(all_upgrades.len().saturating_sub(1)),
    ));
    frame.render_stateful_widget(list, chunks[1], &mut state);
}

/// Get a string describing what requirement is missing for a prestige upgrade
fn get_requirement_string(upgrade: &PrestigeUpgrade, app: &App) -> String {
    match upgrade.requirement {
        None => upgrade.description.to_string(),
        Some(PrestigeRequirement::Ascensions(count)) => {
            format!(
                "Requires {} ascensions (have {})",
                count, app.game.total_ascensions
            )
        }
        Some(PrestigeRequirement::TotalChips(count)) => {
            format!(
                "Requires {} total chips earned (have {})",
                count, app.game.total_stellar_chips_earned
            )
        }
        Some(PrestigeRequirement::PrestigeUpgrade(req_id)) => {
            if let Some(req_upgrade) = PrestigeUpgrade::by_id(req_id) {
                format!("Requires: {}", req_upgrade.name)
            } else {
                format!("Requires upgrade #{}", req_id)
            }
        }
    }
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
