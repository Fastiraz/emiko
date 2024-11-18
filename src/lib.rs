#![allow(unused_imports)]

use std::{io::Stdin, process::Stdio};
use serde_json::json;
use tokio::process::Command;
use std::io::{self, Write, Read};

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

pub async fn ask(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
  let (url, model) = get_config().unwrap();

  let preprompt = r#"
    You are a shell assistant.
    Only return a single command and nothing else.
    This command will be automatically executed by a program.

    Context: {context}
    Question: {question}

    For example :
    Question: Write 'this is amazing!' in ~/Developer/test.txt file.
    Command: echo 'this is amazing!' > ~/Developer/test.txt
  "#;

  let body = serde_json::json!({
    "model": model,
    "prompt": format!("{}\n{}", preprompt, prompt),
    "stream": false
  });

  let res = reqwest::Client::new()
    .post(url)
    .json(&body)
    .send().await?;

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
  println!("Do you want to execute the following command? [yes/no]\n\x1b[48;5;235m\x1b[91m{}\x1b[0m", command);

  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Failed to read line.");

  if input.contains("n") {
    panic!("Command execution aborted by user.");
  }
}
