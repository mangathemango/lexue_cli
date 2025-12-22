use crate::{get, set_cookie::set_cookie};
use anyhow::Result;
use cookie::Cookie;
use reqwest::{cookie::{Jar, CookieStore}, header};
use std::sync::Arc;
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

pub async fn login() -> Result<()> {
    let server_url = "http://127.0.0.1:56666/callback";

    let login_url = format!(
        "https://sso.bit.edu.cn/cas/login?service={}",
        urlencoding::encode(server_url)
    );

    webbrowser::open(&login_url)?;
    let ticket = wait_for_ticket()?;

    let jar = Arc::new(Jar::default());

    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .build()?;

    client
        .get(format!(
            "https://lexue.bit.edu.cn/login/index.php?ticket={}",
            ticket
        ))
        .send()
        .await?;

    let resp = client.get("https://lexue.bit.edu.cn/my/").send().await?;

    let url = "https://lexue.bit.edu.cn/".parse::<reqwest::Url>()?;
    if let Some(cookie_str) = jar.cookies(&url) {
        let cookie_str = cookie_str.to_str()?;
        println!("All cookies: {}", cookie_str);
    }

    // 3️⃣ Extract MoodleSession from cookie jar
    let cookies = resp.headers().get_all(header::SET_COOKIE);

    for c in cookies {
        let parsed = Cookie::parse(c.to_str()?)?;
        println!("Found cookie: {:?}", parsed);
        if parsed.name() == "MoodleSession" {
            let cookie = parsed.value().to_string();
            println!(
                "Saved cookie {} at {}",
                cookie,
                set_cookie(cookie.as_str())?.to_str().unwrap()
            );
        }
    }

    println!("Verifying connection...");
    get("https://lexue.bit.edu.cn/my").await?;

    Ok(())
}
