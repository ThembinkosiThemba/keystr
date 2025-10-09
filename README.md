# Keystroke Counter

A privacy-focused CLI tool that counts your keyboard activity without storing any actual keypress data. Perfect for tracking typing productivity and habits!

![Logo](/src/stats.png)

## Getting Started

```bash
cargo install keystr
keystr init
keystr start
```

> If you face installation issues on Debian/Ubuntu, do the [Prerequisites](#Prerequisites)

- **Only counts keystrokes** - no actual key data is captured or stored
- **Fully transparent** - all data stored locally
- **Open source** - audit the code yourself
- **No network access** - everything stays on your machine

## Features

- Track total, daily, weekly, and monthly keystroke counts
- Runs in detached background mode
- Export statistics to text files
- Beautiful colored terminal output
- Simple configuration in your config directory

## Prerequisites

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

## Installation

1. Clone the repository:

```bash
git clone https://github.com/thembinkosimkhonta01/keystr
cd keystr
```

2. Build the project:

```bash
make build
# or
cargo build --release
```

3. (Optional) Install globally:

```bash
make install
# or
cargo install --path .
```

Or copy the binary to your PATH:

```bash
sudo cp target/release/keystr /usr/local/bin/
```

## 📖 Usage

### Initialize

Set up the configuration directory and data files:

```bash
keystr init
```

### Start Monitoring

Start the background daemon (runs in detached mode):

```bash
keystr start
```

The daemon will run in the background and count keystrokes silently.

### Check Status

Check if monitoring is currently running:

```bash
keystr status
```

### Stop Monitoring

Stop the background daemon:

```bash
keystr stop
```

### View Statistics

Show all statistics (default shows daily for last 7 days):

```bash
keystr stats
```

Show daily statistics:

```bash
keystr stats --daily
```

Show weekly statistics:

```bash
keystr stats --weekly
```

Show monthly statistics:

```bash
keystr stats --monthly
```

Combine flags:

```bash
keystr stats --daily --weekly --monthly
```

### Export Statistics

Export statistics to a text file:

```bash
keystr export
```

Custom output file:

```bash
keystr export --output my_stats.txt
```

### Reset Statistics

Clear all statistics (requires confirmation):

```bash
keystr reset
```

## 📁 Configuration

All data is stored in your system's config directory:

- **Linux**: `~/.config/keystr/`
- **macOS**: `~/Library/Application Support/keystr/`
- **Windows**: `C:\Users\<User>\AppData\Roaming\keystr\`

Files:

- `data.json` - Stores keystroke counts and statistics
- `daemon.pid` - Process ID of running daemon (when active)

## 🐛 Troubleshooting

### "Permission denied" errors on Linux

The daemon might need elevated permissions to monitor keyboard events. Try:

```bash
sudo keystr start
```

### Daemon won't start

1. Check if it's already running: `keystr status`
2. Check system logs for errors
3. Try stopping and starting again:

```bash
keystr stop
keystr start
```

### Build fails with X11 errors

Make sure you've installed the X11 development libraries (see Prerequisites section).

## 📊 Example Output

```bash
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
