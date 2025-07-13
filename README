# Gnome Desktop Entry Fixer

A Rust utility that automatically fixes and manages Steam game desktop entries on GNOME desktop environments. This tool scans for Steam game desktop files, adds proper `StartupWMClass` entries, and copies desktop entries to the applications menu for better integration.

## What It Does

This tool addresses common issues with Steam games on GNOME:

1. **Missing StartupWMClass**: Many Steam games don't have proper `StartupWMClass` entries, which can cause issues with window management and taskbar grouping
2. **Desktop File Organization**: Automatically copies desktop entries from Desktop to the applications menu for better system integration
3. **Real-time Monitoring**: Watches for new desktop files and processes them immediately

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

Run the tool to process all existing desktop files:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/gnome_desktop_fixer
```

## Development

### Project Structure

```
src/
├── main.rs          # Main entry point
├── constants.rs     # Configuration constants
├── utils.rs         # Core utility functions
└── watch.rs         # File watching functionality
```


## Known Issues and TODOs

- **Performance**: The tool opens many files simultaneously, which can cause "too many open files" errors
- **Loops**: Some performance issues with file processing loops
- **Non-Steam Games**: Currently only works with Steam games, could be extended for other game platforms
