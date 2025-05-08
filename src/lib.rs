#![warn(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod logger;
pub mod memory;
pub mod rag;
pub mod args;

use crate::args::Opt;
// mod logger;
// mod memory;
use memory::memory::get_memory;
use std::{
  // env::args,
  io::{
    self,
    Read,
    Write
  },
  process::Stdio, sync::{
    Arc,
    Mutex
  },
  thread::{
    self,
    spawn
  },
  time::Duration
};
use serde_json::{ json, Value };
use tokio::process::Command;
use reqwest::header::{
  HeaderMap,
  HeaderValue,
  AUTHORIZATION
};

#[derive(serde::Deserialize)]
struct Response {
  message: Message,
}

#[derive(serde::Deserialize)]
struct Message {
  content: String,
}

#[derive(serde::Deserialize)]
struct Config {
  provider: String,
  url: String,
  model: String,
  api_key: Option<String>,
}

fn get_config(provider: Option<&str>) -> Result<(String, String, Option<String>), String> {
  logger::logger::log("INFO", "Retrieving configuration.");
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
    logger::logger::log("WARNING", "Create configuration file.");
    println!("Create conf...");
    std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap()).unwrap();
    let cloned_config_path = config_path.clone();
    let mut file = std::fs::File::create(cloned_config_path).unwrap();

    let config_content = json!({
      "ollama": {
        "provider": "ollama",
        "model": "qwen2.5-coder:14b",
        "url": "http://localhost:11434/api/generate"
      },
      "openai": {
        "provider": "openai",
        "model": "gpt-4o",
        "url": "https://api.openai.com/v1/chat/completions",
        "api_key": "YOUR_API_KEY"
      }
    });

    let json_string = serde_json::to_string_pretty(&config_content).unwrap();

    file.write_all(json_string.as_bytes()).unwrap();
  }

  let mut file = std::fs::File::open(config_path).map_err(|e| e.to_string())?;
  let mut content = String::new();
  file.read_to_string(&mut content).map_err(|e| e.to_string())?;

  let json_config: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

  let provider_name = provider.unwrap_or("ollama");

  if let Some(entry) = json_config.get(provider_name) {
    let config_entry: Config = serde_json::from_value(entry.clone()).map_err(|e| e.to_string())?;
    Ok((config_entry.url, config_entry.model, config_entry.api_key))
  } else {
    Err(format!("Provider '{}' not found in config", provider_name))
  }
}

fn get_env() -> Result<(String, String, String), String> {
  logger::logger::log("INFO", "Retrieving environment informations.");
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
    print!("\rThinking... {}", spinner[i % spinner.len()]);
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

pub async fn ask(args: &Opt) -> Result<String, Box<dyn std::error::Error>> {
  let prompt = &args.prompt.clone();
  let provider = &args.provider.clone();

  if args.debug {
    dbg!(prompt.clone());
    dbg!(provider.clone());
  }

  logger::logger::log("INFO", "Interrogating LLM.");
  let loading_active = Arc::new(Mutex::new(true));
  let loading_active_clone = Arc::clone(&loading_active);

  let handle = spawn(move || {
    start_loading_effect(loading_active_clone);
  });

  let (
    url,
    model,
    api_key,
  ) = get_config(provider.as_deref()).unwrap();

  if args.debug {
    dbg!(url.clone());
    dbg!(model.clone());
    dbg!(api_key.clone());
  }

  let (os, arch, shell) = get_env().unwrap();

  if args.debug {
    dbg!(os.clone());
    dbg!(arch.clone());
    dbg!(shell.clone());
  }

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
    "\n--- History of interactions ---\n{}--- End of history of interactions ---\n\n{}\n\n{}\n\n{}",
    get_memory(),
    ROLE_TEMPLATE,
    DEFAULT,
    TEMPLATE_CHAIN_OF_COMMANDS
  );

  let mut headers = HeaderMap::new();
  if let Some(api_key) = api_key {
    let auth_value = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value).unwrap());
  }

  let body = serde_json::json!({
    "model": model,
    "messages": [
      {
        "role": "user",
        "content": format!("{}\n\n{}", preprompt, prompt),
      }
    ],
    "stream": false
  });

  if args.debug {
    dbg!(headers.clone());
    dbg!(body.clone());
  }

  let res = reqwest::Client::new()
    .post(url)
    .headers(headers)
    .json(&body)
    .send().await?;

  stop_loading_effect(&loading_active);
  handle.join().unwrap();

  match res.status().as_u16() {
    200 => {
      logger::logger::log("INFO", "Successfully retrieve the response from LLM.");
      let answer: Response = res.json().await?;
      Ok(answer.message.content)
    },
    _ => {
      logger::logger::log("ERROR", "Fail to retrieve response from Ollama.");
      panic!("Error while calling ollama.");
    }
  }
}

pub fn extract_command(answer: String) -> String {
  // let re = Regex::new(r"`{3}([\w]*)\n([\S\s]+?)\n`{3}").unwrap();

  logger::logger::log("INFO", "Extracting command from LLM response.");
  let re = regex::Regex::new(r"```(?:\w*\n)?([\S\s]+?)\n```").unwrap();

  if let Some(captures) = re.captures(&answer) {
    logger::logger::log("INFO", "Command extracted.");
    return captures.get(1).map_or("", |m| m.as_str()).to_string();
  }

  logger::logger::log("ERROR", "No command found.");
  panic!("No command found sorry :/");
}

pub async fn execute(command: String) -> String {
  // dbg!(&command);
  logger::logger::log("INFO", "Running command.");
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
          logger::logger::log("INFO", "Command successfully executed.");
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
          logger::logger::log("ERROR", "An error occurred while running the command.");
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
  logger::logger::log("INFO", "Human callback handler");
  print!("Do you want to execute the following command? [yes/no]\n\x1b[48;5;235m\x1b[91m{}\x1b[0m\n> ", command);
  std::io::stdout().flush().unwrap();

  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Failed to read line.");

  if input.to_lowercase().contains('n') || input.is_empty() || input.starts_with('\n') {
    logger::logger::log("WARNING", "Command execution aborted by user.");
    panic!("Command execution aborted by user.");
  }
}

pub fn update_clipboard(command: String) {
  arboard::Clipboard::new().unwrap().set_text(command).unwrap();
  logger::logger::log("INFO", "Command has been copied to the clipboard.");
  println!("Command copied to clipboard!");
}
