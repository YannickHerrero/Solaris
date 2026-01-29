use ratatui::prelude::*;

use crate::app::{App, LayoutMode};

pub const MIN_WIDTH_TWO_COLUMN: u16 = 130;

pub struct LayoutChunks {
    pub header: Rect,
    pub left_panel: Rect,
    pub right_top: Option<Rect>,
    pub right_bottom: Option<Rect>,
}

pub fn determine_layout_mode(width: u16) -> LayoutMode {
    if width >= MIN_WIDTH_TWO_COLUMN {
        LayoutMode::TwoColumn
    } else {
        LayoutMode::Single
    }
}

pub fn create_layout(area: Rect, app: &App) -> LayoutChunks {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
        ])
        .split(area);

    let header = chunks[0];
    let main = chunks[1];

    match app.layout_mode {
        LayoutMode::Single => {
            LayoutChunks {
                header,
                left_panel: main,
                right_top: None,
                right_bottom: None,
            }
        }
        LayoutMode::TwoColumn => {
            // Split main horizontally: left (Producers) | right (Viz + Upgrades)
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(main);

            // Split right side vertically: top (Visualization) | bottom (Upgrades)
            let right_vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(horizontal[1]);

            LayoutChunks {
                header,
                left_panel: horizontal[0],
                right_top: Some(right_vertical[0]),
                right_bottom: Some(right_vertical[1]),
            }
        }
    }
}
