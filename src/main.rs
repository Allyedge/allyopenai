use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize)]
struct OpenAIChoices {
    text: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoices>,
}

#[derive(Serialize)]
struct OpenAIRequest {
    prompt: String,
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let https = HttpsConnector::new();

    let client = Client::builder().build(https);

    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let open_ai_token: String = env::var("OPEN_AI_TOKEN").unwrap();

    let auth_header_value = format!("Bearer {}", open_ai_token);

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read the line.");

        let open_ai_request = OpenAIRequest {
            prompt: user_text,
            max_tokens: 100,
        };

        let body = Body::from(serde_json::to_vec(&open_ai_request)?);

        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_value)
            .body(body)
            .unwrap();

        let result = client.request(request).await?;

        let body = hyper::body::aggregate(result).await?;

        let json: OpenAIResponse = serde_json::from_reader(body.reader())?;

        println!("{}", json.choices[0].text);
    }
}
