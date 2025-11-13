use anyhow::Result;
use clap::{Parser, Subcommand};
use dirs::home_dir;
use std::fs;
use tokio;

#[derive(Parser)]
#[command(name = "lexue-cli", version, about = "Lexue Automation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Log into your Lexue account manually
    Login {
        /// The cookie value (MoodleSession)
        cookie: String,
    },
    /// Fetch programming exercises
    Fetch {
        #[arg(short, long)]
        course: String,
    },
    /// Submit your finished exercise
    Submit {
        #[arg(short, long)]
        path: String,
    },
    /// Ping the https://lexue.bit.edu.cn/my/ server
    Ping,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login { cookie } => {
            println!(
                "Saved cookie {} at {}",
                cookie,
                save_cookie(cookie)?.to_str().unwrap()
            );
        }
        Commands::Fetch { course } => {
            println!("Fetching assignments for course: {}", course);
            // later: download problem statements, setup folder
        }
        Commands::Submit { path } => {
            println!("Submitting solution from path: {}", path);
            // later: zip and upload
        }
        Commands::Ping => {
            println!("Pinging Lexue Servers... ");
            let mut path = home_dir().expect("Could not find home directory");
            path.push(".lexue_cli");
            path.push("session.txt");
            let cookie = fs::read_to_string(path)?;
            let client = reqwest::Client::new();
            let resp = client
                .get("https://lexue.bit.edu.cn/my/")
                .header("Cookie", format!("MoodleSession={}", cookie))
                .send()
                .await?;
            println!("Response: {}", resp.text().await?);
        }
    }

    Ok(())
}
