use serde::Deserialize;
use std::fs;
use anyhow::Result;
use reqwest::Response;
use dirs::home_dir;
use std::collections::HashMap;
use crate::get::get;
use scraper::{Selector,Html};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize)]
struct Config {
    exercise_id: String,
    lexue_cli_version: String,
}

fn load_config() -> Result<Config> {
    let contents = fs::read_to_string("lexue.toml")?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

pub async fn submit() -> Result<()> {
    let config = load_config()?;
    
    let url = &format!("https://lexue.bit.edu.cn/mod/programming/result.php?id={}&lang=en",config.exercise_id);
    println!("Sending get request to {} for sesskey... ", url);
    let resp = get(url).await?;
    let html = resp.text().await?;
    let document = Html::parse_document(&html);
    let sesskey = document
        .select(&Selector::parse("input[name=sesskey]").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap();
    println!("Sesskey obtained: {}", sesskey);

    // Building form data
    let code = fs::read_to_string("main.c")?;
    let mut form = HashMap::new();
    form.insert("id", config.exercise_id.as_str());
    form.insert("sesskey", sesskey);
    form.insert("_qf__submit_form", "1");
    form.insert("code", &code);
    form.insert("language", "9");
    form.insert("submitbutton", "保存更改");
    
    // Getting cookie
    let client = reqwest::Client::new();
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    path.push("session.txt");
    let cookie = fs::read_to_string(path)?;

    println!("Sending post request to https://lexue.bit.edu.cn/mod/programming/submit.php");
    // Building POST request + send
    let resp: Response = client
        .post("https://lexue.bit.edu.cn/mod/programming/submit.php")
        .header("Cookie", format!("MoodleSession={}", cookie))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()
        .await?;
    println!("Server has responded with status code: {}", resp.status());
    println!("Code has been submitted! Awaiting result...");
    
    let selector = Selector::parse("#test-result-detail").unwrap();
    // compilemessage
    loop {
        let resp = client
            .get(url)
            .header("Cookie", format!("MoodleSession={}", cookie))
            .send()
            .await?;
        let body = resp.text().await?;
        let document = Html::parse_document(&body);

        if let Some(result) = document.select(&selector).next() {
            
            let text = result.text().collect::<String>().trim().to_string();
            println!("Compile Message:");
            println!("{}", document.select(&Selector::parse(".compilemessage").unwrap()).next().unwrap().text().collect::<String>().trim().to_string());
            println!("Judge Result:");
            println!("{}", text);
            break;
        }

        println!("Results not ready, waiting 3s...");
        sleep(Duration::from_secs(3)).await; // wait a bit before retrying
    }
    Ok(())
}