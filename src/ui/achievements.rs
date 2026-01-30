use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs};

use crate::app::App;
use crate::game::Achievement;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    // Create a centered popup
    let popup_area = centered_rect(80, 85, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Achievements [Tab to switch, A to close] ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightYellow))
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Split into tabs and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tab bar
            Constraint::Length(2),  // Stats
            Constraint::Min(1),     // Achievement list
        ])
        .split(inner);

    // Render tab bar
    render_tabs(frame, chunks[0], app);

    // Render stats
    render_stats(frame, chunks[1], app);

    // Render achievement list based on selected tab
    render_achievement_list(frame, chunks[2], app);
}

fn render_tabs(frame: &mut Frame, area: Rect, app: &App) {
    let tab_titles = vec![
        "All",
        "Producers",
        "Production",
        "Lifetime",
        "Clicks",
        "Upgrades",
        "Time",
        "Prestige",
    ];

    let tabs = Tabs::new(tab_titles)
        .select(app.achievement_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .divider(" | ");

    frame.render_widget(tabs, area);
}

fn render_stats(frame: &mut Frame, area: Rect, app: &App) {
    let total = Achievement::all().len();
    let unlocked = app.game.achievements_unlocked.len();
    let bonus = (app.game.get_achievement_multiplier() - 1.0) * 100.0;

    let stats_text = format!(
        "  Unlocked: {}/{} ({:.1}%)  |  Production Bonus: +{:.1}%",
        unlocked,
        total,
        (unlocked as f64 / total as f64) * 100.0,
        bonus
    );

    let paragraph = Paragraph::new(stats_text)
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(paragraph, area);
}

fn render_achievement_list(frame: &mut Frame, area: Rect, app: &App) {
    let all_achievements = Achievement::all();
    let unlocked_ids = &app.game.achievements_unlocked;

    // Filter achievements based on selected tab
    let filtered: Vec<&Achievement> = all_achievements
        .iter()
        .filter(|a| matches_tab(a, app.achievement_tab))
        .collect();

    if filtered.is_empty() {
        let paragraph = Paragraph::new("\n  No achievements in this category.")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(i, achievement)| {
            let is_unlocked = unlocked_ids.contains(&achievement.id);

            let status = if is_unlocked { "[X]" } else { "[ ]" };
            let name = if is_unlocked {
                achievement.name.to_string()
            } else {
                "???".to_string()
            };
            let desc = if is_unlocked {
                achievement.description.to_string()
            } else {
                get_hint(achievement)
            };

            let line = format!(
                " {} {:<30} {}",
                status,
                name,
                desc
            );

            let style = if is_unlocked {
                if i == app.selected_achievement {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                }
            } else {
                if i == app.selected_achievement {
                    Style::default().fg(Color::Black).bg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::DarkGray)
                }
            };

            ListItem::new(line).style(style)
        })
        .collect();

    // Header
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    let header = format!(
        " {:<3} {:<30} {}",
        "", "Achievement", "Requirement"
    );
    let header_widget = Paragraph::new(header)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(header_widget, chunks[0]);

    let list = List::new(items);
    let mut state = ListState::default();
    state.select(Some(app.selected_achievement.min(filtered.len().saturating_sub(1))));
    frame.render_stateful_widget(list, chunks[1], &mut state);
}

fn matches_tab(achievement: &Achievement, tab: usize) -> bool {
    use crate::game::AchievementRequirement::*;

    match tab {
        0 => true, // All
        1 => matches!(achievement.requirement, ProducerCount { .. }), // Producers
        2 => matches!(achievement.requirement, TotalEnergyPerSecond(_)), // Production
        3 => matches!(achievement.requirement, TotalEnergyEarned(_)), // Lifetime
        4 => matches!(achievement.requirement, TotalClicks(_)), // Clicks
        5 => matches!(achievement.requirement, UpgradesPurchased(_)), // Upgrades
        6 => matches!(achievement.requirement, TimePlayed(_)), // Time
        7 => matches!(achievement.requirement, Ascensions(_) | TotalProducers(_)), // Prestige
        _ => true,
    }
}

fn get_hint(achievement: &Achievement) -> String {
    use crate::game::AchievementRequirement::*;

    match achievement.requirement {
        ProducerCount { count, .. } => format!("Own {} of a producer", count),
        TotalEnergyPerSecond(rate) => format!("Reach {} E/s", format_number(rate)),
        TotalEnergyEarned(amount) => format!("Earn {} total energy", format_number(amount)),
        TotalClicks(clicks) => format!("Click {} times", clicks),
        UpgradesPurchased(count) => format!("Purchase {} upgrades", count),
        TimePlayed(secs) => format!("Play for {}", format_duration(secs)),
        TotalProducers(count) => format!("Own {} total producers", count),
        Ascensions(count) => format!("Ascend {} times", count),
    }
}

fn format_number(n: f64) -> String {
    if n >= 1e15 {
        format!("{:.1}Q", n / 1e15)
    } else if n >= 1e12 {
        format!("{:.1}T", n / 1e12)
    } else if n >= 1e9 {
        format!("{:.1}B", n / 1e9)
    } else if n >= 1e6 {
        format!("{:.1}M", n / 1e6)
    } else if n >= 1e3 {
        format!("{:.1}K", n / 1e3)
    } else {
        format!("{:.0}", n)
    }
}

fn format_duration(secs: u64) -> String {
    if secs >= 86400 * 30 {
        format!("{} month(s)", secs / (86400 * 30))
    } else if secs >= 86400 * 7 {
        format!("{} week(s)", secs / (86400 * 7))
    } else if secs >= 86400 {
        format!("{} day(s)", secs / 86400)
    } else if secs >= 3600 {
        format!("{} hour(s)", secs / 3600)
    } else if secs >= 60 {
        format!("{} minute(s)", secs / 60)
    } else {
        format!("{} second(s)", secs)
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

/// Get filtered achievement count for current tab
pub fn filtered_achievement_count(app: &App) -> usize {
    Achievement::all()
        .iter()
        .filter(|a| matches_tab(a, app.achievement_tab))
        .count()
}
