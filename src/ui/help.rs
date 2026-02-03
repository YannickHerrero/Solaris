use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::{App, LayoutMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    if !app.show_help {
        return;
    }

    let help_text = match app.layout_mode {
        LayoutMode::Single => {
            r#"
  SOLARIS - Keyboard Shortcuts

  Navigation
  ----------
  j / Down      Move selection down
  k / Up        Move selection up
  Enter         Purchase selected item

  Panels
  ------
  u             Toggle Upgrades panel
  s             Toggle Stats panel
  v             Toggle Visualization panel
  a             Toggle Ascension panel
  x             Toggle Achievements panel

  Actions
  -------
  Space         Manual mine
  Tab           Cycle buy amount (1/10/Max)
  d             Toggle producer detail (in Producers)
  b             Toggle boss mode
  ?             Toggle this help

  q / Ctrl+C    Quit

  Press ? to close
"#
        }
        LayoutMode::TwoColumn => {
            r#"
  SOLARIS - Keyboard Shortcuts

  Navigation
  ----------
  j / Down      Move selection down
  k / Up        Move selection up
  h / Left      Focus previous panel
  l / Right     Focus next panel
  Enter         Purchase selected item

  Panels
  ------
  p             Focus Producers panel
  u             Focus Upgrades panel
  v             Focus Visualization panel
  a             Toggle Ascension panel
  x             Toggle Achievements panel

  Actions
  -------
  Space         Manual mine
  Tab           Cycle buy amount (1/10/Max)
  d             Toggle producer detail (in Producers)
  b             Toggle boss mode
  ?             Toggle this help

  q / Ctrl+C    Quit

  Press ? to close
"#
        }
    };

    // Calculate popup size and position (clamp to fit terminal)
    let popup_width = 44.min(area.width.saturating_sub(4));
    let popup_height = 26.min(area.height.saturating_sub(4));

    // Ensure we have minimum viable size
    if popup_width < 20 || popup_height < 10 {
        return;
    }

    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;

    // Ensure popup fits within bounds
    let popup_area = Rect {
        x,
        y,
        width: popup_width.min(area.width - x),
        height: popup_height.min(area.height - y),
    };

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, popup_area);
}
