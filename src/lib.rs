#![warn(unused_imports)]
#![allow(non_snake_case)]

use std::{
  process::Stdio,
  thread,
  time::Duration,
  io::{ self, Write, Read },
  sync::{ Arc, Mutex },
  thread::spawn
};
use serde_json::json;
use tokio::process::Command;

#[derive(serde::Deserialize)]
struct Response {
  response: String,
}

#[derive(serde::Deserialize)]
struct Config {
  url: String,
  model: String,
}

fn get_config() -> Result<(String, String), String> {
  let home_directory = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .unwrap_or_else(|_| "Unable to get your home dir!".to_string());

  let mut config_path = std::path::PathBuf::new();
  config_path.push(home_directory);
  config_path.push(".config");
  config_path.push("emiko");
  config_path.push("emiko");
  config_path.set_extension("json");

  if !std::path::Path::new(&config_path).exists() {
    println!("Create conf...");
    std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap()).unwrap();
    let cloned_config_path = config_path.clone();
    let mut file = std::fs::File::create(cloned_config_path).unwrap();

    let config_content = json!({
      "url": "http://localhost:11434/api/generate",
      "model": "llama3.2:latest"
    });

    let json_string = serde_json::to_string_pretty(&config_content).unwrap();

    file.write_all(json_string.as_bytes()).unwrap();
  }

  let mut file = std::fs::File::open(config_path).map_err(|e| e.to_string())?;
  let mut content = String::new();
  file.read_to_string(&mut content).map_err(|e| e.to_string())?;

  let config: Config = serde_json::from_str(&content).map_err(|e| e.to_string())?;

  Ok((config.url, config.model))
}

fn get_env() -> Result<(String, String, String), String> {
  Ok((
    std::env::consts::OS.to_string(),
    std::env::consts::ARCH.to_string(),
    std::env::var("SHELL").map_err(|e| format!("{e}"))?,
  ))
}

fn start_loading_effect(loading_active: Arc<Mutex<bool>>) {
  let spinner = vec![
    // "|", "/", "-", "\\"
    // "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
    // "⣾ ", "⣽ ", "⣻ ", "⢿ ", "⡿ ", "⣟ ", "⣯ ", "⣷ "
    // "⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"
    // "█", "▓", "▒", "░"
    // "∙∙∙", "●∙∙", "∙●∙", "∙∙●"
    // "🌍", "🌎", "🌏"
    // "🙈", "🙉", "🙊"
    // "▱▱▱", "▰▱▱", "▰▰▱", "▰▰▰", "▰▰▱", "▰▱▱", "▱▱▱",
    // "☱", "☲", "☴", "☲"
    // "", ".", "..", "..."
    "🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"
  ];
  let mut i = 0;

  while *loading_active.lock().unwrap() {
    print!("\rGenerate command... {}", spinner[i % spinner.len()]);
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(100));
    i += 1;
  }
}

fn stop_loading_effect(loading_active: &Arc<Mutex<bool>>) {
  let mut active = loading_active.lock().unwrap();
  *active = false;
  print!("\n\r");
}

pub async fn ask(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
  let loading_active = Arc::new(Mutex::new(true));
  let loading_active_clone = Arc::clone(&loading_active);

  let handle = spawn(move || {
    start_loading_effect(loading_active_clone);
  });

  let (url, model) = get_config().unwrap();
  let (os, arch, shell) = get_env().unwrap();

  let DEFAULT = format!(r#"
    You are programming and system administration assistant.
    Provide only commands without any description.
    If there is a lack of details, provide most logical solution.

    Provide short responses in about 100 words, unless you are specifically asked for more details.
    If you need to store any data, assume it will be stored in the conversation.

    Ensure the output is a valid shell command.
    This command will be automatically executed by a program.

    For example :
    Question: Write 'this is amazing!' in ~/Developer/test.txt file.
    Command: echo 'this is amazing!' > ~/Developer/test.txt

    You are running on {} with {} architecture and {} shell.
  "#,
  os, arch, shell);

  let TEMPLATE_CHAIN_OF_COMMANDS = r#"
    If you need to use multiple commands or if multiple steps required, try to combine them. Here's how.

    # Chaining Commands

    In many command-line interfaces, especially Unix-like systems, there are several characters that can be used to chain or manipulate commands.


    * `;` (Semicolon): Allows you to execute multiple commands sequentially.
    * `&&` (AND): Execute the second command only if the first command succeeds (returns a zero exit status).
    * `||` (OR): Execute the second command only if the first command fails (returns a non-zero exit status).
    * `&` (Background): Execute the command in the background, allowing the user to continue using the shell.
    * `|` (Pipe):  Takes the output of the first command and uses it as the input for the second command.

    ```powershell
    command1; command2   # Execute command1 and then command2
    command1 && command2 # Execute command2 only if command1 succeeds
    command1 || command2 # Execute command2 only if command1 fails
    command1 & command2  # Execute command1 in the background
    command1 | command2  # Pipe the output of command1 into command2
    ```
  "#;

  let ROLE_TEMPLATE = format!(
    "You are {}.\n{}",
    "emiko",
    "An assistant like J.A.R.V.I.S. in Iron Man."
  );

  let preprompt = format!(
    "{}\n\n{}\n\n{}",
    ROLE_TEMPLATE,
    DEFAULT,
    TEMPLATE_CHAIN_OF_COMMANDS
  );

  let body = serde_json::json!({
    "model": model,
    "prompt": format!("{}\n\n{}", preprompt, prompt),
    "stream": false
  });

  let res = reqwest::Client::new()
    .post(url)
    .json(&body)
    .send().await?;

  stop_loading_effect(&loading_active);
  handle.join().unwrap();

  match res.status().as_u16() {
    200 => {
      let answer: Response = res.json().await?;
      Ok(answer.response)
    },
    _ => {
      panic!("Error while calling ollama.");
    }
  }
}

pub fn extract_command(answer: String) -> String {
  // let re = Regex::new(r"`{3}([\w]*)\n([\S\s]+?)\n`{3}").unwrap();

  let re = regex::Regex::new(r"```(?:\w*\n)?([\S\s]+?)\n```").unwrap();

  if let Some(captures) = re.captures(&answer) {
    return captures.get(1).map_or("", |m| m.as_str()).to_string();
  }

  panic!("No command found sorry :/");
}

pub async fn execute(command: String) -> String {
  // dbg!(&command);
  let output = Command::new("sh")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
    .await
    .expect("Failed to execute command");

  match output.status.success() {
    true => {
      match String::from_utf8(output.stdout) {
        Ok(stdout_str) => {
          stdout_str
        }
        Err(_) => {
          let error_message = "Output is not valid UTF-8".to_string();
          error_message
        }
      }
    }
    false => {
      match String::from_utf8(output.stderr) {
        Ok(stderr_str) => {
          let error_message = format!("Error: {}", stderr_str);
          error_message
        }
        Err(_) => {
          let error_message = "Error output is not valid UTF-8".to_string();
          error_message
        }
      }
    }
  }
}

pub fn human_callback_handler(command: String) {
  print!("Do you want to execute the following command? [yes/no]\n\x1b[48;5;235m\x1b[91m{}\x1b[0m\n> ", command);
  std::io::stdout().flush().unwrap();

  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Failed to read line.");

  if input.to_lowercase().contains('n') || input.is_empty() || input.starts_with('\n') {
    panic!("Command execution aborted by user.");
  }
}

pub fn update_clipboard(command: String) {
  arboard::Clipboard::new().unwrap().set_text(command).unwrap();
  println!("Command copied to clipboard!");
}
