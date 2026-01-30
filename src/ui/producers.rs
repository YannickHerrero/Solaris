use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::format::{format_cost, format_duration, format_energy, format_rate};
use crate::game::{calculate_bulk_cost, Producer};

const INDICATOR_HEIGHT: u16 = 8;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let visible = app.game.visible_producers();

    let border_color = if focused { Color::Yellow } else { Color::DarkGray };
    let title = if focused {
        format!(" Producers [Buy: {}] *", app.buy_amount.label())
    } else {
        format!(" Producers [Buy: {}] ", app.buy_amount.label())
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Determine if we should show the indicator section
    let show_indicator = should_show_indicator(app);
    let indicator_height = if show_indicator { INDICATOR_HEIGHT } else { 0 };

    // Split inner area: header, producer list, and (optionally) indicator
    let chunks = if show_indicator && inner.height > INDICATOR_HEIGHT + 3 {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),                              // Header
                Constraint::Min(3),                                 // Producer list
                Constraint::Length(indicator_height),               // Indicator section
            ])
            .split(inner)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Header
                Constraint::Min(1),     // Producer list
            ])
            .split(inner)
    };

    // Render header
    let header = format!(
        "  {:<19} {:>4}  {:>10}  {:>12}",
        "Producer", "Own", "Rate", "Cost"
    );
    let header_widget = Paragraph::new(header)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(header_widget, chunks[0]);

    // Render producer list
    render_producer_list(frame, chunks[1], app, &visible);

    // Render indicator section if applicable
    if show_indicator && chunks.len() > 2 {
        render_producer_indicator(frame, chunks[2], app, &visible);
    }
}

fn should_show_indicator(app: &App) -> bool {
    app.game.total_producers_owned() > 0
}

fn render_producer_list(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    visible: &[(usize, &Producer)],
) {
    let items: Vec<ListItem> = visible
        .iter()
        .enumerate()
        .map(|(display_idx, (_, producer))| {
            let owned = app.game.producer_count(producer.id);
            let quantity = app.get_buy_quantity_for_producer(producer);
            let cost = calculate_bulk_cost(producer.base_cost, owned, quantity.max(1));
            let can_afford = app.game.energy >= cost && quantity > 0;

            let effective_rate = producer.base_energy_per_second
                * app.game.get_producer_multiplier(producer.id)
                * app.game.get_global_multiplier();

            let buy_label = if quantity == 0 {
                format!("({})", app.buy_amount.label())
            } else if quantity == 1 {
                String::new()
            } else {
                format!("(x{})", quantity)
            };

            let line = format!(
                "{} {:<19} {:>4}  {:>10}  {:>12} {}",
                producer.icon,
                producer.name,
                owned,
                format_rate(effective_rate),
                format_cost(cost),
                buy_label
            );

            let style = if display_idx == app.selected_producer {
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

    let list = List::new(items);
    let mut state = ListState::default();
    let max_selection = visible.len().saturating_sub(1);
    state.select(Some(app.selected_producer.min(max_selection)));
    frame.render_stateful_widget(list, area, &mut state);
}

fn render_producer_indicator(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    visible: &[(usize, &Producer)],
) {
    // Get the selected producer
    let selected_idx = app.selected_producer.min(visible.len().saturating_sub(1));
    let (_, producer) = match visible.get(selected_idx) {
        Some(p) => p,
        None => return,
    };

    // Clear the area first
    frame.render_widget(Clear, area);

    // Create the indicator block
    let indicator_block = Block::default()
        .title(format!(" {} {} ", producer.icon, producer.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let indicator_inner = indicator_block.inner(area);
    frame.render_widget(indicator_block, area);

    // Calculate stats
    let owned = app.game.producer_count(producer.id);
    let share = app.game.producer_production_percentage(producer.id);
    let total_rate = app.game.producer_total_rate(producer.id);
    let lifetime = app.game.producer_lifetime_production(producer.id);

    let quantity = app.get_buy_quantity_for_producer(producer).max(1);
    let next_cost = calculate_bulk_cost(producer.base_cost, owned, quantity);

    // Calculate ROI (time to pay back next purchase)
    let effective_rate = producer.base_energy_per_second
        * app.game.get_producer_multiplier(producer.id)
        * app.game.get_global_multiplier();
    let rate_gain = effective_rate * quantity as f64;
    let roi_seconds = if rate_gain > 0.0 {
        (next_cost / rate_gain).ceil() as u64
    } else {
        0
    };

    // Split indicator into two columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(indicator_inner);

    // Left column stats
    let left_text = vec![
        Line::from(vec![
            Span::styled("Owned:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", owned), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Total Rate: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format_rate(total_rate), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Lifetime:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} E", format_energy(lifetime)),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    // Right column stats
    let right_text = vec![
        Line::from(vec![
            Span::styled("Share:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.1}%", share), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("Next Cost: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format_cost(next_cost), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("ROI:       ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if roi_seconds > 0 {
                    format_duration(roi_seconds)
                } else {
                    "N/A".to_string()
                },
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    let left_paragraph = Paragraph::new(left_text);
    let right_paragraph = Paragraph::new(right_text);

    frame.render_widget(left_paragraph, columns[0]);
    frame.render_widget(right_paragraph, columns[1]);
}
