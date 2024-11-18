#![allow(unused)]
#![warn(deprecated)]

use structopt::StructOpt;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = args::Opt::from_args();
  match emiko::ask(args.prompt).await {
    Ok(response) => println!("{}", response),
    Err(e) => eprintln!("Error: {}", e),
  }
  Ok(())
}

/*#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = args::Opt::from_args();
  let prompt = args.prompt;

  let client = reqwest::Client::builder()
    .build()?;

  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

  let mut params = std::collections::HashMap::new();
  // params.insert("{\"model\": \"llama3.2:latest\",\"prompt\":\"Why is the sky blue?\"}", "");

  params.insert("model", "llama3.2:latest");
  params.insert("prompt", &prompt);

  let request = client.request(reqwest::Method::POST, "http://localhost:11434/api/generate")
    .headers(headers)
    .form(&params);

  let response = request.send().await?;
  let body = response.text().await?;

  // dbg!(body);
  println!("{}", body);

  Ok(())
}*/
