use dirs::home_dir;
use reqwest::Response;
use std::fs;

pub async fn get(url: &str) -> anyhow::Result<Response> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    path.push("session.txt");
    let cookie = fs::read_to_string(path)?;
    let client = reqwest::Client::new();
    let maybe_resp = client
        .get(url)
        .header("Cookie", format!("MoodleSession={}", cookie))
        .send()
        .await;
    match maybe_resp {
        Ok(resp) => {
            Ok(resp)
        },
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("too many redirects") {
                eprintln!("{}",e.to_string());
                eprintln!("Session expired! Please run `lexue-cli login` or `lexue-cli set-cookie  <cookie>` again.");
                std::process::exit(1);
            }

            return Err(e.into());
        }
    }
}
