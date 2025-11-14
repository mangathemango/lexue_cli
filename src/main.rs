
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login { cookie } => login(cookie)?,
        Commands::Fetch { id } => fetch(id).await?,
        Commands::Submit { path } => {
            println!("Submitting solution from path: {}", path);
            // later: zip and upload
        },
        Commands::Ping => {
            println!("Pinging Lexue Servers... ");
            let resp = get("https://lexue.bit.edu.cn/my/").await?;
            println!("Response: {}", resp.text().await?);
        },
        Commands::Get { url } => {
            let resp = get(url).await?;
            println!("Response: \n{}", resp.text().await?);
        }
    }

    Ok(())
}
