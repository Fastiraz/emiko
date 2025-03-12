use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "emiko",
  about = "An assistant for your shell.",
  long_about = "Never Google a command again.\nEmiko is an assistant that can execute commands on your shell.\n\n\
  EXAMPLES:
    emiko -p 'List the content of ~/pwn folder.'
    emiko --force --prompt 'Convert `test` string in hex.'
    emiko -fp 'display hello world'"
)]
pub struct Opt {
  #[structopt(
    short = "p",
    long = "prompt",
    help = "Ask how to do something to Emiko."
  )]
  pub prompt: String,

  #[structopt(
    short = "f",
    long = "force",
    help = "Bypass the Human CallBack Handler."
  )]
  pub force: bool,

  #[structopt(
    short = "c",
    long = "clipboard",
    help = "Copy the generated command into the clipboard."
  )]
  pub clipboard: bool,

  #[structopt(
    short = "r",
    long = "provider",
    help = "Set a new provider for the next request."
  )]
  pub provider: Option<String>,
}
