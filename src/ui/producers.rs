use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::format::{format_cost, format_duration, format_energy, format_rate};
use crate::game::{calculate_bulk_cost, Producer};

const INDICATOR_HEIGHT: u16 = 8;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let visible = app.game.visible_producers();

    let border_color = if focused {
        Color::Yellow
    } else {
        Color::DarkGray
    };
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
                Constraint::Length(1),                // Header
                Constraint::Min(3),                   // Producer list
                Constraint::Length(indicator_height), // Indicator section
            ])
            .split(inner)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Header
                Constraint::Min(1),    // Producer list
            ])
            .split(inner)
    };

    // Calculate dynamic column widths
    let name_width = visible
        .iter()
        .map(|(_, p)| p.name.len())
        .max()
        .unwrap_or(0)
        .max("Producer".len())
        + 1;

    let owned_width = visible
        .iter()
        .map(|(_, p)| app.game.producer_count(p.id).to_string().len())
        .max()
        .unwrap_or(0)
        .max("Own".len())
        + 1;

    let rate_width = visible
        .iter()
        .map(|(_, p)| {
            let rate = p.base_energy_per_second
                * app.game.get_producer_multiplier(p.id)
                * app.game.get_global_multiplier();
            format_rate(rate).len()
        })
        .max()
        .unwrap_or(0)
        .max("Rate".len())
        + 1;

    let cost_width = visible
        .iter()
        .map(|(_, p)| {
            let owned = app.game.producer_count(p.id);
            let qty = app.get_display_quantity_for_producer(p);
            format_cost(calculate_bulk_cost(p.base_cost, owned, qty, p.id)).len()
        })
        .max()
        .unwrap_or(0)
        .max("Cost".len())
        + 1;

    // Render header
    let header = format!(
        "  {:<name_width$} {:>owned_width$}  {:>rate_width$}  {:>cost_width$}",
        "Producer",
        "Own",
        "Rate",
        "Cost",
        name_width = name_width,
        owned_width = owned_width,
        rate_width = rate_width,
        cost_width = cost_width
    );
    let header_widget = Paragraph::new(header).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(header_widget, chunks[0]);

    // Render producer list
    render_producer_list(
        frame,
        chunks[1],
        app,
        &visible,
        name_width,
        owned_width,
        rate_width,
        cost_width,
    );

    // Render indicator section if applicable
    if show_indicator && chunks.len() > 2 {
        render_producer_indicator(frame, chunks[2], app, &visible);
    }
}

fn should_show_indicator(app: &App) -> bool {
    app.show_producer_detail && app.game.total_producers_owned() > 0
}

fn render_producer_list(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    visible: &[(usize, &Producer)],
    name_width: usize,
    owned_width: usize,
    rate_width: usize,
    cost_width: usize,
) {
    let mut items: Vec<ListItem> = visible
        .iter()
        .enumerate()
        .map(|(display_idx, (_, producer))| {
            let owned = app.game.producer_count(producer.id);
            let quantity = app.get_buy_quantity_for_producer(producer);
            let display_quantity = app.get_display_quantity_for_producer(producer);
            let cost =
                calculate_bulk_cost(producer.base_cost, owned, display_quantity, producer.id);
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
                "{} {:<name_width$} {:>owned_width$}  {:>rate_width$}  {:>cost_width$} {}",
                producer.icon,
                producer.name,
                owned,
                format_rate(effective_rate),
                format_cost(cost),
                buy_label,
                name_width = name_width,
                owned_width = owned_width,
                rate_width = rate_width,
                cost_width = cost_width
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

    // Add empty padding items so the last producer can be centered
    let visible_height = area.height as usize;
    let half_height = visible_height / 2;
    let padding_needed = half_height;
    for _ in 0..padding_needed {
        items.push(ListItem::new(""));
    }

    let list = List::new(items);
    let mut state = ListState::default();
    let max_selection = visible.len().saturating_sub(1);
    let selected = app.selected_producer.min(max_selection);
    state.select(Some(selected));

    // Center the selected item - no max_offset clamp needed with padding
    let offset = selected.saturating_sub(half_height);
    *state.offset_mut() = offset;

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

    let display_quantity = app.get_display_quantity_for_producer(producer);
    let next_cost = calculate_bulk_cost(producer.base_cost, owned, display_quantity, producer.id);

    // Calculate ROI (time to pay back next purchase)
    // For Solar Panel (id 1), include Thousand Rays bonus which is the main production source
    let rate_gain = if producer.id == 1 {
        // Solar Panel: base rate + Thousand Rays bonus per panel
        let base_rate =
            producer.base_energy_per_second * app.game.get_producer_multiplier(producer.id);
        let tr_bonus = app.game.get_thousand_rays_bonus();
        let non_collector_count = app.game.get_non_collector_building_count();
        let rate_per_panel = base_rate + (tr_bonus * non_collector_count as f64);
        rate_per_panel * app.game.get_global_multiplier() * display_quantity as f64
    } else {
        let effective_rate = producer.base_energy_per_second
            * app.game.get_producer_multiplier(producer.id)
            * app.game.get_global_multiplier();
        effective_rate * display_quantity as f64
    };
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
