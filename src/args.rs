use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "emiko",
  about = "An assistant for your shell.",
  long_about = "Emiko is an assistant that can execute commands on your shell\n\n\
  EXAMPLES:
    emiko -p 'List the content of ~/pwn folder.'
    emiko --prompt 'Write `pwn` in file.txt'"
)]
pub struct Opt {
  #[structopt(
    short = "p",
    long = "prompt",
    help = "Ask how to make something to Emiko."
  )]
  pub prompt: String,
}

