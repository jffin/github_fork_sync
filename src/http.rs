use url::{Url};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use reqwest::{StatusCode};

const REQUEST_URL: &str = "https://api.github.com/repos";
const AUTH_HEADER: &str = "Authorization";
const ACCEPT_HEADER: &str = "Accept";
const ACCEPT_HEADER_VALUE: &str = "application/vnd.github.v3+json";
const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.90 Safari/537.36";


#[derive(Deserialize)]
struct JsonResult {
    html_url: String,
}

pub async fn make_request(target: &str, token: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = get_path(target);
    let url: String = String::from(format!("{}{}/forks", REQUEST_URL, path));

    let client = reqwest::Client::new();
    let response = client.post(&url)
        .header(USER_AGENT, SAFARI_USER_AGENT)
        .header(ACCEPT_HEADER, ACCEPT_HEADER_VALUE)
        .header(AUTH_HEADER, format!("token {}", token))
        .send().await?;

    let status: StatusCode = response.status();

    if status == StatusCode::ACCEPTED {
        let result: JsonResult = response.json::<JsonResult>().await?;
        println!("Success, your new url: {}", result.html_url);
    } else if status == StatusCode::FORBIDDEN || status == StatusCode::UNAUTHORIZED {
        println!("Your token is invalid");
    } else {
        println!("Something Went Wrong, check provided info {}", status);
    }

    Ok(())
}

fn get_path(url: &str) -> String {
    match Url::parse(url) {
        Ok(parsed) => parsed.path().to_string(),
        Err(_) => "Wrong GitHub url".to_string(),
    }
}
