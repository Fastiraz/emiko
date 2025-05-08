#![allow(unused)]
#![warn(deprecated)]

use std::process::exit;
use clap::Parser;
mod args;
use emiko::args::Opt;
use rag::rag::RAG;
mod rag;
// use memory::memory::learn;
mod memory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut args = Opt::parse();
  let mut prompt = args.prompt.clone();
  let mut command = String::new();
  let mut stdout;

  // if args.rag {
  //   let mut rag: RAG = RAG::new();
  //   let documents = rag.loader(true);
  //   let embeddings = rag.embeddings().await?;
  //   args.prompt = format!(r#"
  //     Using this data: {:?}. Respond to this prompt: {}
  //   "#,
  //   embeddings, prompt);
  // }

  if args.question {
    prompt = format!(
      "[QUESTION MODE]\n\
      You are in question mode.\n\
      Only respond to that user's question.\n\
      In this mode, you are a simple chatbot and you don't generate commands.\n\
      Answer short and simple sentences.
      {}",
      prompt
    );
    args.prompt = prompt.clone();
    let res = emiko::ask(&args).await?;
    println!("{}", res);
    return Ok(());
  }

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
      memory::memory::learn(args.prompt.clone(), command, stdout);
      break;
    }

    prompt = format!(
      "Fix this error:\nInitial prompt:\n{}\nCommand:\n{}\n{}",
      args.prompt, command, stdout
    );
    memory::memory::learn(args.prompt.clone(), command, stdout);
  }

  return Ok(())
}
