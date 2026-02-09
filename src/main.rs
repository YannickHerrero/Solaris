mod app;
mod auto;
mod format;
mod game;
mod hint;
mod input;
mod save;
mod ui;

use std::io::{self, Write};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::App;
use auto::AutoPlayer;

pub const TICK_RATE_MS: u64 = 100; // 10 ticks/second for game logic
pub const TICKS_PER_SECOND: f64 = 1000.0 / TICK_RATE_MS as f64;
const FRAME_RATE_MS: u64 = 16; // ~60 FPS for rendering
const AUTOSAVE_INTERVAL_SECS: u64 = 30;

fn main() -> io::Result<()> {
    // Handle command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut auto_mode = false;
    let mut auto_speed: f64 = 1.0;
    let mut explicit_label: Option<String> = None;
    let mut create_new: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--reset" | "--delete" => {
                // Get optional label
                let label = if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    i += 1;
                    Some(args[i].clone())
                } else {
                    None
                };
                return handle_delete(label);
            }
            "--list" => {
                return handle_list();
            }
            "--new" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --new requires a label (e.g. --new my-save)");
                    return Ok(());
                }
                match save::validate_label(&args[i]) {
                    Ok(sanitized) => create_new = Some(sanitized),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return Ok(());
                    }
                }
            }
            "--load" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --load requires a label (e.g. --load my-save)");
                    return Ok(());
                }
                match save::validate_label(&args[i]) {
                    Ok(sanitized) => explicit_label = Some(sanitized),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return Ok(());
                    }
                }
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--auto" => {
                auto_mode = true;
            }
            "--speed" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --speed requires a numeric value (e.g. --speed 2)");
                    return Ok(());
                }
                match args[i].parse::<f64>() {
                    Ok(v) if v > 0.0 => auto_speed = v,
                    _ => {
                        eprintln!(
                            "Error: --speed value must be a positive number (e.g. --speed 2)"
                        );
                        return Ok(());
                    }
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                eprintln!("Use --help for usage information");
                return Ok(());
            }
        }
        i += 1;
    }

    // --speed implies --auto
    if auto_speed != 1.0 {
        auto_mode = true;
    }

    // Migrate legacy save if it exists
    if let Ok(Some(migrated_label)) = save::migrate_legacy_save() {
        println!("Migrated existing save to '{}'", migrated_label);
    }

    // Determine which save to use
    let is_new_save = create_new.is_some();
    let save_label = if let Some(label) = create_new {
        // Creating a new save
        if save::save_exists(&label)? {
            eprintln!(
                "Error: Save '{}' already exists. Use --load to load it.",
                label
            );
            return Ok(());
        }
        label
    } else if let Some(label) = explicit_label {
        // Loading a specific save
        if !save::save_exists(&label)? {
            eprintln!(
                "Error: Save '{}' not found. Use --list to see available saves.",
                label
            );
            return Ok(());
        }
        label
    } else {
        // Auto-resolve: last used > "main" > create "main"
        save::resolve_save_label(None)?.unwrap_or_else(|| "main".to_string())
    };

    // Create app
    let mut app = App::new(save_label);
    app.auto_mode = auto_mode;
    app.auto_speed = auto_speed;

    // Load saved game if exists (for existing saves), or save immediately for new saves
    if is_new_save {
        // Save immediately so the save file exists
        if let Err(e) = app.save() {
            eprintln!("Warning: Could not create save file: {}", e);
        }
    } else if let Err(e) = app.load() {
        eprintln!("Warning: Could not load save file: {}", e);
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut auto_player = if auto_mode {
        Some(AutoPlayer::new(auto_speed))
    } else {
        None
    };

    let result = run_app(&mut terminal, &mut app, &mut auto_player);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn print_help() {
    println!("Solaris - Terminal-based idle game");
    println!();
    println!("Usage: solaris [OPTIONS]");
    println!();
    println!("Save Management:");
    println!("  --new <label>    Create a new save with the given label");
    println!("  --load <label>   Load a specific save");
    println!("  --list           List all available saves");
    println!("  --delete <label> Delete a specific save (with confirmation)");
    println!();
    println!("  Without options, loads the last used save or creates 'main' if none exist.");
    println!();
    println!("Game Options:");
    println!("  --auto           Enable auto-play mode (buys producers and upgrades)");
    println!("  --speed <N>      Set auto-play speed multiplier (default: 1, max effective: ~10)");
    println!("  --help           Show this help message");
}

fn handle_list() -> io::Result<()> {
    // Migrate legacy save first if needed
    if let Ok(Some(migrated_label)) = save::migrate_legacy_save() {
        println!("Migrated existing save to '{}'", migrated_label);
        println!();
    }

    let saves = save::list_saves()?;
    let last_used = save::get_last_used()?.unwrap_or_default();

    if saves.is_empty() {
        println!("No saves found.");
        println!();
        println!("Create a new save with: solaris --new <label>");
        return Ok(());
    }

    println!("Available saves:");
    println!();

    for save_info in &saves {
        let marker = if save_info.label == last_used {
            " *"
        } else {
            ""
        };

        // Try to read the save to get last save time
        if let Ok(Some(save_data)) = save::load_game(&save_info.label) {
            let time_ago = format_time_ago(save_data.last_save);
            println!(
                "  {}{} (last played: {})",
                save_info.label, marker, time_ago
            );
        } else {
            println!("  {}{}", save_info.label, marker);
        }
    }

    println!();
    println!("* = last used");
    println!();
    println!("Load a save with: solaris --load <label>");

    Ok(())
}

fn format_time_ago(time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(time);

    let secs = duration.num_seconds();
    if secs < 60 {
        return "just now".to_string();
    }

    let mins = duration.num_minutes();
    if mins < 60 {
        return format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" });
    }

    let hours = duration.num_hours();
    if hours < 24 {
        return format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" });
    }

    let days = duration.num_days();
    if days < 30 {
        return format!("{} day{} ago", days, if days == 1 { "" } else { "s" });
    }

    let months = days / 30;
    format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
}

fn handle_delete(label: Option<String>) -> io::Result<()> {
    // Migrate legacy save first if needed
    if let Ok(Some(migrated_label)) = save::migrate_legacy_save() {
        println!("Migrated existing save to '{}'", migrated_label);
        println!();
    }

    let label = match label {
        Some(l) => match save::validate_label(&l) {
            Ok(sanitized) => sanitized,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Ok(());
            }
        },
        None => {
            // If no label provided, show available saves and ask
            let saves = save::list_saves()?;
            if saves.is_empty() {
                println!("No saves found. Nothing to delete.");
                return Ok(());
            }

            println!("Available saves:");
            for save_info in &saves {
                println!("  {}", save_info.label);
            }
            println!();
            print!("Enter the label of the save to delete: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                println!("Delete cancelled.");
                return Ok(());
            }

            match save::validate_label(input) {
                Ok(sanitized) => sanitized,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Ok(());
                }
            }
        }
    };

    let save_path = save::get_save_path(&label)?;

    if !save_path.exists() {
        println!("Save '{}' not found.", label);
        return Ok(());
    }

    println!("This will permanently delete the save '{}':", label);
    println!("  {}", save_path.display());
    println!();
    print!("Are you sure you want to delete this save? [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    if response == "y" || response == "yes" {
        if save::delete_save(&label)? {
            println!("Save '{}' deleted.", label);
        }
    } else {
        println!("Delete cancelled.");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    auto_player: &mut Option<AutoPlayer>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut last_save = Instant::now();

    loop {
        // Render every frame (~60 FPS)
        terminal.draw(|f| ui::render(f, app))?;

        // Auto-player mines every frame (~60 FPS), matching space-bar repeat rate
        if let Some(ref player) = auto_player {
            player.mine(app);
        }

        // Handle events with short timeout for responsiveness
        let timeout = Duration::from_millis(FRAME_RATE_MS);

        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                if input::handle_key(app, key) {
                    // Save on quit
                    let _ = app.save();
                    return Ok(());
                }
                // Pause auto-player on any user input
                if let Some(ref mut player) = auto_player {
                    player.pause();
                }
            }
        }

        // Game tick at 10 Hz
        if last_tick.elapsed() >= Duration::from_millis(TICK_RATE_MS) {
            app.tick();

            // Auto-player tick (runs after game tick so it sees fresh state)
            if let Some(ref mut player) = auto_player {
                player.tick(app);
                app.auto_paused = player.is_paused();
            }

            last_tick = Instant::now();
        }

        // Auto-save every 30 seconds
        if last_save.elapsed() >= Duration::from_secs(AUTOSAVE_INTERVAL_SECS) {
            let _ = app.save();
            last_save = Instant::now();
        }
    }
}
