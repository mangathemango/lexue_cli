use anyhow::Result;
use cookie::Cookie;
use tiny_http::{Response, Server};
use url::Url;
use crate::set_cookie::set_cookie;

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
        let parsed = Cookie::parse(cookie_header.to_str()?)?;

        let cookie = parsed.value().to_string();

        println!(
            "Saved cookie {} at {}",
            cookie,
            set_cookie(cookie.as_str())?.to_str().unwrap()
        );
    }

    Ok(())
}
