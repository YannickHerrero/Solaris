use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let lines = match &app.hint_message {
        Some(lines) => lines,
        None => return,
    };

    let content = lines.join("\n");
    let content_lines = lines.len() as u16;

    // Popup dimensions: enough for the content + border + padding
    let popup_width = 40.min(area.width.saturating_sub(4));
    let popup_height = (content_lines + 3).min(area.height.saturating_sub(4)); // +2 border +1 padding

    if popup_width < 20 || popup_height < 4 {
        return;
    }

    // Position: bottom-center (above the bottom edge, horizontally centered)
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = area.height.saturating_sub(popup_height).saturating_sub(2);

    let popup_area = Rect::new(
        x,
        y,
        popup_width.min(area.width.saturating_sub(x)),
        popup_height.min(area.height.saturating_sub(y)),
    );

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Hint (i) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(content)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, popup_area);
}
