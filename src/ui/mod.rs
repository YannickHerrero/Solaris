mod boss;
mod header;
mod help;
mod layout;
mod producers;
mod stats;
mod upgrades;
mod visualization;

use ratatui::prelude::*;

use crate::app::{App, LayoutMode, Panel};

pub fn render(frame: &mut Frame, app: &mut App) {
    // Check minimum terminal size
    let size = frame.area();
    if size.width < 60 || size.height < 20 {
        let msg = format!(
            "Terminal too small!\nMinimum: 60x20\nCurrent: {}x{}",
            size.width, size.height
        );
        let paragraph = ratatui::widgets::Paragraph::new(msg)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, size);
        return;
    }

    // Boss mode
    if app.boss_mode {
        boss::render(frame, size);
        return;
    }

    // Offline report overlay
    if let Some(report) = &app.offline_report {
        render_offline_report(frame, size, report);
        return;
    }

    // Determine layout mode based on terminal width
    app.layout_mode = layout::determine_layout_mode(size.width);

    // Normal UI
    let chunks = layout::create_layout(size, app);

    header::render(frame, chunks.header, app);

    match app.layout_mode {
        LayoutMode::Single => {
            // Single panel mode: render active panel only
            let focused = true;
            match app.active_panel {
                Panel::Producers => producers::render(frame, chunks.left_panel, app, focused),
                Panel::Upgrades => upgrades::render(frame, chunks.left_panel, app, focused),
                Panel::Stats => stats::render(frame, chunks.left_panel, app, focused),
                Panel::Visualization => visualization::render(frame, chunks.left_panel, app, focused),
            }
        }
        LayoutMode::TwoColumn => {
            // Two-column mode: Producers left, Visualization top-right, Upgrades bottom-right
            let producers_focused = app.active_panel == Panel::Producers;
            let viz_focused = app.active_panel == Panel::Visualization;
            let upgrades_focused = app.active_panel == Panel::Upgrades;

            producers::render(frame, chunks.left_panel, app, producers_focused);

            if let Some(right_top) = chunks.right_top {
                visualization::render(frame, right_top, app, viz_focused);
            }

            if let Some(right_bottom) = chunks.right_bottom {
                upgrades::render(frame, right_bottom, app, upgrades_focused);
            }
        }
    }

    // Help popup overlay (rendered last so it appears on top)
    help::render(frame, size, app);
}

fn render_offline_report(frame: &mut Frame, area: Rect, report: &crate::app::OfflineReport) {
    use ratatui::widgets::{Block, Borders, Paragraph, Clear};

    // Center the popup
    let popup_width = 40;
    let popup_height = 7;
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);

    let duration = crate::format::format_duration(report.duration_secs);
    let energy = crate::format::format_energy(report.energy_earned);

    let text = format!(
        "\nWelcome back!\n\nYou were away for {}\nEarned: {} ⚛\n\nPress any key to continue",
        duration, energy
    );

    let block = Block::default()
        .title(" Offline Progress ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, popup_area);
}
