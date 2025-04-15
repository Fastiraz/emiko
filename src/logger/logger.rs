use std::io::Write;

pub fn log(level: &str, message: &str) {
  let home_directory = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .unwrap_or_else(|_| "Unable to get your home dir!".to_string());

  let mut log_path = std::path::PathBuf::new();
  log_path.push(home_directory);
  log_path.push(".config");
  log_path.push("emiko");
  log_path.push("emiko");
  log_path.set_extension("log");

  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(log_path)
    .expect("Failed to open log file");

  let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs();

  writeln!(file, "[{}] {}: {}", timestamp, level, message)
    .expect("Failed to write to log file");
}
