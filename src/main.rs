#![allow(unused)]
#![warn(deprecated)]

use std::process::exit;

use structopt::StructOpt;
mod args;
use emiko::args::Opt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Opt::from_args();
  let mut prompt = args.prompt.clone();
  let mut command = String::new();
  let mut stdout;

  loop {
    let res = emiko::ask(&args).await?;

    if res.contains("```") {
      command = emiko::extract_command(res.clone());
    } else {
      command = res;
    }

    if args.debug {
      dbg!(command.clone());
    }

    if !args.force {
      if args.debug {
        dbg!(args.force.clone());
      }
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
