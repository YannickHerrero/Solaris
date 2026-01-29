use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, LayoutMode, Panel};

/// Handle a key event, returns true if the app should quit
pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    // Clear offline report on any key press
    if app.offline_report.is_some() {
        app.clear_offline_report();
        return false;
    }

    // Boss mode toggle
    if key.code == KeyCode::Char('`') {
        app.boss_mode = !app.boss_mode;
        return false;
    }

    // In boss mode, only backtick works
    if app.boss_mode {
        return false;
    }

    // Help toggle
    if key.code == KeyCode::Char('?') {
        app.toggle_help();
        return false;
    }

    // When help is shown, most keys close the popup
    if app.show_help {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return true; // Ctrl+C still quits
        }
        app.toggle_help();
        return false;
    }

    match key.code {
        // Quit
        KeyCode::Char('q') => return true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,

        // Navigation (up/down within panel)
        KeyCode::Char('j') | KeyCode::Down => app.move_selection_down(),
        KeyCode::Char('k') | KeyCode::Up => app.move_selection_up(),

        // Focus cycling (left/right between panels in two-column mode)
        KeyCode::Char('h') | KeyCode::Left => app.cycle_focus_left(),
        KeyCode::Char('l') | KeyCode::Right => app.cycle_focus_right(),

        // Purchase
        KeyCode::Enter => app.purchase_selected(),

        // Buy amount cycling
        KeyCode::Tab => app.cycle_buy_amount(),

        // Panel focus/toggle
        KeyCode::Char('p') => {
            if app.layout_mode == LayoutMode::TwoColumn {
                app.focus_panel(Panel::Producers);
            } else {
                app.toggle_panel(Panel::Producers);
            }
        }
        KeyCode::Char('u') => {
            if app.layout_mode == LayoutMode::TwoColumn {
                app.focus_panel(Panel::Upgrades);
            } else {
                app.toggle_panel(Panel::Upgrades);
            }
        }
        KeyCode::Char('s') => {
            if app.layout_mode == LayoutMode::TwoColumn {
                // Stats not visible in two-column mode, stay on current panel
            } else {
                app.toggle_panel(Panel::Stats);
            }
        }
        KeyCode::Char('v') => {
            if app.layout_mode == LayoutMode::TwoColumn {
                app.focus_panel(Panel::Visualization);
            } else {
                app.toggle_panel(Panel::Visualization);
            }
        }

        // Manual mining
        KeyCode::Char(' ') => app.manual_mine(),

        _ => {}
    }

    false
}
