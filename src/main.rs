mod app;
mod auto;
mod format;
mod game;
mod input;
mod save;
mod ui;

use std::io::{self, Write};
use std::time::{Duration, Instant};

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

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--reset" => {
                return handle_reset();
            }
            "--help" | "-h" => {
                println!("Solaris - Terminal-based idle game");
                println!();
                println!("Usage: solaris [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --auto     Enable auto-play mode (buys producers and upgrades)");
                println!("  --reset    Reset the save file (with confirmation)");
                println!("  --help     Show this help message");
                return Ok(());
            }
            "--auto" => {
                auto_mode = true;
            }
            _ => {
                eprintln!("Unknown option: {}", arg);
                eprintln!("Use --help for usage information");
                return Ok(());
            }
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    app.auto_mode = auto_mode;

    // Load saved game if exists
    if let Err(e) = app.load() {
        eprintln!("Warning: Could not load save file: {}", e);
    }

    let mut auto_player = if auto_mode {
        Some(AutoPlayer::new())
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

fn handle_reset() -> io::Result<()> {
    let save_path = save::get_save_path()?;

    if !save_path.exists() {
        println!("No save file found. Nothing to reset.");
        return Ok(());
    }

    println!("This will permanently delete your save file:");
    println!("  {}", save_path.display());
    println!();
    print!("Are you sure you want to reset? [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    if response == "y" || response == "yes" {
        if save::delete_save()? {
            println!("Save file deleted. Your progress has been reset.");
        }
    } else {
        println!("Reset cancelled.");
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
