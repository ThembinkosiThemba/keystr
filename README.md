# Keystroke Counter 🔢

A privacy-focused CLI tool that counts your keyboard activity without storing any actual keypress data. Perfect for tracking typing productivity and habits!

## 🔐 Privacy First

- ✅ **Only counts keystrokes** - no actual key data is captured or stored
- ✅ **Fully transparent** - all data stored locally in plain JSON
- ✅ **Open source** - audit the code yourself
- ✅ **No network access** - everything stays on your machine

## ✨ Features

- 📊 Track total, daily, weekly, and monthly keystroke counts
- 🚀 Runs in detached background mode
- 📁 Export statistics to text files
- 🎨 Beautiful colored terminal output
- 🔧 Simple configuration in your config directory

## 📋 Prerequisites

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install libx11-dev libxtst-dev libevdev-dev libxcb1-dev
```

### Linux (Fedora/RHEL/CentOS)
```bash
sudo dnf install libX11-devel libXtst-devel libevdev-devel libxcb-devel
```

### Linux (Arch)
```bash
sudo pacman -S libx11 libxtst libevdev libxcb
```

## 🚀 Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd keystroke-counter
```

2. Build the project:
```bash
cargo build --release
```

3. (Optional) Install globally:
```bash
cargo install --path .
```

Or copy the binary to your PATH:
```bash
sudo cp target/release/keystroke-counter /usr/local/bin/
```

## 📖 Usage

### Initialize
Set up the configuration directory and data files:
```bash
keystroke-counter init
```

### Start Monitoring
Start the background daemon (runs in detached mode):
```bash
keystroke-counter start
```

The daemon will run in the background and count keystrokes silently.

### Check Status
Check if monitoring is currently running:
```bash
keystroke-counter status
```

### Stop Monitoring
Stop the background daemon:
```bash
keystroke-counter stop
```

### View Statistics

Show all statistics (default shows daily for last 7 days):
```bash
keystroke-counter stats
```

Show daily statistics:
```bash
keystroke-counter stats --daily
```

Show weekly statistics:
```bash
keystroke-counter stats --weekly
```

Show monthly statistics:
```bash
keystroke-counter stats --monthly
```

Combine flags:
```bash
keystroke-counter stats --daily --weekly --monthly
```

### Export Statistics
Export statistics to a text file:
```bash
keystroke-counter export
```

Custom output file:
```bash
keystroke-counter export --output my_stats.txt
```

### Reset Statistics
Clear all statistics (requires confirmation):
```bash
keystroke-counter reset
```

## 📁 Configuration

All data is stored in your system's config directory:

- **Linux**: `~/.config/keystroke/`
- **macOS**: `~/Library/Application Support/keystroke/`
- **Windows**: `C:\Users\<User>\AppData\Roaming\keystroke\`

Files:
- `data.json` - Stores keystroke counts and statistics
- `daemon.pid` - Process ID of running daemon (when active)

## 🔧 Development

### Dependencies

Add to your `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
colored = "2.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rdev = "0.5"
dirs = "5.0"
ctrlc = "3.4"
```

### Build
```bash
cargo build
```

### Run in development
```bash
cargo run -- init
cargo run -- start
cargo run -- stats
```

## 🐛 Troubleshooting

### "Permission denied" errors on Linux
The daemon might need elevated permissions to monitor keyboard events. Try:
```bash
sudo keystroke-counter start
```

### Daemon won't start
1. Check if it's already running: `keystroke-counter status`
2. Check system logs for errors
3. Try stopping and starting again:
```bash
keystroke-counter stop
keystroke-counter start
```

### Build fails with X11 errors
Make sure you've installed the X11 development libraries (see Prerequisites section).

## 📊 Example Output

```
=== Keystroke Statistics ===
Total Keystrokes: 45,782

📅 Daily Stats (Last 7 Days):
  2025-10-07 8,234 keystrokes
  2025-10-06 7,891 keystrokes
  2025-10-05 6,543 keystrokes
  2025-10-04 9,012 keystrokes
  2025-10-03 5,678 keystrokes
  2025-10-02 4,321 keystrokes
  2025-10-01 4,103 keystrokes

📊 Weekly Stats (Last 7 Days):
  45,782 keystrokes

📈 Monthly Stats (Last 30 Days):
  45,782 keystrokes
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⚠️ Disclaimer

This tool is designed for personal productivity tracking only. Always respect privacy laws and obtain proper consent before monitoring any keyboard activity that isn't your own.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [rdev](https://github.com/Narsil/rdev) for keyboard event monitoring
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Beautiful colors by [colored](https://github.com/mackwic/colored)