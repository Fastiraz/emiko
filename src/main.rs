#![allow(unused)]
#![warn(deprecated)]

use std::process::exit;

use structopt::StructOpt;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = args::Opt::from_args();
  let mut prompt = args.prompt.clone();
  let mut command = String::new();
  let mut stdout;

  loop {
    let res = emiko::ask(prompt.clone()).await?;

    if res.contains("```") {
      command = emiko::extract_command(res.clone());
    } else {
      command = res;
    }

    if !args.force {
      emiko::human_callback_handler(command.clone());
    }

    if args.clipboard {
      emiko::update_clipboard(command.clone());
      exit(0);
    }

    stdout = emiko::execute(command.clone()).await;
    println!("\n{}", stdout);

    if !stdout.starts_with("Error") {
      break;
    }

    prompt = format!(
      "Fix this error:\nInitial prompt:\n{}\nCommand:\n{}\n{}",
      args.prompt, command, stdout
    );
  }

  Ok(())
}
