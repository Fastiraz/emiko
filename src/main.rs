#![allow(unused)]
#![warn(deprecated)]

use structopt::StructOpt;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = args::Opt::from_args();

  let res = emiko::ask(args.prompt).await?;

  if res.contains("```") {
    let res_clone = res.clone();
    let command = emiko::extract_command(res_clone);
  }

  if !args.force {
    let res_clone = res.clone();
    emiko::human_callback_handler(res_clone);
  }

  let stdout = emiko::execute(res).await;
  println!("{}", stdout);
  Ok(())
}
