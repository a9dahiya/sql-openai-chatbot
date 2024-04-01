use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct Choices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<Choices>,
}

#[derive(Serialize, Debug)]
struct RequestData {
    prompt: String,
    max_tokens: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let preamble = "Generate a Sql code for the given statement.";

    let token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", token);

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_input = String::new();

        stdin().read_line(&mut user_input).expect("Failed to read line");

        println!("");

        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into());

        let request_data = RequestData {
            prompt: format!("{} {}", preamble, user_input),
            max_tokens: 1000,
        };

        let body = Body::from(serde_json::to_vec(&request_data)?);

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        sp.stop();

        println!("");

        let json: Response = serde_json::from_reader(body.reader())?;

        println!("{}", json.choices[0].text);
    }

    Ok(())
}
