#![allow(unused)]
#![warn(deprecated)]

use structopt::StructOpt;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = args::Opt::from_args();

  let res = emiko::ask(args.prompt).await?;
  // let command = emiko::extract_command(res);
  if !args.force {
    let res_clone = res.clone();
    emiko::human_callback_handler(res_clone);
  }
  let stdout = emiko::execute(res).await;
  println!("{}", stdout);
  Ok(())
}
