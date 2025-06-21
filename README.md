# Nerd Font Installer CLI

A Rust-based command-line tool to list, download, and install Nerd Fonts on your system with beautiful colored output and progress indicators.

## ✨ Features

- 🌈 **Colored terminal output** for better readability
- 📊 **Progress bars** during font downloads
- 🔍 **Detailed font information** including descriptions, variants, and sizes
- 🎯 **Interactive font selection** with numbered options
- 🚀 **Fast downloads** with proper error handling
- 🖥️ **Cross-platform support** (macOS, Linux, Windows)
- ⚡ **Batch installation** support
- 📁 **Automatic font directory detection**

## 🚀 Quick Start

### List available fonts
```bash
cargo run -- list                 # Simple list
cargo run -- list --details       # Detailed information
```

### Install fonts interactively
```bash
cargo run -- install              # Interactive selection
```

### Install specific fonts
```bash
cargo run -- install firacode hack        # Install specific fonts
cargo run -- install --yes "JetBrains"    # Skip confirmation
```

### Other commands
```bash
cargo run -- info                 # Show installed fonts info
cargo run -- remove fontname      # Remove fonts (planned)
cargo run -- update              # Update fonts (planned)
```

## 📋 Available Fonts

1. **FiraCode Nerd Font** (2.1 MB) - Monospaced font with programming ligatures
2. **Hack Nerd Font** (1.8 MB) - A typeface designed for source code
3. **JetBrainsMono Nerd Font** (2.3 MB) - Typeface for developers by JetBrains
4. **SourceCodePro Nerd Font** (1.9 MB) - Monospaced font family by Adobe
5. **DejaVuSansMono Nerd Font** (1.5 MB) - Monospaced version of DejaVu Sans
6. **CascadiaCode Nerd Font** (2.0 MB) - Microsoft's programming font with ligatures
7. **Meslo Nerd Font** (1.7 MB) - Customized version of Apple's Menlo font

## 🛠️ Installation

1. **Prerequisites**: Rust (https://rustup.rs)

2. **Clone and build**:
```bash
git clone <repository>
cd nerd-font-installer
cargo build --release
```

3. **Run the installer**:
```bash
cargo run -- list
```

## 📁 Font Installation Locations

- **macOS**: `~/Library/Fonts/`
- **Linux**: `~/.local/share/fonts/`
- **Windows**: `%APPDATA%/Microsoft/Windows/Fonts/`

## 🎨 Usage Examples

### Interactive Installation
```bash
$ cargo run -- install
Available Nerd Fonts:
  1. FiraCode Nerd Font (2.1 MB)
  2. Hack Nerd Font (1.8 MB)
  3. JetBrainsMono Nerd Font (2.3 MB)
  ...

Enter numbers separated by commas to select fonts, or 'all' to install all:
> 1,3
```

### Direct Installation
```bash
$ cargo run -- install --yes firacode jetbrains
INFO [1/2] Installing FiraCode Nerd Font...
✓ Successfully installed 'FiraCode Nerd Font'!

INFO [2/2] Installing JetBrainsMono Nerd Font...
✓ Successfully installed 'JetBrainsMono Nerd Font'!

SUMMARY Installed 2/2 fonts successfully.
```

## 🔧 Command Reference

| Command | Description | Options |
|---------|-------------|---------|
| `list` | Show available fonts | `--details` for full info |
| `install` | Install selected fonts | `--yes` to skip confirmation |
| `info` | Show installed fonts | |
| `remove` | Remove fonts | Coming soon |
| `update` | Update all fonts | Coming soon |

## 🛡️ Error Handling

The CLI includes robust error handling for:
- Network connectivity issues
- Invalid font names
- Permission errors
- Disk space issues
- Corrupted downloads

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📜 License

This project is open source and available under the MIT License.
