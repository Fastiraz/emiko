#![allow(unused_imports)]

#[derive(serde::Deserialize)]
struct Response {
  response: String,
}

pub async fn ask(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
  let url: &str = "http://localhost:11434/api/generate";
  let model: &str = "llama3.2:latest";

  let body = serde_json::json!({
    "model": model,
    "prompt": prompt,
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
      Ok("Error!".to_string())
    }
  }
}
