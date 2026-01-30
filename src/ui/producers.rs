use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

use crate::app::App;
use crate::format::{format_cost, format_rate};
use crate::game::calculate_bulk_cost;

pub fn render(frame: &mut Frame, area: Rect, app: &App, focused: bool) {
    let visible = app.game.visible_producers();

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

    let header = format!(
        "  {:<19} {:>4}  {:>10}  {:>12}",
        "Producer", "Own", "Rate", "Cost"
    );

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

    // Create a layout with header and list
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Render header
    let header_widget = ratatui::widgets::Paragraph::new(header)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    frame.render_widget(header_widget, chunks[0]);

    // Render list
    let list = List::new(items);
    let mut state = ListState::default();
    // Clamp selected producer to visible range
    let max_selection = visible.len().saturating_sub(1);
    state.select(Some(app.selected_producer.min(max_selection)));
    frame.render_stateful_widget(list, chunks[1], &mut state);
}
