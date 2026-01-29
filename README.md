# Solaris

A terminal-based idle game where you build a solar energy empire, from humble solar panels to galaxy-spanning star forges.

![Solaris Demo](assets/demo.png)

## Features

- **10 Producer Types**: Progress from Solar Panels to Dyson Spheres and beyond
- **48 Upgrades**: Enhance your energy production with tiered upgrades
- **Offline Progress**: Earn energy even while away (up to 8 hours)
- **Auto-save**: Game saves automatically every 30 seconds
- **Boss Mode**: Quick-hide spreadsheet view (press `B`)
- **TUI Interface**: Beautiful terminal interface with solar system visualization

## Installation

### From source

Requires Rust 1.70 or later.

```bash
git clone https://github.com/yourusername/solaris.git
cd solaris
cargo build --release
./target/release/solaris
```

### Run directly

```bash
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| `1-9`, `0` | Buy producer (1-10) |
| `Shift+1-9`, `Shift+0` | Buy 10 producers |
| `Ctrl+1-9`, `Ctrl+0` | Buy 100 producers |
| `U` | Toggle upgrades panel |
| `J/K` or `↑/↓` | Navigate upgrades |
| `Enter/Space` | Purchase selected upgrade |
| `B` | Toggle boss mode |
| `?` | Toggle help |
| `Q` or `Esc` | Quit |

## Producers

| # | Name | Energy/s | Base Cost |
|---|------|----------|-----------|
| 1 | Solar Panel | 0.1 | 15 |
| 2 | Mining Drone | 0.5 | 100 |
| 3 | Asteroid Mine | 2.0 | 500 |
| 4 | Orbital Station | 10.0 | 3,000 |
| 5 | Lunar Colony | 50.0 | 15,000 |
| 6 | Planetary Harvester | 250.0 | 80,000 |
| 7 | Fusion Reactor | 1,200.0 | 400,000 |
| 8 | Dyson Swarm | 6,000.0 | 2,000,000 |
| 9 | Dyson Sphere | 30,000.0 | 10,000,000 |
| 10 | Star Forge | 150,000.0 | 50,000,000 |

## Save Data

Save files are stored in your system's data directory:
- **Linux**: `~/.local/share/solaris/`
- **macOS**: `~/Library/Application Support/solaris/`
- **Windows**: `%APPDATA%\solaris\`

## Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run

# Build optimized release
cargo build --release
```

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal library

## License

MIT
