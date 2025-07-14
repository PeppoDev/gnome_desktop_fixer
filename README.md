# Gnome Desktop Entry Fixer

A Rust utility that automatically fixes Steam game desktop entries on GNOME desktop environments. Steam creates desktop shortcuts for games on Linux, but most of these shortcuts are missing the `StartupWMClass` property. This causes a common issue where the game's icon doesn't appear in the taskbar or dock when the game is running, making it difficult to identify which game is currently active. This tool monitors for new desktop files and adds proper `StartupWMClass` entries for Steam games.

## What It Does

This tool addresses common issues with Steam games on GNOME:

1. **Missing StartupWMClass**: Many Steam games don't have proper `StartupWMClass` entries, which can cause issues with window management and taskbar grouping
2. **Real-time Monitoring**: Watches for new desktop files and processes them automatically

## Installation

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd gnome_desktop_fixer

# Build the project
cargo build --release

# The binary will be available at target/release/gnome_desktop_fixer
```

## Usage

### Basic Usage

Run the tool to start monitoring for new desktop files:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/gnome_desktop_fixer
```

### What Happens When You Run It

1. **File Watching**: The tool monitors the applications directory for new `.desktop` files
2. **Steam Game Detection**: For each new desktop file, it looks for Steam game icons (`Icon=steam_icon_<id>`)
3. **StartupWMClass Addition**: If a Steam game is found and doesn't have a `StartupWMClass` entry, it adds `StartupWMClass=steam_app_<id>`

## Development

### Project Structure

```
src/
├── main.rs          # Main entry point
├── constants.rs     # Configuration constants
├── utils.rs         # Core utility functions
└── watch.rs         # File watching functionality
```

### Dependencies

- `inotify`: For file system monitoring
- `dirs`: For home directory detection

## Known Issues and TODOs

- **Performance**: The tool opens many files simultaneously, which can cause "too many open files" errors
- **Loops**: Some performance issues with file processing loops
- **Non-Steam Games**: Currently only works with Steam games, could be extended for other game platforms
- **Steam Native Games**: Currently only works with Steam games with proton, could be extended for native games but its harder to track
