use clap::{Parser, Subcommand};
use colored::*;
use rdev::{Event, EventType, listen};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "keystr")]
#[command(about = "A CLI tool to count keyboard presses (no key data stored)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the keystroke counter
    Init,
    /// Start monitoring keystrokes in detached mode
    Start,
    /// Stop the background monitoring process
    Stop,
    /// Check if monitoring is running
    Status,
    /// Show statistics
    Stats {
        /// Show daily stats
        #[arg(short, long)]
        daily: bool,
        /// Show weekly stats
        #[arg(short, long)]
        weekly: bool,
        /// Show monthly stats
        #[arg(short, long)]
        monthly: bool,
    },
    /// Export statistics to a text file
    Export {
        /// Output file path
        #[arg(short, long, default_value = "keystroke_stats.txt")]
        output: String,
    },
    /// Reset all statistics
    Reset,
    /// Internal command - do not use directly
    #[command(hide = true)]
    Daemon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct KeystrokeData {
    total_count: u64,
    daily_records: Vec<DailyRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DailyRecord {
    date: String,
    count: u64,
    timestamp: u64,
}

impl KeystrokeData {
    fn new() -> Self {
        KeystrokeData {
            total_count: 0,
            daily_records: Vec::new(),
        }
    }

    fn increment(&mut self) {
        self.total_count += 1;
        let today = format_date_storage();
        let timestamp = current_timestamp();

        if let Some(record) = self.daily_records.iter_mut().find(|r| r.date == today) {
            record.count += 1;
        } else {
            self.daily_records.push(DailyRecord {
                date: today,
                count: 1,
                timestamp,
            });
        }
    }

    fn get_daily_stats(&self, days: usize) -> Vec<DailyRecord> {
        let mut records = self.daily_records.clone();
        records.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        records.into_iter().take(days).collect()
    }

    fn get_weekly_stats(&self) -> u64 {
        let seven_days_ago = current_timestamp() - (7 * 24 * 60 * 60);
        self.daily_records
            .iter()
            .filter(|r| r.timestamp >= seven_days_ago)
            .map(|r| r.count)
            .sum()
    }

    fn get_monthly_stats(&self) -> u64 {
        let thirty_days_ago = current_timestamp() - (30 * 24 * 60 * 60);
        self.daily_records
            .iter()
            .filter(|r| r.timestamp >= thirty_days_ago)
            .map(|r| r.count)
            .sum()
    }
}

fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not find config directory");
    path.push("keystroke");
    path
}

fn get_data_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("data.json");
    path
}

fn get_pid_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("daemon.pid");
    path
}

fn load_data() -> KeystrokeData {
    let file_path = get_data_file();
    if file_path.exists() {
        let content = fs::read_to_string(&file_path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&content).unwrap_or_else(|_| KeystrokeData::new())
    } else {
        KeystrokeData::new()
    }
}

fn save_data(data: &KeystrokeData) {
    let file_path = get_data_file();
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize data");
    fs::write(file_path, json).expect("Failed to write data file");
}

fn format_date_display(time: &SystemTime) -> String {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let secs = duration.as_secs();
    let days_since_epoch = (secs / 86400) as i64;

    let mut year = 1970;
    let mut days_remaining = days_since_epoch;

    let years_passed = days_remaining / 365;
    year += years_passed as i32;
    days_remaining -= years_passed * 365;

    let leap_days = years_passed / 4;
    days_remaining -= leap_days;

    if days_remaining < 0 {
        year -= 1;
        days_remaining += 365;
    }

    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 0;
    let mut day = days_remaining as i32;

    for (i, &month_len) in month_days.iter().enumerate() {
        if day <= month_len {
            month = i + 1;
            break;
        }
        day -= month_len;
    }

    if month == 0 {
        month = 12;
        day = month_days[11];
    }

    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    format!("{:02} {} {}", day, month_names[month - 1], year)
}

fn format_date_storage() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let days = now.as_secs() / 86400;
    format!("{}", days)
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn is_running() -> Option<u32> {
    let pid_file = get_pid_file();
    if !pid_file.exists() {
        return None;
    }

    let pid_str = fs::read_to_string(&pid_file).ok()?;
    let pid: u32 = pid_str.trim().parse().ok()?;

    // we need to check if process is actually running
    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()
            .ok()?;

        if output.status.success() {
            Some(pid)
        } else {
            let _ = fs::remove_file(&pid_file);
            None
        }
    }

    #[cfg(not(unix))]
    {
        Some(pid)
    }
}

fn cmd_init() {
    println!(
        "\n{}",
        "╭─────────────────────────────────────╮".bright_black()
    );
    println!(
        "{}",
        "│  Keystroke Counter Initialization  │"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}\n",
        "╰─────────────────────────────────────╯".bright_black()
    );

    let config_dir = get_config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        println!(
            "  {} {}",
            "✓".green().bold(),
            "Config directory created".dimmed()
        );
    } else {
        println!(
            "  {} {}",
            "✓".green().bold(),
            "Config directory ready".dimmed()
        );
    }

    let data_file = get_data_file();
    if !data_file.exists() {
        let initial_data = KeystrokeData::new();
        save_data(&initial_data);
        println!(
            "  {} {}",
            "✓".green().bold(),
            "Data file created with sample history".dimmed()
        );
    } else {
        println!("  {} {}", "✓".green().bold(), "Data file ready".dimmed());
    }

    println!("\n  {} {}", "→".bright_cyan(), "Ready to start monitoring!");
    println!(
        "  {} Run {} to begin\n",
        "→".bright_cyan(),
        "keystr start".bright_yellow().bold()
    );
}

fn cmd_start() {
    if let Some(pid) = is_running() {
        println!(
            "\n  {} Monitoring is already active (PID: {})\n",
            "●".green().bold(),
            pid.to_string().bright_cyan()
        );
        return;
    }

    println!("\n  {} Starting keystroke monitor...", "→".bright_cyan());

    let exe = std::env::current_exe().expect("Failed to get current executable path");

    #[cfg(unix)]
    {
        Command::new(exe)
            .arg("daemon")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start daemon");
    }

    #[cfg(not(unix))]
    {
        Command::new(exe)
            .arg("daemon")
            .creation_flags(0x08000000)
            .spawn()
            .expect("Failed to start daemon");
    }

    std::thread::sleep(std::time::Duration::from_millis(500));

    if let Some(pid) = is_running() {
        println!(
            "  {} Monitor active (PID: {})",
            "✓".green().bold(),
            pid.to_string().bright_cyan()
        );
        println!(
            "  {} Only counting keystrokes - no data captured",
            "ℹ".blue()
        );
        println!(
            "  {} Use {} to stop\n",
            "→".bright_cyan(),
            "keystr stop".bright_yellow()
        );
    } else {
        println!("  {} Failed to start monitor\n", "✗".red().bold());
    }
}

fn cmd_stop() {
    if let Some(pid) = is_running() {
        println!(
            "\n  {} Stopping monitor (PID: {})...",
            "→".bright_yellow(),
            pid.to_string().bright_cyan()
        );

        #[cfg(unix)]
        {
            Command::new("kill")
                .arg(pid.to_string())
                .output()
                .expect("Failed to stop daemon");
        }

        #[cfg(not(unix))]
        {
            Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output()
                .expect("Failed to stop daemon");
        }

        let _ = fs::remove_file(get_pid_file());
        println!("  {} Monitor stopped\n", "✓".green().bold());
    } else {
        println!("\n  {} Monitor is not running\n", "ℹ".blue());
    }
}

fn cmd_status() {
    println!();
    if let Some(pid) = is_running() {
        println!(
            "  {} {} │ PID: {}",
            "●".green().bold(),
            "Active".bright_green().bold(),
            pid.to_string().bright_cyan()
        );
    } else {
        println!("  {} {}", "○".dimmed(), "Inactive".dimmed());
    }
    println!();
}

fn cmd_daemon() {
    let pid = std::process::id();
    let pid_file = get_pid_file();
    fs::write(&pid_file, pid.to_string()).expect("Failed to write PID file");

    let data = Arc::new(Mutex::new(load_data()));
    let data_clone = Arc::clone(&data);

    let callback = move |event: Event| {
        if let EventType::KeyPress(_) = event.event_type {
            let mut data = data_clone.lock().unwrap();
            data.increment();

            if data.total_count % 10 == 0 {
                save_data(&*data);
            }
        }
    };

    ctrlc::set_handler(move || {
        let data = data.lock().unwrap();
        save_data(&*data);
        let _ = fs::remove_file(get_pid_file());
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
        let _ = fs::remove_file(&pid_file);
    }
}

fn draw_line_graph(records: &[DailyRecord], max_height: usize) {
    if records.is_empty() {
        return;
    }

    let max_count = records.iter().map(|r| r.count).max().unwrap_or(1);
    let scale = max_count as f64 / max_height as f64;

    // Draw Y-axis label
    println!("     {}", format!("{}", max_count).bright_black());

    for row in (0..max_height).rev() {
        let threshold = (row as f64 * scale) as u64;
        print!("     ");

        for (i, record) in records.iter().enumerate() {
            if record.count > threshold {
                let bar = if record.count == max_count && row == max_height - 1 {
                    "█".bright_cyan().bold()
                } else {
                    "█".bright_green()
                };
                print!("{}", bar);
            } else {
                print!("{}", "·".truecolor(40, 40, 40));
            }
            if i < records.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    // Draw X-axis
    print!("     ");
    for i in 0..records.len() {
        print!("{}", "─".bright_black());
        if i < records.len() - 1 {
            print!(" ");
        }
    }
    println!();

    // Draw dates
    print!("     ");
    for record in records {
        let timestamp = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(record.timestamp);
        let formatted = format_date_display(&timestamp);
        let parts: Vec<&str> = formatted.split_whitespace().collect();
        if parts.len() >= 2 {
            print!("{} ", format!("{}", parts[0]).truecolor(100, 100, 100));
        }
    }
    println!("\n");
}

fn cmd_stats(daily: bool, weekly: bool, monthly: bool) {
    let data = load_data();

    println!(
        "\n{}",
        "╭────────────────────────────────────╮".bright_black()
    );
    println!(
        "{}",
        "│      Keystroke Statistics          │"
            .bright_cyan()
            .bold()
    );
    println!(
        "{}",
        "╰────────────────────────────────────╯".bright_black()
    );

    println!(
        "\n     {} {}",
        "Total:".dimmed(),
        data.total_count.to_string().bright_cyan().bold()
    );

    if daily || (!weekly && !monthly) {
        println!(
            "\n     {}",
            "Daily Activity (Last 7 Days)".bright_white().bold()
        );
        println!("     {}\n", "─".repeat(28).bright_black());

        let daily_stats = data.get_daily_stats(7);

        // Reverse for chronological order in graph
        let mut graph_data = daily_stats.clone();
        graph_data.reverse();

        if !graph_data.is_empty() {
            draw_line_graph(&graph_data, 10);
        }

        for record in &daily_stats {
            let timestamp =
                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(record.timestamp);
            let formatted_date = format_date_display(&timestamp);
            println!(
                "     {} │ {}",
                formatted_date.truecolor(120, 120, 120),
                record.count.to_string().bright_green()
            );
        }
    }

    if weekly {
        let weekly_count = data.get_weekly_stats();
        println!("\n     {}", "Weekly Summary (7 days)".bright_white().bold());
        println!("     {}", "─".repeat(28).bright_black());
        println!(
            "     {} keystrokes\n",
            weekly_count.to_string().bright_cyan().bold()
        );
    }

    if monthly {
        let monthly_count = data.get_monthly_stats();
        println!(
            "\n     {}",
            "Monthly Summary (30 days)".bright_white().bold()
        );
        println!("     {}", "─".repeat(28).bright_black());
        println!(
            "     {} keystrokes\n",
            monthly_count.to_string().bright_cyan().bold()
        );
    }

    println!();
}

fn cmd_export(output: &str) {
    let data = load_data();

    let mut content = String::new();
    content.push_str("╭────────────────────────────────────╮\n");
    content.push_str("│   Keystroke Counter Statistics     │\n");
    content.push_str("╰────────────────────────────────────╯\n\n");
    content.push_str(&format!("Total Keystrokes: {}\n\n", data.total_count));

    content.push_str("Daily Records:\n");
    content.push_str("────────────────────────────────────\n");
    for record in data.daily_records.iter().rev() {
        let timestamp = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(record.timestamp);
        let formatted_date = format_date_display(&timestamp);
        content.push_str(&format!(
            "{}: {} keystrokes\n",
            formatted_date, record.count
        ));
    }

    content.push_str("\nWeekly Summary (7 days):  ");
    content.push_str(&format!("{} keystrokes\n", data.get_weekly_stats()));

    content.push_str("Monthly Summary (30 days): ");
    content.push_str(&format!("{} keystrokes\n", data.get_monthly_stats()));

    fs::write(output, content).expect("Failed to write export file");
    println!(
        "\n  {} Exported to {}\n",
        "✓".green().bold(),
        output.bright_cyan()
    );
}

fn cmd_reset() {
    println!();
    print!("  {} ", "Reset all statistics? (y/N):".bright_yellow());
    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        let new_data = KeystrokeData::new();
        save_data(&new_data);
        println!("  {} All statistics cleared\n", "✓".green().bold());
    } else {
        println!("  {} Reset cancelled\n", "ℹ".blue());
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Start => cmd_start(),
        Commands::Stop => cmd_stop(),
        Commands::Status => cmd_status(),
        Commands::Stats {
            daily,
            weekly,
            monthly,
        } => cmd_stats(daily, weekly, monthly),
        Commands::Export { output } => cmd_export(&output),
        Commands::Reset => cmd_reset(),
        Commands::Daemon => cmd_daemon(),
    }
}
