use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, LayoutMode, Panel};
use crate::ui::filtered_achievement_count;

/// Handle a key event, returns true if the app should quit
pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    // Clear offline report on any key press
    if app.offline_report.is_some() {
        app.clear_offline_report();
        return false;
    }

    // Boss mode toggle
    if key.code == KeyCode::Char('b') {
        app.boss_mode = !app.boss_mode;
        return false;
    }

    // In boss mode, only 'b' works
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

    // When prestige panel is shown
    if app.show_prestige {
        match key.code {
            KeyCode::Char('a') | KeyCode::Esc => app.toggle_prestige(),
            KeyCode::Char('j') | KeyCode::Down => app.move_selection_down(),
            KeyCode::Char('k') | KeyCode::Up => app.move_selection_up(),
            KeyCode::Enter => app.purchase_selected(),
            KeyCode::Char('q') => return true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            _ => {}
        }
        return false;
    }

    // When achievements panel is shown
    if app.show_achievements {
        match key.code {
            KeyCode::Char('x') | KeyCode::Char('q') | KeyCode::Esc => app.toggle_achievements(),
            KeyCode::Tab => app.cycle_achievement_tab(),
            KeyCode::Char('j') | KeyCode::Down => {
                let max = filtered_achievement_count(app).saturating_sub(1);
                if app.selected_achievement < max {
                    app.selected_achievement += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if app.selected_achievement > 0 {
                    app.selected_achievement -= 1;
                }
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            _ => {}
        }
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

        // Prestige/Ascension panel
        KeyCode::Char('a') => app.toggle_prestige(),

        // Achievements panel
        KeyCode::Char('x') => app.toggle_achievements(),

        // Producer detail toggle (only when Producers panel is focused)
        KeyCode::Char('d') => {
            if app.active_panel == Panel::Producers {
                app.toggle_producer_detail();
            }
        }

        // Purchase hint
        KeyCode::Char('i') => app.show_hint(),

        // Manual mining
        KeyCode::Char(' ') => app.manual_mine(),

        _ => {}
    }

    false
}
