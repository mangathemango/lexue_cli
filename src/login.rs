use anyhow::Result;
use dirs::home_dir;
use std::fs;

use cookie::Cookie;
use tiny_http::{Response, Server};
use url::Url;

pub fn wait_for_ticket() -> anyhow::Result<String> {
    // This might panic and i dont know how to handle Box<dyn Error>
    let server = Server::http("127.0.0.1:56666").unwrap();
    println!("Listening for CAS callback...");

    for request in server.incoming_requests() {
        let url = format!("http://localhost{}", request.url());

        let parsed = Url::parse(&url)?;
        let ticket = parsed
            .query_pairs()
            .find(|(k, _)| k == "ticket")
            .map(|(_, v)| v.to_string());

        let body = "Login successful! You can close this tab now.";
        request.respond(Response::from_string(body))?;

        if let Some(ticket) = ticket {
            println!("Got CAS ticket: {}", ticket);
            return Ok(ticket);
        } else {
            println!("Callback received but no ticket found.");
        }
    }

    Err(anyhow::anyhow!("Server stopped unexpectedly"))
}

// Save cookie to a file in user's home directory
fn save_cookie(cookie: &str) -> Result<std::path::PathBuf> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    fs::create_dir_all(&path)?;
    path.push("session.txt");
    fs::write(path.clone(), cookie)?;
    Ok(path)
}

pub async fn login(cookie: &String) -> Result<()> {
    let server_url = "http://127.0.0.1:56666/callback";

    let login_url = format!(
        "https://sso.bit.edu.cn/cas/login?service={}",
        urlencoding::encode(server_url)
    );

    webbrowser::open(&login_url)?;
    let ticket = wait_for_ticket()?;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = client
        .get(format!(
            "https://lexue.bit.edu.cn/login/index.php?ticket={}",
            ticket
        ))
        .send()
        .await?;

    if let Some(cookie_header) = resp.headers().get("set-cookie") {
        let set_cookie = cookie_header.to_str()?; // header value as &str
        let parsed = Cookie::parse(set_cookie)?;

        let cookie = parsed.value().to_string();

        println!(
            "Saved cookie {} at {}",
            cookie,
            save_cookie(cookie.as_str())?.to_str().unwrap()
        );
    }

    Ok(())
}
