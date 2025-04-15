use clap::Parser;

use clap::builder::{
  styling::{AnsiColor, Effects},
  Styles,
};


const STYLES: Styles = Styles::styled()
  .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
  .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
  .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
  .placeholder(AnsiColor::Cyan.on_default())
  .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
  .valid(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
  .invalid(AnsiColor::BrightRed.on_default().effects(Effects::BOLD));

#[derive(Debug, Parser)]
#[command(
  styles=STYLES,
  name = "emiko",
  version = env!("CARGO_PKG_VERSION"),
  about = "An assistant for your shell.",
  long_about = "Never Google a command again.\nEmiko is an assistant that can execute commands on your shell.\n\n\
  EXAMPLES:
    emiko -p 'List the content of ~/pwn folder.'
    emiko --force --prompt 'Convert `test` string in hex.'
    emiko -fp 'display hello world'"
)]
pub struct Opt {
  #[arg(
    short = 'p',
    long = "prompt",
    help = "Ask how to do something to Emiko."
  )]
  pub prompt: String,

  #[arg(
    short = 'f',
    long = "force",
    help = "Bypass the Human CallBack Handler."
  )]
  pub force: bool,

  #[arg(
    short = 'c',
    long = "clipboard",
    help = "Copy the generated command into the clipboard."
  )]
  pub clipboard: bool,

  #[arg(
    short = 'o',
    long = "provider",
    help = "Set a new provider for the next request."
  )]
  pub provider: Option<String>,

  #[arg(
    short = 'd',
    long = "debug",
    alias = "verbose",
    help = "Enable the debug mode."
  )]
  pub debug: bool,

  #[arg(
    short = 'r',
    long = "rag",
    help = "Enable the RAG mode."
  )]
  pub rag: bool,
}
