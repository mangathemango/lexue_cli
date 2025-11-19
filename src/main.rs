
use clap::Parser;

use tokio;
mod cli;
use cli::*;
mod login;
use login::login;
mod fetch;
use fetch::fetch;
mod get;
use get::get;
mod submit;
use submit::submit;
mod set_cookie;
use set_cookie::set_cookie;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login  => login().await?,
        Commands::Fetch { id } => fetch(id).await?,
        Commands::Submit {} => submit().await?,
        Commands::Ping => {
            println!("Pinging Lexue Servers... ");
            let resp = get("https://lexue.bit.edu.cn/my/").await?;
            println!("Response: {}", resp.text().await?);
        },
        Commands::Get { url } => {
            let resp = get(url).await?;
            println!("Response: \n{}", resp.text().await?);
        },
        Commands::SetCookie { cookie } => {
            set_cookie(cookie)?;
            ()
        },
    }

    Ok(())
}
