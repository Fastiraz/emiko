// use crate::logger::logger::log;

use std::io::Write;

pub fn get_memory() -> String {
  let home_directory = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .unwrap_or_else(|_| "Unable to get your home dir!".to_string());

  let mut log_path = std::path::PathBuf::new();
  log_path.push(home_directory);
  log_path.push(".config");
  log_path.push("emiko");
  log_path.push("memory");
  log_path.set_extension("txt");

  // return std::fs::read_to_string(&log_path).unwrap_or_else(|_| String::new())

  // log("INFO", "Remember...");

  match std::fs::read_to_string(&log_path) {
    Ok(content) => return content,
    Err(_) => {
      // log("ERROR", "Fail to remember.");
      return String::new()
    }
  }
}

pub fn learn(prompt: String, command: String, stdout: String) {
  let home_directory = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .unwrap_or_else(|_| "Unable to get your home dir!".to_string());

  let mut memory_path = std::path::PathBuf::new();
  memory_path.push(home_directory);
  memory_path.push(".config");
  memory_path.push("emiko");
  memory_path.push("memory");
  memory_path.set_extension("txt");

  let mut file = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(memory_path)
    .expect("Failed to open memory file");

  writeln!(file, "user: {}\nassistant: {}\nstandard output: {}", prompt, command, stdout)
    .expect("Failed to learn");

  // match writeln!(file, "user: {}\nassistant: {}\nstandard output: {}", prompt, command, stdout) {
  //   Ok(_) => log("INFO", "Learned."),
  //   Err(_) => {
  //     log("ERROR", "Fail to learn.");
  //   }
  // }
}
