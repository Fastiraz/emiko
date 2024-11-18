#![allow(unused_imports)]

use regex::Regex;
use std::{io::Stdin, process::Stdio};
use tokio::process::Command;
use std::io::{self, Write};

#[derive(serde::Deserialize)]
struct Response {
  response: String,
}

pub async fn ask(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
  let url: &str = "http://localhost:11434/api/generate";
  let model: &str = "llama3.2:latest";

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
      // panic!();
      Ok("Error!".to_string())
    }
  }
}

/*pub fn extract_command(answer: String) -> String {
  let re = Regex::new(r"`{3}([\w]*)\n([\S\s]+?)\n`{3}").unwrap();

  if let Some(captures) = re.captures(&answer) {
    let command = captures.get(2).map_or("", |m| m.as_str()).to_string();
    return Some(command);
  }
}*/

pub async fn execute(command: String) -> String {
  dbg!(&command);
  let output = Command::new("sh")
    .arg("-c")
    .arg(command)
    // .stdout(Stdio::piped())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()
    .await
    .expect("Failed to execute command");

  match output.status.success() {
    true => {
      match String::from_utf8(output.stdout) {
        Ok(stdout_str) => {
          // println!("Output: {}", stdout_str);
          stdout_str
        }
        Err(_) => {
          let error_message = "Output is not valid UTF-8".to_string();
          // eprintln!("{}", error_message);
          error_message
        }
      }
    }
    false => {
      match String::from_utf8(output.stderr) {
        Ok(stderr_str) => {
          let error_message = format!("Error: {}", stderr_str);
          // eprintln!("{}", error_message);
          error_message
        }
        Err(_) => {
          let error_message = "Error output is not valid UTF-8".to_string();
          // eprintln!("{}", error_message);
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
