use dirs::home_dir;
use std::fs;
use reqwest::Response;

pub async fn get(url: &str) -> anyhow::Result<Response> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    path.push("session.txt");
    let cookie = fs::read_to_string(path)?;
    let client = reqwest::Client::new();
    let resp: Response = client
        .get(url)
        .header("Cookie", format!("MoodleSession={}", cookie))
        .send()
        .await?;
    Ok(resp)
}